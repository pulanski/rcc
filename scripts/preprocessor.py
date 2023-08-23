import re

def tokenize_cpp(code):
    # Define regular expressions for common C preprocessor tokens
    token_patterns = [
        (r'#\s*include\s*[<"].*?[>"]', 'PREPROCESSOR_DIRECTIVE'),  # Include directive
        (r'#\s*define\s+\w+\s+.*', 'PREPROCESSOR_DIRECTIVE'),      # Define directive
        (r'#\s*ifdef\s+\w+', 'PREPROCESSOR_DIRECTIVE'),            # Ifdef directive
        (r'#\s*ifndef\s+\w+', 'PREPROCESSOR_DIRECTIVE'),           # Ifndef directive
        (r'#\s*else', 'PREPROCESSOR_DIRECTIVE'),                   # Else directive
        (r'#\s*endif', 'PREPROCESSOR_DIRECTIVE'),                  # Endif directive
        (r'#\s*\w+', 'PREPROCESSOR_DIRECTIVE'),                    # Other preprocessor directives
        (r'//.*', 'COMMENT'),                                     # Single-line comment
        (r'/\*.*?\*/', 'COMMENT'),                                # Multi-line comment
        (r'\w+', 'IDENTIFIER'),                                   # Identifiers
        (r'\d+', 'NUMBER'),                                       # Numbers
        (r'"(.*?)"', 'STRING_LITERAL'),                           # String literals
        (r'\s+', 'WHITESPACE'),                                   # Whitespace
        (r'.', 'UNKNOWN')                                         # Any other character
    ]

    tokens = []

    while code:
        for pattern, token_type in token_patterns:
            match = re.match(pattern, code)
            if match:
                token = match.group(0)
                code = code[len(token):]
                if token_type != 'WHITESPACE':
                    tokens.append((token, token_type))
                break
        else:
            raise ValueError(f"Unable to tokenize code: {code}")

    return tokens

# Example usage:
cpp_code = """
#include <stdio.h>
#define MAX 100
int main() {
    printf("Hello, World!\n");
    return 0;
}
"""

tokens = tokenize_cpp(cpp_code)

for token, token_type in tokens:
    print(f"Token: {token}, Type: {token_type}")
