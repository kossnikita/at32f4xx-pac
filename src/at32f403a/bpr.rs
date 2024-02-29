#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dt: [DT; 10],
    rtccal: RTCCAL,
    ctrl: CTRL,
    ctrlsts: CTRLSTS,
    _reserved4: [u8; 0x08],
    dt11: DT,
    dt12: DT,
    dt13: DT,
    dt14: DT,
    dt15: DT,
    dt16: DT,
    dt17: DT,
    dt18: DT,
    dt19: DT,
    dt20: DT,
    dt21: DT,
    dt22: DT,
    dt23: DT,
    dt24: DT,
    dt25: DT,
    dt26: DT,
    dt27: DT,
    dt28: DT,
    dt29: DT,
    dt30: DT,
    dt31: DT,
    dt32: DT,
    dt33: DT,
    dt34: DT,
    dt35: DT,
    dt36: DT,
    dt37: DT,
    dt38: DT,
    dt39: DT,
    dt40: DT,
    dt41: DT,
    dt42: DT,
}
impl RegisterBlock {
    #[doc = "0x00..0x28 - Battery powered domain data register %s"]
    #[inline(always)]
    pub const fn dt(&self, n: usize) -> &DT {
        &self.dt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x28 - Battery powered domain data register %s"]
    #[inline(always)]
    pub fn dt_iter(&self) -> impl Iterator<Item = &DT> {
        self.dt.iter()
    }
    #[doc = "0x00 - Battery powered domain data register 1"]
    #[inline(always)]
    pub const fn dt1(&self) -> &DT {
        self.dt(0)
    }
    #[doc = "0x04 - Battery powered domain data register 2"]
    #[inline(always)]
    pub const fn dt2(&self) -> &DT {
        self.dt(1)
    }
    #[doc = "0x08 - Battery powered domain data register 3"]
    #[inline(always)]
    pub const fn dt3(&self) -> &DT {
        self.dt(2)
    }
    #[doc = "0x0c - Battery powered domain data register 4"]
    #[inline(always)]
    pub const fn dt4(&self) -> &DT {
        self.dt(3)
    }
    #[doc = "0x10 - Battery powered domain data register 5"]
    #[inline(always)]
    pub const fn dt5(&self) -> &DT {
        self.dt(4)
    }
    #[doc = "0x14 - Battery powered domain data register 6"]
    #[inline(always)]
    pub const fn dt6(&self) -> &DT {
        self.dt(5)
    }
    #[doc = "0x18 - Battery powered domain data register 7"]
    #[inline(always)]
    pub const fn dt7(&self) -> &DT {
        self.dt(6)
    }
    #[doc = "0x1c - Battery powered domain data register 8"]
    #[inline(always)]
    pub const fn dt8(&self) -> &DT {
        self.dt(7)
    }
    #[doc = "0x20 - Battery powered domain data register 9"]
    #[inline(always)]
    pub const fn dt9(&self) -> &DT {
        self.dt(8)
    }
    #[doc = "0x24 - Battery powered domain data register 10"]
    #[inline(always)]
    pub const fn dt10(&self) -> &DT {
        self.dt(9)
    }
    #[doc = "0x28 - RTC clock calibration register (BPR_RTCCAL)"]
    #[inline(always)]
    pub const fn rtccal(&self) -> &RTCCAL {
        &self.rtccal
    }
    #[doc = "0x2c - BPR control register (BPR_CTRL)"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x30 - BPR control/status register (BPR_CTRLSTS)"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &CTRLSTS {
        &self.ctrlsts
    }
    #[doc = "0x3c - Battery powered domain data register 11"]
    #[inline(always)]
    pub const fn dt11(&self) -> &DT {
        &self.dt11
    }
    #[doc = "0x3c - Battery powered domain data register 12"]
    #[inline(always)]
    pub const fn dt12(&self) -> &DT {
        &self.dt12
    }
    #[doc = "0x3c - Battery powered domain data register 13"]
    #[inline(always)]
    pub const fn dt13(&self) -> &DT {
        &self.dt13
    }
    #[doc = "0x3c - Battery powered domain data register 14"]
    #[inline(always)]
    pub const fn dt14(&self) -> &DT {
        &self.dt14
    }
    #[doc = "0x3c - Battery powered domain data register 15"]
    #[inline(always)]
    pub const fn dt15(&self) -> &DT {
        &self.dt15
    }
    #[doc = "0x3c - Battery powered domain data register 16"]
    #[inline(always)]
    pub const fn dt16(&self) -> &DT {
        &self.dt16
    }
    #[doc = "0x3c - Battery powered domain data register 17"]
    #[inline(always)]
    pub const fn dt17(&self) -> &DT {
        &self.dt17
    }
    #[doc = "0x3c - Battery powered domain data register 18"]
    #[inline(always)]
    pub const fn dt18(&self) -> &DT {
        &self.dt18
    }
    #[doc = "0x3c - Battery powered domain data register 19"]
    #[inline(always)]
    pub const fn dt19(&self) -> &DT {
        &self.dt19
    }
    #[doc = "0x3c - Battery powered domain data register 20"]
    #[inline(always)]
    pub const fn dt20(&self) -> &DT {
        &self.dt20
    }
    #[doc = "0x3c - Battery powered domain data register 21"]
    #[inline(always)]
    pub const fn dt21(&self) -> &DT {
        &self.dt21
    }
    #[doc = "0x3c - Battery powered domain data register 22"]
    #[inline(always)]
    pub const fn dt22(&self) -> &DT {
        &self.dt22
    }
    #[doc = "0x3c - Battery powered domain data register 23"]
    #[inline(always)]
    pub const fn dt23(&self) -> &DT {
        &self.dt23
    }
    #[doc = "0x3c - Battery powered domain data register 24"]
    #[inline(always)]
    pub const fn dt24(&self) -> &DT {
        &self.dt24
    }
    #[doc = "0x3c - Battery powered domain data register 25"]
    #[inline(always)]
    pub const fn dt25(&self) -> &DT {
        &self.dt25
    }
    #[doc = "0x3c - Battery powered domain data register 26"]
    #[inline(always)]
    pub const fn dt26(&self) -> &DT {
        &self.dt26
    }
    #[doc = "0x3c - Battery powered domain data register 27"]
    #[inline(always)]
    pub const fn dt27(&self) -> &DT {
        &self.dt27
    }
    #[doc = "0x3c - Battery powered domain data register 28"]
    #[inline(always)]
    pub const fn dt28(&self) -> &DT {
        &self.dt28
    }
    #[doc = "0x3c - Battery powered domain data register 29"]
    #[inline(always)]
    pub const fn dt29(&self) -> &DT {
        &self.dt29
    }
    #[doc = "0x3c - Battery powered domain data register 30"]
    #[inline(always)]
    pub const fn dt30(&self) -> &DT {
        &self.dt30
    }
    #[doc = "0x3c - Battery powered domain data register 31"]
    #[inline(always)]
    pub const fn dt31(&self) -> &DT {
        &self.dt31
    }
    #[doc = "0x3c - Battery powered domain data register 32"]
    #[inline(always)]
    pub const fn dt32(&self) -> &DT {
        &self.dt32
    }
    #[doc = "0x3c - Battery powered domain data register 33"]
    #[inline(always)]
    pub const fn dt33(&self) -> &DT {
        &self.dt33
    }
    #[doc = "0x3c - Battery powered domain data register 34"]
    #[inline(always)]
    pub const fn dt34(&self) -> &DT {
        &self.dt34
    }
    #[doc = "0x3c - Battery powered domain data register 35"]
    #[inline(always)]
    pub const fn dt35(&self) -> &DT {
        &self.dt35
    }
    #[doc = "0x3c - Battery powered domain data register 36"]
    #[inline(always)]
    pub const fn dt36(&self) -> &DT {
        &self.dt36
    }
    #[doc = "0x3c - Battery powered domain data register 37"]
    #[inline(always)]
    pub const fn dt37(&self) -> &DT {
        &self.dt37
    }
    #[doc = "0x3c - Battery powered domain data register 38"]
    #[inline(always)]
    pub const fn dt38(&self) -> &DT {
        &self.dt38
    }
    #[doc = "0x3c - Battery powered domain data register 39"]
    #[inline(always)]
    pub const fn dt39(&self) -> &DT {
        &self.dt39
    }
    #[doc = "0x3c - Battery powered domain data register 40"]
    #[inline(always)]
    pub const fn dt40(&self) -> &DT {
        &self.dt40
    }
    #[doc = "0x3c - Battery powered domain data register 41"]
    #[inline(always)]
    pub const fn dt41(&self) -> &DT {
        &self.dt41
    }
    #[doc = "0x3c - Battery powered domain data register 42"]
    #[inline(always)]
    pub const fn dt42(&self) -> &DT {
        &self.dt42
    }
}
#[doc = "DT (rw) register accessor: Battery powered domain data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Battery powered domain data register %s"]
pub mod dt;
pub use dt as dt11;
pub use DT as DT11;
#[doc = "RTCCAL (rw) register accessor: RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccal`]
module"]
pub type RTCCAL = crate::Reg<rtccal::RTCCAL_SPEC>;
#[doc = "RTC clock calibration register (BPR_RTCCAL)"]
pub mod rtccal;
#[doc = "CTRL (rw) register accessor: BPR control register (BPR_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "BPR control register (BPR_CTRL)"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "BPR control/status register (BPR_CTRLSTS)"]
pub mod ctrlsts;
