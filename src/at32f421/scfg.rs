#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg1: CFG1,
    _reserved1: [u8; 0x04],
    exintc1: EXINTC1,
    exintc2: EXINTC2,
    exintc3: EXINTC3,
    exintc4: EXINTC4,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exintc1(&self) -> &EXINTC1 {
        &self.exintc1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn exintc2(&self) -> &EXINTC2 {
        &self.exintc2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn exintc3(&self) -> &EXINTC3 {
        &self.exintc3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn exintc4(&self) -> &EXINTC4 {
        &self.exintc4
    }
}
#[doc = "CFG1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "configuration register 1"]
pub mod cfg1;
#[doc = "EXINTC1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc1`] module"]
pub type EXINTC1 = crate::Reg<exintc1::EXINTC1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exintc1;
#[doc = "EXINTC2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc2`] module"]
pub type EXINTC2 = crate::Reg<exintc2::EXINTC2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exintc2;
#[doc = "EXINTC3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc3`] module"]
pub type EXINTC3 = crate::Reg<exintc3::EXINTC3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exintc3;
#[doc = "EXINTC4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exintc4`] module"]
pub type EXINTC4 = crate::Reg<exintc4::EXINTC4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exintc4;
