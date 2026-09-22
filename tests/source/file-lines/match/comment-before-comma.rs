// rustfmt-file_lines: [{"file":"tests/source/file-lines/match/comment-before-comma.rs","range":[6,6]}]

fn main() {
    match x {
        Some(x)=>foo( x )  /* post-comment */  ,
        None=>bar( ),
        None=>bar ( ) /* comment */ ,
    }
}
