#![allow(dead_code)]
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmcFid {
    RmiVersion             = 0xc4000150,
    RmiGranuleDelegate     = 0xc4000151,
    RmiGranuleUndelegate   = 0xc4000152,
    RmiDataCreate          = 0xc4000153,
    RmiDataCreateUnknown   = 0xc4000154,
    RmiDataDestroy         = 0xc4000155,
    RmiRealmActivate       = 0xc4000157,
    RmiRealmCreate         = 0xc4000158,
    RmiRealmDestroy        = 0xc4000159,
    RmiRecCreate           = 0xc400015a,
    RmiRecDestroy          = 0xc400015b,
    RmiRecEnter            = 0xc400015c,
    RmiRttCreate           = 0xc400015d,
    RmiRttDestroy          = 0xc400015e,
    RmiRttMapUnprotected   = 0xc400015f,
    RmiRttReadEntry        = 0xc4000161,
    RmiRttUnmapUnprotected = 0xc4000162,
    RmiPsciComplete        = 0xc4000164,
    RmiFeatures            = 0xc4000165,
    RmiRttFold             = 0xc4000166,
    RmiRecAuxCount         = 0xc4000167,
    RmiRttInitRipas        = 0xc4000168,
    RmiRttSetRipas         = 0xc4000169,
    SmcRmiDevMap           = 0xc4000172,
    SmcRmiDevUnmap         = 0xc4000173,
}



pub fn smc_version(x0: usize) {
    smc_call!(SmcFid::RmiVersion as usize, x0);
}

pub fn smc_granule_delegate(granule_address: usize) {
    smc_call!(SmcFid::RmiGranuleDelegate as usize, granule_address);
}

pub fn smc_granule_undelegate(granule_address: usize) {
    smc_call!(SmcFid::RmiGranuleUndelegate as usize, granule_address);
}

pub fn smc_data_create(rd: usize, data: usize, ipa: usize, src: usize) {
    smc_call!(SmcFid::RmiDataCreate as usize, rd, data, ipa, src);
}

pub fn smc_data_create_unknown(rd: usize, data: usize, ipa: usize) {
    smc_call!(SmcFid::RmiDataCreateUnknown as usize, rd, data, ipa);
}

pub fn smc_data_destroy(x0: usize, x1: usize) {
    smc_call!(SmcFid::RmiDataDestroy as usize, x0, x1);
}

pub fn smc_realm_activate(rd: usize) {
    smc_call!(SmcFid::RmiRealmActivate as usize, rd);
}

pub fn smc_realm_create(rd: usize, realm_params: usize) {
    smc_call!(SmcFid::RmiRealmCreate as usize, rd, realm_params);
}

pub fn smc_realm_destroy(x0: usize) {
    smc_call!(SmcFid::RmiRealmDestroy as usize, x0);
}

pub fn smc_rec_create(rd: usize, rec: usize, rec_params: usize) {
    smc_call!(SmcFid::RmiRecCreate as usize, rd, rec, rec_params);
}

pub fn smc_rec_destroy(x0: usize) {
    smc_call!(SmcFid::RmiRecDestroy as usize, x0);
}

pub fn smc_rec_enter(rec: usize, rec_run: usize) {
    smc_call!(SmcFid::RmiRecEnter as usize, rec, rec_run);
}

pub fn smc_rtt_create(x0: usize, x1: usize, x2: usize, x3: usize) {
    smc_call!(SmcFid::RmiRttCreate as usize, x0, x1, x2, x3);
}

pub fn smc_rtt_destroy(x0: usize, x1: usize, x2: usize) {
    smc_call!(SmcFid::RmiRttDestroy as usize, x0, x1, x2);
}

pub fn smc_rtt_map_unprotected(x0: usize, x1: usize, x2: usize, x3: usize) {
    smc_call!(SmcFid::RmiRttMapUnprotected as usize, x0, x1, x2, x3);
}

pub fn smc_rtt_read_entry(x0: usize, x1: usize, x2: usize) {
    smc_call!(SmcFid::RmiRttReadEntry as usize, x0, x1, x2);
}

pub fn smc_rtt_unmap_unprotected(x0: usize, x1: usize, x2: usize) {
    smc_call!(SmcFid::RmiRttUnmapUnprotected as usize, x0, x1, x2);
}

pub fn smc_psci_complete(x0: usize, x1: usize, x2: usize) {
    smc_call!(SmcFid::RmiPsciComplete as usize, x0, x1, x2);
}

pub fn smc_features(x0: usize) {
    smc_call!(SmcFid::RmiFeatures as usize, x0);
}

pub fn smc_rtt_fold(x0: usize, x1: usize, x2: usize) {
    smc_call!(SmcFid::RmiRttFold as usize, x0, x1, x2);
}

pub fn smc_rec_aux_count(x0: usize) {
    smc_call!(SmcFid::RmiRecAuxCount as usize, x0);
}

pub fn smc_rtt_init_ripas(rd: usize, base: usize, top: usize) {
    smc_call!(SmcFid::RmiRttInitRipas as usize, rd, base, top);
}

pub fn smc_rtt_set_ripas(x0: usize, x1: usize, x2: usize, x3: usize) {
    smc_call!(SmcFid::RmiRttSetRipas as usize, x0, x1, x2, x3);
}

pub fn smc_smc_rmi_dev_map(x0: usize, x1: usize, x2: usize, x3: usize) {
    smc_call!(SmcFid::SmcRmiDevMap as usize, x0, x1, x2, x3);
}

pub fn smc_smc_rmi_dev_unmap(x0: usize, x1: usize, x2: usize, x3: usize) {
    smc_call!(SmcFid::SmcRmiDevUnmap as usize, x0, x1, x2, x3);
}