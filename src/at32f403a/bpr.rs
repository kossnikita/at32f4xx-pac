#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x28 - Battery powered domain data register %s"]
    pub dt: [DT; 10],
    #[doc = "0x28 - RTC clock calibration register (BPR_RTCCAL)"]
    pub rtccal: RTCCAL,
    #[doc = "0x2c - BPR control register (BPR_CTRL)"]
    pub ctrl: CTRL,
    #[doc = "0x30 - BPR control/status register (BPR_CTRLSTS)"]
    pub ctrlsts: CTRLSTS,
    _reserved4: [u8; 0x08],
    #[doc = "0x3c - Battery powered domain data register %s"]
    pub dt11: DT11,
    #[doc = "0x40 - Battery powered domain data register %s"]
    pub dt12: DT11,
    #[doc = "0x44 - Battery powered domain data register %s"]
    pub dt13: DT11,
    #[doc = "0x48 - Battery powered domain data register %s"]
    pub dt14: DT11,
    #[doc = "0x4c - Battery powered domain data register %s"]
    pub dt15: DT11,
    #[doc = "0x50 - Battery powered domain data register %s"]
    pub dt16: DT11,
    #[doc = "0x54 - Battery powered domain data register %s"]
    pub dt17: DT11,
    #[doc = "0x58 - Battery powered domain data register %s"]
    pub dt18: DT11,
    #[doc = "0x5c - Battery powered domain data register %s"]
    pub dt19: DT11,
    #[doc = "0x60 - Battery powered domain data register %s"]
    pub dt20: DT11,
    #[doc = "0x64 - Battery powered domain data register %s"]
    pub dt21: DT11,
    #[doc = "0x68 - Battery powered domain data register %s"]
    pub dt22: DT11,
    #[doc = "0x6c - Battery powered domain data register %s"]
    pub dt23: DT11,
    #[doc = "0x70 - Battery powered domain data register %s"]
    pub dt24: DT11,
    #[doc = "0x74 - Battery powered domain data register %s"]
    pub dt25: DT11,
    #[doc = "0x78 - Battery powered domain data register %s"]
    pub dt26: DT11,
    #[doc = "0x7c - Battery powered domain data register %s"]
    pub dt27: DT11,
    #[doc = "0x80 - Battery powered domain data register %s"]
    pub dt28: DT11,
    #[doc = "0x84 - Battery powered domain data register %s"]
    pub dt29: DT11,
    #[doc = "0x88 - Battery powered domain data register %s"]
    pub dt30: DT11,
    #[doc = "0x8c - Battery powered domain data register %s"]
    pub dt31: DT11,
    #[doc = "0x90 - Battery powered domain data register %s"]
    pub dt32: DT11,
    #[doc = "0x94 - Battery powered domain data register %s"]
    pub dt33: DT11,
    #[doc = "0x98 - Battery powered domain data register %s"]
    pub dt34: DT11,
    #[doc = "0x9c - Battery powered domain data register %s"]
    pub dt35: DT11,
    #[doc = "0xa0 - Battery powered domain data register %s"]
    pub dt36: DT11,
    #[doc = "0xa4 - Battery powered domain data register %s"]
    pub dt37: DT11,
    #[doc = "0xa8 - Battery powered domain data register %s"]
    pub dt38: DT11,
    #[doc = "0xac - Battery powered domain data register %s"]
    pub dt39: DT11,
    #[doc = "0xb0 - Battery powered domain data register %s"]
    pub dt40: DT11,
    #[doc = "0xb4 - Battery powered domain data register %s"]
    pub dt41: DT11,
    #[doc = "0xb8 - Battery powered domain data register %s"]
    pub dt42: DT11,
}
impl RegisterBlock {
    #[doc = "0x00 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt1(&self) -> &DT {
        &self.dt[0]
    }
    #[doc = "0x04 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt2(&self) -> &DT {
        &self.dt[1]
    }
    #[doc = "0x08 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt3(&self) -> &DT {
        &self.dt[2]
    }
    #[doc = "0x0c - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt4(&self) -> &DT {
        &self.dt[3]
    }
    #[doc = "0x10 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt5(&self) -> &DT {
        &self.dt[4]
    }
    #[doc = "0x14 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt6(&self) -> &DT {
        &self.dt[5]
    }
    #[doc = "0x18 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt7(&self) -> &DT {
        &self.dt[6]
    }
    #[doc = "0x1c - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt8(&self) -> &DT {
        &self.dt[7]
    }
    #[doc = "0x20 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt9(&self) -> &DT {
        &self.dt[8]
    }
    #[doc = "0x24 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt10(&self) -> &DT {
        &self.dt[9]
    }
}
#[doc = "DT (rw) register accessor: Battery powered domain data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Battery powered domain data register %s"]
pub mod dt;
pub use dt as dt11;
pub use DT as DT11;
#[doc = "RTCCAL (rw) register accessor: RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtccal`]
module"]
pub type RTCCAL = crate::Reg<rtccal::RTCCAL_SPEC>;
#[doc = "RTC clock calibration register (BPR_RTCCAL)"]
pub mod rtccal;
#[doc = "CTRL (rw) register accessor: BPR control register (BPR_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "BPR control register (BPR_CTRL)"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "BPR control/status register (BPR_CTRLSTS)"]
pub mod ctrlsts;
