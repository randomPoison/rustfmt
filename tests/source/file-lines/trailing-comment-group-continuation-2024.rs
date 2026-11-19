//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-comment-group-continuation-2024.rs","range":[8,8]},{"file":"tests/source/file-lines/trailing-comment-group-continuation-2024.rs","range":[15,15]},{"file":"tests/source/file-lines/trailing-comment-group-continuation-2024.rs","range":[21,21]}]
//@ rustfmt-style_edition: 2024

// Selecting only a continuation line of a trailing group.
fn mid_block() {
    foo();
    bar( ); // tail
            // more
    baz();
}

fn end_of_block() {
    foo();
    bar( ); // tail
            // more
}

fn middle_of_three() {
    foo();
    bar( ); // tail
            // more
            // most
    baz();
}
