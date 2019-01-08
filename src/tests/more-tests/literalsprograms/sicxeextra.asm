PROG1    START  0000         
         LDA    #70
         LDA    SYMBOL
         LDA    @SYMBOL
         LDA    #SYMBOL
         +LDA   SYMBOL 
         LDA    =C'Z' 
         LDA    =X'F1' 
         LDA    LITERAL
         CLEAR  A 
LITERAL  BYTE   X'F1'          
SYMBOL   RESB   1 
         END    PROG1 
                  
