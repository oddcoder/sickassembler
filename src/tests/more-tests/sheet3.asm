prog1    START   0000 
         EXTDEF  ALPHA,MAX 
         EXTREF  INF,ZERO 
         LDS     #3 
         LDT     #300 
         LDX     #0 
CLOOP    LDA     ALPHA,X 
         COMP    MAX 
         +JLT    NOCH 
         STA     MAX 
NOCH     ADDR    S,X 
         COMPR   X,T 
         JLT     CLOOP 
ALPHA    RESW    100 
MAX      WORD    32768 

prog2    CSECT 
         EXTDEF  MIN,ZERO 
         EXTREF  ALPHA,MAX 
         LDS     THREE 
         LDT     #300 
         LDX     #0 
CLOOP    +LDA    ALPHA,X 
         COMP    MIN 
         JGT     NOCH 
         STA     MIN 
NOCH     ADDR    S,X 
         COMPR   X,T 
         JLT     CLOOP 
ZERO     WORD    0 
THREE    WORD    3 
MIN      word    MAX 

Prog3    CSECT  
         EXTDEF  INF
         EXTREF  MAX,MIN 
         +LDA    MAX 
         +LDS    MIN 
         DIVR    S,A
         +STA    ALPHA 
         MULR    S,A    
         LDS     BETA 
         SUBR    A,S 
         STS     DELTA 
ALPHA    RESW    1 
BETA     RESW    1 
GAMMA    RESW    1 
DELTA    RESW    1 
INF      WORD    32768
         END     0000 
