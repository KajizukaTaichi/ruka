	mov ar, 10
	sta 0, ar
	lda ar, 0
	psh ar
	mov ar, 7
	mov dr, ar
	pop ar
	neg dr
	add ar, dr
	sta 1, ar
	lda ar, 0
	psh ar
	mov ar, 2
	psh ar
	lda ar, 1
	mov dr, ar
	pop ar
	mul ar, dr
	mov dr, ar
	pop ar
	add ar, dr
	psh ar
	mov ar, 8
	mov dr, ar
	pop ar
	add ar, dr
	hlt
