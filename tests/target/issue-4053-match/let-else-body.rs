// rustfmt-file_lines: [{"file":"tests/source/issue-4053-match/let-else-body.rs","range":[6,6]}]

fn fmt_match(val: Option<i32>) {
let Some(_) = val else {
println!("None!");
        return;
};
}
