// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/fn-decl/params-full-horizontal.rs","range":[7,15]}]

// Tests that we force a vertical layout if the open and close parens weren't
// selected, even if the args would normally want to be laid out horizontally.

fn foo(  
    first: i32,

    selected: i32,

    last: i32,
) {}
