#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved3: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved7: [u8; 0x14],
    diepempmsk: DIEPEMPMSK,
    _reserved8: [u8; 0xc8],
    diepctl0: DIEPCTL0,
    _reserved9: [u8; 0x04],
    diepint0: DIEPINT0,
    _reserved10: [u8; 0x04],
    dieptsiz0: DIEPTSIZ0,
    _reserved11: [u8; 0x04],
    dtxfsts0: DTXFSTS0,
    _reserved12: [u8; 0x04],
    diepctl1: DIEPCTL1,
    _reserved13: [u8; 0x04],
    diepint1: DIEPINT1,
    _reserved14: [u8; 0x04],
    dieptsiz1: DIEPTSIZ1,
    _reserved15: [u8; 0x04],
    dtxfsts1: DTXFSTS1,
    _reserved16: [u8; 0x04],
    diepctl2: DIEPCTL2,
    _reserved17: [u8; 0x04],
    diepint2: DIEPINT2,
    _reserved18: [u8; 0x04],
    dieptsiz2: DIEPTSIZ2,
    _reserved19: [u8; 0x04],
    dtxfsts2: DTXFSTS2,
    _reserved20: [u8; 0x04],
    diepctl3: DIEPCTL3,
    _reserved21: [u8; 0x04],
    diepint3: DIEPINT3,
    _reserved22: [u8; 0x04],
    dieptsiz3: DIEPTSIZ3,
    _reserved23: [u8; 0x04],
    dtxfsts3: DTXFSTS3,
    _reserved24: [u8; 0x04],
    diepctl4: DIEPCTL4,
    _reserved25: [u8; 0x04],
    diepint4: DIEPINT4,
    _reserved26: [u8; 0x04],
    dieptsiz4: DIEPTSIZ4,
    _reserved27: [u8; 0x04],
    dtxfsts4: DTXFSTS4,
    _reserved28: [u8; 0x04],
    diepctl5: DIEPCTL5,
    _reserved29: [u8; 0x04],
    diepint5: DIEPINT5,
    _reserved30: [u8; 0x04],
    dieptsiz5: DIEPTSIZ5,
    _reserved31: [u8; 0x04],
    dtxfsts5: DTXFSTS5,
    _reserved32: [u8; 0x04],
    diepctl6: DIEPCTL6,
    _reserved33: [u8; 0x04],
    diepint6: DIEPINT6,
    _reserved34: [u8; 0x04],
    dieptsiz6: DIEPTSIZ6,
    _reserved35: [u8; 0x04],
    dtxfsts6: DTXFSTS6,
    _reserved36: [u8; 0x04],
    diepctl7: DIEPCTL7,
    _reserved37: [u8; 0x04],
    diepint7: DIEPINT7,
    _reserved38: [u8; 0x04],
    dieptsiz7: DIEPTSIZ7,
    _reserved39: [u8; 0x04],
    dtxfsts7: DTXFSTS7,
    _reserved40: [u8; 0x0104],
    doepctl0: DOEPCTL0,
    _reserved41: [u8; 0x04],
    doepint0: DOEPINT0,
    _reserved42: [u8; 0x04],
    doeptsiz0: DOEPTSIZ0,
    _reserved43: [u8; 0x0c],
    doepctl1: DOEPCTL1,
    _reserved44: [u8; 0x04],
    doepint1: DOEPINT1,
    _reserved45: [u8; 0x04],
    doeptsiz1: DOEPTSIZ1,
    _reserved46: [u8; 0x0c],
    doepctl2: DOEPCTL2,
    _reserved47: [u8; 0x04],
    doepint2: DOEPINT2,
    _reserved48: [u8; 0x04],
    doeptsiz2: DOEPTSIZ2,
    _reserved49: [u8; 0x0c],
    doepctl3: DOEPCTL3,
    _reserved50: [u8; 0x04],
    doepint3: DOEPINT3,
    _reserved51: [u8; 0x04],
    doeptsiz3: DOEPTSIZ3,
    _reserved52: [u8; 0x0c],
    doepctl4: DOEPCTL4,
    _reserved53: [u8; 0x04],
    doepint4: DOEPINT4,
    _reserved54: [u8; 0x04],
    doeptsiz4: DOEPTSIZ4,
    _reserved55: [u8; 0x0c],
    doepctl5: DOEPCTL5,
    _reserved56: [u8; 0x04],
    doepint5: DOEPINT5,
    _reserved57: [u8; 0x04],
    doeptsiz5: DOEPTSIZ5,
    _reserved58: [u8; 0x0c],
    doepctl6: DOEPCTL6,
    _reserved59: [u8; 0x04],
    doepint6: DOEPINT6,
    _reserved60: [u8; 0x04],
    doeptsiz6: DOEPTSIZ6,
    _reserved61: [u8; 0x0c],
    doepctl7: DOEPCTL7,
    _reserved62: [u8; 0x04],
    doepint7: DOEPINT7,
    _reserved63: [u8; 0x04],
    doeptsiz7: DOEPTSIZ7,
}
impl RegisterBlock {
    #[doc = "0x00 - OTGFS device configuration register (OTGFS_DCFG)"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x04 - OTGFS device control register (OTGFS_DCTL)"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x08 - OTGFS device status register (OTGFS_DSTS)"]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x10 - OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)"]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x14 - OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)"]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x18 - OTGFS device all endpoints interrupt register (OTGFS_DAINT)"]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x1c - OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)"]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x34 - OTGFS device IN endpoint FIFO empty interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x100 - OTGFS device control IN endpoint 0 control register (OTGFS_DIEPCTL0)"]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &DIEPCTL0 {
        &self.diepctl0
    }
    #[doc = "0x108 - OTGFS device IN endpoint-0 interrupt register"]
    #[inline(always)]
    pub const fn diepint0(&self) -> &DIEPINT0 {
        &self.diepint0
    }
    #[doc = "0x110 - OTGFS device IN endpoint-0 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &DIEPTSIZ0 {
        &self.dieptsiz0
    }
    #[doc = "0x118 - OTGFS device IN endpoint-0 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &DTXFSTS0 {
        &self.dtxfsts0
    }
    #[doc = "0x120 - OTGFS device IN endpoint-1 control register"]
    #[inline(always)]
    pub const fn diepctl1(&self) -> &DIEPCTL1 {
        &self.diepctl1
    }
    #[doc = "0x128 - OTGFS device IN endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn diepint1(&self) -> &DIEPINT1 {
        &self.diepint1
    }
    #[doc = "0x130 - OTGFS device IN endpoint-1 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &DIEPTSIZ1 {
        &self.dieptsiz1
    }
    #[doc = "0x138 - OTGFS device IN endpoint-1 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &DTXFSTS1 {
        &self.dtxfsts1
    }
    #[doc = "0x140 - OTGFS device IN endpoint-2 control register"]
    #[inline(always)]
    pub const fn diepctl2(&self) -> &DIEPCTL2 {
        &self.diepctl2
    }
    #[doc = "0x148 - OTGFS device IN endpoint-2 interrupt register"]
    #[inline(always)]
    pub const fn diepint2(&self) -> &DIEPINT2 {
        &self.diepint2
    }
    #[doc = "0x150 - OTGFS device IN endpoint-2 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &DIEPTSIZ2 {
        &self.dieptsiz2
    }
    #[doc = "0x158 - OTGFS device IN endpoint-2 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &DTXFSTS2 {
        &self.dtxfsts2
    }
    #[doc = "0x160 - OTGFS device IN endpoint-3 control register"]
    #[inline(always)]
    pub const fn diepctl3(&self) -> &DIEPCTL3 {
        &self.diepctl3
    }
    #[doc = "0x168 - OTGFS device IN endpoint-3 interrupt register"]
    #[inline(always)]
    pub const fn diepint3(&self) -> &DIEPINT3 {
        &self.diepint3
    }
    #[doc = "0x170 - OTG device IN endpoint-3 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &DIEPTSIZ3 {
        &self.dieptsiz3
    }
    #[doc = "0x178 - OTGFS device IN endpoint-3 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &DTXFSTS3 {
        &self.dtxfsts3
    }
    #[doc = "0x180 - OTGFS device IN endpoint-4 control register"]
    #[inline(always)]
    pub const fn diepctl4(&self) -> &DIEPCTL4 {
        &self.diepctl4
    }
    #[doc = "0x188 - OTGFS device IN endpoint-4 interrupt register"]
    #[inline(always)]
    pub const fn diepint4(&self) -> &DIEPINT4 {
        &self.diepint4
    }
    #[doc = "0x190 - OTG device IN endpoint-4 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz4(&self) -> &DIEPTSIZ4 {
        &self.dieptsiz4
    }
    #[doc = "0x198 - OTGFS device IN endpoint-4 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts4(&self) -> &DTXFSTS4 {
        &self.dtxfsts4
    }
    #[doc = "0x1a0 - OTGFS device IN endpoint-5 control register"]
    #[inline(always)]
    pub const fn diepctl5(&self) -> &DIEPCTL5 {
        &self.diepctl5
    }
    #[doc = "0x1a8 - OTGFS device IN endpoint-5 interrupt register"]
    #[inline(always)]
    pub const fn diepint5(&self) -> &DIEPINT5 {
        &self.diepint5
    }
    #[doc = "0x1b0 - OTG device IN endpoint-5 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz5(&self) -> &DIEPTSIZ5 {
        &self.dieptsiz5
    }
    #[doc = "0x1b8 - OTGFS device IN endpoint-5 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts5(&self) -> &DTXFSTS5 {
        &self.dtxfsts5
    }
    #[doc = "0x1c0 - OTGFS device IN endpoint-6 control register"]
    #[inline(always)]
    pub const fn diepctl6(&self) -> &DIEPCTL6 {
        &self.diepctl6
    }
    #[doc = "0x1c8 - OTGFS device IN endpoint-6 interrupt register"]
    #[inline(always)]
    pub const fn diepint6(&self) -> &DIEPINT6 {
        &self.diepint6
    }
    #[doc = "0x1d0 - OTG device IN endpoint-6 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz6(&self) -> &DIEPTSIZ6 {
        &self.dieptsiz6
    }
    #[doc = "0x1d8 - OTGFS device IN endpoint-6 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts6(&self) -> &DTXFSTS6 {
        &self.dtxfsts6
    }
    #[doc = "0x1e0 - OTGFS device IN endpoint-7 control register"]
    #[inline(always)]
    pub const fn diepctl7(&self) -> &DIEPCTL7 {
        &self.diepctl7
    }
    #[doc = "0x1e8 - OTGFS device IN endpoint-7 interrupt register"]
    #[inline(always)]
    pub const fn diepint7(&self) -> &DIEPINT7 {
        &self.diepint7
    }
    #[doc = "0x1f0 - OTG device IN endpoint-7 transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz7(&self) -> &DIEPTSIZ7 {
        &self.dieptsiz7
    }
    #[doc = "0x1f8 - OTGFS device IN endpoint-7 transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts7(&self) -> &DTXFSTS7 {
        &self.dtxfsts7
    }
    #[doc = "0x300 - OTGFS device OUT endpoint-0 control register"]
    #[inline(always)]
    pub const fn doepctl0(&self) -> &DOEPCTL0 {
        &self.doepctl0
    }
    #[doc = "0x308 - OTGFS device OUT endpoint-0 interrupt register"]
    #[inline(always)]
    pub const fn doepint0(&self) -> &DOEPINT0 {
        &self.doepint0
    }
    #[doc = "0x310 - OTGFS device OUT endpoint-0 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &DOEPTSIZ0 {
        &self.doeptsiz0
    }
    #[doc = "0x320 - OTGFS device OUT endpoint-1 control register"]
    #[inline(always)]
    pub const fn doepctl1(&self) -> &DOEPCTL1 {
        &self.doepctl1
    }
    #[doc = "0x328 - OTGFS device OUT endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn doepint1(&self) -> &DOEPINT1 {
        &self.doepint1
    }
    #[doc = "0x330 - OTGFS device OUT endpoint-1 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &DOEPTSIZ1 {
        &self.doeptsiz1
    }
    #[doc = "0x340 - OTGFS device OUT endpoint-2 control register"]
    #[inline(always)]
    pub const fn doepctl2(&self) -> &DOEPCTL2 {
        &self.doepctl2
    }
    #[doc = "0x348 - OTGFS device OUT endpoint-2 interrupt register"]
    #[inline(always)]
    pub const fn doepint2(&self) -> &DOEPINT2 {
        &self.doepint2
    }
    #[doc = "0x350 - OTGFS device OUT endpoint-2 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &DOEPTSIZ2 {
        &self.doeptsiz2
    }
    #[doc = "0x360 - OTGFS device OUT endpoint-3 control register"]
    #[inline(always)]
    pub const fn doepctl3(&self) -> &DOEPCTL3 {
        &self.doepctl3
    }
    #[doc = "0x368 - OTGFS device OUT endpoint-3 interrupt register"]
    #[inline(always)]
    pub const fn doepint3(&self) -> &DOEPINT3 {
        &self.doepint3
    }
    #[doc = "0x370 - OTGFS device OUT endpoint-3 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &DOEPTSIZ3 {
        &self.doeptsiz3
    }
    #[doc = "0x380 - OTGFS device OUT endpoint-4 control register"]
    #[inline(always)]
    pub const fn doepctl4(&self) -> &DOEPCTL4 {
        &self.doepctl4
    }
    #[doc = "0x388 - OTGFS device OUT endpoint-4 interrupt register"]
    #[inline(always)]
    pub const fn doepint4(&self) -> &DOEPINT4 {
        &self.doepint4
    }
    #[doc = "0x390 - OTGFS device OUT endpoint-4 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz4(&self) -> &DOEPTSIZ4 {
        &self.doeptsiz4
    }
    #[doc = "0x3a0 - OTGFS device OUT endpoint-5 control register"]
    #[inline(always)]
    pub const fn doepctl5(&self) -> &DOEPCTL5 {
        &self.doepctl5
    }
    #[doc = "0x3a8 - OTGFS device OUT endpoint-5 interrupt register"]
    #[inline(always)]
    pub const fn doepint5(&self) -> &DOEPINT5 {
        &self.doepint5
    }
    #[doc = "0x3b0 - OTGFS device OUT endpoint-5 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz5(&self) -> &DOEPTSIZ5 {
        &self.doeptsiz5
    }
    #[doc = "0x3c0 - OTGFS device OUT endpoint-6 control register"]
    #[inline(always)]
    pub const fn doepctl6(&self) -> &DOEPCTL6 {
        &self.doepctl6
    }
    #[doc = "0x3c8 - OTGFS device OUT endpoint-6 interrupt register"]
    #[inline(always)]
    pub const fn doepint6(&self) -> &DOEPINT6 {
        &self.doepint6
    }
    #[doc = "0x3d0 - OTGFS device OUT endpoint-6 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz6(&self) -> &DOEPTSIZ6 {
        &self.doeptsiz6
    }
    #[doc = "0x3e0 - OTGFS device OUT endpoint-7 control register"]
    #[inline(always)]
    pub const fn doepctl7(&self) -> &DOEPCTL7 {
        &self.doepctl7
    }
    #[doc = "0x3e8 - OTGFS device OUT endpoint-7 interrupt register"]
    #[inline(always)]
    pub const fn doepint7(&self) -> &DOEPINT7 {
        &self.doepint7
    }
    #[doc = "0x3f0 - OTGFS device OUT endpoint-7 transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz7(&self) -> &DOEPTSIZ7 {
        &self.doeptsiz7
    }
}
#[doc = "DCFG (rw) register accessor: OTGFS device configuration register (OTGFS_DCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTGFS device configuration register (OTGFS_DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTGFS device control register (OTGFS_DCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTGFS device control register (OTGFS_DCTL)"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: OTGFS device status register (OTGFS_DSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTGFS device status register (OTGFS_DSTS)"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: OTGFS device all endpoints interrupt register (OTGFS_DAINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTGFS device all endpoints interrupt register (OTGFS_DAINT)"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTGFS all endpoints interrupt mask register (OTGFS_DAINTMSK)"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK (rw) register accessor: OTGFS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTGFS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DIEPCTL0 (rw) register accessor: OTGFS device control IN endpoint 0 control register (OTGFS_DIEPCTL0)\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`]
module"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "OTGFS device control IN endpoint 0 control register (OTGFS_DIEPCTL0)"]
pub mod diepctl0;
#[doc = "DIEPCTL1 (rw) register accessor: OTGFS device IN endpoint-1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl1`]
module"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = "OTGFS device IN endpoint-1 control register"]
pub mod diepctl1;
#[doc = "DIEPCTL2 (rw) register accessor: OTGFS device IN endpoint-2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl2`]
module"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = "OTGFS device IN endpoint-2 control register"]
pub mod diepctl2;
#[doc = "DIEPCTL3 (rw) register accessor: OTGFS device IN endpoint-3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl3`]
module"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = "OTGFS device IN endpoint-3 control register"]
pub mod diepctl3;
#[doc = "DIEPCTL4 (rw) register accessor: OTGFS device IN endpoint-4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl4`]
module"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = "OTGFS device IN endpoint-4 control register"]
pub mod diepctl4;
#[doc = "DIEPCTL5 (rw) register accessor: OTGFS device IN endpoint-5 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl5`]
module"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
#[doc = "OTGFS device IN endpoint-5 control register"]
pub mod diepctl5;
#[doc = "DIEPCTL6 (rw) register accessor: OTGFS device IN endpoint-6 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl6`]
module"]
pub type DIEPCTL6 = crate::Reg<diepctl6::DIEPCTL6_SPEC>;
#[doc = "OTGFS device IN endpoint-6 control register"]
pub mod diepctl6;
#[doc = "DIEPCTL7 (rw) register accessor: OTGFS device IN endpoint-7 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl7`]
module"]
pub type DIEPCTL7 = crate::Reg<diepctl7::DIEPCTL7_SPEC>;
#[doc = "OTGFS device IN endpoint-7 control register"]
pub mod diepctl7;
#[doc = "DOEPCTL0 (rw) register accessor: OTGFS device OUT endpoint-0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl0`]
module"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "OTGFS device OUT endpoint-0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL1 (rw) register accessor: OTGFS device OUT endpoint-1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl1`]
module"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = "OTGFS device OUT endpoint-1 control register"]
pub mod doepctl1;
#[doc = "DOEPCTL2 (rw) register accessor: OTGFS device OUT endpoint-2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl2`]
module"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = "OTGFS device OUT endpoint-2 control register"]
pub mod doepctl2;
#[doc = "DOEPCTL3 (rw) register accessor: OTGFS device OUT endpoint-3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl3`]
module"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = "OTGFS device OUT endpoint-3 control register"]
pub mod doepctl3;
#[doc = "DOEPCTL4 (rw) register accessor: OTGFS device OUT endpoint-4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl4`]
module"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
#[doc = "OTGFS device OUT endpoint-4 control register"]
pub mod doepctl4;
#[doc = "DOEPCTL5 (rw) register accessor: OTGFS device OUT endpoint-5 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl5`]
module"]
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5_SPEC>;
#[doc = "OTGFS device OUT endpoint-5 control register"]
pub mod doepctl5;
#[doc = "DOEPCTL6 (rw) register accessor: OTGFS device OUT endpoint-6 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl6`]
module"]
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6_SPEC>;
#[doc = "OTGFS device OUT endpoint-6 control register"]
pub mod doepctl6;
#[doc = "DOEPCTL7 (rw) register accessor: OTGFS device OUT endpoint-7 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl7`]
module"]
pub type DOEPCTL7 = crate::Reg<doepctl7::DOEPCTL7_SPEC>;
#[doc = "OTGFS device OUT endpoint-7 control register"]
pub mod doepctl7;
#[doc = "DIEPINT0 (rw) register accessor: OTGFS device IN endpoint-0 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint0`]
module"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "OTGFS device IN endpoint-0 interrupt register"]
pub mod diepint0;
#[doc = "DIEPINT1 (rw) register accessor: OTGFS device IN endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint1`]
module"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = "OTGFS device IN endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "DIEPINT2 (rw) register accessor: OTGFS device IN endpoint-2 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint2`]
module"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = "OTGFS device IN endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "DIEPINT3 (rw) register accessor: OTGFS device IN endpoint-3 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint3`]
module"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = "OTGFS device IN endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "DIEPINT4 (rw) register accessor: OTGFS device IN endpoint-4 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint4`]
module"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = "OTGFS device IN endpoint-4 interrupt register"]
pub mod diepint4;
#[doc = "DIEPINT5 (rw) register accessor: OTGFS device IN endpoint-5 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint5`]
module"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
#[doc = "OTGFS device IN endpoint-5 interrupt register"]
pub mod diepint5;
#[doc = "DIEPINT6 (rw) register accessor: OTGFS device IN endpoint-6 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint6`]
module"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
#[doc = "OTGFS device IN endpoint-6 interrupt register"]
pub mod diepint6;
#[doc = "DIEPINT7 (rw) register accessor: OTGFS device IN endpoint-7 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint7`]
module"]
pub type DIEPINT7 = crate::Reg<diepint7::DIEPINT7_SPEC>;
#[doc = "OTGFS device IN endpoint-7 interrupt register"]
pub mod diepint7;
#[doc = "DOEPINT0 (rw) register accessor: OTGFS device OUT endpoint-0 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint0`]
module"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "OTGFS device OUT endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "DOEPINT1 (rw) register accessor: OTGFS device OUT endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint1`]
module"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = "OTGFS device OUT endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "DOEPINT2 (rw) register accessor: OTGFS device OUT endpoint-2 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint2`]
module"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = "OTGFS device OUT endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "DOEPINT3 (rw) register accessor: OTGFS device OUT endpoint-3 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint3`]
module"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = "OTGFS device OUT endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "DOEPINT4 (rw) register accessor: OTGFS device OUT endpoint-4 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint4`]
module"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = "OTGFS device OUT endpoint-4 interrupt register"]
pub mod doepint4;
#[doc = "DOEPINT5 (rw) register accessor: OTGFS device OUT endpoint-5 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint5`]
module"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
#[doc = "OTGFS device OUT endpoint-5 interrupt register"]
pub mod doepint5;
#[doc = "DOEPINT6 (rw) register accessor: OTGFS device OUT endpoint-6 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint6`]
module"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
#[doc = "OTGFS device OUT endpoint-6 interrupt register"]
pub mod doepint6;
#[doc = "DOEPINT7 (rw) register accessor: OTGFS device OUT endpoint-7 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint7`]
module"]
pub type DOEPINT7 = crate::Reg<doepint7::DOEPINT7_SPEC>;
#[doc = "OTGFS device OUT endpoint-7 interrupt register"]
pub mod doepint7;
#[doc = "DIEPTSIZ0 (rw) register accessor: OTGFS device IN endpoint-0 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz0`]
module"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "OTGFS device IN endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "DOEPTSIZ0 (rw) register accessor: OTGFS device OUT endpoint-0 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz0`]
module"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "OTGFS device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "DIEPTSIZ1 (rw) register accessor: OTGFS device IN endpoint-1 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz1`]
module"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = "OTGFS device IN endpoint-1 transfer size register"]
pub mod dieptsiz1;
#[doc = "DIEPTSIZ2 (rw) register accessor: OTGFS device IN endpoint-2 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz2`]
module"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = "OTGFS device IN endpoint-2 transfer size register"]
pub mod dieptsiz2;
#[doc = "DIEPTSIZ3 (rw) register accessor: OTG device IN endpoint-3 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz3`]
module"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = "OTG device IN endpoint-3 transfer size register"]
pub mod dieptsiz3;
#[doc = "DIEPTSIZ4 (rw) register accessor: OTG device IN endpoint-4 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz4`]
module"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = "OTG device IN endpoint-4 transfer size register"]
pub mod dieptsiz4;
#[doc = "DIEPTSIZ5 (rw) register accessor: OTG device IN endpoint-5 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz5`]
module"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
#[doc = "OTG device IN endpoint-5 transfer size register"]
pub mod dieptsiz5;
#[doc = "DIEPTSIZ6 (rw) register accessor: OTG device IN endpoint-6 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz6`]
module"]
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6_SPEC>;
#[doc = "OTG device IN endpoint-6 transfer size register"]
pub mod dieptsiz6;
#[doc = "DIEPTSIZ7 (rw) register accessor: OTG device IN endpoint-7 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz7`]
module"]
pub type DIEPTSIZ7 = crate::Reg<dieptsiz7::DIEPTSIZ7_SPEC>;
#[doc = "OTG device IN endpoint-7 transfer size register"]
pub mod dieptsiz7;
#[doc = "DTXFSTS0 (r) register accessor: OTGFS device IN endpoint-0 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts0`]
module"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "OTGFS device IN endpoint-0 transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "DTXFSTS1 (r) register accessor: OTGFS device IN endpoint-1 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts1`]
module"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = "OTGFS device IN endpoint-1 transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "DTXFSTS2 (r) register accessor: OTGFS device IN endpoint-2 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts2`]
module"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = "OTGFS device IN endpoint-2 transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "DTXFSTS3 (r) register accessor: OTGFS device IN endpoint-3 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts3`]
module"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = "OTGFS device IN endpoint-3 transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "DTXFSTS4 (r) register accessor: OTGFS device IN endpoint-4 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts4`]
module"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = "OTGFS device IN endpoint-4 transmit FIFO status register"]
pub mod dtxfsts4;
#[doc = "DTXFSTS5 (r) register accessor: OTGFS device IN endpoint-5 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts5`]
module"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
#[doc = "OTGFS device IN endpoint-5 transmit FIFO status register"]
pub mod dtxfsts5;
#[doc = "DTXFSTS6 (r) register accessor: OTGFS device IN endpoint-6 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts6`]
module"]
pub type DTXFSTS6 = crate::Reg<dtxfsts6::DTXFSTS6_SPEC>;
#[doc = "OTGFS device IN endpoint-6 transmit FIFO status register"]
pub mod dtxfsts6;
#[doc = "DTXFSTS7 (r) register accessor: OTGFS device IN endpoint-7 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts7`]
module"]
pub type DTXFSTS7 = crate::Reg<dtxfsts7::DTXFSTS7_SPEC>;
#[doc = "OTGFS device IN endpoint-7 transmit FIFO status register"]
pub mod dtxfsts7;
#[doc = "DOEPTSIZ1 (rw) register accessor: OTGFS device OUT endpoint-1 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz1`]
module"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = "OTGFS device OUT endpoint-1 transfer size register"]
pub mod doeptsiz1;
#[doc = "DOEPTSIZ2 (rw) register accessor: OTGFS device OUT endpoint-2 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz2`]
module"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = "OTGFS device OUT endpoint-2 transfer size register"]
pub mod doeptsiz2;
#[doc = "DOEPTSIZ3 (rw) register accessor: OTGFS device OUT endpoint-3 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz3`]
module"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = "OTGFS device OUT endpoint-3 transfer size register"]
pub mod doeptsiz3;
#[doc = "DOEPTSIZ4 (rw) register accessor: OTGFS device OUT endpoint-4 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz4`]
module"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = "OTGFS device OUT endpoint-4 transfer size register"]
pub mod doeptsiz4;
#[doc = "DOEPTSIZ5 (rw) register accessor: OTGFS device OUT endpoint-5 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz5`]
module"]
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5_SPEC>;
#[doc = "OTGFS device OUT endpoint-5 transfer size register"]
pub mod doeptsiz5;
#[doc = "DOEPTSIZ6 (rw) register accessor: OTGFS device OUT endpoint-6 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz6`]
module"]
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6_SPEC>;
#[doc = "OTGFS device OUT endpoint-6 transfer size register"]
pub mod doeptsiz6;
#[doc = "DOEPTSIZ7 (rw) register accessor: OTGFS device OUT endpoint-7 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz7`]
module"]
pub type DOEPTSIZ7 = crate::Reg<doeptsiz7::DOEPTSIZ7_SPEC>;
#[doc = "OTGFS device OUT endpoint-7 transfer size register"]
pub mod doeptsiz7;
