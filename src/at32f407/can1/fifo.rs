#[repr(C)]
#[derive(Debug)]
#[doc = "FIFO %s"]
#[doc(alias = "FIFO")]
pub struct Fifo {
    rfi: RFI,
    rfc: RFC,
    rfdtl: RFDTL,
    rfdth: RFDTH,
}
impl Fifo {
    #[doc = "0x00 - Receive FIFO mailbox identifier register"]
    #[inline(always)]
    pub const fn rfi(&self) -> &RFI {
        &self.rfi
    }
    #[doc = "0x04 - Receive FIFO mailbox data length and time stamp register"]
    #[inline(always)]
    pub const fn rfc(&self) -> &RFC {
        &self.rfc
    }
    #[doc = "0x08 - Receive FIFO mailbox data low register"]
    #[inline(always)]
    pub const fn rfdtl(&self) -> &RFDTL {
        &self.rfdtl
    }
    #[doc = "0x0c - Receive FIFO mailbox data high register"]
    #[inline(always)]
    pub const fn rfdth(&self) -> &RFDTH {
        &self.rfdth
    }
}
#[doc = "RFI (r) register accessor: Receive FIFO mailbox identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfi`] module"]
pub type RFI = crate::Reg<rfi::RFI_SPEC>;
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfi;
#[doc = "RFC (r) register accessor: Receive FIFO mailbox data length and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc`] module"]
pub type RFC = crate::Reg<rfc::RFC_SPEC>;
#[doc = "Receive FIFO mailbox data length and time stamp register"]
pub mod rfc;
#[doc = "RFDTL (r) register accessor: Receive FIFO mailbox data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfdtl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdtl`] module"]
pub type RFDTL = crate::Reg<rfdtl::RFDTL_SPEC>;
#[doc = "Receive FIFO mailbox data low register"]
pub mod rfdtl;
#[doc = "RFDTH (r) register accessor: Receive FIFO mailbox data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfdth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdth`] module"]
pub type RFDTH = crate::Reg<rfdth::RFDTH_SPEC>;
#[doc = "Receive FIFO mailbox data high register"]
pub mod rfdth;
