// rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-wrap.rs","range":[6,6]},{"file":"tests/source/file-lines/trailing-wrap.rs","range":[17,17]},{"file":"tests/source/file-lines/trailing-wrap.rs","range":[19,19]}]
// rustfmt-wrap_comments: true
// rustfmt-comment_width: 40

fn main() {
    let selected=0;
    let x=1;    /* trailing block */
    let y=2;    // This trailing comment is long enough to wrap onto several lines when comment wrapping is enabled.
    //standalone line stays unchanged
    const VALUE:i32=3; /* trailing item */
    fn local( ){ } /* trailing function */
    let z=4; /* multiline
              * trailing comment */
    let preserved=5;
    /* standalone block stays unchanged */
    let separator=6;
    // This selected standalone comment is long enough to wrap onto several lines when comment wrapping is enabled.

    /* This selected block comment is long enough to wrap onto several lines when comment wrapping is enabled. */
}
