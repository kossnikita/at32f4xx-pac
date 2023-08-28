#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DEBUG IDCODE"]
    pub debug_idcode: DEBUG_IDCODE,
    #[doc = "0x04 - MCUDBG_CTRL"]
    pub ctrl: CTRL,
}
#[doc = "DEBUG_IDCODE (r) register accessor: DEBUG IDCODE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_idcode::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`debug_idcode`]
module"]
pub type DEBUG_IDCODE = crate::Reg<debug_idcode::DEBUG_IDCODE_SPEC>;
#[doc = "DEBUG IDCODE"]
pub mod debug_idcode;
#[doc = "CTRL (rw) register accessor: MCUDBG_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MCUDBG_CTRL"]
pub mod ctrl;
