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
	lda ar, 0	; bit
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
	hlt
subroutine_pow:
	pop ar
	sta 2, ar	; y
	pop ar
	sta 3, ar	; x
	mov ar, 1
	sta 4, ar	; n
	mov ar, 0
	sta 5, ar	; i
while_start_0:
	lda ar, 5	; i
	psh ar
	lda ar, 2	; y
	mov dr, ar
	pop ar
	les ar, dr
	mov cr, ar
	nor cr, cr
	jmp cr, while_end_0
	lda ar, 5	; i
	psh ar
	mov ar, 1
	mov dr, ar
	pop ar
	add ar, dr
	sta 5, ar	; i
	lda ar, 4	; n
	psh ar
	lda ar, 3	; x
	mov dr, ar
	pop ar
	mul ar, dr
	sta 4, ar	; n
jmp 1, while_start_0
while_end_0:	lda ar, 4	; n
	psh ar
	ret
	ret
