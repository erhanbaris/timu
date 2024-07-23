#include <stdio.h>
int main()
{
    volatile long v1 = 10;
    volatile long v2 = 20;
    volatile long v3 = v1 + v2;
}

/*
arch -x86_64 zsh  

gcc -O0 -S -arch x86_64 -masm=intel test.c     
as test.s -o test.o
objdump  -M intel -d test.o
----
as test.s -o hello_assembly.o
gcc -o hello_assembly hello_assembly.o -nostdlib -static
./hello_assembly
-------
objconv  -fnasm test.o /dev/stdout | less
*/
