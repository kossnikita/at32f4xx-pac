#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: IDCODE,
    ctrl: CTRL,
    _reserved2: [u8; 0x18],
    ser_id: SER_ID,
}
impl RegisterBlock {
    #[doc = "0x00 - DEBUG_IDCODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &IDCODE {
        &self.idcode
    }
    #[doc = "0x04 - DEBUG_CTRL"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x20 - SERIES ID"]
    #[inline(always)]
    pub const fn ser_id(&self) -> &SER_ID {
        &self.ser_id
    }
}
#[doc = "IDCODE (r) register accessor: DEBUG_IDCODE\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "DEBUG_IDCODE"]
pub mod idcode;
#[doc = "CTRL (rw) register accessor: DEBUG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DEBUG_CTRL"]
pub mod ctrl;
#[doc = "SER_ID (r) register accessor: SERIES ID\n\nYou can [`read`](crate::Reg::read) this register and get [`ser_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ser_id`] module"]
pub type SER_ID = crate::Reg<ser_id::SER_ID_SPEC>;
#[doc = "SERIES ID"]
pub mod ser_id;
