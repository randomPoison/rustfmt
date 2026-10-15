// rustfmt-file_lines: [{"file":"tests/source/file-lines/closing-brace-comment-group-2024.rs","range":[5,5]}]
// rustfmt-style_edition: 2024

fn main() {
    let selected = 0;
    let untouched = 1; // Trailing comment grouped with the standalone comment below.
    // Preserve this standalone comment's indentation.
}
