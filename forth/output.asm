	cal word_始まり
	hlt
word_倍:
	psh 2
	cal word_を
	pop dr
	pop ar
	mul ar, dr
	psh ar
	cal word_ける
	ret
word_半分:
	psh 2
	cal word_で
	pop dr
	pop ar
	inv dr
	mul ar, dr
	psh ar
	cal word_る
	ret
word_覚:
	psh 8
	cal word_番地に
	pop ba
	pop ar
	sta ba, ar
	cal word_き込む
	ret
word_思出:
	psh 8
	cal word_番地から
	pop ba
	lda ar, ba
	psh ar
	cal word_み込む
	ret
word_始まり:
	psh 5
	cal word_の
	cal word_倍
	cal word_の
	cal word_倍
	cal word_の
	cal word_半分
	cal word_が
	psh 10
	cal word_と
	pop dr
	pop ar
	eql ar, dr
	psh ar
	cal word_しい
	pop cr
	jmp cr, then_0
	jmp 1, else_0
then_0:
	psh 1
	cal word_と
	psh 2
	cal word_を
	jmp 1, end_0
else_0:
	psh 3
	cal word_と
	psh 4
	cal word_を
end_0:
	pop dr
	pop ar
	add ar, dr
	psh ar
	cal word_しあわせて
	cal word_半分
	cal word_にして
	cal word_覚
	cal word_え、
	psh 10
	cal word_と
	psh 10
	cal word_を
	pop dr
	pop ar
	mul ar, dr
	psh ar
	cal word_けて
	cal word_倍
	cal word_したものに
	cal word_思出
	cal word_して
	pop dr
	pop ar
	mul ar, dr
	psh ar
	cal word_ける
	ret
