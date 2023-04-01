use core::arch::asm;

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(mut n: usize) -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall1(mut n: usize, a1: usize) -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        in("r8") a1,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall2(mut n: usize, a1: usize, a2: usize) -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        in("r8") a1,
        in("r9") a2,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall3(mut n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        in("r8") a1,
        in("r9") a2,
        in("r10") a3,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall4(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize)
                       -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        in("r8") a1,
        in("r9") a2,
        in("r10") a3,
        in("r11") a4,
        out("rcx") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall5(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize)
                       -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        in("r8") a1,
        in("r9") a2,
        in("r10") a3,
        in("r11") a4,
        in("r12") a5,
        out("rcx") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall6(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize,
                       a6: usize)
                       -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        in("r8") a1,
        in("r9") a2,
        in("r10") a3,
        in("r11") a4,
        in("r12") a5,
        in("r13") a6,
        out("rcx") _,
        options(nostack),
    );
    n
}

#[inline(always)]
pub unsafe fn syscall7(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize,
                       a6: usize,
                       a7: usize)
                       -> usize {
    asm!(
        "int 0x80",
        inout("rax") n,
        in("r8") a1,
        in("r9") a2,
        in("r10") a3,
        in("r11") a4,
        in("r12") a5,
        in("r13") a6,
        in("r14") a7,
        out("rcx") _,
        options(nostack),
    );
    n
}