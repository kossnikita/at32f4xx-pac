#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: STS,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    c1: C1,
    _reserved4: [u8; 0x02],
    c2: C2,
    _reserved5: [u8; 0x02],
    c3: C3,
}
impl RegisterBlock {
    #[doc = "0x00 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x08 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x0c - compare value 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x10 - compare value 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x14 - compare value 3"]
    #[inline(always)]
    pub const fn c3(&self) -> &C3 {
        &self.c3
    }
}
#[doc = "STS (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "C1 (rw) register accessor: compare value 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1_SPEC>;
#[doc = "compare value 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: compare value 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2_SPEC>;
#[doc = "compare value 2"]
pub mod c2;
#[doc = "C3 (rw) register accessor: compare value 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`]
module"]
pub type C3 = crate::Reg<c3::C3_SPEC>;
#[doc = "compare value 3"]
pub mod c3;
