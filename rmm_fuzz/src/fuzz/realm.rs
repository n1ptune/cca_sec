#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmiRealmParams {
    pub flags0: u64,                // 0x0
    pub s2sz: u32,                  // 0x8
    pub sve_vl: u32,                // 0x10
    pub num_bps: u32,               // 0x18
    pub num_wps: u32,               // 0x20
    pub pmu_num_ctrs: u32,          // 0x28
    pub algorithm: u8,              // 0x30
    _pad0: [u8; 7],                 // 0x31 ~ 0x38 (padding to 8 bytes)
    pub num_aux_planes: u32,        // 0x38
    _pad1: [u8; 0x400 - 0x3C],      // 0x3C ~ 0x400 (padding)
    pub rpv: [u8; 0x400],           // 0x400
    pub vm_info: VmInfo,            // 0x800
    pub flags1: u64,                // 0x820
    pub mecid: i64,                 // 0x828
    _pad2: [u8; 0xF00 - 0x830],     // 0x830 ~ 0xF00 (padding)
    pub aux_vmid: [u16; 3],         // 0xF00
    _pad3: [u8; 0xF80 - 0xF06],     // 0xF06 ~ 0xF80 (padding)
    pub aux_rtt_base: [u64; 3],     // 0xF80
    _pad4: [u8; 0x1000 - 0xF98],    // 0xF98 ~ 0x1000 (padding)
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VmInfo {
    pub vmid: u16,                  // 0x800
    _pad0: [u8; 6],                 // 0x802 ~ 0x808 (padding)
    pub rtt_base: usize,              // 0x808
    pub rtt_level_start: i64,       // 0x810
    pub rtt_num_start: u32,         // 0x818
    _pad1: [u8; 0x820 - 0x81C],     // 0x81C ~ 0x820 (padding)
}

impl RmiRealmParams {
    pub fn from_usize<'a>(addr: usize) -> &'a mut Self {
        unsafe { &mut *(addr as *mut RmiRealmParams) }
    }

    pub fn into_usize(self:&mut Self) -> usize {
        self as *mut RmiRealmParams as usize
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmiRecParams {
    pub flags: u64,                       // 0x000 ~ 0x100
    _pad0: [u8; 0x100 - 8],               // 填充到 0x100
    pub mpidr: u64,                       // 0x100 ~ 0x200
    _pad1: [u8; 0x200 - 0x100 - 8],       // 填充到 0x200
    pub pc: u64,                          // 0x200 ~ 0x300
    _pad2: [u8; 0x300 - 0x200 - 8],       // 填充到 0x300
    pub gprs: [u64; 8],                   // 0x300 ~ 0x340 (8*8=64字节)
    _pad3: [u8; 0x800 - 0x300 - 8*8],     // 填充到 0x800
    pub num_aux: u64,                     // 0x800 ~ 0x808
    pub aux: [u64; 16],                   // 0x808 ~ 0x888 (16*8=128字节)
    _pad4: [u8; 0x1000 - 0x808 - 8*16],   // 填充到 0x1000
}

impl RmiRecParams {
    pub fn from_usize<'a>(addr: usize) -> &'a mut Self {
        unsafe { &mut *(addr as *mut RmiRecParams) }
    }

    pub fn into_usize(self:&mut Self) -> usize {
        self as *mut RmiRecParams as usize
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmiRecEntry {
    pub flags: u64,                        // 0x000 ~ 0x200
    _pad0: [u8; 0x200 - 8],                // 填充到 0x200
    pub gprs: [u64; 31],                   // 0x200 ~ 0x300 (31*8=248字节)
    _pad1: [u8; 0x300 - 0x200 - 8*31],     // 填充到 0x300
    pub gicv3_hcr: u64,                    // 0x300 ~ 0x308
    pub gicv3_lrs: [u64; 16],              // 0x308 ~ 0x388 (16*8=128字节)
    _pad2: [u8; 0x800 - 0x308 - 8*16],     // 填充到 0x800
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmiRecExit {
    pub exit_reason: u64,                  // 0x000 ~ 0x100
    _pad0: [u8; 0x100 - 8],                // 填充到 0x100
    pub esr: u64,                          // 0x100 ~ 0x108
    pub far: u64,                          // 0x108 ~ 0x110
    pub hpfar: u64,                        // 0x110 ~ 0x118
    _pad1: [u8; 0x118 - 0x110 - 8],        // 填充到 0x118
    pub rtt_tree: u64,                     // 0x118 ~ 0x120
    pub rtt_level: u64,                    // 0x120 ~ 0x128
    _pad2: [u8; 0x200 - 0x128],            // 填充到 0x200
    pub gprs: [u64; 31],                   // 0x200 ~ 0x300
    pub gicv3_hcr: u64,                    // 0x300 ~ 0x308
    pub gicv3_lrs: [u64; 16],              // 0x308 ~ 0x388
    pub gicv3_misr: u64,                   // 0x388 ~ 0x390
    pub gicv3_vmcr: u64,                   // 0x390 ~ 0x398
    _pad3: [u8; 0x400 - 0x398],            // 填充到 0x400
    pub cntp_ctl: u64,                     // 0x400 ~ 0x408
    pub cntp_cval: u64,                    // 0x408 ~ 0x410
    pub cntv_ctl: u64,                     // 0x410 ~ 0x418
    pub cntv_cval: u64,                    // 0x418 ~ 0x420
    _pad4: [u8; 0x500 - 0x420],            // 填充到 0x500
    pub ripas_base: u64,                   // 0x500 ~ 0x508
    pub ripas_size: u64,                   // 0x508 ~ 0x510
    pub ripas_value: u8,                   // 0x510 ~ 0x511
    _pad5: [u8; 0x518 - 0x511],            // 填充到 0x518
    pub ripas_io_pa: u64,                  // 0x518 ~ 0x520
    pub s2ap_base: u64,                    // 0x520 ~ 0x528
    pub s2ap_top: u64,                     // 0x528 ~ 0x530
    _pad6: [u8; 0x600 - 0x530],            // 填充到 0x600
    pub imm: u32,                          // 0x600 ~ 0x604
    _pad7: [u8; 0x700 - 0x604],            // 填充到 0x700
    pub pmu_ovf_status: u64,               // 0x700 ~ 0x708
    _pad8: [u8; 0x800 - 0x708],            // 填充到 0x800
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmiRecRun {
    pub entry: RmiRecEntry,                // 0x000 ~ 0x800
    pub exit: RmiRecExit,                  // 0x800 ~ 0x1000
}