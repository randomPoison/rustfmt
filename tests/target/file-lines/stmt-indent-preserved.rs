// rustfmt-file_lines: [{"file":"tests/source/file-lines/stmt-indent-preserved.rs","range":[8,8]}]

fn do_match(arg: Option<i32>) {
    // Preserve unselected statements only when their indentation is correct.
    let tup = first_call(a, b, c, d, |arg| {
        a+b;
        a+b;
        a + b;
        a+b;
        a + b;
        a + b;
    });
}
