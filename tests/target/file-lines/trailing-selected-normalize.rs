// rustfmt-file_lines: [{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[6,6]},{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[8,8]},{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[10,10]},{"file":"tests/source/file-lines/trailing-selected-normalize.rs","range":[14,14]}]
// rustfmt-normalize_comments: true

fn main() {
    let x = 1; // trailing block
               // selected continuation
    const VALUE: i32 = 3; // trailing item
                          // selected continuation
    fn local() {} // trailing function
                  // selected continuation
    let preserved=5;    /* unselected block
                          unselected continuation */
    let also_preserved=6;    // trailing line
    // selected standalone line
    let last = 7; // unselected block with
                  // trailing code
    let next = 8;
}
