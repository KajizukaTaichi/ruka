	cal word_главное
	hlt
word_двойной:
	psh 2
	pop dr
	pop ar
	mul ar, dr
	psh ar
	ret
word_половина:
	psh 2
	pop dr
	pop ar
	inv dr
	mul ar, dr
	psh ar
	ret
word_главное:
	psh 5
	cal word_двойной
	cal word_двойной
	cal word_половина
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
	cal word_и
	psh 2
	jmp 1, end_0
else_0:
	psh 3
	cal word_и
	psh 4
end_0:
	pop dr
	pop ar
	add ar, dr
	psh ar
	cal word_половина
	psh 8
	pop ba
	pop ar
	sta ba, ar
	ret
