// rustfmt-file_lines: [{"file":"tests/source/file-lines/item-path-import-group.rs","range":[7,7]},{"file":"tests/source/file-lines/item-path-import-group.rs","range":[15,15]},{"file":"tests/source/file-lines/item-path-import-group.rs","range":[23,23]}]
// rustfmt-edition: 2021
// rustfmt-reorder_imports: true

// Only the middle import has incorrect indentation; rewrite and sort all three.
fn group_indent() {
    foo();
    use a::first;
    use m::middle;
    use z::last;
}

// Only the final import shares its line with code; rewrite and sort the group.
fn group_trailing_code() {
    foo();
    use a::first;
    use m::middle;
    use z::last;
    let x = 1;
}

// Control: preserve the spacing and order of an entirely eligible group.
fn preserved_group() {
    foo();
    use z :: last;
    use m :: middle;
    use a :: first;
}
