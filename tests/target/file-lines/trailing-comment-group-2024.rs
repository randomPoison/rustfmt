//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[7,7]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[15,15]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[22,22]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[32,32]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[40,40]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[48,48]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[56,56]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[63,63]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[71,71]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[79,79]},{"file":"tests/source/file-lines/trailing-comment-group-2024.rs","range":[89,89]}]
//@ rustfmt-style_edition: 2024

// An unselected trailing comment is preserved, and
// the comments below it are block indented.
fn canonical_mid() {
    let selected = 0;
    bar(); // tail
    // more
    baz();
}

// The same applies at the end of a block.
fn canonical_end() {
    let selected = 0;
    bar(); // tail
    // more
}

// Every following comment stays block indented.
fn canonical_three_lines() {
    let selected = 0;
    bar(); // tail
    // more
    // most
    baz();
}

// Selecting the code rewrites only its trailing line.
fn code_selected() {
    foo();
    bar(); // tail
    // more
    baz();
}

// Reindenting the code does not move the comments
// below it.
fn code_misindented() {
    let selected = 0;
    bar(); // tail
    // more
    baz();
}

// Items follow the same rules as statements.
fn item_unselected() {
    let selected = 0;
    fn f() {} // tail
    // more
    baz();
}

fn item_selected() {
    foo();
    fn f() {} // tail
    // more
    baz();
}

// Macro statements follow the same rules.
fn macro_unselected() {
    let selected = 0;
    m!(1); // tail
    // more
    baz();
}

fn macro_selected() {
    foo();
    m!(1); // tail
    // more
    baz();
}

// Rewriting a multi-line statement does not move
// the comments below it.
fn multiline_selected() {
    let value = compute(1, 2); // tail
    // more
    baz();
}

// A trailing comment in a nested block is preserved
// when nearby code is selected.
fn nested_block() {
    if x {
        let selected = 0;
        bar(); // tail
        // more
    }
}
