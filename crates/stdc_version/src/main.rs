use std::env;
use std::fs::File;
use std::io::{
    self,
    Write,
};
use std::process::{
    Command,
    Output,
};

fn create_temp_c_file(c_code: &str) -> io::Result<()> {
    let mut file = File::create("temp.c")?;
    file.write_all(c_code.as_bytes())?;
    Ok(())
}

fn preprocess_c_code_with_clang() -> io::Result<String> {
    create_temp_c_file("__STDC_VERSION__")?;

    let output = Command::new("clang").arg("-E").arg("temp.c").arg("-o").arg("temp.out").output();

    let output = match output {
        Ok(output) => output,
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    };

    // Remove any lines that start with a hash sign
    let output = std::fs::read_to_string("temp.out")?;
    let output =
        output.lines().filter(|line| !line.starts_with('#')).collect::<Vec<_>>().join("\n");

    // Cleanup
    std::fs::remove_file("temp.c")?;
    std::fs::remove_file("temp.out")?;

    Ok(output)
}

fn current_stdc_version() -> Option<String> {
    match preprocess_c_code_with_clang() {
        Ok(output) => Some(output),
        _ => None,
    }
}

fn main() {
    if let Some(version) = current_stdc_version() {
        println!("C Standard version: {}", version);
    } else {
        println!("Unable to determine C Standard version");
    }
}
