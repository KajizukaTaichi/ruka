Let bit = (1 + 2) * 3 - 7
Let max_1byte = pow(bit, 8) - 1
Exit Program

Sub pow(x, y)
    Let n = 1
    Let i = 0
    While i < y
        Let i = i + 1
        Let n = n * x
    End While
    Return n
End Sub
