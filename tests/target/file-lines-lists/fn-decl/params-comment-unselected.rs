// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/fn-decl/params-comment-unselected.rs","range":[10,10]}]

fn foo(
/* keep me */
first    :    i32,
        // keep me
    
// preserve this comment
        second    :    i32,
    selected: i32,
) {}
