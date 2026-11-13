//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[7,7]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[15,15]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[22,22]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[32,32]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[39,39]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[47,47]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[55,55]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[62,62]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[70,70]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[78,78]},{"file":"tests/source/file-lines/trailing-comment-group-2015.rs","range":[88,88]}]
//@ rustfmt-style_edition: 2015

// An unselected trailing group is preserved,
// including its visual alignment.
fn canonical_mid() {
    let selected=0;
    bar(); // tail
           // more
    baz();
}

// The same applies at the end of a block.
fn canonical_end() {
    let selected=0;
    bar(); // tail
           // more
}

// Every continuation line stays aligned.
fn canonical_three_lines() {
    let selected=0;
    bar(); // tail
           // more
           // most
    baz();
}

// Selecting the code realigns the whole group.
fn code_selected() {
    foo();
    bar( ); // tail
            // more
    baz();
}

// Reindenting the code realigns the whole group.
fn code_misindented() {
    let selected=0;
  bar(); // tail
         // more
    baz();
}

// Items follow the same rules as statements.
fn item_unselected() {
    let selected=0;
    fn f() {} // tail
              // more
    baz();
}

fn item_selected() {
    foo();
    fn f( ){ } // tail
               // more
    baz();
}

// Macro statements follow the same rules.
fn macro_unselected() {
    let selected=0;
    m!(1); // tail
           // more
    baz();
}

fn macro_selected() {
    foo();
    m!( 1 ); // tail
             // more
    baz();
}

// Rewriting a multi-line statement moves its
// final line, and the group with it.
fn multiline_selected() {
    let value = compute(
        1, 2); // tail
               // more
    baz();
}

// A group in a nested block is preserved when
// nearby code is selected.
fn nested_block() {
    if x {
        let selected=0;
        bar(); // tail
               // more
    }
}
