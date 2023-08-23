#[cfg(test)]
mod tests;

use anyhow::Result;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{
  Child, Command as SystemCommand, ExitStatus, Stdio,
};
use std::string::FromUtf8Error;

/// Context of the current processing.
///
/// Contains a set of currently defined macros, as well as the number of nested if statements that
/// are being ignored; this is so that if the parser failed an if statement, and it is currently
/// ignoring data, it knows how many endifs it needs to encounter before resuming reading data
/// again. Only if this value is 0 then the parser will read data. It also stores whether the
/// current if group has been accepted; this is for if groups with over three parts.
///
/// There are no limits on what variable names can be; by directly altering Context::macros, you
/// can set variable names not possible with #defines. However, when replacing variable names in
/// text the variable name must be surrounded by two characters that are **not** alphanumeric or an
/// underscore.
#[derive(Debug, Default)]
pub struct Context {
  /// Map of all currently defined macros.
  pub macros: HashMap<String, String>,
  /// Number of layers of inactive if statements.
  pub inactive_stack: u32,
  /// Whether the current if statement has been accepted.
  pub used_if: bool,
  /// Whether #exec and #in commands are allowed.
  pub allow_exec: bool,
  /// The stack of processes that #in is piping to.
  pub in_stack: Vec<Child>,
}

impl Context {
  /// Create a new empty context with no macros or inactive stack and exec commands disallowed.
  pub fn new() -> Self {
    Self::default()
  }
  /// Create a new empty context with no macros or inactive stack and exec commands allowed.
  pub fn new_exec() -> Self {
    Self::new().exec(true)
  }
  /// Create a context from a map of macros.
  pub fn from_macros(
    macros: impl Into<HashMap<String, String>>,
  ) -> Self {
    Self { macros: macros.into(), ..Default::default() }
  }
  /// Create a context from an iterator over tuples.
  pub fn from_macros_iter(
    macros: impl IntoIterator<Item = (String, String)>,
  ) -> Self {
    Self::from_macros(
      macros.into_iter().collect::<HashMap<_, _>>(),
    )
  }
  /// Set whther exec commands are allowed.
  pub fn exec(mut self, allow_exec: bool) -> Self {
    self.allow_exec = allow_exec;
    self
  }
}

/// Error enum for parsing errors.
///
/// # Examples
///
/// ```
/// let error = gpp::Error::TooManyParameters { command: "my_command" };
/// assert_eq!(format!("{}", error), "Too many parameters for #my_command");
/// ```
/// ```
/// let error = gpp::Error::FileError {
///     filename: "my_file".to_string(),
///     line: 10,
///     error: Box::new(gpp::Error::UnexpectedCommand {
///         command: "this_command",
///     }),
/// };
/// assert_eq!(format!("{}", error), "Error in my_file:10: Unexpected command #this_command");
/// ```
#[derive(Debug)]
pub enum Error {
  /// An unknown command was encountered.
  InvalidCommand {
    command_name: String,
  },
  /// Too many parameters were given for a command (for example using #endif with parameters).
  TooManyParameters {
    command: &'static str,
  },
  /// There was an unexpected command; currently only generated for unexpected #endins.
  UnexpectedCommand {
    command: &'static str,
  },
  /// The child process for an #exec exited with a nonzero status.
  ChildFailed {
    status: ExitStatus,
  },
  /// A pipe was unable to be set up to the child.
  PipeFailed,
  /// An error with I/O occurred.
  IoError(io::Error),
  /// An error occurred parsing a child's standard output as UTF-8.
  FromUtf8Error(FromUtf8Error),
  /// An error occurred in another file.
  FileError {
    filename: String,
    line: usize,
    error: Box<anyhow::Error>,
  },
  MacroNotFound {
    macro_name: String,
  },
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::InvalidCommand { command_name } => {
        write!(f, "Invalid command '{}'", command_name)
      }
      Error::TooManyParameters { command } => {
        write!(f, "Too many parameters for #{}", command)
      }
      Error::UnexpectedCommand { command } => {
        write!(f, "Unexpected command #{}", command)
      }
      Error::ChildFailed { status } => {
        write!(f, "Child failed with exit code {}", status)
      }
      Error::PipeFailed => write!(f, "Pipe to child failed"),
      Error::IoError(e) => write!(f, "I/O Error: {}", e),
      Error::FromUtf8Error(e) => write!(f, "UTF-8 Error: {}", e),
      Error::FileError { filename, line, error } => {
        write!(f, "Error in {}:{}: {}", filename, line, error)
      }
      Error::MacroNotFound { macro_name } => {
        write!(f, "Macro not found: {}", macro_name)
      }
    }
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::IoError(e) => Some(e),
      Error::FromUtf8Error(e) => Some(e),
      Error::FileError { error: e, .. } => None,
      // FIXME: This doesn't work because of the Box
      //   Error::FileError { error: e, .. } => Some(e),
      _ => None,
    }
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Error::IoError(e)
  }
}

