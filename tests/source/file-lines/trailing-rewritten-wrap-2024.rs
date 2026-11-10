//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-rewritten-wrap-2024.rs","range":[9,9]},{"file":"tests/source/file-lines/trailing-rewritten-wrap-2024.rs","range":[16,16]},{"file":"tests/source/file-lines/trailing-rewritten-wrap-2024.rs","range":[23,23]},{"file":"tests/source/file-lines/trailing-rewritten-wrap-2024.rs","range":[31,31]}]
//@ rustfmt-edition: 2021
//@ rustfmt-style_edition: 2024
//@ rustfmt-wrap_comments: true
//@ rustfmt-comment_width: 40

// Reindenting the statement requires normal wrapping of its block comment.
fn changed_indent() {
    foo();
  let x=1;    /* This block comment is long enough to wrap onto several lines. */
    bar();
}

// Rewriting a multiline statement changes its trailing comment's position.
fn selected_start() {
    let value = compute(
        1, 2);    /* This block comment is long enough to wrap onto several lines. */
    bar();
}

// Line comments follow the style edition's normal wrapping and alignment.
fn line_comment() {
    foo();
  let x=1;    // This trailing line comment is long enough to wrap onto several lines.
    //standalone comment stays unchanged
    bar();
}

// Unselected statements and their comments retain their original layout.
fn preserved() {
    foo();
    let x=1;    /* This block comment is long enough to wrap onto several lines. */
    //standalone comment stays unchanged
    bar();
}
