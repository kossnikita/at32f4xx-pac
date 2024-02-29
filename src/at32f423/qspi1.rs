#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd_w0: CMD_W0,
    cmd_w1: CMD_W1,
    cmd_w2: CMD_W2,
    cmd_w3: CMD_W3,
    ctrl: CTRL,
    actr: ACTR,
    fifosts: FIFOSTS,
    _reserved7: [u8; 0x04],
    ctrl2: CTRL2,
    cmdsts: CMDSTS,
    rsts: RSTS,
    fsize: FSIZE,
    xip_cmd_w0: XIP_CMD_W0,
    xip_cmd_w1: XIP_CMD_W1,
    xip_cmd_w2: XIP_CMD_W2,
    xip_cmd_w3: XIP_CMD_W3,
    _reserved15: [u8; 0x10],
    rev: REV,
    _reserved16: [u8; 0xac],
    dt: DT,
}
impl RegisterBlock {
    #[doc = "0x00 - Command word 0"]
    #[inline(always)]
    pub const fn cmd_w0(&self) -> &CMD_W0 {
        &self.cmd_w0
    }
    #[doc = "0x04 - Command word 1"]
    #[inline(always)]
    pub const fn cmd_w1(&self) -> &CMD_W1 {
        &self.cmd_w1
    }
    #[doc = "0x08 - Command word 2"]
    #[inline(always)]
    pub const fn cmd_w2(&self) -> &CMD_W2 {
        &self.cmd_w2
    }
    #[doc = "0x0c - Command word 3"]
    #[inline(always)]
    pub const fn cmd_w3(&self) -> &CMD_W3 {
        &self.cmd_w3
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x14 - AC timing control register"]
    #[inline(always)]
    pub const fn actr(&self) -> &ACTR {
        &self.actr
    }
    #[doc = "0x18 - FIFO Status register"]
    #[inline(always)]
    pub const fn fifosts(&self) -> &FIFOSTS {
        &self.fifosts
    }
    #[doc = "0x20 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x24 - CMD status register"]
    #[inline(always)]
    pub const fn cmdsts(&self) -> &CMDSTS {
        &self.cmdsts
    }
    #[doc = "0x28 - SPI read status register"]
    #[inline(always)]
    pub const fn rsts(&self) -> &RSTS {
        &self.rsts
    }
    #[doc = "0x2c - SPI flash size"]
    #[inline(always)]
    pub const fn fsize(&self) -> &FSIZE {
        &self.fsize
    }
    #[doc = "0x30 - XIP command word 0"]
    #[inline(always)]
    pub const fn xip_cmd_w0(&self) -> &XIP_CMD_W0 {
        &self.xip_cmd_w0
    }
    #[doc = "0x34 - XIP command word 1"]
    #[inline(always)]
    pub const fn xip_cmd_w1(&self) -> &XIP_CMD_W1 {
        &self.xip_cmd_w1
    }
    #[doc = "0x38 - XIP command word 2"]
    #[inline(always)]
    pub const fn xip_cmd_w2(&self) -> &XIP_CMD_W2 {
        &self.xip_cmd_w2
    }
    #[doc = "0x3c - XIP command word 3"]
    #[inline(always)]
    pub const fn xip_cmd_w3(&self) -> &XIP_CMD_W3 {
        &self.xip_cmd_w3
    }
    #[doc = "0x50 - Revision"]
    #[inline(always)]
    pub const fn rev(&self) -> &REV {
        &self.rev
    }
    #[doc = "0x100 - 32/16/8 bit data port register"]
    #[inline(always)]
    pub const fn dt(&self) -> &DT {
        &self.dt
    }
}
#[doc = "CMD_W0 (rw) register accessor: Command word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w0`]
module"]
pub type CMD_W0 = crate::Reg<cmd_w0::CMD_W0_SPEC>;
#[doc = "Command word 0"]
pub mod cmd_w0;
#[doc = "CMD_W1 (rw) register accessor: Command word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w1`]
module"]
pub type CMD_W1 = crate::Reg<cmd_w1::CMD_W1_SPEC>;
#[doc = "Command word 1"]
pub mod cmd_w1;
#[doc = "CMD_W2 (rw) register accessor: Command word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w2`]
module"]
pub type CMD_W2 = crate::Reg<cmd_w2::CMD_W2_SPEC>;
#[doc = "Command word 2"]
pub mod cmd_w2;
#[doc = "CMD_W3 (rw) register accessor: Command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_w3`]
module"]
pub type CMD_W3 = crate::Reg<cmd_w3::CMD_W3_SPEC>;
#[doc = "Command word 3"]
pub mod cmd_w3;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "ACTR (rw) register accessor: AC timing control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actr`]
module"]
pub type ACTR = crate::Reg<actr::ACTR_SPEC>;
#[doc = "AC timing control register"]
pub mod actr;
#[doc = "FIFOSTS (r) register accessor: FIFO Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifosts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifosts`]
module"]
pub type FIFOSTS = crate::Reg<fifosts::FIFOSTS_SPEC>;
#[doc = "FIFO Status register"]
pub mod fifosts;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "CMDSTS (r) register accessor: CMD status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdsts`]
module"]
pub type CMDSTS = crate::Reg<cmdsts::CMDSTS_SPEC>;
#[doc = "CMD status register"]
pub mod cmdsts;
#[doc = "RSTS (r) register accessor: SPI read status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsts`]
module"]
pub type RSTS = crate::Reg<rsts::RSTS_SPEC>;
#[doc = "SPI read status register"]
pub mod rsts;
#[doc = "FSIZE (rw) register accessor: SPI flash size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsize`]
module"]
pub type FSIZE = crate::Reg<fsize::FSIZE_SPEC>;
#[doc = "SPI flash size"]
pub mod fsize;
#[doc = "XIP_CMD_W0 (rw) register accessor: XIP command word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w0`]
module"]
pub type XIP_CMD_W0 = crate::Reg<xip_cmd_w0::XIP_CMD_W0_SPEC>;
#[doc = "XIP command word 0"]
pub mod xip_cmd_w0;
#[doc = "XIP_CMD_W1 (rw) register accessor: XIP command word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w1`]
module"]
pub type XIP_CMD_W1 = crate::Reg<xip_cmd_w1::XIP_CMD_W1_SPEC>;
#[doc = "XIP command word 1"]
pub mod xip_cmd_w1;
#[doc = "XIP_CMD_W2 (rw) register accessor: XIP command word 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w2`]
module"]
pub type XIP_CMD_W2 = crate::Reg<xip_cmd_w2::XIP_CMD_W2_SPEC>;
#[doc = "XIP command word 2"]
pub mod xip_cmd_w2;
#[doc = "XIP_CMD_W3 (rw) register accessor: XIP command word 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xip_cmd_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xip_cmd_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xip_cmd_w3`]
module"]
pub type XIP_CMD_W3 = crate::Reg<xip_cmd_w3::XIP_CMD_W3_SPEC>;
#[doc = "XIP command word 3"]
pub mod xip_cmd_w3;
#[doc = "REV (rw) register accessor: Revision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rev`]
module"]
pub type REV = crate::Reg<rev::REV_SPEC>;
#[doc = "Revision"]
pub mod rev;
#[doc = "DT (rw) register accessor: 32/16/8 bit data port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "32/16/8 bit data port register"]
pub mod dt;
