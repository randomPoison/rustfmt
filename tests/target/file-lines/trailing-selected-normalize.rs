//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[7,7]},{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[9,9]},{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[11,11]},{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[15,15]}]
//@ rustfmt-normalize_comments: true
//@ rustfmt-style_edition: 2015

fn main() {
    let x = 1; // trailing block
               // selected continuation
    const VALUE: i32 = 3; // trailing item
                          // selected continuation
    fn local() {} // trailing function
                  // selected continuation
    let preserved=5;    /* unselected block
                          unselected continuation */
    let also_preserved = 6; // trailing line
                            // selected continuation line
    let last = 7; // unselected block with
                  // trailing code
    let next = 8;
}
