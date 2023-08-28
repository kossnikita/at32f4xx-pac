#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MMC control register"]
    pub mmcctrl: MMCCTRL,
    #[doc = "0x04 - Ethernet MMC receive interrupt register"]
    pub mmcri: MMCRI,
    #[doc = "0x08 - Ethernet MMC transmit interrupt register"]
    pub mmcti: MMCTI,
    #[doc = "0x0c - Ethernet MMC receive interrupt mask register"]
    pub mmcrim: MMCRIM,
    #[doc = "0x10 - Ethernet MMC transmit interrupt mask register"]
    pub mmctim: MMCTIM,
    _reserved5: [u8; 0x38],
    #[doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter"]
    pub mmctfscc: MMCTFSCC,
    #[doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision"]
    pub mmctfmscc: MMCTFMSCC,
    _reserved7: [u8; 0x14],
    #[doc = "0x68 - Ethernet MMC transmitted good frames counter register"]
    pub mmctfcnt: MMCTFCNT,
    _reserved8: [u8; 0x28],
    #[doc = "0x94 - Ethernet MMC received frames with CRC error counter register"]
    pub mmcrfcecnt: MMCRFCECNT,
    #[doc = "0x98 - Ethernet MMC received frames with alignment error counter register"]
    pub mmcrfaecnt: MMCRFAECNT,
    _reserved10: [u8; 0x28],
    #[doc = "0xc4 - MMC received good unicast frames counter register"]
    pub mmcrgufcnt: MMCRGUFCNT,
}
#[doc = "MMCCTRL (rw) register accessor: Ethernet MMC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmcctrl`]
module"]
pub type MMCCTRL = crate::Reg<mmcctrl::MMCCTRL_SPEC>;
#[doc = "Ethernet MMC control register"]
pub mod mmcctrl;
#[doc = "MMCRI (rw) register accessor: Ethernet MMC receive interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmcri`]
module"]
pub type MMCRI = crate::Reg<mmcri::MMCRI_SPEC>;
#[doc = "Ethernet MMC receive interrupt register"]
pub mod mmcri;
#[doc = "MMCTI (rw) register accessor: Ethernet MMC transmit interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcti::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcti::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmcti`]
module"]
pub type MMCTI = crate::Reg<mmcti::MMCTI_SPEC>;
#[doc = "Ethernet MMC transmit interrupt register"]
pub mod mmcti;
#[doc = "MMCRIM (rw) register accessor: Ethernet MMC receive interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmcrim`]
module"]
pub type MMCRIM = crate::Reg<mmcrim::MMCRIM_SPEC>;
#[doc = "Ethernet MMC receive interrupt mask register"]
pub mod mmcrim;
#[doc = "MMCTIM (rw) register accessor: Ethernet MMC transmit interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmctim`]
module"]
pub type MMCTIM = crate::Reg<mmctim::MMCTIM_SPEC>;
#[doc = "Ethernet MMC transmit interrupt mask register"]
pub mod mmctim;
#[doc = "MMCTFSCC (r) register accessor: Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfscc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmctfscc`]
module"]
pub type MMCTFSCC = crate::Reg<mmctfscc::MMCTFSCC_SPEC>;
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub mod mmctfscc;
#[doc = "MMCTFMSCC (r) register accessor: Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfmscc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmctfmscc`]
module"]
pub type MMCTFMSCC = crate::Reg<mmctfmscc::MMCTFMSCC_SPEC>;
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub mod mmctfmscc;
#[doc = "MMCTFCNT (r) register accessor: Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmctfcnt`]
module"]
pub type MMCTFCNT = crate::Reg<mmctfcnt::MMCTFCNT_SPEC>;
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub mod mmctfcnt;
#[doc = "MMCRFCECNT (r) register accessor: Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfcecnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmcrfcecnt`]
module"]
pub type MMCRFCECNT = crate::Reg<mmcrfcecnt::MMCRFCECNT_SPEC>;
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub mod mmcrfcecnt;
#[doc = "MMCRFAECNT (r) register accessor: Ethernet MMC received frames with alignment error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfaecnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmcrfaecnt`]
module"]
pub type MMCRFAECNT = crate::Reg<mmcrfaecnt::MMCRFAECNT_SPEC>;
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub mod mmcrfaecnt;
#[doc = "MMCRGUFCNT (r) register accessor: MMC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrgufcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mmcrgufcnt`]
module"]
pub type MMCRGUFCNT = crate::Reg<mmcrgufcnt::MMCRGUFCNT_SPEC>;
#[doc = "MMC received good unicast frames counter register"]
pub mod mmcrgufcnt;
