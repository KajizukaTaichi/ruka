	jmp 1, end_inc
function_inc:
	pop ar
	sta 0, ar	; n
	mov ar, 1
	psh ar
	lda ar, 0	; n
	mov dr, ar
	pop ar
	add ar, dr
	ret
end_inc:
	jmp 1, end_sub
function_sub:
	pop ar
	sta 1, ar	; b
	pop ar
	sta 2, ar	; a
	lda ar, 2	; a
	psh ar
	lda ar, 1	; b
	mov dr, ar
	pop ar
	neg dr
	add ar, dr
	ret
end_sub:
	mov ar, 2
	psh ar
	mov ar, 3
	mov dr, ar
	pop ar
	mul ar, dr
	psh ar
	cal function_inc
	psh ar
	mov ar, 6
	psh ar
	cal function_sub
	hlt
