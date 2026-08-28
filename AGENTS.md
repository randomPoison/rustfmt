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

# Feature Design

Currently we are implementing the `--file-lines` feature, which allows for
partially formatting specific ranges of the file. Right now we are focusing on
function argument lists, which requires that we start adding file-lines support
to the central list processing machinery. The relevant list formatting code is
in `src/lists.rs` in `write_list` and the `Iterator` implementation for
`ListItems`, and the surrounding logic for laying out function signatures is in
`src/items.rs` in `rewrite_fn_base`.

The general rules I'm trying to follow are:

- Any code on selected lines must be rewritten. 
- Any unselected lines must be preserved exactly.
- If the arguments list is only partially selected, we force a vertical layout
  even if the args list would fit on one line when fully formatted.
- Otherwise, the selected lines must be rewritten as they would be laid out had
  the whole surrounding context been selected.

For now I want to avoid handling a couple of things:

- Comments, because this requires doing parsing of the text between items, and I
  don't want to mess with that yet.
- Whitespace lines between items, to the extent possible (though we're going to
  need to do some minimal handling of whitespace between items).
- Partially formatting individual items. I think I want to test the case where
  an item is fully unselected, and so shouldn't be re-formatted, but partially
  selecting a list item isn't handled in the central list formatting machinery,
  and I want to try to focus on getting initial support for partially formatting
  lists.

At this point I'm still trying to identify a clean decomposition of this work
into pieces that can be implemented in stages that are easy to turn into pull
requests and review.
