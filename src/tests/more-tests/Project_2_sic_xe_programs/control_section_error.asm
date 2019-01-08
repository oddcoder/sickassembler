PROG1    START  0000
         EXTDEF INDEV        
         LDX    #0
         LDT    #10
RLOOP    TD     INDEV
         JEQ    RLOOP
         RD     INDEV 
         STCH   RECORD,X
         TIXR   T 
         JLT    RLOOP 
INDEV    BYTE   X'F1' 
RECORD   RESB   100
PROG2    CSCET
         EXTREF INDEV         
         LDX    #0
         LDT    #10
RLOOP    TD     INDEV
         JEQ    RLOOP
         WD     INDEV 
         +STCH  RECORD,X
         TIXR   T 
         JLT    RLOOP 
         END    PROG1 

                
                  
