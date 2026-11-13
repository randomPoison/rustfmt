//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-normalize.rs","range":[6,6]}]
//@ rustfmt-normalize_comments: true
//@ rustfmt-style_edition: 2015

fn main() {
    let selected=0;
    let x=1;    /* trailing block */
    let y=2;    //trailing line
            //continuation of the trailing line stays unchanged
    const VALUE:i32=3; /* trailing item */
    fn local( ){ } /* trailing function */
    let z=4; /* multiline
              * trailing comment */
    let preserved=5;
    /* standalone block stays unchanged */
}
