	cal word_main
	hlt
word_twice:
	psh 2
	pop dr
	pop ar
	mul ar, dr
	psh ar
	ret
word_half:
	psh 2
	pop dr
	pop ar
	inv dr
	mul ar, dr
	psh ar
	ret
word_main:
	psh 5
	cal word_twice
	cal word_twice
	cal word_half
	psh 10
	pop dr
	pop ar
	eql ar, dr
	psh ar
	pop cr
	jmp cr, then_0
	jmp 1, else_0
then_0:
	psh 1
	psh 2
	jmp 1, end_0
else_0:
	psh 3
	psh 4
end_0:
	pop dr
	pop ar
	add ar, dr
	psh ar
	cal word_half
	psh 8
	pop ba
	pop ar
	sta ba, ar
	ret
