line_0:
	mov ar, 1
	psh ar
	mov ar, 2
	mov dr, ar
	pop ar
	add ar, dr
	psh ar
	mov ar, 3
	mov dr, ar
	pop ar
	mul ar, dr
	psh ar
	mov ar, 7
	mov dr, ar
	pop ar
	neg dr
	add ar, dr
	sta 0, ar	; bit

line_1:
	lda ar, 0
	psh ar
	mov ar, 8
	psh ar
	cal subroutine_pow
	pop ar
	psh ar
	mov ar, 1
	mov dr, ar
	pop ar
	neg dr
	add ar, dr
	sta 1, ar	; max_1byte

line_2:
	hlt

line_4:
subroutine_pow:
	pop ar
	sta 2, ar	; y
	pop ar
	sta 3, ar	; x

line_5:
	mov ar, 1
	sta 4, ar	; n

line_6:
	mov ar, 0
	sta 5, ar	; i

line_7:
while_start_0:
	lda ar, 5
	psh ar
	lda ar, 2
	mov dr, ar
	pop ar
	les ar, dr
	mov cr, ar
	nor cr, cr
	jmp cr, while_end_0

line_8:
	lda ar, 5
	psh ar
	mov ar, 1
	mov dr, ar
	pop ar
	add ar, dr
	sta 5, ar	; i

line_9:
	lda ar, 4
	psh ar
	lda ar, 3
	mov dr, ar
	pop ar
	mul ar, dr
	sta 4, ar	; n

line_10:
	jmp 1, while_start_0
while_end_0:

line_11:
	lda ar, 4
	psh ar
	ret

line_12:
	ret

