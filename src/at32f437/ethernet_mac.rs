#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"]
    pub macctrl: MACCTRL,
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    pub macfrmf: MACFRMF,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub machth: MACHTH,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub machtl: MACHTL,
    #[doc = "0x10 - Ethernet MAC MII address register"]
    pub macmiiaddr: MACMIIADDR,
    #[doc = "0x14 - Ethernet MAC MII data register"]
    pub macmiidt: MACMIIDT,
    #[doc = "0x18 - Ethernet MAC flow control register"]
    pub macfctrl: MACFCTRL,
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    pub macvlt: MACVLT,
    _reserved8: [u8; 0x08],
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register"]
    pub macrwff: MACRWFF,
    #[doc = "0x2c - Ethernet MAC PMT control and status register"]
    pub macpmtctrlsts: MACPMTCTRLSTS,
    _reserved10: [u8; 0x08],
    #[doc = "0x38 - Ethernet MAC interrupt status register"]
    pub macists: MACISTS,
    #[doc = "0x3c - Ethernet MAC interrupt mask register"]
    pub macimr: MACIMR,
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    pub maca0h: MACA0H,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0l: MACA0L,
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    pub maca1h: MACA1H,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1l: MACA1L,
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    pub maca2h: MACA2H,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub maca2l: MACA2L,
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    pub maca3h: MACA3H,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub maca3l: MACA3L,
}
#[doc = "MACCTRL (rw) register accessor: Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macctrl`]
module"]
pub type MACCTRL = crate::Reg<macctrl::MACCTRL_SPEC>;
#[doc = "Ethernet MAC configuration register"]
pub mod macctrl;
#[doc = "MACFRMF (rw) register accessor: Ethernet MAC frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfrmf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfrmf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macfrmf`]
module"]
pub type MACFRMF = crate::Reg<macfrmf::MACFRMF_SPEC>;
#[doc = "Ethernet MAC frame filter register"]
pub mod macfrmf;
#[doc = "MACHTH (rw) register accessor: Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`machth`]
module"]
pub type MACHTH = crate::Reg<machth::MACHTH_SPEC>;
#[doc = "Ethernet MAC hash table high register"]
pub mod machth;
#[doc = "MACHTL (rw) register accessor: Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`machtl`]
module"]
pub type MACHTL = crate::Reg<machtl::MACHTL_SPEC>;
#[doc = "Ethernet MAC hash table low register"]
pub mod machtl;
#[doc = "MACMIIADDR (rw) register accessor: Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiiaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiiaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macmiiaddr`]
module"]
pub type MACMIIADDR = crate::Reg<macmiiaddr::MACMIIADDR_SPEC>;
#[doc = "Ethernet MAC MII address register"]
pub mod macmiiaddr;
#[doc = "MACMIIDT (rw) register accessor: Ethernet MAC MII data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macmiidt`]
module"]
pub type MACMIIDT = crate::Reg<macmiidt::MACMIIDT_SPEC>;
#[doc = "Ethernet MAC MII data register"]
pub mod macmiidt;
#[doc = "MACFCTRL (rw) register accessor: Ethernet MAC flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macfctrl`]
module"]
pub type MACFCTRL = crate::Reg<macfctrl::MACFCTRL_SPEC>;
#[doc = "Ethernet MAC flow control register"]
pub mod macfctrl;
#[doc = "MACVLT (rw) register accessor: Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macvlt`]
module"]
pub type MACVLT = crate::Reg<macvlt::MACVLT_SPEC>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod macvlt;
#[doc = "MACRWFF (rw) register accessor: Ethernet MAC remote wakeup frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macrwff`]
module"]
pub type MACRWFF = crate::Reg<macrwff::MACRWFF_SPEC>;
#[doc = "Ethernet MAC remote wakeup frame filter register"]
pub mod macrwff;
#[doc = "MACPMTCTRLSTS (rw) register accessor: Ethernet MAC PMT control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macpmtctrlsts`]
module"]
pub type MACPMTCTRLSTS = crate::Reg<macpmtctrlsts::MACPMTCTRLSTS_SPEC>;
#[doc = "Ethernet MAC PMT control and status register"]
pub mod macpmtctrlsts;
#[doc = "MACISTS (rw) register accessor: Ethernet MAC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macists::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macists::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macists`]
module"]
pub type MACISTS = crate::Reg<macists::MACISTS_SPEC>;
#[doc = "Ethernet MAC interrupt status register"]
pub mod macists;
#[doc = "MACIMR (rw) register accessor: Ethernet MAC interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`macimr`]
module"]
pub type MACIMR = crate::Reg<macimr::MACIMR_SPEC>;
#[doc = "Ethernet MAC interrupt mask register"]
pub mod macimr;
#[doc = "MACA0H (rw) register accessor: Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca0h`]
module"]
pub type MACA0H = crate::Reg<maca0h::MACA0H_SPEC>;
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0h;
#[doc = "MACA0L (rw) register accessor: Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca0l`]
module"]
pub type MACA0L = crate::Reg<maca0l::MACA0L_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0l;
#[doc = "MACA1H (rw) register accessor: Ethernet MAC address 1 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca1h`]
module"]
pub type MACA1H = crate::Reg<maca1h::MACA1H_SPEC>;
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1h;
#[doc = "MACA1L (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca1l`]
module"]
pub type MACA1L = crate::Reg<maca1l::MACA1L_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1l;
#[doc = "MACA2H (rw) register accessor: Ethernet MAC address 2 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca2h`]
module"]
pub type MACA2H = crate::Reg<maca2h::MACA2H_SPEC>;
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2h;
#[doc = "MACA2L (rw) register accessor: Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca2l`]
module"]
pub type MACA2L = crate::Reg<maca2l::MACA2L_SPEC>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2l;
#[doc = "MACA3H (rw) register accessor: Ethernet MAC address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca3h`]
module"]
pub type MACA3H = crate::Reg<maca3h::MACA3H_SPEC>;
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3h;
#[doc = "MACA3L (rw) register accessor: Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maca3l`]
module"]
pub type MACA3L = crate::Reg<maca3l::MACA3L_SPEC>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3l;
