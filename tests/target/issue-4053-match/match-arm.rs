// rustfmt-file_lines: [{"file":"tests/source/issue-4053-match/match-arm.rs","range":[8,10]}]

fn fmt_match(val: Option<i32>) {
    match val {
        Some(val) => {
        println!("{val}");
        }
        None => {
            println!("None!");
        }
    }
}
