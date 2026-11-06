// rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-rewritten-normalize.rs","range":[8,8]},{"file":"tests/source/file-lines/trailing-rewritten-normalize.rs","range":[15,15]},{"file":"tests/source/file-lines/trailing-rewritten-normalize.rs","range":[22,22]},{"file":"tests/source/file-lines/trailing-rewritten-normalize.rs","range":[30,30]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024
// rustfmt-normalize_comments: true

// Reindenting an unselected statement also invalidates its trailing comment.
fn changed_indent() {
    foo();
    let x = 1; // trailing
    bar();
}

// Selecting the start rewrites the statement's unselected final line too.
fn selected_start() {
    let value = compute(1, 2); // trailing
    bar();
}

// Preserving a statement must preserve its trailing comment and spacing.
fn preserved() {
    foo();
    let x=1;    /* trailing */
    //standalone comment stays unchanged
    bar();
}

// The same rule applies when the trailing comment ends the block.
fn closing_comment() {
    foo();
    let x = 1; // trailing
}
