MAIN     START  0000         
FIRST    +LDA    #BEGIN
SECOND   +LDX    #FINAL
LOOP     ADDR   X,A
         TIX    #11
         JLT    LOOP
BEGIN    EQU    FIRST-SECOND+LOOP   . assume this is 5
DUMP     RESW   100
FINAL    EQU    LOOP+SECOND-DUMP-BEGIN . assume this is 7
         END    MAIN  
                  
