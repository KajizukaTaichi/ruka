dup : 2 * ;
half : 2 / ;

main :
    5 dup dup half
    10 =
        if 1 2
        else 3 4
    then + half
    8 ! ;
