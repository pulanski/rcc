window.SIDEBAR_ITEMS = {
    constant: [["COMMANDS", ""]],
    enum: [
        ["Error", "Error enum for parsing errors."],
        ["Line", ""],
    ],
    fn: [
        ["is_word_char", ""],
        ["process_buf", "Process a generic BufRead."],
        ["process_define", ""],
        ["process_elifdef", ""],
        ["process_else", ""],
        ["process_endif", ""],
        ["process_endin", ""],
        ["process_exec", ""],
        ["process_file", "Process a file."],
        ["process_ifdef", ""],
        ["process_in", ""],
        ["process_include", ""],
        ["process_line", "Process a string line of input."],
        ["process_str", "Process a multi-line string of text."],
        ["process_undef", ""],
        [
            "replace_next_macro",
            "Finds the next macro name word in the line, and replaces it with its value, returning None when it can’t find a macro.",
        ],
        ["shell", ""],
    ],
    struct: [
        ["Command", ""],
        ["Context", "Context of the current processing."],
    ],
};
