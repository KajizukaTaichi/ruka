Let bit = (1 + 2) * 3 - 7
Let max_1byte = pow(bit, 8) - 1
Exit Program

Sub pow(base, exponent)
    Let number = 1
    Let index = 0
    Let flag = true
    While flag
        Let index = index + 1
        If index > exponent
            Let flag = false
        Else
            Let number = number * base
        End If
    End While
    Return number
End Sub
