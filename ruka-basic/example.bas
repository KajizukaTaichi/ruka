let n = (1 + 2) * 3
let result = pow(n, 2)
exit program

sub pow(x, y)
    let n = 1
    while y > 0
        let n = n * x
        let y = y - 1
    end while
    return n
end sub
