Eval START 100

ALPHA WORD 48934DH

BETA WORD 102030H

GAMMA BYTE X’4C’

FIRST BYTE X’36’

RESULT RESW 1

BEGIN LDA GAMMA

    SUB FIRST
    
    STA RESULT
    
    LDA ALPHA
    
    ADD BETA
    
    SUB RESULT

    STA RESULT

    END BEGIN