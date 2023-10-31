#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub dt: DT,
    #[doc = "0x04 - Common data register"]
    pub cdt: CDT,
    #[doc = "0x08 - Control register"]
    pub ctrl: CTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Initial data register"]
    pub idt: IDT,
}
#[doc = "DT (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Data register"]
pub mod dt;
#[doc = "CDT (rw) register accessor: Common data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cdt`]
module"]
pub type CDT = crate::Reg<cdt::CDT_SPEC>;
#[doc = "Common data register"]
pub mod cdt;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "IDT (rw) register accessor: Initial data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idt`]
module"]
pub type IDT = crate::Reg<idt::IDT_SPEC>;
#[doc = "Initial data register"]
pub mod idt;
