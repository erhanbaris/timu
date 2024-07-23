#!python3
# -*- coding: utf-8 -*-
# pip install unicorn
import unicorn
import unicorn.x86_const


def code_tracer(emulator, address, _size, _user_data):
    rax = emulator.reg_read(unicorn.x86_const.UC_X86_REG_RAX)
    rbx = emulator.reg_read(unicorn.x86_const.UC_X86_REG_RBX)
    rcx = emulator.reg_read(unicorn.x86_const.UC_X86_REG_RCX)
    print(f'[!] Executing instruction at 0x{address:016x} | rax = {rax} | rbx = {rbx} | rdx = {rcx}')


def main():
    emulation_address = 0x08000000

    # https://defuse.ca/online-x86-assembler.htm
    #
    # 0:  48 c7 c0 01 00 00 00    mov    rax,0x1
    # 7:  48 c7 c3 02 00 00 00    mov    rbx,0x2
    # e:  48 01 d8                add    rax,rbx
    # 11: 48 01 c8                add    rax,rcx
    # 14: 90                      nop
    code = b'\x48\xC7\xC0\x01\x00\x00\x00\x48\xC7\xC3\x02\x00\x00\x00\x48\x01\xD8\x48\x01\xC8\x90'

    emulator = unicorn.Uc(unicorn.UC_ARCH_X86, unicorn.UC_MODE_64)

    emulator.mem_map(emulation_address, 4 * 1024)
    emulator.mem_write(emulation_address, code)
    emulator.reg_write(unicorn.x86_const.UC_X86_REG_RCX, 3)
    emulator.hook_add(unicorn.UC_HOOK_CODE, code_tracer, None, emulation_address, emulation_address + len(code))
    emulator.emu_start(emulation_address, emulation_address + len(code))

    rax = emulator.reg_read(unicorn.x86_const.UC_X86_REG_RAX)

    print(f'[!] RAX value after emulation = {rax}')  # expects 6

    # Output:
    # [!] Executing instruction at 0x0000000008000000 | rax = 0 | rbx = 0 | rdx = 3
    # [!] Executing instruction at 0x0000000008000007 | rax = 1 | rbx = 0 | rdx = 3
    # [!] Executing instruction at 0x000000000800000e | rax = 1 | rbx = 2 | rdx = 3
    # [!] Executing instruction at 0x0000000008000011 | rax = 3 | rbx = 2 | rdx = 3
    # [!] Executing instruction at 0x0000000008000014 | rax = 6 | rbx = 2 | rdx = 3
    # [!] RAX value after emulation = 6


if __name__ == '__main__':
    main()