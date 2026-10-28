// rustfmt-file_lines: [{"file":"tests/source/file-lines/item-path-imports-no-reorder.rs","range":[7,7]},{"file":"tests/source/file-lines/item-path-imports-no-reorder.rs","range":[13,13]},{"file":"tests/source/file-lines/item-path-imports-no-reorder.rs","range":[19,19]},{"file":"tests/source/file-lines/item-path-imports-no-reorder.rs","range":[25,25]},{"file":"tests/source/file-lines/item-path-imports-no-reorder.rs","range":[31,31]}]
// rustfmt-edition: 2021
// rustfmt-reorder_imports: false

// An unselected import with incorrect indentation must be rewritten.
fn import_indent() {
    foo();
  use a :: b;
}

// Trailing code independently prevents preservation, even at the right indent.
fn import_trailing_code() {
    foo();
    use a :: b; let x=1;
}

// Reproduce the combined indentation and trailing-code case.
fn import_indent_and_trailing_code() {
    foo();
  use a :: b; let x=1;
}

// An import following a statement must move onto its own line.
fn import_preceding_code() {
    foo();
    let x=1; use a :: b;
}

// Control: preserve an unselected import at the correct indentation.
fn preserved_import() {
    foo();
    use a :: b;
}
