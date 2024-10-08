#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg1: CFG1,
    _reserved1: [u8; 0x04],
    extic1: EXTIC1,
    extic2: EXTIC2,
    extic3: EXTIC3,
    extic4: EXTIC4,
    cfg2: CFG2,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn extic1(&self) -> &EXTIC1 {
        &self.extic1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn extic2(&self) -> &EXTIC2 {
        &self.extic2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn extic3(&self) -> &EXTIC3 {
        &self.extic3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn extic4(&self) -> &EXTIC4 {
        &self.extic4
    }
    #[doc = "0x18 - configuration register 2"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
}
#[doc = "CFG1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "configuration register 1"]
pub mod cfg1;
#[doc = "EXTIC1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`extic1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extic1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extic1`]
module"]
pub type EXTIC1 = crate::Reg<extic1::EXTIC1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod extic1;
#[doc = "EXTIC2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`extic2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extic2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extic2`]
module"]
pub type EXTIC2 = crate::Reg<extic2::EXTIC2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod extic2;
#[doc = "EXTIC3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`extic3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extic3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extic3`]
module"]
pub type EXTIC3 = crate::Reg<extic3::EXTIC3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod extic3;
#[doc = "EXTIC4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`extic4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extic4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extic4`]
module"]
pub type EXTIC4 = crate::Reg<extic4::EXTIC4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod extic4;
#[doc = "CFG2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "configuration register 2"]
pub mod cfg2;
