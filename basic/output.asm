line_0:
	mov ar, 7
	psh ar
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
	mov dr, ar
	pop ar
	neg ar
	add ar, dr
	sta 0, ar

line_1:
	mov ar, 1
	psh ar
	lda ar, 0
	psh ar
	mov ar, 8
	psh ar
	cal subroutine_pow
	pop ar
	mov dr, ar
	pop ar
	neg ar
	add ar, dr
	sta 1, ar

line_2:
	hlt

line_4:
subroutine_pow:
	pop ar
	sta 2, ar
	pop ar
	sta 3, ar

line_5:
	mov ar, 1
	sta 4, ar

line_6:
while_start_0:
	mov ar, 0
	psh ar
	lda ar, 2
	mov dr, ar
	pop ar
	les ar, dr
	mov cr, ar
	nor cr, cr
	jmp cr, while_end_0

line_7:
	lda ar, 4
	psh ar
	lda ar, 3
	mov dr, ar
	pop ar
	mul ar, dr
	sta 4, ar

line_8:
	mov ar, 1
	psh ar
	lda ar, 2
	mov dr, ar
	pop ar
	neg ar
	add ar, dr
	sta 2, ar

line_9:
	jmp 1, while_start_0
while_end_0:

line_10:
	lda ar, 4
	psh ar
	ret

line_11:
	ret

