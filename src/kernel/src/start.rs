use core::{arch::asm, hint::unreachable_unchecked};

use crate::riscv::registers::{
    mepc, mhartid, mstatus, pmpaddr0,
    pmpcfg0::{self, Permission, Range},
    satp,
};

pub unsafe fn start() -> ! {
    mstatus::set_mpp(mstatus::MPP::Supervisor);

    mepc::write(main as usize);

    satp::write(0);

    pmpaddr0::write(0x3f_ffff_ffff_ffff);
    pmpcfg0::set_pmp(0, Range::TOR, Permission::RWX, false);

    let id = mhartid::read();
    asm!("mv tp, {0}", in(reg) id);

    asm!("mret");

    extern "C" {
        fn main() -> !;
    }

    unreachable_unchecked();
}
