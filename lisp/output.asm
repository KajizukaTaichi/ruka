	jmp 1, end_inc
function_inc:
	pop ar
	sta 0, ar	; n
	lda ar, 0; n
	psh ar
	mov ar, 1
	mov dr, ar
	pop ar
	add ar, dr
	ret
end_inc:
	mov ar, 2
	psh ar
	mov ar, 3
	mov dr, ar
	pop ar
	mul ar, dr
	psh ar
	cal function_inc
	hlt
