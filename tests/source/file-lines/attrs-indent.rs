// rustfmt-file_lines: [{"file":"tests/source/file-lines/attrs-indent.rs","range":[7,7]},{"file":"tests/source/file-lines/attrs-indent.rs","range":[14,14]},{"file":"tests/source/file-lines/attrs-indent.rs","range":[22,22]},{"file":"tests/source/file-lines/attrs-indent.rs","range":[32,32]},{"file":"tests/source/file-lines/attrs-indent.rs","range":[40,40]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024

// Unselected outer attributes must move with their containing item.
fn outer_attribute() {
    foo();
  #[allow(unused)]
  fn inner() {}
}

// Every attribute in a group must use the new indentation.
fn attribute_group() {
    foo();
  #[allow( unused )]
  #[inline(never)]
  fn inner() {}
}

// Multiline attribute contents must be reformatted along with the item.
fn multiline_attribute() {
    foo();
  #[allow(
    unused,
    dead_code
  )]
  fn inner() {}
}

// Associated items need the same attribute placement rules.
impl Example {
    fn selected() {}
  #[allow(unused)]
  fn inner() {}
}

// Inner attributes must follow the indentation of the enclosing block.
fn inner_attribute() {
  #![allow(unused)]
    foo();
}
