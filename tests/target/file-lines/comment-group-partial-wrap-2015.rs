//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[12,12]},{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[22,22]},{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[32,32]},{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[39,39]},{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[48,48]},{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[57,57]},{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[65,65]},{"file":"tests/source/file-lines/comment-group-partial-wrap-2015.rs","range":[76,76]}]
//@ rustfmt-style_edition: 2015
//@ rustfmt-wrap_comments: true
//@ rustfmt-comment_width: 40

// A comment group is rewritten as a whole if any of its lines is selected,
// since wrapping can move text between its lines.

// Selecting the first line of a group.
fn mid_first() {
    foo();
    // short line
    // this second line is quite long
    // and should be wrapped by rustfmt
    // here ok
    bar();
}

// Selecting the middle line of a group.
fn mid_middle() {
    foo();
    // short line
    // this second line is quite long
    // and should be wrapped by rustfmt
    // here ok
    bar();
}

// Selecting the last line of a group.
fn mid_last() {
    foo();
    // short line
    // this second line is quite long
    // and should be wrapped by rustfmt
    // here ok
    bar();
}

// The same at the end of a block, first line.
fn end_first() {
    foo();
    // short line
    // this second line is quite long
    // and should be wrapped by rustfmt
    // here ok
}

// The same at the end of a block, middle line.
fn end_middle() {
    foo();
    // short line
    // this second line is quite long
    // and should be wrapped by rustfmt
    // here ok
}

// The same at the end of a block, last line.
fn end_last() {
    foo();
    // short line
    // this second line is quite long
    // and should be wrapped by rustfmt
    // here ok
}

// The lines below a trailing comment stay attached to it in 2015;
// selecting its middle line rewrites the whole group and the statement.
fn trailing_middle() {
    foo();
    bar(); // tail
           // this second line is quite
           // long and should be wrapped
           // by rustfmt here ok
    baz();
}

// The lines below a trailing comment stay attached to it in 2015;
// selecting its last line rewrites the whole group and the statement.
fn trailing_last() {
    foo();
    bar(); // tail
           // this second line is quite
           // long and should be wrapped
           // by rustfmt here ok
    baz();
}
