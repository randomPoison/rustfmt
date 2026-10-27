// rustfmt-file_lines: [{"file":"tests/source/file-lines/closing-brace-if-else-2024.rs","range":[7,7]},{"file":"tests/source/file-lines/closing-brace-if-else-2024.rs","range":[13,13]},{"file":"tests/source/file-lines/closing-brace-if-else-2024.rs","range":[19,19]},{"file":"tests/source/file-lines/closing-brace-if-else-2024.rs","range":[25,25]},{"file":"tests/source/file-lines/closing-brace-if-else-2024.rs","range":[33,33]},{"file":"tests/source/file-lines/closing-brace-if-else-2024.rs","range":[39,39]}]
// rustfmt-style_edition: 2024
// rustfmt-normalize_comments: true

fn main() {
    let repaired = if true {
        1
  /* Reindent and normalize this comment. */
    } else {
        0
    };
    let preserved = if true {
        1
    /* Preserve this comment, including its syntax. */
    } else {
        0
    };
    let body_aligned = if true {
        1
        /* Preserve the existing body alignment. */
    } else {
        0
    };
    let grouped = if true {
        1;
        2;    // Preserve this trailing comment's spacing.
    //Rewrite this correctly indented comment with the misindented line below.
  // Reindent this later comment in the same group.
    } else {
        0
    };
    let selected_trailing = if true {
        1    //Trailing comment on the selected line.
    // The rest of this group uses edition-specific alignment.
    } else {
        0
    };
    let preserved_trailing = if true {
        1;
        2    //Trailing comment on an unselected line.
    //Preserve this correctly indented group in both editions.
    } else {
        0
    };
}
