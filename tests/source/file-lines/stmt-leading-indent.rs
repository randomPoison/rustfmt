// rustfmt-file_lines: [{"file":"tests/source/file-lines/stmt-leading-indent.rs","range":[3,3]}]

fn main() {
let x=1; let y=2;
  // Keep this unselected comment's indentation.
  let z=3;
    let preserved=4;
    // The let starts at column four, but another statement precedes it.
0;  let same_column=5;
}
