	mov ar, 10
	sta 0, ar
	lda ar, 0
	psh ar
	mov ar, 2
	mov dr, ar
	pop ar
	add ar, dr
	hlt
