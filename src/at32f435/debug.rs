#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: IDCODE,
    ctrl: CTRL,
    apb1_pause: APB1_PAUSE,
    apb2_pause: APB2_PAUSE,
    _reserved4: [u8; 0x10],
    ser_id: SER_ID,
}
impl RegisterBlock {
    #[doc = "0x00 - DEBUG IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    #[doc = "0x04 - DEBUG CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x08 - DEBUG APB1 PAUSE"]
    #[inline(always)]
    pub const fn apb1_pause(&self) -> &APB1_PAUSE {
        &self.apb1_pause
    }
    #[doc = "0x0c - DEBUG APB2 PAUSE"]
    #[inline(always)]
    pub const fn apb2_pause(&self) -> &APB2_PAUSE {
        &self.apb2_pause
    }
    #[doc = "0x20 - SERIES ID"]
    #[inline(always)]
    pub const fn ser_id(&self) -> &SER_ID {
        &self.ser_id
    }
}
#[doc = "IDCODE (r) register accessor: DEBUG IDCODE\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "DEBUG IDCODE"]
pub mod idcode;
#[doc = "CTRL (rw) register accessor: DEBUG CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DEBUG CTRL"]
pub mod ctrl;
#[doc = "APB1_PAUSE (rw) register accessor: DEBUG APB1 PAUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1_pause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_pause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1_pause`] module"]
pub type APB1_PAUSE = crate::Reg<apb1_pause::APB1_PAUSE_SPEC>;
#[doc = "DEBUG APB1 PAUSE"]
pub mod apb1_pause;
#[doc = "APB2_PAUSE (rw) register accessor: DEBUG APB2 PAUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2_pause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_pause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2_pause`] module"]
pub type APB2_PAUSE = crate::Reg<apb2_pause::APB2_PAUSE_SPEC>;
#[doc = "DEBUG APB2 PAUSE"]
pub mod apb2_pause;
#[doc = "SER_ID (r) register accessor: SERIES ID\n\nYou can [`read`](crate::Reg::read) this register and get [`ser_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser_id`] module"]
pub type SER_ID = crate::Reg<ser_id::SER_ID_SPEC>;
#[doc = "SERIES ID"]
pub mod ser_id;
