MAIN START 0
 CLEAR X
 JSUB GETPAIR
 STCH ADDR2
 JSUB GETPAIR
 STCH ADDR3
 LDB ADDR
 JSUB GETPAIR
 STCH ADDR
 JSUB GETPAIR
 STCH ADDR2
LOOP JSUB GETPAIR
 STCH ADDR,X
 TIXR X
 J LOOP
 END 0
