	mov ar, 1
	sta 0, ar	; a
	mov ar, 5
	psh ar
	lda ar, 0	; a
	mov dr, ar
	pop ar
	les ar, dr
	mov cr, ar
	jmp cr, if_then_0
	jmp 1, if_else_0
if_then_0:
	jmp 1, if_end_0
if_else_0:
if_end_0:
