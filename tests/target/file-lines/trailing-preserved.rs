// rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-preserved.rs","range":[4,4]}]

fn main() {
    let selected = 0;
    let x=1;    /* trailing block */
    let y=2;    //trailing line
    //standalone line stays unchanged
    const VALUE:i32=3; /* trailing item */
    fn local( ){ } /* trailing function */
    let z=4; /* multiline
              * trailing comment */
    let preserved=5;
    /* standalone block stays unchanged */
}
