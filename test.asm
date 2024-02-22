.intel_syntax noprefix
mov r8,rcx

# as test.asm -o test.o
# gcc -S -masm=intel -Og -fverbose-asm test.asm -o test.o
# objdump -d  --disassembler-options=intel-mnemonic test.o