impl From<FromUtf8Error> for Error {
  fn from(e: FromUtf8Error) -> Self {
    Error::FromUtf8Error(e)
  }
}

fn shell(cmd: &str) -> SystemCommand {
  let (shell, flag) = if cfg!(target_os = "windows") {
    ("cmd", "/C")
  } else {
    ("/bin/sh", "-c")
  };
  let mut command = SystemCommand::new(shell);
  command.args(&[flag, cmd]);
  command
}

fn process_exec(line: &str, _: &mut Context) -> Result<String> {
  let output = shell(line).output()?;
  if !output.status.success() {
    return Err(
      Error::ChildFailed { status: output.status }.into(),
    );
  }
  Ok(String::from_utf8(output.stdout)?)
}

fn process_in(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  let child = shell(line)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
  context.in_stack.push(child);
  Ok(String::new())
}

fn process_endin(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  if !line.is_empty() {
    return Err(
      Error::TooManyParameters { command: "endin" }.into(),
    );
  }
  if context.in_stack.is_empty() {
    return Err(
      Error::UnexpectedCommand { command: "endin" }.into(),
    );
  }
  let child = context.in_stack.pop().unwrap();
  let output = child.wait_with_output()?;
  if !output.status.success() {
    return Err(
      Error::ChildFailed { status: output.status }.into(),
    );
  }
  Ok(String::from_utf8(output.stdout)?)
}

fn process_include(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  process_file(line, context)
}

fn process_define(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  let mut parts = line.splitn(2, ' ');
  let name = parts.next().unwrap();
  let value = parts.next().unwrap_or("");

  context.macros.insert(name.to_owned(), value.to_owned());
  Ok(String::new())
}

fn process_undef(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  context.macros.remove(line);
  Ok(String::new())
}

fn process_ifdef(
  line: &str,
  context: &mut Context,
  inverted: bool,
) -> Result<String> {
  if context.inactive_stack > 0 {
    context.inactive_stack += 1;
  } else if context.macros.contains_key(line) == inverted {
    context.inactive_stack = 1;
    context.used_if = false;
  } else {
    context.used_if = true;
  }
  Ok(String::new())
}

fn process_elifdef(
  line: &str,
  context: &mut Context,
  inverted: bool,
) -> Result<String> {
  if context.inactive_stack == 0 {
    context.inactive_stack = 1;
  } else if context.inactive_stack == 1
    && !context.used_if
    && context.macros.contains_key(line) != inverted
  {
    context.inactive_stack = 0;
  }
  Ok(String::new())
}

fn process_else(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  if !line.is_empty() {
    return Err(
      Error::TooManyParameters { command: "else" }.into(),
    );
  }
  context.inactive_stack = match context.inactive_stack {
    0 => 1,
    1 if !context.used_if => 0,
    val => val,
  };
  Ok(String::new())
}

