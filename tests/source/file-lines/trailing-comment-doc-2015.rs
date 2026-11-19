//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-comment-doc-2015.rs","range":[9,9]},{"file":"tests/source/file-lines/trailing-comment-doc-2015.rs","range":[16,16]},{"file":"tests/source/file-lines/trailing-comment-doc-2015.rs","range":[24,24]},{"file":"tests/source/file-lines/trailing-comment-doc-2015.rs","range":[32,32]}]
//@ rustfmt-style_edition: 2015

// A doc comment after a trailing comment belongs to the next item, so
// selecting only the doc comment does not rewrite the preceding statement.
fn doc_selected() {
    foo();
    let x=1; // a
    /// doc
    fn f() {}
}

fn block_doc_selected() {
    foo();
    let x=1; // a
    /** doc */
    fn f() {}
}

// Selecting the trailing comment rewrites the statement, but the doc
// comment stays with its item instead of joining the trailing group.
fn trailing_selected() {
    foo();
    let x=1; // a
    /// doc
    fn f() {}
}

// The same applies to items in an impl.
impl S {
    fn a( ) {} // a
    /// doc
    fn b() {}
}
