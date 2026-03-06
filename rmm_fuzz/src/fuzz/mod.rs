#[macro_use]
mod smc_call;
mod rmi;
mod realm;

use alloc::vec;
use alloc::vec::Vec;
use core::slice;
pub use realm::*;
pub use rmi::*;
use crate::mem::*;
pub fn done_work(val:u64) {
   smc_call::afl_call(4, 0 as *mut u8, val);
}

pub fn get_work() -> Vec<u8> {
   let size = smc_call::afl_call(3, 0 as *mut u8, 0);
   if size < 1957 {
      done_work(0);
   }
   let mut data = vec![0u8; size as usize];
   smc_call::afl_call(2, data.as_mut_ptr(), size);
   data
}

pub fn mem_copy(dst: usize, start: usize, src: &Vec<u8>, size: usize, offset: usize) {
    unsafe {
        core::ptr::copy_nonoverlapping(
            src.as_ptr().add(offset),
            (dst as *mut u8).add(start),
            size.min(src.len().saturating_sub(offset)),
        );
    }
}

pub fn page_to_vec(page:usize) -> Vec<u8> {
    unsafe {
      let ptr = page as *const u8;
      slice::from_raw_parts(ptr, 4096).to_vec()
    }
}

fn fill_realm_params(realm_params: usize, afl_data: &Vec<u8>) {
    // 字段                  目标地址,            目标偏移,  源数据,  大小,   源偏移 (已重算)
    mem_copy(realm_params, 0x0,   afl_data, 8,       0);      // flags0: u64.            源下一字节: 0+8=8
    // s2sz 由外部设置，跳过
    mem_copy(realm_params, 0x10,  afl_data, 4,       8);      // sve_vl: u32.            源下一字节: 8+4=12
    // num_bps 由外部设置，跳过
    // num_wps 由外部设置，跳过
    mem_copy(realm_params, 0x28,  afl_data, 4,       12);     // pmu_num_ctrs: u32.      源下一字节: 12+4=16
    mem_copy(realm_params, 0x30,  afl_data, 1,       16);     // algorithm: u8.          源下一字节: 16+1=17
    mem_copy(realm_params, 0x38,  afl_data, 4,       17);     // num_aux_planes: u32.    源下一字节: 17+4=21
    mem_copy(realm_params, 0x400, afl_data, 1024,    21);     // rpv: [u8; 1024].        源下一字节: 21+1024=1045
    
    // --- 填充嵌套的 VmInfo ---
    mem_copy(realm_params, 0x800, afl_data, 2,       1045);   // vm_info.vmid.           源下一字节: 1045+2=1047
    // vm_info.rtt_base 由外部设置，跳过
    // vm_info.rtt_level_start 由外部设置，跳过
    // vm_info.rtt_num_start 由外部设置，跳过
    
    // --- 继续填充 RmiRealmParams ---
    mem_copy(realm_params, 0x820, afl_data, 8,       1047);   // flags1: u64.            源下一字节: 1047+8=1055
    mem_copy(realm_params, 0x828, afl_data, 8,       1055);   // mecid: i64.             源下一字节: 1055+8=1063
    mem_copy(realm_params, 0xF00, afl_data, 6,       1063);   // aux_vmid: [u16; 3].     源下一字节: 1063+6=1069
    mem_copy(realm_params, 0xF80, afl_data, 24,      1069);   // aux_rtt_base: [u64; 3]. 源下一字节: 1069+24=1093
}

fn fill_rec_params(rec_params: usize, afl_data: &Vec<u8>) {
    // 字段                  目标地址,        目标偏移,  源数据,  大小,   源偏移 (已重算)
    mem_copy(rec_params, 0x0,   afl_data, 8,       1093);   // flags: u64.           源下一字节: 1093+8=1101
    mem_copy(rec_params, 0x100, afl_data, 8,       1101);   // mpidr: u64.           源下一字节: 1101+8=1109
    // pc 由外部设置，跳过
    mem_copy(rec_params, 0x300, afl_data, 64,      1109);   // gprs: [u64; 8].       源下一字节: 1109+64=1173
    mem_copy(rec_params, 0x800, afl_data, 8,       1173);   // num_aux: u64.         源下一字节: 1173+8=1181
    mem_copy(rec_params, 0x808, afl_data, 128,     1181);   // aux: [u64; 16].       源下一字节: 1181+128=1309
}

