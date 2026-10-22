// rustfmt-file_lines: [{"file":"tests/source/file-lines/assoc-shared-line.rs","range":[4,4]}]

impl Example {
    const SELECTED:i32=0;
    const FIRST:i32=1; const SECOND:i32=2;
    fn first( ){ } fn second( ){ }
    type First= u8; type Second= u16;
    invoke!( 3 ); const AFTER_MACRO:i32=4;
    fn preserved( ){ }
}
