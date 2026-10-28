// rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-comment-group-selected.rs","range":[8,8]},{"file":"tests/source/file-lines/trailing-comment-group-selected.rs","range":[16,17]},{"file":"tests/source/file-lines/trailing-comment-group-selected.rs","range":[25,26]},{"file":"tests/source/file-lines/trailing-comment-group-selected.rs","range":[34,34]},{"file":"tests/source/file-lines/trailing-comment-group-selected.rs","range":[41,41]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024

// Selecting the trailing comment must retain its separator from the statement.
fn selected_first_line() {
    foo();
    bar(); // tail
    // more
    baz();
}

// Selecting a prefix still leaves the comment group only partially selected.
fn selected_prefix() {
    foo();
    bar(); // tail
    // selected continuation
    // unselected continuation
    baz();
}

// Control: selecting the entire group does not require splitting it.
fn selected_whole_group() {
    foo();
    bar(); // tail
    // more
    baz();
}

// Control: selecting only the continuation preserves the trailing line.
fn selected_continuation() {
    foo();
    bar(); // tail
    // more
    baz();
}

// Control: a trailing comment without a continuation retains its separator.
fn selected_single_comment() {
    foo();
    bar(); // tail
    baz();
}
