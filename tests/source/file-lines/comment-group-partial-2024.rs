//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/comment-group-partial-2024.rs","range":[10,10]},{"file":"tests/source/file-lines/comment-group-partial-2024.rs","range":[20,20]},{"file":"tests/source/file-lines/comment-group-partial-2024.rs","range":[30,30]},{"file":"tests/source/file-lines/comment-group-partial-2024.rs","range":[37,37]},{"file":"tests/source/file-lines/comment-group-partial-2024.rs","range":[46,46]},{"file":"tests/source/file-lines/comment-group-partial-2024.rs","range":[55,55]}]
//@ rustfmt-style_edition: 2024

// Selecting one line of an already formatted comment group leaves
// the group, including its indentation, unchanged.

// Selecting the first line of a group.
fn mid_first() {
    foo();
    // first
    // second
    // third
    bar();
}

// Selecting the middle line of a group.
fn mid_middle() {
    foo();
    // first
    // second
    // third
    bar();
}

// Selecting the last line of a group.
fn mid_last() {
    foo();
    // first
    // second
    // third
    bar();
}

// The same at the end of a block, first line.
fn end_first() {
    foo();
    // first
    // second
    // third
}

// The same at the end of a block, middle line.
fn end_middle() {
    foo();
    // first
    // second
    // third
}

// The same at the end of a block, last line.
fn end_last() {
    foo();
    // first
    // second
    // third
}
