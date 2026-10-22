// rustfmt-file_lines: [{"file":"tests/source/file-lines/item-shared-line.rs","range":[4,4]}]

mod cases {
    const SELECTED: i32 = 0;
    // Items ending in semicolons and braces can both have trailing siblings.
    const FIRST: i32 = 1;
    const SECOND: i32 = 2;
    fn first() {}
    fn second() {}
    struct First {
        value: i32,
    }
    struct Second {
        value: i32,
    }
    // Check the final line of a multiline item as well.
    fn multiline() {
        call( 3 );
    }
    fn following() {}
    // Macro items own their semicolons, but not the code after them.
    invoke!(1);
    const AFTER_MACRO: i32 = 2;
    invoke![3];
    fn after_macro() {}
    // Preserve unselected macros on their own lines, including their spacing.
    invoke!( 4 ) ;
    invoke![ 5 ];
}
