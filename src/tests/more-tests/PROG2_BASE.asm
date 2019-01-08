MAIN     START  1000
         BASE   ADDR1      
         CLEAR  X
         JSUB   GETPAI
         STCH   ADDR2
         JSUB   GETPAI
         STCH   ADDR3
         LDB    ADDR1
         JSUB   GETPAI
         STCH   ADDR1 
         JSUB   GETPAI
         STCH   ADDR2
LOOP     JSUB   GETPAI
         BASE   RTADDR
         STCH   ADDR1,X
         NOBASE
         TIXR   X
         J      LOOP
GETPAI   STL    RTADDR
         JSUB   READ
         SHIFTL A,4
         STCH   HEX
         OR     ORADDR
         J      @RTADDR
READ	 TD     #X'F1'
         JEQ    READ 
         CLEAR  A
         RD     #X'F1'
         COMP   #48
         JLT    EOFCK
         SUB    #48
         COMP   #10
         JLT    GOBACK
         SUB    #7
GOBACK   RSUB
EOFCK    COMP   #33
         JEQ    EXIT
         COMP   #4
         JGT    READ
EXIT     CLEAR  L 
         J      @ADDR2 
HEX      RESB   1
ADDR2    RESB   1
ADDR3    RESB   1
ORADDR   RESB   2
RTADDR   RESB   4096
ADDR1    RESB   1
         END    1000
