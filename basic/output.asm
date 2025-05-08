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
	sta 2, ar	; exponent
	pop ar
	sta 3, ar	; base
	mov ar, 1
	sta 4, ar	; number
	mov ar, 0
	sta 5, ar	; index
	mov ar, 1
	sta 6, ar	; flag
while_start_1:
	lda ar, 6	; flag
	mov cr, ar
	nor cr, cr
	jmp cr, while_end_1
	lda ar, 5	; index
	psh ar
	mov ar, 1
	mov dr, ar
	pop ar
	add ar, dr
	sta 5, ar	; index
	lda ar, 5	; index
	psh ar
	lda ar, 2	; exponent
	mov dr, ar
	pop ar
	eql ar, dr
	mov cr, ar
	jmp cr, if_then_0
	jmp 1, if_else_0
if_then_0:
	mov ar, 0
	sta 6, ar	; flag
	jmp 1, if_end_0
if_else_0:
	lda ar, 4	; number
	psh ar
	lda ar, 3	; base
	mov dr, ar
	pop ar
	mul ar, dr
	sta 4, ar	; number
if_end_0:
	jmp 1, while_start_1
while_end_1:
	lda ar, 4	; number
	psh ar
	ret
