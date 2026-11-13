//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/closing-brace-comment-group.rs","range":[5,5]}]
//@ rustfmt-style_edition: 2015

fn main() {
    let selected=0;
    let untouched=1; // Trailing comment grouped with the continuation below.
        // Preserve this misaligned continuation as written.
  }
