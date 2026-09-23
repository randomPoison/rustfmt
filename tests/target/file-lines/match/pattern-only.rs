// rustfmt-file_lines: [{"file":"tests/source/file-lines/match/pattern-only.rs","range":[5,5]},{"file":"tests/source/file-lines/match/pattern-only.rs","range":[10,10]},{"file":"tests/source/file-lines/match/pattern-only.rs","range":[13,13]},{"file":"tests/source/file-lines/match/pattern-only.rs","range":[18,18]}]

fn main() {
    match value {
        Some(x) if check(x) => {  work( x );  } /* keep suffix */ ,
        Some(y) => // keep this comment
        call(  y  )  /* keep suffix */  ,
        None => { nested(  ); }
        Some(z) if check(z) =>
        // comment-only selection
        {
              work(  z  );
            },
        _=>untouched(  ),
    }
}
