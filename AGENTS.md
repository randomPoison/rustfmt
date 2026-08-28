# Test Fixture Conventions

Rustfmt system-test fixtures are paired by path:

- Inputs live under `tests/source/`.
- Expected formatted outputs live under `tests/target/` at the same relative path.

When adding, moving, or renaming a source fixture, make the corresponding change to its target fixture as well. Keep the source and target directory structures and filenames synchronized.

Fixtures that use file-line selection declare it in a significant comment such as:

```rust
// rustfmt-file_lines: [{"file":"tests/source/path/to/fixture.rs","range":[3,5]}]
```

The `file` value must be the exact repository-relative path to the source fixture, even in the copy under `tests/target/`. Update this value in both files whenever the source fixture moves or is renamed. Otherwise rustfmt will not match the selected range to the input file and the fixture may be formatted as though no file-line restriction applied.

New tests should go under `tests/source/file-lines-lists/fn-decl` unless
specified otherwise.

When asked to setup a new test from some example code, simply create the source
and target files along with the rustfmt-file_lines comment with the correct path
and an initial range of `[1,1]`. Don't otherwise try to figure out what lines
should be selected or what the output should look like, I'll manually set those
up after you've created the fixture.
