// rustfmt-file_lines: [{"file":"tests/source/file-lines/stmt-shared-final-line.rs","range":[4,4]}]

fn main() {
    let selected = 0;
    // Inspect the final line, not just the line where the statement starts.
    let value = call(1, 2);
    call(3);
    if true {
        call( 4 );
    }
    call(5);
    // Even a correctly indented final delimiter can have trailing code.
    let pair = (6, 7);
    let next = 8;
}
