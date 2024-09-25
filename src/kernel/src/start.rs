use core::{arch::asm, hint::unreachable_unchecked};

use crate::riscv::registers::{
    mepc, mhartid, mstatus, pmpaddr0,
    pmpcfg0::{self, Permission, Range},
    satp,
};

/// This function's purpose is to prepare everything to jump from machine mode into supervisor mode
/// and begin to initialise the kernel.
///
/// # Safety
///
/// This funtion is marked as unsafe because it uses inline assembly, which cannot be checked by
/// rust's compiler.
pub unsafe fn start() -> ! {
    // Prepares the state to run the `main` function in supervisor mode upon calling `mret`.
    mstatus::set_mpp(mstatus::MPP::Supervisor);
    mepc::write(main as usize);

    // Disables paging for now as there's no notion of virtual memory yet.
    satp::write(0);

    // Gives supervisor mode unrestricted access to all the memory.
    pmpaddr0::write(0x3f_ffff_ffff_ffff);
    pmpcfg0::set_pmp(0, Range::TOR, Permission::RWX, false);

    // Stores the contents of the `mhartid` register into the `tp` register to be able to identify
    // the current hart in supervisor mode.
    let id = mhartid::read();
    asm!("mv tp, {0}", in(reg) id);

    // "Returns" to supervisor mode and begins executing the kernel's `main` function.
    asm!("mret");

    extern "C" {
        fn main() -> !;
    }

    unreachable_unchecked();
}
