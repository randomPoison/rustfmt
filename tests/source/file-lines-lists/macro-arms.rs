// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/macro-arms.rs","range":[7,7]}]

macro_rules! macro_arms {
    ($first:expr)   => { first($first) };
    (
        $second:expr
    )=>{second($second)};
    ($third:expr)=>{third($third)};
}
