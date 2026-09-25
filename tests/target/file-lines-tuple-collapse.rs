// rustfmt-file_lines: [{"file":"tests/source/file-lines-tuple-collapse.rs","range":[7,7]}]
// Rewrite all elements when collapsing the tuple changes their columns.

fn main() {
    let tuple = (a + b, a + b, a + b);
}
