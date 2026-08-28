// rustfmt-file_lines: [{"file":"tests/source/file-lines-lists/fn-decl/params-unselected-trailing-whitespace.rs","range":[13,13]}]

fn foo(
    // leading comment  
                        /*comment*/      /*another*/  
/*a third*/   /*another */ first: u8 /*comment */,   
    // comment  
    
    // another comment after a line break  
    /*leading*/ second: u8, // trailing  
                            /*leading?*/  
                            // leading? trailing?  
    third: char,
fourth: bool,  
        /* commetn */   
fifth: u64, // comment   
sixth   : char) {
}
