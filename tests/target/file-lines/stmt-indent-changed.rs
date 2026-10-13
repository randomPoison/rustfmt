// rustfmt-file_lines: [{"file":"tests/source/file-lines/stmt-indent-changed.rs","range":[8,8]}]

fn do_match(arg: Option<i32>) {
    // Vertical call layout changes the expected indentation of every statement.
    let tup = sec_call(
        a,
        b,
        c,
        d,
        |arg| {
            a + b;
            a + b;
            a + b;
            a + b;
            a + b;
            a + b;
        },
        another,
    );
}
