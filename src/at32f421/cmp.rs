#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP control/status register"]
    pub ctrlsts: CTRLSTS,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - G_FILTER_EN"]
    pub g_filter_en: G_FILTER_EN,
    #[doc = "0x0c - HIGH_PULSE"]
    pub high_pulse: HIGH_PULSE,
    #[doc = "0x10 - LOW_PULSE"]
    pub low_pulse: LOW_PULSE,
}
#[doc = "CTRLSTS (rw) register accessor: CMP control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "CMP control/status register"]
pub mod ctrlsts;
#[doc = "G_FILTER_EN (rw) register accessor: G_FILTER_EN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g_filter_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g_filter_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`g_filter_en`]
module"]
pub type G_FILTER_EN = crate::Reg<g_filter_en::G_FILTER_EN_SPEC>;
#[doc = "G_FILTER_EN"]
pub mod g_filter_en;
#[doc = "HIGH_PULSE (rw) register accessor: HIGH_PULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_pulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_pulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`high_pulse`]
module"]
pub type HIGH_PULSE = crate::Reg<high_pulse::HIGH_PULSE_SPEC>;
#[doc = "HIGH_PULSE"]
pub mod high_pulse;
#[doc = "LOW_PULSE (rw) register accessor: LOW_PULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_pulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_pulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`low_pulse`]
module"]
pub type LOW_PULSE = crate::Reg<low_pulse::LOW_PULSE_SPEC>;
#[doc = "LOW_PULSE"]
pub mod low_pulse;
