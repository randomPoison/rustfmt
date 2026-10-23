// rustfmt-file_lines: [{"file":"tests/source/file-lines/leading-normalize.rs","range":[5,5]}]
// rustfmt-normalize_comments: true

fn main() {
    let selected=0;
    /* Leading comments with trailing code must be rewritten, not just moved. */ let x=1;
    /* Before an item. */ const VALUE:i32=2;
    /* A multiline comment with trailing code
     * on its closing line. */ let y=4;
    /* No trailing code: preserve this comment. */
    let preserved=3;
    /* An isolated multiline comment
     * without trailing code. */
    let also_preserved=5;
}
