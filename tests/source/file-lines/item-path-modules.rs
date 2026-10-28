// rustfmt-file_lines: [{"file":"tests/source/file-lines/item-path-modules.rs","range":[7,7]},{"file":"tests/source/file-lines/item-path-modules.rs","range":[13,13]},{"file":"tests/source/file-lines/item-path-modules.rs","range":[19,19]},{"file":"tests/source/file-lines/item-path-modules.rs","range":[25,25]},{"file":"tests/source/file-lines/item-path-modules.rs","range":[31,31]},{"file":"tests/source/file-lines/item-path-modules.rs","range":[37,37]},{"file":"tests/source/file-lines/item-path-modules.rs","range":[45,45]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024

// An unselected module with incorrect indentation must be rewritten.
fn module_indent() {
    foo();
  mod  m {}
}

// Trailing code prevents preservation even at the correct indentation.
fn module_trailing_code() {
    foo();
    mod  m {} let x=1;
}

// Incorrect indentation and trailing code must both be corrected.
fn module_indent_and_trailing_code() {
    foo();
  mod  m {} let x=1;
}

// A module following a statement must move onto its own line.
fn module_preceding_code() {
    foo();
    let x=1; mod  m {}
}

// Control: preserve an unselected module at the correct indentation.
fn preserved_module() {
    foo();
    mod  m {}
}

// Moving a nonempty module must also correct its body's indentation.
fn module_body_indent() {
    foo();
  mod  m {
      fn  inner( ) {}
  }
}

// Control: preserve an unselected module and its body at the correct indentation.
fn preserved_module_body() {
    foo();
    mod  m {
        fn  inner( ) {}
    }
}
