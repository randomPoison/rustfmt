// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/fn-decl/params-comment-before-selected.rs","range":[8,8]}]

fn foo(
first    :    i32,
// preserve this comment
    
        // preserve me too
        second    :    i32,
    selected    :    i32,
) {}
