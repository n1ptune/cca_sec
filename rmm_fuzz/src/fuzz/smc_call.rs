
macro_rules! smc_call {
    ($x0:expr) => {
        smc_call!($x0, 0, 0, 0, 0, 0, 0)
    };
    ($x0:expr, $x1:expr) => {
        smc_call!($x0, $x1, 0, 0, 0, 0, 0)
    };
    ($x0:expr, $x1:expr, $x2:expr) => {
        smc_call!($x0, $x1, $x2, 0, 0, 0, 0)
    };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr) => {
        smc_call!($x0, $x1, $x2, $x3, 0, 0, 0)
    };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr) => {
        smc_call!($x0, $x1, $x2, $x3, $x4, 0, 0)
    };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr) => {
        smc_call!($x0, $x1, $x2, $x3, $x4, $x5, 0)
    };
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr) => {
        unsafe {
            core::arch::asm!(
                "smc #0",
                in("x0") $x0,
                in("x1") $x1,
                in("x2") $x2,
                in("x3") $x3,
                in("x4") $x4,
                in("x5") $x5,
                in("x6") $x6,
            );
        }
    };
}

#[inline(always)]
pub fn afl_call(a0: u64, a1: *mut u8, a2: u64) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            ".byte 0xba, 0xdc, 0xcd, 0xab", // 自定义指令
            inout("x0") a0 => ret,          // x0 同时作为输入和输出
            in("x1") a1,                   // a1 直接传入 x1
            in("x2") a2,                   // a2 直接传入 x2
            lateout("x1") _,                   // 明确声明 x1 被修改
            lateout("x2") _,                   // 明确声明 x2 被修改
        );
    }
    ret
}

