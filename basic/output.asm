	mov ar, 1
	sta 0, ar	; a
	mov ar, 5
	psh ar
	lda ar, 0	; a
	mov dr, ar
	pop ar
	les ar, dr
	mov cr, ar
	jmp cr, if_then_1
	jmp 1, if_else_1
if_then_1:
	mov ar, 100
	sta 0, ar	; a
	jmp 1, if_end_1
	mov ar, 100
	sta 0, ar	; a
if_else_1:
	lda ar, 0	; a
	psh ar
	mov ar, 1
	mov dr, ar
	pop ar
	eql ar, dr
	mov cr, ar
	jmp cr, if_then_0
	jmp 1, if_end_0
if_then_0:
	mov ar, 50
	sta 0, ar	; a
if_end_0:
if_end_1:
