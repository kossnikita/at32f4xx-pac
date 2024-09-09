#[repr(C)]
#[derive(Debug)]
#[doc = "Mailbox %s"]
pub struct Mailbox {
    tmi: TMI,
    tmc: TMC,
    tmdtl: TMDTL,
    tmdth: TMDTH,
}
impl Mailbox {
    #[doc = "0x00 - Transmit mailbox identifier register"]
    #[inline(always)]
    pub const fn tmi(&self) -> &TMI {
        &self.tmi
    }
    #[doc = "0x04 - Transmit mailbox data length and time stamp register"]
    #[inline(always)]
    pub const fn tmc(&self) -> &TMC {
        &self.tmc
    }
    #[doc = "0x08 - Transmit mailbox data low register"]
    #[inline(always)]
    pub const fn tmdtl(&self) -> &TMDTL {
        &self.tmdtl
    }
    #[doc = "0x0c - Transmit mailbox data high register"]
    #[inline(always)]
    pub const fn tmdth(&self) -> &TMDTH {
        &self.tmdth
    }
}
#[doc = "TMI (rw) register accessor: Transmit mailbox identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmi`]
module"]
pub type TMI = crate::Reg<tmi::TMI_SPEC>;
#[doc = "Transmit mailbox identifier register"]
pub mod tmi;
#[doc = "TMC (rw) register accessor: Transmit mailbox data length and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmc`]
module"]
pub type TMC = crate::Reg<tmc::TMC_SPEC>;
#[doc = "Transmit mailbox data length and time stamp register"]
pub mod tmc;
#[doc = "TMDTL (rw) register accessor: Transmit mailbox data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmdtl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmdtl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdtl`]
module"]
pub type TMDTL = crate::Reg<tmdtl::TMDTL_SPEC>;
#[doc = "Transmit mailbox data low register"]
pub mod tmdtl;
#[doc = "TMDTH (rw) register accessor: Transmit mailbox data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmdth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmdth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdth`]
module"]
pub type TMDTH = crate::Reg<tmdth::TMDTH_SPEC>;
#[doc = "Transmit mailbox data high register"]
pub mod tmdth;
