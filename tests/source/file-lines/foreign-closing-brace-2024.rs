// rustfmt-file_lines: [{"file":"tests/source/file-lines/foreign-closing-brace-2024.rs","range":[7,7]},{"file":"tests/source/file-lines/foreign-closing-brace-2024.rs","range":[15,15]},{"file":"tests/source/file-lines/foreign-closing-brace-2024.rs","range":[23,23]},{"file":"tests/source/file-lines/foreign-closing-brace-2024.rs","range":[28,28]},{"file":"tests/source/file-lines/foreign-closing-brace-2024.rs","range":[37,37]},{"file":"tests/source/file-lines/foreign-closing-brace-2024.rs","range":[45,45]},{"file":"tests/source/file-lines/foreign-closing-brace-2024.rs","range":[51,51]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024

// Selecting only the header must leave the canonical closing brace unchanged.
fn selected_header() {
    extern "C" {
        fn f();
    }
}

// Selecting a declaration must also leave the unselected closing brace unchanged.
fn selected_declaration() {
    extern "C" {
        fn g();
    }
}

// Control: selecting the closing brace uses the selected missing-span path.
fn selected_closing_brace() {
    extern "C" {
        fn h();
    }
}

// A comment before the unselected closing brace must retain its indentation.
fn closing_comment() {
    extern "C" {
        fn i();
        // End of declarations.
    }
}

// Check the closing brace at a deeper nesting level too.
fn nested_foreign() {
    if ready() {
        extern "C" {
            fn j();
        }
    }
}

// An unselected closing brace must be moved off the declaration's line.
fn shared_closing_brace() {
    extern "C" {
        fn k(); }
}

// An unselected closing brace must use the block's outer indentation.
fn misindented_closing_brace() {
    extern "C" {
        fn l();
      }
}
