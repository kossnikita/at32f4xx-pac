#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DEBUG IDCODE"]
    pub idcode: IDCODE,
    #[doc = "0x04 - DEBUG CTRL"]
    pub ctrl: CTRL,
    #[doc = "0x08 - DEBUG APB1 PAUSE"]
    pub apb1_pause: APB1_PAUSE,
    #[doc = "0x0c - DEBUG APB2 PAUSE"]
    pub apb2_pause: APB2_PAUSE,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - SERIES ID"]
    pub ser_id: SER_ID,
}
#[doc = "IDCODE (r) register accessor: DEBUG IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`idcode`]
module"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "DEBUG IDCODE"]
pub mod idcode;
#[doc = "CTRL (rw) register accessor: DEBUG CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DEBUG CTRL"]
pub mod ctrl;
#[doc = "APB1_PAUSE (rw) register accessor: DEBUG APB1 PAUSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1_pause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1_pause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb1_pause`]
module"]
pub type APB1_PAUSE = crate::Reg<apb1_pause::APB1_PAUSE_SPEC>;
#[doc = "DEBUG APB1 PAUSE"]
pub mod apb1_pause;
#[doc = "APB2_PAUSE (rw) register accessor: DEBUG APB2 PAUSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2_pause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2_pause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`apb2_pause`]
module"]
pub type APB2_PAUSE = crate::Reg<apb2_pause::APB2_PAUSE_SPEC>;
#[doc = "DEBUG APB2 PAUSE"]
pub mod apb2_pause;
#[doc = "SER_ID (r) register accessor: SERIES ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ser_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ser_id`]
module"]
pub type SER_ID = crate::Reg<ser_id::SER_ID_SPEC>;
#[doc = "SERIES ID"]
pub mod ser_id;
