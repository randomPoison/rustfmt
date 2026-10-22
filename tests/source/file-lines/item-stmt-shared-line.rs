// rustfmt-file_lines: [{"file":"tests/source/file-lines/item-stmt-shared-line.rs","range":[4,4]}]

fn main() {
    let selected=0;
    // An item can be followed by a statement, or vice versa.
    const LOCAL:i32=4; call( 5 );
    fn local( ){ } let value=6;
    let other=7; const NEXT:i32=8;
}