fn process_endif(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  if !line.is_empty() {
    return Err(
      Error::TooManyParameters { command: "endif" }.into(),
    );
  }
  if context.inactive_stack != 0 {
    context.inactive_stack -= 1;
  }
  Ok(String::new())
}

#[derive(Clone, Copy)]
struct Command {
  name: &'static str,
  requires_exec: bool,
  ignored_by_if: bool,
  supports_variadic: bool,
  execute: fn(&str, &mut Context) -> Result<String>,
}

const COMMANDS: &[Command] = &[
  Command {
    name: "exec",
    requires_exec: true,
    ignored_by_if: false,
    supports_variadic: false,
    execute: process_exec,
  },
  Command {
    name: "in",
    requires_exec: true,
    ignored_by_if: false,
    supports_variadic: false,
    execute: process_in,
  },
  Command {
    name: "endin",
    requires_exec: true,
    ignored_by_if: false,
    supports_variadic: false,
    execute: process_endin,
  },
  Command {
    name: "include",
    requires_exec: false,
    ignored_by_if: false,
    supports_variadic: false,
    execute: process_include,
  },
  Command {
    name: "define",
    requires_exec: false,
    ignored_by_if: false,
    supports_variadic: true,
    execute: process_define,
  },
  Command {
    name: "undef",
    requires_exec: false,
    ignored_by_if: false,
    supports_variadic: false,
    execute: process_undef,
  },
  Command {
    name: "ifdef",
    requires_exec: false,
    ignored_by_if: true,
    supports_variadic: false,
    execute: |line, context| process_ifdef(line, context, false),
  },
  Command {
    name: "ifndef",
    requires_exec: false,
    ignored_by_if: true,
    supports_variadic: false,
    execute: |line, context| process_ifdef(line, context, true),
  },
  Command {
    name: "elifdef",
    requires_exec: false,
    ignored_by_if: true,
    supports_variadic: false,
    execute: |line, context| {
      process_elifdef(line, context, false)
    },
  },
  Command {
    name: "elifndef",
    requires_exec: false,
    ignored_by_if: true,
    supports_variadic: false,
    execute: |line, context| {
      process_elifdef(line, context, true)
    },
  },
  Command {
    name: "else",
    requires_exec: false,
    ignored_by_if: true,
    supports_variadic: false,
    execute: process_else,
  },
  Command {
    name: "endif",
    requires_exec: false,
    ignored_by_if: true,
    supports_variadic: false,
    execute: process_endif,
  },
];

fn is_word_char(c: char) -> bool {
  c.is_alphanumeric() || c == '_'
}

/// Finds the next macro name word in the line, and replaces it with its value, returning None when
/// it can't find a macro.
fn replace_next_macro(
  line: &str,
  macros: &HashMap<String, String>,
) -> Option<String> {
  macros.iter().find_map(|(name, value)| {
    let mut parts = line.splitn(2, name);
    let before = parts.next().unwrap();
    let after = parts.next()?;

    if before.chars().next_back().map_or(false, is_word_char)
      || after.chars().next().map_or(false, is_word_char)
    {
      return None;
    }
    let mut new_line = String::with_capacity(
      before.len() + value.len() + after.len(),
    );
    new_line.push_str(before);
    new_line.push_str(value);
    new_line.push_str(after);
    Some(new_line)
  })
}

// fn process_variadic_macro(
//   line: &str,
//   context: &mut Context,
// ) -> Result<String> {
// Identify the macro and capture arguments
// Process and expand the macro with the captured arguments
// Return the expanded content

// #define FOO(x, y, ...) x + y + __VA_ARGS__
// FOO(1, 2, 3, 4, 5)
// 1 + 2 + 3, 4, 5
// 1 + 2 + 3 + 4 + 5

//   TODO: Add support for variadic macros

