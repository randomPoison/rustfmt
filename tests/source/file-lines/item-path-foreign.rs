// rustfmt-file_lines: [{"file":"tests/source/file-lines/item-path-foreign.rs","range":[7,7]},{"file":"tests/source/file-lines/item-path-foreign.rs","range":[13,13]},{"file":"tests/source/file-lines/item-path-foreign.rs","range":[19,19]},{"file":"tests/source/file-lines/item-path-foreign.rs","range":[25,25]},{"file":"tests/source/file-lines/item-path-foreign.rs","range":[31,31]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024

// An unselected foreign block with incorrect indentation must be rewritten.
fn foreign_indent() {
    foo();
  extern  "C" {}
}

// Trailing code prevents preservation even at the correct indentation.
fn foreign_trailing_code() {
    foo();
    extern  "C" {} let x=1;
}

// Incorrect indentation and trailing code must both be corrected.
fn foreign_indent_and_trailing_code() {
    foo();
  extern  "C" {} let x=1;
}

// A foreign block following a statement must move onto its own line.
fn foreign_preceding_code() {
    foo();
    let x=1; extern  "C" {}
}

// Control: preserve an unselected foreign block at the correct indentation.
fn preserved_foreign() {
    foo();
    extern  "C" {}
}
