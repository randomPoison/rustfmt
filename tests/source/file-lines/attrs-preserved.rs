// rustfmt-file_lines: [{"file":"tests/source/file-lines/attrs-preserved.rs","range":[7,7]},{"file":"tests/source/file-lines/attrs-preserved.rs","range":[14,14]},{"file":"tests/source/file-lines/attrs-preserved.rs","range":[23,23]},{"file":"tests/source/file-lines/attrs-preserved.rs","range":[28,28]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024

// Control: preserve unselected attributes and items at the correct indentation.
fn preserved_outer() {
    foo();
    #[allow( unused )]
    fn  inner( ) {}
}

// Control: preserve an unselected attribute group with unchanged placement.
fn preserved_group() {
    foo();
    #[allow( unused )]
    #[inline( never )]
    fn  inner( ) {}
}

// Control: an unselected inner attribute can retain its original spelling.
fn preserved_inner() {
    #![allow( unused )]
    foo();
}

// Skip preserves attribute spelling and the item, but corrects leading indentation.
fn skipped_item() {
    foo();
  #[rustfmt :: skip]
  fn  inner( ) {}
}