fn fill_rec_run(rec_run: usize, afl_data: &Vec<u8>) {

   // --- 步骤 1: 填充 RmiRecRun.entry 部分 ---
    // 已根据您的最新指令进行修改：
    // 1. 起始源偏移量从 1325 开始 (1309 + 0x10)。
    // 2. 仅填充 RmiRecRun.entry 部分，并只对已填充字段进行异或。
    
    // 字段                  目标地址,      目标偏移,    源数据,    大小,      源偏移 (已重算)
    mem_copy(rec_run, 0x0,     afl_data, 8,         1325);     // flags: u64.            下一源字节: 1325+8=1333
    mem_copy(rec_run, 0x200,   afl_data, 248,       1333);     // gprs: [u64; 31].       下一源字节: 1333+248=1581
    mem_copy(rec_run, 0x300,   afl_data, 8,         1581);     // gicv3_hcr: u64.        下一源字节: 1581+8=1589
    mem_copy(rec_run, 0x308,   afl_data, 128,       1589);     // gicv3_lrs: [u64; 16].  下一源字节: 1589+128=1717    
}
fn xor_rec_data(rec_run: usize, xor_val: u8){
   // 对 flags 字段 (offset 0x0, size 8) 进行异或
    unsafe {
        let slice = core::slice::from_raw_parts_mut((rec_run as *mut u8).add(0x0), 8);
        for byte in slice {
            *byte ^= xor_val;
        }
    }

    // 对 gprs 字段 (offset 0x200, size 248) 进行异或
    unsafe {
        let slice = core::slice::from_raw_parts_mut((rec_run as *mut u8).add(0x200), 248);
        for byte in slice {
            *byte ^= xor_val;
        }
    }

    // 对 gicv3_hcr 字段 (offset 0x300, size 8) 进行异或
    unsafe {
        let slice = core::slice::from_raw_parts_mut((rec_run as *mut u8).add(0x300), 8);
        for byte in slice {
            *byte ^= xor_val;
        }
    }

    // 对 gicv3_lrs 字段 (offset 0x308, size 128) 进行异或
    unsafe {
        let slice = core::slice::from_raw_parts_mut((rec_run as *mut u8).add(0x308), 128);
        for byte in slice {
            *byte ^= xor_val;
        }
    }
}
fn set_realm_params(realm_params:&mut RmiRealmParams, afl_data: &Vec<u8>) {
   realm_params.s2sz = 32;
   realm_params.vm_info.rtt_num_start = 4;
   realm_params.vm_info.rtt_level_start = 2;
   realm_params.num_bps = 1;
   realm_params.num_wps = 1;
   fill_realm_params(realm_params.into_usize(), afl_data);
}
pub fn realm_fuzz() {
   let afl_data = get_work();

   let rd = alloc_page().unwrap();
   smc_granule_delegate(rd);

   let realm_params = RmiRealmParams::from_usize(alloc_page().unwrap());
   set_realm_params(realm_params, &afl_data);
   let mut rtts = Vec::new();
   for i in 0..realm_params.vm_info.rtt_num_start {
       rtts.push(alloc_page().unwrap());
       smc_granule_delegate(rtts[i as usize]);
   }
   realm_params.vm_info.rtt_base = rtts[0];
   
   smc_realm_create(rd, realm_params.into_usize());

   smc_rtt_init_ripas(rd, 0x1000, 0x1000*10);

   let rec = alloc_page().unwrap();
   smc_granule_delegate(rec);
   let rec_params = alloc_page().unwrap();
   fill_rec_params(rec_params, &afl_data);


   let r_data = alloc_page().unwrap();
   let src = alloc_page().unwrap();
   mem_copy(src, 0,&afl_data, 0x10, 1309);
   smc_granule_delegate(r_data);
   smc_data_create(rd , r_data , 0x1000, src);

   let rec_params = RmiRecParams::from_usize(rec_params);
   let mut rec_aux_granules = Vec::new();
   for i in 0..rec_params.num_aux.min(16) {
      rec_aux_granules.push(alloc_page().unwrap());
      smc_granule_delegate(rec_aux_granules[i as usize]);
   }
   rec_params.pc = 0x1000;

   smc_rec_create(rd, rec, rec_params.into_usize());
   
   smc_realm_activate(rd);

   let mut i = 1;
   let rec_run = alloc_page().unwrap();
   fill_rec_run(rec_run, &afl_data);
   while i < 16 {
      smc_rec_enter(rec, rec_run);
      xor_rec_data(rec_run, i);
      i +=1;
   }

}