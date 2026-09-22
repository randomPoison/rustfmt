// rustfmt-file_lines: [{"file":"tests/source/file-lines/match/scrutinee.rs","range":[5,6]}]

fn main() {
    match foo(  x  ) {
        Some(x)=>{ bar( x ); }
        None=>baz( ),
    }
}
