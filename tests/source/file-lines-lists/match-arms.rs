// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/match-arms.rs","range":[4,7]}]

fn match_arms(value: Value) {
    match value {
        Value::First =>    first(),
        Value::Second =>
            second(),
        Value::Third =>    third(),
    }
}
