// rustfmt-file_lines: [{"file":"tests/source/file-lines/stmt-shared-line.rs","range":[4,4]}]

fn main() {
    let selected=0;
    // The first statement starts at the correct indentation.
    let first=1; let second=2;
    call( 1 ); call( 2 ); call( 3 );
    if true { call( 4 ); } call( 5 );
    invoke!( 6 ); call( 7 );
    // A tail expression can also follow a semicolon-terminated statement.
    let last=8; last+1
}
