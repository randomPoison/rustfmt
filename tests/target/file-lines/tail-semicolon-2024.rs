// rustfmt-file_lines: [{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[8,8]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[14,14]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[21,21]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[29,29]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[36,36]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[42,42]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[48,48]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[53,53]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[59,59]},{"file":"tests/source/file-lines/tail-semicolon-2024.rs","range":[65,65]}]
// rustfmt-edition: 2021
// rustfmt-style_edition: 2024
// rustfmt-trailing_semicolon: true

// Preserve unselected tail statements, including their missing punctuation.
fn returns() {
    selected();
    return  ( )
}

fn breaks() {
    loop {
        selected();
        break  ( )
    }
}

fn continues() {
    loop {
        selected();
        continue
    }
}

// Ordinary loop tails depend on both edition gates.
fn loop_tail() {
    loop {
        selected();
        call  ( )
    }
}

// Existing punctuation permits preservation.
fn existing_semicolon() {
    selected();
    return  ( );
}

// Already canonical code must remain unchanged.
fn canonical() {
    selected();
    return ();
}

// Selecting the tail applies the normal punctuation policy.
fn selected_return() {
    return ();
}

fn selected_loop_tail() {
    loop {
        call()
    }
}

// An indentation change also requires rewriting an unselected tail.
fn reindented_return() {
    selected();
    return ();
}

fn reindented_loop_tail() {
    loop {
        selected();
        call()
    }
}
