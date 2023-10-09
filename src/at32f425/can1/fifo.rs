#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct FIFO {
    #[doc = "0x00 - Receive FIFO mailbox identifier register"]
    pub rfi: RFI,
    #[doc = "0x04 - Receive FIFO mailbox data length and time stamp register"]
    pub rfc: RFC,
    #[doc = "0x08 - Receive FIFO mailbox data low register"]
    pub rfdtl: RFDTL,
    #[doc = "0x0c - Receive FIFO mailbox data high register"]
    pub rfdth: RFDTH,
}
#[doc = "RFI (r) register accessor: Receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfi`]
module"]
pub type RFI = crate::Reg<rfi::RFI_SPEC>;
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfi;
#[doc = "RFC (r) register accessor: Receive FIFO mailbox data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfc`]
module"]
pub type RFC = crate::Reg<rfc::RFC_SPEC>;
#[doc = "Receive FIFO mailbox data length and time stamp register"]
pub mod rfc;
#[doc = "RFDTL (r) register accessor: Receive FIFO mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfdtl`]
module"]
pub type RFDTL = crate::Reg<rfdtl::RFDTL_SPEC>;
#[doc = "Receive FIFO mailbox data low register"]
pub mod rfdtl;
#[doc = "RFDTH (r) register accessor: Receive FIFO mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rfdth`]
module"]
pub type RFDTH = crate::Reg<rfdth::RFDTH_SPEC>;
#[doc = "Receive FIFO mailbox data high register"]
pub mod rfdth;
