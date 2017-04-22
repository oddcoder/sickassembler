COPY	START	1000	. Copy file from input to output
FIRST	STL	RETADR	. save return address
CLOOP	JSUB	RDREC	. read input record
	LDA	LENGTH	. test for EOF (LENGTH = 0)
	COMP	ZERO
	JEQ	ENDFIL	. exit if eof
	JSUB	WRREC	. write output to record
	J	CLOOP	. loop
ENDFIL	LDA	EOF	. insert end of file marker
	STA	BUFFER
	LDA	=X'3'	. set LENGTH = 3
	BASE  EOD
	STA	LENGTH
	+JSUB	WRREC	. write EOF
	LTORG
	LDL	=C'RETARD'	. get return address
	RSUB
EOD	BYTE	C'EOF'
THREE	WORD	3
ZERO	WORD	0
RETADR	RESW	1
LENGTH	RESW	1
BUFFER	RESB	4096

. Whole line comment
RDREC	LDX	ZERO	. CLEAR LOOP COUNTER
	LDA	ZERO
RLOOP	TD	INPUT
	JEQ	RLOOP
	RD	INPUT
	COMP	ZERO
	JEQ	EXIT
	STCH	BUFFER,X
	TIX	MAXLEN
	JLT	RLOOP
EXIT	STX	LENGTH
	RSUB

INPUT	BYTE	X'F1'
MAXLEN	WORD	4096


WRREC	LDX	ZERO
WLOOP	TD	OUTPUT
	JEQ	WLOOP
	LDCH	BUFFER,X
	WD	OUTPUT
	TIX	LENGTH
	JLT	WLOOP
OUTPUT	BYTE	X'05'
	RSUB
LDL	=C'AERETARDS'	. get return address
	END	FIRST
