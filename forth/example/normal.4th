dup : 2 * ;
half : 2 / ;

main :
    5 dup dup half
    10 =
        ? 1 2
        ¥ 3 4
    # + half ;
