#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dt: DT,
    cdt: CDT,
    ctrl: CTRL,
    _reserved3: [u8; 0x04],
    idt: IDT,
    poly: POLY,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &DT {
        &self.dt
    }
    #[doc = "0x04 - Common data register"]
    #[inline(always)]
    pub const fn cdt(&self) -> &CDT {
        &self.cdt
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x10 - Initial data register"]
    #[inline(always)]
    pub const fn idt(&self) -> &IDT {
        &self.idt
    }
    #[doc = "0x14 - Polynomial coefficient register"]
    #[inline(always)]
    pub const fn poly(&self) -> &POLY {
        &self.poly
    }
}
#[doc = "DT (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Data register"]
pub mod dt;
#[doc = "CDT (rw) register accessor: Common data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdt`]
module"]
pub type CDT = crate::Reg<cdt::CDT_SPEC>;
#[doc = "Common data register"]
pub mod cdt;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "IDT (rw) register accessor: Initial data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idt`]
module"]
pub type IDT = crate::Reg<idt::IDT_SPEC>;
#[doc = "Initial data register"]
pub mod idt;
#[doc = "POLY (rw) register accessor: Polynomial coefficient register\n\nYou can [`read`](crate::Reg::read) this register and get [`poly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`]
module"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "Polynomial coefficient register"]
pub mod poly;
