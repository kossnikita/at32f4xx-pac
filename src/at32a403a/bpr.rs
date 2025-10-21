#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dt: [DT; 10],
    rtccal: RTCCAL,
    ctrl: CTRL,
    ctrlsts: CTRLSTS,
    _reserved4: [u8; 0x08],
    dt11: DT11,
    dt12: DT11,
    dt13: DT11,
    dt14: DT11,
    dt15: DT11,
    dt16: DT11,
    dt17: DT11,
    dt18: DT11,
    dt19: DT11,
    dt20: DT11,
    dt21: DT11,
    dt22: DT11,
    dt23: DT11,
    dt24: DT11,
    dt25: DT11,
    dt26: DT11,
    dt27: DT11,
    dt28: DT11,
    dt29: DT11,
    dt30: DT11,
    dt31: DT11,
    dt32: DT11,
    dt33: DT11,
    dt34: DT11,
    dt35: DT11,
    dt36: DT11,
    dt37: DT11,
    dt38: DT11,
    dt39: DT11,
    dt40: DT11,
    dt41: DT11,
    dt42: DT11,
}
impl RegisterBlock {
    #[doc = "0x00..0x28 - Battery powered domain data register %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `DT1` register.</div>"]
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
    pub const fn dt11(&self) -> &DT11 {
        &self.dt11
    }
    #[doc = "0x3c - Battery powered domain data register 12"]
    #[inline(always)]
    pub const fn dt12(&self) -> &DT11 {
        &self.dt12
    }
    #[doc = "0x3c - Battery powered domain data register 13"]
    #[inline(always)]
    pub const fn dt13(&self) -> &DT11 {
        &self.dt13
    }
    #[doc = "0x3c - Battery powered domain data register 14"]
    #[inline(always)]
    pub const fn dt14(&self) -> &DT11 {
        &self.dt14
    }
    #[doc = "0x3c - Battery powered domain data register 15"]
    #[inline(always)]
    pub const fn dt15(&self) -> &DT11 {
        &self.dt15
    }
    #[doc = "0x3c - Battery powered domain data register 16"]
    #[inline(always)]
    pub const fn dt16(&self) -> &DT11 {
        &self.dt16
    }
    #[doc = "0x3c - Battery powered domain data register 17"]
    #[inline(always)]
    pub const fn dt17(&self) -> &DT11 {
        &self.dt17
    }
    #[doc = "0x3c - Battery powered domain data register 18"]
    #[inline(always)]
    pub const fn dt18(&self) -> &DT11 {
        &self.dt18
    }
    #[doc = "0x3c - Battery powered domain data register 19"]
    #[inline(always)]
    pub const fn dt19(&self) -> &DT11 {
        &self.dt19
    }
    #[doc = "0x3c - Battery powered domain data register 20"]
    #[inline(always)]
    pub const fn dt20(&self) -> &DT11 {
        &self.dt20
    }
    #[doc = "0x3c - Battery powered domain data register 21"]
    #[inline(always)]
    pub const fn dt21(&self) -> &DT11 {
        &self.dt21
    }
    #[doc = "0x3c - Battery powered domain data register 22"]
    #[inline(always)]
    pub const fn dt22(&self) -> &DT11 {
        &self.dt22
    }
    #[doc = "0x3c - Battery powered domain data register 23"]
    #[inline(always)]
    pub const fn dt23(&self) -> &DT11 {
        &self.dt23
    }
    #[doc = "0x3c - Battery powered domain data register 24"]
    #[inline(always)]
    pub const fn dt24(&self) -> &DT11 {
        &self.dt24
    }
    #[doc = "0x3c - Battery powered domain data register 25"]
    #[inline(always)]
    pub const fn dt25(&self) -> &DT11 {
        &self.dt25
    }
    #[doc = "0x3c - Battery powered domain data register 26"]
    #[inline(always)]
    pub const fn dt26(&self) -> &DT11 {
        &self.dt26
    }
    #[doc = "0x3c - Battery powered domain data register 27"]
    #[inline(always)]
    pub const fn dt27(&self) -> &DT11 {
        &self.dt27
    }
    #[doc = "0x3c - Battery powered domain data register 28"]
    #[inline(always)]
    pub const fn dt28(&self) -> &DT11 {
        &self.dt28
    }
    #[doc = "0x3c - Battery powered domain data register 29"]
    #[inline(always)]
    pub const fn dt29(&self) -> &DT11 {
        &self.dt29
    }
    #[doc = "0x3c - Battery powered domain data register 30"]
    #[inline(always)]
    pub const fn dt30(&self) -> &DT11 {
        &self.dt30
    }
    #[doc = "0x3c - Battery powered domain data register 31"]
    #[inline(always)]
    pub const fn dt31(&self) -> &DT11 {
        &self.dt31
    }
    #[doc = "0x3c - Battery powered domain data register 32"]
    #[inline(always)]
    pub const fn dt32(&self) -> &DT11 {
        &self.dt32
    }
    #[doc = "0x3c - Battery powered domain data register 33"]
    #[inline(always)]
    pub const fn dt33(&self) -> &DT11 {
        &self.dt33
    }
    #[doc = "0x3c - Battery powered domain data register 34"]
    #[inline(always)]
    pub const fn dt34(&self) -> &DT11 {
        &self.dt34
    }
    #[doc = "0x3c - Battery powered domain data register 35"]
    #[inline(always)]
    pub const fn dt35(&self) -> &DT11 {
        &self.dt35
    }
    #[doc = "0x3c - Battery powered domain data register 36"]
    #[inline(always)]
    pub const fn dt36(&self) -> &DT11 {
        &self.dt36
    }
    #[doc = "0x3c - Battery powered domain data register 37"]
    #[inline(always)]
    pub const fn dt37(&self) -> &DT11 {
        &self.dt37
    }
    #[doc = "0x3c - Battery powered domain data register 38"]
    #[inline(always)]
    pub const fn dt38(&self) -> &DT11 {
        &self.dt38
    }
    #[doc = "0x3c - Battery powered domain data register 39"]
    #[inline(always)]
    pub const fn dt39(&self) -> &DT11 {
        &self.dt39
    }
    #[doc = "0x3c - Battery powered domain data register 40"]
    #[inline(always)]
    pub const fn dt40(&self) -> &DT11 {
        &self.dt40
    }
    #[doc = "0x3c - Battery powered domain data register 41"]
    #[inline(always)]
    pub const fn dt41(&self) -> &DT11 {
        &self.dt41
    }
    #[doc = "0x3c - Battery powered domain data register 42"]
    #[inline(always)]
    pub const fn dt42(&self) -> &DT11 {
        &self.dt42
    }
}
#[doc = "DT (rw) register accessor: Battery powered domain data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`] module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Battery powered domain data register %s"]
pub mod dt;
pub use DT as DT11;
pub use dt as dt11;
#[doc = "RTCCAL (rw) register accessor: RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccal`] module"]
pub type RTCCAL = crate::Reg<rtccal::RTCCAL_SPEC>;
#[doc = "RTC clock calibration register (BPR_RTCCAL)"]
pub mod rtccal;
#[doc = "CTRL (rw) register accessor: BPR control register (BPR_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "BPR control register (BPR_CTRL)"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`] module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "BPR control/status register (BPR_CTRLSTS)"]
pub mod ctrlsts;
