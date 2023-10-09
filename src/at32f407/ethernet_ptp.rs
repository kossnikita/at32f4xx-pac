#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register"]
    pub ptptsctrl: PTPTSCTRL,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    pub ptpssinc: PTPSSINC,
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    pub ptptsh: PTPTSH,
    #[doc = "0x0c - Ethernet PTP time stamp low register"]
    pub ptptsl: PTPTSL,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    pub ptptshud: PTPTSHUD,
    #[doc = "0x14 - Ethernet PTP time stamp low update register"]
    pub ptptslud: PTPTSLUD,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    pub ptptsad: PTPTSAD,
    #[doc = "0x1c - Ethernet PTP target time high register"]
    pub ptptth: PTPTTH,
    #[doc = "0x20 - Ethernet PTP target time low register"]
    pub ptpttl: PTPTTL,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Ethernet PTP time stamp status register"]
    pub ptptssr: PTPTSSR,
    #[doc = "0x2c - Ethernet PTP PPS control register"]
    pub ptpppscr: PTPPPSCR,
}
#[doc = "PTPTSCTRL (rw) register accessor: Ethernet PTP time stamp control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptsctrl`]
module"]
pub type PTPTSCTRL = crate::Reg<ptptsctrl::PTPTSCTRL_SPEC>;
#[doc = "Ethernet PTP time stamp control register"]
pub mod ptptsctrl;
#[doc = "PTPSSINC (rw) register accessor: Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssinc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssinc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptpssinc`]
module"]
pub type PTPSSINC = crate::Reg<ptpssinc::PTPSSINC_SPEC>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssinc;
#[doc = "PTPTSH (r) register accessor: Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptsh`]
module"]
pub type PTPTSH = crate::Reg<ptptsh::PTPTSH_SPEC>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptsh;
#[doc = "PTPTSL (r) register accessor: Ethernet PTP time stamp low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptsl`]
module"]
pub type PTPTSL = crate::Reg<ptptsl::PTPTSL_SPEC>;
#[doc = "Ethernet PTP time stamp low register"]
pub mod ptptsl;
#[doc = "PTPTSHUD (rw) register accessor: Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptshud`]
module"]
pub type PTPTSHUD = crate::Reg<ptptshud::PTPTSHUD_SPEC>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshud;
#[doc = "PTPTSLUD (rw) register accessor: Ethernet PTP time stamp low update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptslud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptslud`]
module"]
pub type PTPTSLUD = crate::Reg<ptptslud::PTPTSLUD_SPEC>;
#[doc = "Ethernet PTP time stamp low update register"]
pub mod ptptslud;
#[doc = "PTPTSAD (rw) register accessor: Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptsad`]
module"]
pub type PTPTSAD = crate::Reg<ptptsad::PTPTSAD_SPEC>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsad;
#[doc = "PTPTTH (rw) register accessor: Ethernet PTP target time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptth`]
module"]
pub type PTPTTH = crate::Reg<ptptth::PTPTTH_SPEC>;
#[doc = "Ethernet PTP target time high register"]
pub mod ptptth;
#[doc = "PTPTTL (rw) register accessor: Ethernet PTP target time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpttl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpttl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptpttl`]
module"]
pub type PTPTTL = crate::Reg<ptpttl::PTPTTL_SPEC>;
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttl;
#[doc = "PTPTSSR (r) register accessor: Ethernet PTP time stamp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptptssr`]
module"]
pub type PTPTSSR = crate::Reg<ptptssr::PTPTSSR_SPEC>;
#[doc = "Ethernet PTP time stamp status register"]
pub mod ptptssr;
#[doc = "PTPPPSCR (r) register accessor: Ethernet PTP PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpppscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ptpppscr`]
module"]
pub type PTPPPSCR = crate::Reg<ptpppscr::PTPPPSCR_SPEC>;
#[doc = "Ethernet PTP PPS control register"]
pub mod ptpppscr;
