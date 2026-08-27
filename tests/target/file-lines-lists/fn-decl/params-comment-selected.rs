// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/fn-decl/params-comment-selected.rs","range":[5,6]}]

fn foo(
first    :    i32 /* comment before separator */,
    // preserve this comment
    second: i32,
    selected    :    i32,
) {}
