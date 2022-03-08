# GENERAL

S has to be -1 to win.
If I set S := 11 in *"if S == 0"* statement => Win

# Code abstraction

if __INCLUDE_LEVEL__ == 0:
    - Set S == 0
    - ROM gets defined (is partly function of FLAG)
    - Define: LD(x,y) returns value at ROM_<x>_<y>
    - Define: MD ???

If __INCLUDE_LEVEL__ > 12:
    if S == 0:
        S := 24

    if S == 10:
        #error "BUG"
    if S == 11:
        S := -1 !!!
    ...

    if S == 24:
        S := 25
        #undef I0 - I7

    if S == 25:
        S := 26
        #undef M0 - M7

    if S == 26:
    
    ... #define N0 !!! 
    ... #undef N1-N7, P0-P7, Q0-Q7
    
    if S == 29:
        #define B0, B2, B5, B6, B7
        #undef B1, B3, B4

    if S == 30:
        Buchstaben werden definiert

    .. I dont know yet

    if S == 35:
        Checking if some ROM is defined by LD


Else:
    include "cpp.c"