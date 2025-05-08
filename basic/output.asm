	mov ar, 6
	sta 0, ar	; a
	mov ar, 5
	psh ar
	lda ar, 0	; a
	mov dr, ar
	pop ar
	les ar, dr
	mov cr, ar
	jmp cr, if_then_2
	jmp 1, if_else_2
if_then_2:
	lda ar, 0	; a
	psh ar
	mov ar, 10
	mov dr, ar
	pop ar
	eql ar, dr
	mov cr, ar
	jmp cr, if_then_0
	jmp 1, if_else_0
if_then_0:
	mov ar, 666
	sta 0, ar	; a
	jmp 1, if_end_0
if_else_0:
	mov ar, 3.14
	sta 1, ar	; c
if_end_0:
	jmp 1, if_end_2
if_else_2:
	lda ar, 0	; a
	psh ar
	mov ar, 1
	mov dr, ar
	pop ar
	eql ar, dr
	mov cr, ar
	jmp cr, if_then_1
	jmp 1, if_else_1
if_then_1:
	mov ar, 50
	sta 0, ar	; a
	jmp 1, if_end_1
if_else_1:
	mov ar, 333
	sta 2, ar	; b
if_end_1:
if_end_2:
	hlt
