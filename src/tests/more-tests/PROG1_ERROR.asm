PROG1    START  0000         
    LDX    #0
         LD     #10
RLOOP    TD     INDEV
         JEQ    RLOOP
         RD     INDEV 
         STCH   RECORD,X
         TXR    T 
         JLT    RLOOP
DUMP     BTE    1
DUMP     RESB   MO@3!22
STRING   RESB   'FOOPR'  
INDEV    BYTE   X'F1' 
RECORD   RESB   100
         ED     PROG1
                  
