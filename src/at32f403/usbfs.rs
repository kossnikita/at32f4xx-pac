#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ept0: EPT0,
    ept1: EPT1,
    ept2: EPT2,
    ept3: EPT3,
    ept4: EPT4,
    ept5: EPT5,
    ept6: EPT6,
    ept7: EPT7,
    _reserved8: [u8; 0x20],
    ctrl: CTRL,
    intsts: INTSTS,
    sofrnum: SOFRNUM,
    devaddr: DEVADDR,
    buftbl: BUFTBL,
}
impl RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    #[inline(always)]
    pub const fn ept0(&self) -> &EPT0 {
        &self.ept0
    }
    #[doc = "0x04 - endpoint 1 register"]
    #[inline(always)]
    pub const fn ept1(&self) -> &EPT1 {
        &self.ept1
    }
    #[doc = "0x08 - endpoint 2 register"]
    #[inline(always)]
    pub const fn ept2(&self) -> &EPT2 {
        &self.ept2
    }
    #[doc = "0x0c - endpoint 3 register"]
    #[inline(always)]
    pub const fn ept3(&self) -> &EPT3 {
        &self.ept3
    }
    #[doc = "0x10 - endpoint 4 register"]
    #[inline(always)]
    pub const fn ept4(&self) -> &EPT4 {
        &self.ept4
    }
    #[doc = "0x14 - endpoint 5 register"]
    #[inline(always)]
    pub const fn ept5(&self) -> &EPT5 {
        &self.ept5
    }
    #[doc = "0x18 - endpoint 6 register"]
    #[inline(always)]
    pub const fn ept6(&self) -> &EPT6 {
        &self.ept6
    }
    #[doc = "0x1c - endpoint 7 register"]
    #[inline(always)]
    pub const fn ept7(&self) -> &EPT7 {
        &self.ept7
    }
    #[doc = "0x40 - control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x44 - interrupt status register"]
    #[inline(always)]
    pub const fn intsts(&self) -> &INTSTS {
        &self.intsts
    }
    #[doc = "0x48 - frame number register"]
    #[inline(always)]
    pub const fn sofrnum(&self) -> &SOFRNUM {
        &self.sofrnum
    }
    #[doc = "0x4c - device address"]
    #[inline(always)]
    pub const fn devaddr(&self) -> &DEVADDR {
        &self.devaddr
    }
    #[doc = "0x50 - Buffer table address"]
    #[inline(always)]
    pub const fn buftbl(&self) -> &BUFTBL {
        &self.buftbl
    }
}
#[doc = "EPT0 (rw) register accessor: endpoint 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept0`]
module"]
pub type EPT0 = crate::Reg<ept0::EPT0_SPEC>;
#[doc = "endpoint 0 register"]
pub mod ept0;
#[doc = "EPT1 (rw) register accessor: endpoint 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept1`]
module"]
pub type EPT1 = crate::Reg<ept1::EPT1_SPEC>;
#[doc = "endpoint 1 register"]
pub mod ept1;
#[doc = "EPT2 (rw) register accessor: endpoint 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept2`]
module"]
pub type EPT2 = crate::Reg<ept2::EPT2_SPEC>;
#[doc = "endpoint 2 register"]
pub mod ept2;
#[doc = "EPT3 (rw) register accessor: endpoint 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept3`]
module"]
pub type EPT3 = crate::Reg<ept3::EPT3_SPEC>;
#[doc = "endpoint 3 register"]
pub mod ept3;
#[doc = "EPT4 (rw) register accessor: endpoint 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept4`]
module"]
pub type EPT4 = crate::Reg<ept4::EPT4_SPEC>;
#[doc = "endpoint 4 register"]
pub mod ept4;
#[doc = "EPT5 (rw) register accessor: endpoint 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept5`]
module"]
pub type EPT5 = crate::Reg<ept5::EPT5_SPEC>;
#[doc = "endpoint 5 register"]
pub mod ept5;
#[doc = "EPT6 (rw) register accessor: endpoint 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept6`]
module"]
pub type EPT6 = crate::Reg<ept6::EPT6_SPEC>;
#[doc = "endpoint 6 register"]
pub mod ept6;
#[doc = "EPT7 (rw) register accessor: endpoint 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ept7`]
module"]
pub type EPT7 = crate::Reg<ept7::EPT7_SPEC>;
#[doc = "endpoint 7 register"]
pub mod ept7;
#[doc = "CTRL (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "control register"]
pub mod ctrl;
#[doc = "INTSTS (rw) register accessor: interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts`]
module"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "interrupt status register"]
pub mod intsts;
#[doc = "SOFRNUM (rw) register accessor: frame number register\n\nYou can [`read`](crate::Reg::read) this register and get [`sofrnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofrnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sofrnum`]
module"]
pub type SOFRNUM = crate::Reg<sofrnum::SOFRNUM_SPEC>;
#[doc = "frame number register"]
pub mod sofrnum;
#[doc = "DEVADDR (rw) register accessor: device address\n\nYou can [`read`](crate::Reg::read) this register and get [`devaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devaddr`]
module"]
pub type DEVADDR = crate::Reg<devaddr::DEVADDR_SPEC>;
#[doc = "device address"]
pub mod devaddr;
#[doc = "BUFTBL (rw) register accessor: Buffer table address\n\nYou can [`read`](crate::Reg::read) this register and get [`buftbl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buftbl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buftbl`]
module"]
pub type BUFTBL = crate::Reg<buftbl::BUFTBL_SPEC>;
#[doc = "Buffer table address"]
pub mod buftbl;
