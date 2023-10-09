#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTGFS control and status register (OTGFS_GOTGCTL)"]
    pub gotgctl: GOTGCTL,
    #[doc = "0x04 - OTGFS interrupt register (OTGFS_GOTGINT)"]
    pub gotgint: GOTGINT,
    #[doc = "0x08 - OTGFS AHB configuration register (OTGFS_GAHBCFG)"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - USB configuration register (OTGFS_GUSBCFG)"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - OTGFS reset register (OTGFS_GRSTCTL)"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - OTGFS core interrupt register (OTGFS_GINTSTS)"]
    pub gintsts: GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - OTGFS Receive FIFO size register (OTGFS_GRXFSIZ)"]
    pub grxfsiz: GRXFSIZ,
    _reserved_9_dieptxf0: [u8; 0x04],
    #[doc = "0x2c - OTGFS non-periodic transmit FIFO/queue status register (OTGFS_GNPTXSTS)"]
    pub gnptxsts: GNPTXSTS,
    _reserved11: [u8; 0x08],
    #[doc = "0x38 - OTGFS general core configuration register (OTGFS_GCCFG)"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - Product ID register"]
    pub guid: GUID,
    _reserved13: [u8; 0xc0],
    #[doc = "0x100 - OTGFS Host periodic transmit FIFO size register (OTGFS_HPTXFSIZ)"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0x104 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF1)"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF2)"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x10c - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF3)"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x110 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF4)"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x114 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF5)"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0x118 - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)"]
    pub dieptxf6: DIEPTXF6,
    #[doc = "0x11c - OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF7)"]
    pub dieptxf7: DIEPTXF7,
}
impl RegisterBlock {
    #[doc = "0x1c - OTGFS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - OTGFS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x28 - OTGFS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &GNPTXFSIZ {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "GOTGCTL (rw) register accessor: OTGFS control and status register (OTGFS_GOTGCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgctl`]
module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTGFS control and status register (OTGFS_GOTGCTL)"]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: OTGFS interrupt register (OTGFS_GOTGINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gotgint`]
module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTGFS interrupt register (OTGFS_GOTGINT)"]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: OTGFS AHB configuration register (OTGFS_GAHBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gahbcfg`]
module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "OTGFS AHB configuration register (OTGFS_GAHBCFG)"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: USB configuration register (OTGFS_GUSBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gusbcfg`]
module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB configuration register (OTGFS_GUSBCFG)"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: OTGFS reset register (OTGFS_GRSTCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "OTGFS reset register (OTGFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: OTGFS core interrupt register (OTGFS_GINTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gintsts`]
module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "OTGFS core interrupt register (OTGFS_GINTSTS)"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gintmsk`]
module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod gintmsk;
#[doc = "GRXSTSR_Device (r) register accessor: OTGFS Receive status debug read(Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxstsr_device`]
module"]
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICE_SPEC>;
#[doc = "OTGFS Receive status debug read(Device mode)"]
pub mod grxstsr_device;
#[doc = "GRXSTSR_Host (r) register accessor: OTGFS Receive status debug read(Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxstsr_host`]
module"]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>;
#[doc = "OTGFS Receive status debug read(Host mode)"]
pub mod grxstsr_host;
#[doc = "GRXFSIZ (rw) register accessor: OTGFS Receive FIFO size register (OTGFS_GRXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`grxfsiz`]
module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "OTGFS Receive FIFO size register (OTGFS_GRXFSIZ)"]
pub mod grxfsiz;
#[doc = "DIEPTXF0 (rw) register accessor: IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf0`]
module"]
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0_SPEC>;
#[doc = "IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)"]
pub mod dieptxf0;
#[doc = "GNPTXFSIZ (rw) register accessor: OTGFS non-periodic transmit FIFO size register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gnptxfsiz`]
module"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "OTGFS non-periodic transmit FIFO size register (Host mode)"]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: OTGFS non-periodic transmit FIFO/queue status register (OTGFS_GNPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gnptxsts`]
module"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "OTGFS non-periodic transmit FIFO/queue status register (OTGFS_GNPTXSTS)"]
pub mod gnptxsts;
#[doc = "GCCFG (rw) register accessor: OTGFS general core configuration register (OTGFS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gccfg`]
module"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "OTGFS general core configuration register (OTGFS_GCCFG)"]
pub mod gccfg;
#[doc = "GUID (rw) register accessor: Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`guid`]
module"]
pub type GUID = crate::Reg<guid::GUID_SPEC>;
#[doc = "Product ID register"]
pub mod guid;
#[doc = "HPTXFSIZ (rw) register accessor: OTGFS Host periodic transmit FIFO size register (OTGFS_HPTXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hptxfsiz`]
module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "OTGFS Host periodic transmit FIFO size register (OTGFS_HPTXFSIZ)"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf1`]
module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF1)"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf2`]
module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF2)"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf3`]
module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF3)"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf4`]
module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF4)"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf5`]
module"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF5)"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf6`]
module"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)"]
pub mod dieptxf6;
#[doc = "DIEPTXF7 (rw) register accessor: OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptxf7`]
module"]
pub type DIEPTXF7 = crate::Reg<dieptxf7::DIEPTXF7_SPEC>;
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF7)"]
pub mod dieptxf7;
