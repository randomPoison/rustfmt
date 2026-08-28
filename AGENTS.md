# Test fixture notes

Rustfmt system-test fixtures are paired by relative path:

- `tests/source/<path>.rs` is the unformatted input.
- `tests/target/<path>.rs` is the expected output once the fixture is finalized.

When adding, copying, or renaming a fixture, always update both files. Preserve intentional whitespace and comments; many fixtures exercise those details.

When creating a new fixture, initialize the target as an exact copy of the
source, including intentional whitespace, comments, the file-lines header, and
the following blank line. Only change the header's path when needed so that it
still points to the source fixture. Do not try to infer, generate, or bless the
expected formatter output unless explicitly asked; the user will edit the
target to express the behavior they want to test.

File-lines fixtures begin with an inclusive, 1-based selection such as:

```rust
// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/fn-decl/example.rs","range":[5,6]}]
```

Keep this header in both the source and target files. Its `file` value must point to the source fixture, so update it whenever the fixture is renamed. The header and following blank line count toward the selected line numbers.

Run the system fixtures from the repository root with:

```sh
cargo test system_tests
```

Run the complete test suite with `cargo test`.

For most of our current work we are focused on supporting `--file-lines` in the
list formatting machinery, specifically focusing on handling function arguments
correctly. The list formatting machinery is in `src/lists.rs`, and the
surrounding handling for function signatures is in `src/items.rs`.

By default, new test fixtures should go under
`tests/source/file-lines-lists/fn-decl` unless specified otherwise.
