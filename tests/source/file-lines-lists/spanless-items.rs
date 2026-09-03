// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/spanless-items.rs","range":[4,5]}]

fn spanless_items(value: Value) {
    match value {
        First|Second   |   Third => {}
    }
}
