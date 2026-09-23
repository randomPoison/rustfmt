// rustfmt-file_lines: [{"file":"tests/source/file-lines/match/body-only.rs","range":[8,8]},{"file":"tests/source/file-lines/match/body-only.rs","range":[14,14]},{"file":"tests/source/file-lines/match/body-only.rs","range":[28,28]}]

fn main() {
    match value {
        | Some ( x )
            if check( x )  => /*  keep prefix  */ {
            untouched(  x  );
            work(x);
            untouched(  x  );
        }
        Some ( y )
            => //  keep prefix
            {
            work(y);
            }
        _=>untouched(  ),
    }
}

// Partial call formatting still fails when its callee is outside the selection.
// Keep this in a separate match so it does not mask the block cases above.
fn expression_body() {
    match value {
        Some ( z )
            => //  keep expression prefix
            call(
                z,
                other(  z  ),
            ),
        _=>untouched(  ),
    }
}
