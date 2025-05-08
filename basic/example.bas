Let bit = (1 + 2) * 3 - 7
Let max_1byte = pow(bit, 8) - 1
Exit Program

Sub pow(base, exponent)
    Let number = 1
    Let index = 0
    Let flag = true
    While flag
        If index = exponent
            Let flag = false
        Else
            Let index = index + 1
            Let number = number * base
        End If
    End While
    Return number
End Sub
