// rustfmt-file_lines: [{"file":"tests/source/file-lines/stmt-shared-comment.rs","range":[4,4]}]

fn main() {
    let selected=0;
    // Comment markers in strings are not comments.
    let text="// /*"; call( 5 );
    // These unchanged lines should remain unformatted.
    let preserved=6; // trailing
    let also_preserved=7; /* trailing */
}
