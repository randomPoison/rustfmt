// rustfmt-file_lines: [{"file":"tests/source/issue-4053-match/if-else-middle.rs","range":[5,6]}]

fn fmt_match(val: Option<i32>) {
    if let Some(val) = val {
    println!("{val}");
        } else {
    println!("None!");
    }
}
