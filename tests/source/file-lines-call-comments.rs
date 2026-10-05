// rustfmt-file_lines: [{"file":"tests/source/file-lines-call-comments.rs","range":[4,4]}]

fn main() {
    call(/* before */
                      a+b,
        c+d /* after */,
                                              e+f,
    );
}