//   Ok(String::new())
// }
fn process_variadic_macro(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  // Split the line to extract the macro name and its arguments
  let mut parts = line.trim().splitn(2, ' ');
  let macro_name = parts.next().unwrap();
  let macro_args = parts.next().unwrap_or("").trim();

  // Check if the macro is defined
  if let Some(macro_value) = context.macros.get(macro_name) {
    let mut expanded_macro = macro_value.clone();

    // Split the macro arguments
    let args = macro_args.split(',').map(|arg| arg.trim());

    // Create a vector to hold expanded arguments
    let mut expanded_args = Vec::new();

    // Process each argument
    for arg in args {
      // Check if the argument is another macro and expand it recursively
      let expanded_arg = process_line(arg, context)?;

      // Add the expanded argument to the vector
      expanded_args.push(expanded_arg);
    }

    // Combine the expanded arguments into a single string
    let combined_args = expanded_args.join(", ");

    // Replace the macro arguments in the expanded_macro
    expanded_macro =
      expanded_macro.replace("__VA_ARGS__", &combined_args);

    // Process the expanded_macro
    process_line(&expanded_macro, context)
  } else {
    Err(
      Error::InvalidCommand {
        command_name: macro_name.to_owned(),
      }
      .into(),
    )
  }
}

/// Process a string line of input.
///
/// This is the smallest processing function, and all other processing functions are wrappers
/// around it. It only processes singular lines, and will not work on any string that contains
/// newlines unless that newline is at the end.
///
/// NOTE: If the input did not contain a newline at the end, then this function will add it.
///
/// # Examples
///
/// ```rust
/// let mut context = gpp::Context::new();
/// context.macros.insert("Foo".to_string(), "Two".to_string());
///
/// assert_eq!(gpp::process_line("One Foo Three", &mut context).unwrap(), "One Two Three\n");
/// ```
/// ```
/// let mut context = gpp::Context::new();
///
/// assert_eq!(gpp::process_line("#define Foo Bar", &mut context).unwrap(), "");
/// assert_eq!(context.macros.get("Foo").unwrap(), "Bar");
/// ```
pub fn process_line(
  line: &str,
  context: &mut Context,
) -> Result<String> {
  // Skip lines starting with "//"
  if line.starts_with("//") || line.is_empty() {
    return Ok(String::new());
  }

  let line = line
    .strip_suffix("\r\n")
    .or_else(|| line.strip_suffix('\n'))
    .unwrap_or(line);

  enum Line<'a> {
    Text(&'a str),
    Command(Command, &'a str),
  }

  let line = if let Some(rest) = line.strip_prefix('#') {
    if rest.starts_with('#') {
      Line::Text(rest)
    } else {
      let mut parts = rest.trim_start().splitn(2, ' ');
      let command_name = parts.next().unwrap();
      let content = parts.next().unwrap_or("").trim_start();
      //   println!("command_name: {command_name}");
      //   println!("content: {content}");

      if let Some(command) = COMMANDS
        .iter()
        .copied()
        .filter(|command| {
          context.allow_exec || !command.requires_exec
        })
        .find(|command| command.name == command_name)
      {
        // Check if it's a variadic macro and parse its arguments
        if let Some((macro_name, args)) =
          parse_variadic_macro(content)
        {
          println!("variadic macro: {macro_name} {args:#?}");
          println!("command_name: {command_name}");

          // Handle variadic macros
          if macro_name == command_name {
            if let Some(macro_value) =
              context.macros.get(macro_name)
            {
              let mut expanded_macro = macro_value.clone();

              // Find the position of __VA_ARGS__ in the macro value
              while let Some(args_pos) =
                expanded_macro.find("__VA_ARGS__")
              {
                // Replace __VA_ARGS__ with the actual arguments
                let mut new_macro = String::new();
                new_macro.push_str(&expanded_macro[..args_pos]);
                new_macro.push_str(args.join(", ").as_str());
                new_macro
                  .push_str(&expanded_macro[args_pos + 10..]); // 10 is the length of "__VA_ARGS__"
                expanded_macro = new_macro.clone();

                println!("new_macro: {new_macro}");
              }

              // TODO:Need to handle case such as:
              // #define FOO(x) x
              // #define BAR(x,y) FOO(x) + y

              println!("expanded_macro: {expanded_macro}");

              // Process the expanded_macro recursively
              return process_line(&expanded_macro, context);
            } else {
              return Err(
                Error::MacroNotFound {
                  macro_name: macro_name.to_owned(),
                }
                .into(),
              );
            }

            // // For now, we'll just return the original line
            // return Ok(line.to_owned());
          }
        }

        Line::Command(command, content)
      } else {
        return Err(
          Error::InvalidCommand {
            command_name: command_name.to_owned(),
          }
          .into(),
        );
      }
    }
  } else {
    Line::Text(line)
  };

  let line = match line {
    Line::Text(_)
    | Line::Command(Command { ignored_by_if: false, .. }, _)
      if context.inactive_stack > 0 =>
    {
      String::new()
    }
    Line::Text(text) => {
      let mut line = format!("{text}\n");

      while let Some(s) =
        replace_next_macro(&line, &context.macros)
      {
        line = s;
      }

      line
    }
    Line::Command(command, content) => {
      (command.execute)(content, context)?
    }
  };

  Ok(if let Some(child) = context.in_stack.last_mut() {
    let input = child.stdin.as_mut().ok_or(Error::PipeFailed)?;
    input.write_all(line.as_bytes())?;
    String::new()
  } else {
    line
  })
}

/// Process a multi-line string of text.
///
/// See `process_buf` for more details.
///
/// # Examples
///
/// ```
/// assert_eq!(gpp::process_str("#define A 1\n A 2 3 \n", &mut gpp::Context::new()).unwrap(), " 1 2 3 \n");
/// ```
pub fn process_str(
  s: &str,
  context: &mut Context,
) -> Result<String> {
  process_buf(s.as_bytes(), "<string>", context)
}

/// Process a file.
///
/// See `process_buf` for more details.
pub fn process_file(
  filename: &str,
  context: &mut Context,
) -> Result<String> {
  println!("process_file: {filename}");
  println!("cwd: {:?}", std::env::current_dir()?);

  // concatenate the current directory and the filename
  let canonical = std::env::current_dir()?.join(filename);
  println!("canonical: {canonical:?}");

  let file_raw = File::open(filename)?;
  let file = BufReader::new(file_raw);

  process_buf(file, filename, context)
}

/// Process a generic BufRead.
///
/// This function is a wrapper around `process_line`. It splits up the input into lines (adding a
/// newline on the end if there isn't one) and then processes each line.
pub fn process_buf<T: BufRead>(
  buf: T,
  buf_name: &str,
  context: &mut Context,
) -> Result<String> {
  buf
    .lines()
    .enumerate()
    .map(|(num, line)| {
      Ok({
        process_line(&line?, context).map_err(|e| {
          Error::FileError {
            filename: String::from(buf_name),
            line: num,
            error: Box::new(e),
          }
        })?
      })
    })
    .collect()
}

// Function to parse variadic macro arguments
fn parse_variadic_macro(
  content: &str,
) -> Option<(&str, Vec<&str>)> {
  // Parse the macro definition format: "#define MACRO(args) ..."
  let re =
    regex::Regex::new(r"^\s*(\w+)\s*\(([^)]*)\)").unwrap();

  println!("parsing variadic macro: {content}");

  if let Some(captures) = re.captures(content) {
    // println!("captures: {captures:#?}");
    let macro_name = captures.get(1).unwrap().as_str();
    let args = captures
      .get(2)
      .unwrap()
      .as_str()
      .split(',')
      .map(str::trim)
      .collect();
    println!("(macro_name, args): ({macro_name}, {args:#?})");

    Some((macro_name, args))
  } else {
    None
  }
}
