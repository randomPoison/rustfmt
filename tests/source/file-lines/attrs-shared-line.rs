// rustfmt-file_lines: [{"file":"tests/source/file-lines/attrs-shared-line.rs","range":[7,7]},{"file":"tests/source/file-lines/attrs-shared-line.rs","range":[13,13]},{"file":"tests/source/file-lines/attrs-shared-line.rs","range":[20,20]},{"file":"tests/source/file-lines/attrs-shared-line.rs","range":[26,26]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024

// An attribute and its item must be separated even when both are unselected.
fn attribute_and_item() {
    foo();
    #[allow(unused)] fn inner() {}
}

// An attribute following another statement must move onto its own line.
fn preceding_statement() {
    foo();
    let x=1; #[allow(unused)]
    fn inner() {}
}

// A comment between an attribute and its item must not hide the shared line.
fn intervening_comment() {
    foo();
    #[allow(unused)] /* between */ fn inner() {}
}

// Multiple attributes sharing a line must use the normal attribute layout.
fn shared_attributes() {
    foo();
    #[allow(unused)] #[inline(never)] fn inner() {}
}
