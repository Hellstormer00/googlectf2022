#include <stdio.h>

#if __INCLUDE_LEVEL__ < 3
#include "test.c"
#endif

#define x __INCLUDE_LEVEL__ == 2

int main(int argc, char const *argv[])
{
    printf("%d", x);
    return 0;
}
