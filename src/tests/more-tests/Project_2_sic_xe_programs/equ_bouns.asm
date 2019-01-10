SUM      START  1000         
FIRST    +LDA    #BEGIN
         +LDX    #FINAL
LOOP     ADDR   X,A
         TIX    #11
         JLT    LOOP
         +STA    #POS
         ORG    2000
BEGIN    EQU    *
DUMP     RESW   1
FINAL    EQU    * 
POS      EQU    FINAL-BEGIN
         END    SUM  
                  
