// rustfmt-file_lines: [{"file":"tests/source/file-lines/leading-comment.rs","range":[4,4]}]

fn main() {
    let selected = 0;
    /* leading */
    let x = 1;
    /* multiline
     */
    let y = 2;
    /* item */
    const VALUE: i32 = 4;
    /* function */
    fn local() {}
    /* Already on separate lines: preserve both. */
    let preserved=5;
}
