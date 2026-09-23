// rustfmt-file_lines: [{"file":"tests/source/file-lines/match/skipped-body.rs","range":[5,5]}]

fn main() {
    match value {
        Some ( x ) =>
            call(  x  ),
        _ => untouched(  ),
    }
}
