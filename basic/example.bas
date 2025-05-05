Let bit = (1 + 2) * 3 - 7
Let max_1byte = pow(bit, 8) - 1
Exit Program

Sub pow(x, y)
    let n = 1
    While y > 0
        Let n = n * x
        Let y = y - 1
    End While
    Return n
End Sub
