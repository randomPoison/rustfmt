// rustfmt-file_lines: [{"file":"tests/source/file-lines/comment-indent.rs","range":[3,3]}]

fn main() {
  // Reindent before a preserved statement.
    let untouched=0;  // Preserve this trailing comment's spacing.
    // The first comment is already correctly indented.
 // The second comment in the same group is not.
    let other=1;
  /* Reindent a standalone block comment. */
    let block=2;
if true {
    // Reindent when the enclosing block moves.
        let nested=3;
    }
  // Reindent after the last statement too.
}
