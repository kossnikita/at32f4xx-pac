#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTGHS device configuration register (OTGHS_DCFG)"]
    pub dcfg: DCFG,
    #[doc = "0x04 - OTGHS device control register (OTGHS_DCTL)"]
    pub dctl: DCTL,
    #[doc = "0x08 - OTGHS device status register (OTGHS_DSTS)"]
    pub dsts: DSTS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTGHS device IN endpoint common interrupt mask register (OTGHS_DIEPMSK)"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x14 - OTGHS device OUT endpoint common interrupt mask register (OTGHS_DOEPMSK)"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x18 - OTGHS device all endpoints interrupt register (OTGHS_DAINT)"]
    pub daint: DAINT,
    #[doc = "0x1c - OTGHS all endpoints interrupt mask register (OTGHS_DAINTMSK)"]
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 0x14],
    #[doc = "0x34 - OTGHS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    #[doc = "0x38 - Device Each Endpoints Interrupt Register"]
    pub deachint: DEACHINT,
    #[doc = "0x3c - Device Each Endpoints Interrupt Mask Register"]
    pub deachintmsk: DEACHINTMSK,
    _reserved10: [u8; 0x04],
    #[doc = "0x44 - Device Each IN Endpoint 1 Interrupt Register"]
    pub diepeachmsk1: DIEPEACHMSK1,
    _reserved11: [u8; 0x3c],
    #[doc = "0x84 - Device Each OUT Endpoint 1 Interrupt Register"]
    pub doepeachmsk1: DOEPEACHMSK1,
    _reserved12: [u8; 0x78],
    #[doc = "0x100 - OTGHS device control IN endpoint 0 control register (OTGHS_DIEPCTL0)"]
    pub diepctl0: DIEPCTL0,
    _reserved13: [u8; 0x04],
    #[doc = "0x108 - OTGHS device IN endpoint-0 interrupt register"]
    pub diepint0: DIEPINT0,
    _reserved14: [u8; 0x04],
    #[doc = "0x110 - OTGHS device IN endpoint-0 transfer size register"]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x114 - Device IN Endpoint 0 DMA Address register"]
    pub diepdma0: DIEPDMA0,
    #[doc = "0x118 - OTGHS device IN endpoint-0 transmit FIFO status register"]
    pub dtxfsts0: DTXFSTS0,
    _reserved17: [u8; 0x04],
    #[doc = "0x120 - OTGHS device IN endpoint-1 control register"]
    pub diepctl1: DIEPCTL1,
    _reserved18: [u8; 0x04],
    #[doc = "0x128 - OTGHS device IN endpoint-1 interrupt register"]
    pub diepint1: DIEPINT1,
    _reserved19: [u8; 0x04],
    #[doc = "0x130 - OTGHS device IN endpoint-1 transfer size register"]
    pub dieptsiz1: DIEPTSIZ1,
    #[doc = "0x134 - Device IN Endpoint 1 DMA Address register"]
    pub diepdma1: DIEPDMA1,
    #[doc = "0x138 - OTGHS device IN endpoint-1 transmit FIFO status register"]
    pub dtxfsts1: DTXFSTS1,
    _reserved22: [u8; 0x04],
    #[doc = "0x140 - OTGHS device IN endpoint-2 control register"]
    pub diepctl2: DIEPCTL2,
    _reserved23: [u8; 0x04],
    #[doc = "0x148 - OTGHS device IN endpoint-2 interrupt register"]
    pub diepint2: DIEPINT2,
    _reserved24: [u8; 0x04],
    #[doc = "0x150 - OTGHS device IN endpoint-2 transfer size register"]
    pub dieptsiz2: DIEPTSIZ2,
    #[doc = "0x154 - Device IN Endpoint 2 DMA Address register"]
    pub diepdma2: DIEPDMA2,
    #[doc = "0x158 - OTGHS device IN endpoint-2 transmit FIFO status register"]
    pub dtxfsts2: DTXFSTS2,
    _reserved27: [u8; 0x04],
    #[doc = "0x160 - OTGHS device IN endpoint-3 control register"]
    pub diepctl3: DIEPCTL3,
    _reserved28: [u8; 0x04],
    #[doc = "0x168 - OTGHS device IN endpoint-3 interrupt register"]
    pub diepint3: DIEPINT3,
    _reserved29: [u8; 0x04],
    #[doc = "0x170 - OTG device IN endpoint-3 transfer size register"]
    pub dieptsiz3: DIEPTSIZ3,
    #[doc = "0x174 - Device IN Endpoint 3 DMA Address register"]
    pub diepdma3: DIEPDMA3,
    #[doc = "0x178 - OTGHS device IN endpoint-3 transmit FIFO status register"]
    pub dtxfsts3: DTXFSTS3,
    _reserved32: [u8; 0x04],
    #[doc = "0x180 - OTGHS device IN endpoint-4 control register"]
    pub diepctl4: DIEPCTL4,
    _reserved33: [u8; 0x04],
    #[doc = "0x188 - OTGHS device IN endpoint-4 interrupt register"]
    pub diepint4: DIEPINT4,
    _reserved34: [u8; 0x04],
    #[doc = "0x190 - OTG device IN endpoint-4 transfer size register"]
    pub dieptsiz4: DIEPTSIZ4,
    #[doc = "0x194 - Device IN Endpoint 4 DMA Address register"]
    pub diepdma4: DIEPDMA4,
    #[doc = "0x198 - OTGHS device IN endpoint-4 transmit FIFO status register"]
    pub dtxfsts4: DTXFSTS4,
    _reserved37: [u8; 0x04],
    #[doc = "0x1a0 - OTGHS device IN endpoint-5 control register"]
    pub diepctl5: DIEPCTL5,
    _reserved38: [u8; 0x04],
    #[doc = "0x1a8 - OTGHS device IN endpoint-5 interrupt register"]
    pub diepint5: DIEPINT5,
    _reserved39: [u8; 0x04],
    #[doc = "0x1b0 - OTG device IN endpoint-5 transfer size register"]
    pub dieptsiz5: DIEPTSIZ5,
    #[doc = "0x1b4 - Device IN Endpoint 5 DMA Address register"]
    pub diepdma5: DIEPDMA5,
    #[doc = "0x1b8 - OTGHS device IN endpoint-5 transmit FIFO status register"]
    pub dtxfsts5: DTXFSTS5,
    _reserved42: [u8; 0x04],
    #[doc = "0x1c0 - OTGHS device IN endpoint-6 control register"]
    pub diepctl6: DIEPCTL6,
    _reserved43: [u8; 0x04],
    #[doc = "0x1c8 - OTGHS device IN endpoint-6 interrupt register"]
    pub diepint6: DIEPINT6,
    _reserved44: [u8; 0x04],
    #[doc = "0x1d0 - OTG device IN endpoint-6 transfer size register"]
    pub dieptsiz6: DIEPTSIZ6,
    #[doc = "0x1d4 - Device IN Endpoint 6 DMA Address register"]
    pub diepdma6: DIEPDMA6,
    #[doc = "0x1d8 - OTGHS device IN endpoint-6 transmit FIFO status register"]
    pub dtxfsts6: DTXFSTS6,
    _reserved47: [u8; 0x04],
    #[doc = "0x1e0 - OTGHS device IN endpoint-7 control register"]
    pub diepctl7: DIEPCTL7,
    _reserved48: [u8; 0x04],
    #[doc = "0x1e8 - OTGHS device IN endpoint-7 interrupt register"]
    pub diepint7: DIEPINT7,
    _reserved49: [u8; 0x04],
    #[doc = "0x1f0 - OTG device IN endpoint-7 transfer size register"]
    pub dieptsiz7: DIEPTSIZ7,
    #[doc = "0x1f4 - Device IN Endpoint 7 DMA Address register"]
    pub diepdma7: DIEPDMA7,
    #[doc = "0x1f8 - OTGHS device IN endpoint-7 transmit FIFO status register"]
    pub dtxfsts7: DTXFSTS7,
    _reserved52: [u8; 0x0104],
    #[doc = "0x300 - OTGHS device OUT endpoint-0 control register"]
    pub doepctl0: DOEPCTL0,
    _reserved53: [u8; 0x04],
    #[doc = "0x308 - OTGHS device OUT endpoint-0 interrupt register"]
    pub doepint0: DOEPINT0,
    _reserved54: [u8; 0x04],
    #[doc = "0x310 - OTGHS device OUT endpoint-0 transfer size register"]
    pub doeptsiz0: DOEPTSIZ0,
    #[doc = "0x314 - Device OUT Endpoint 0 DMA Address register"]
    pub doepdma0: DOEPDMA0,
    _reserved56: [u8; 0x08],
    #[doc = "0x320 - OTGHS device OUT endpoint-1 control register"]
    pub doepctl1: DOEPCTL1,
    _reserved57: [u8; 0x04],
    #[doc = "0x328 - OTGHS device OUT endpoint-1 interrupt register"]
    pub doepint1: DOEPINT1,
    _reserved58: [u8; 0x04],
    #[doc = "0x330 - OTGHS device OUT endpoint-1 transfer size register"]
    pub doeptsiz1: DOEPTSIZ1,
    #[doc = "0x334 - Device OUT Endpoint 1 DMA Address register"]
    pub doepdma1: DOEPDMA1,
    _reserved60: [u8; 0x08],
    #[doc = "0x340 - OTGHS device OUT endpoint-2 control register"]
    pub doepctl2: DOEPCTL2,
    _reserved61: [u8; 0x04],
    #[doc = "0x348 - OTGHS device OUT endpoint-2 interrupt register"]
    pub doepint2: DOEPINT2,
    _reserved62: [u8; 0x04],
    #[doc = "0x350 - OTGHS device OUT endpoint-2 transfer size register"]
    pub doeptsiz2: DOEPTSIZ2,
    #[doc = "0x354 - Device OUT Endpoint 2 DMA Address register"]
    pub doepdma2: DOEPDMA2,
    _reserved64: [u8; 0x08],
    #[doc = "0x360 - OTGHS device OUT endpoint-3 control register"]
    pub doepctl3: DOEPCTL3,
    _reserved65: [u8; 0x04],
    #[doc = "0x368 - OTGHS device OUT endpoint-3 interrupt register"]
    pub doepint3: DOEPINT3,
    _reserved66: [u8; 0x04],
    #[doc = "0x370 - OTGHS device OUT endpoint-3 transfer size register"]
    pub doeptsiz3: DOEPTSIZ3,
    #[doc = "0x374 - Device OUT Endpoint 3 DMA Address register"]
    pub doepdma3: DOEPDMA3,
    _reserved68: [u8; 0x08],
    #[doc = "0x380 - OTGHS device OUT endpoint-4 control register"]
    pub doepctl4: DOEPCTL4,
    _reserved69: [u8; 0x04],
    #[doc = "0x388 - OTGHS device OUT endpoint-4 interrupt register"]
    pub doepint4: DOEPINT4,
    _reserved70: [u8; 0x04],
    #[doc = "0x390 - OTGHS device OUT endpoint-4 transfer size register"]
    pub doeptsiz4: DOEPTSIZ4,
    #[doc = "0x394 - Device OUT Endpoint 4 DMA Address register"]
    pub doepdma4: DOEPDMA4,
    _reserved72: [u8; 0x08],
    #[doc = "0x3a0 - OTGHS device OUT endpoint-5 control register"]
    pub doepctl5: DOEPCTL5,
    _reserved73: [u8; 0x04],
    #[doc = "0x3a8 - OTGHS device OUT endpoint-5 interrupt register"]
    pub doepint5: DOEPINT5,
    _reserved74: [u8; 0x04],
    #[doc = "0x3b0 - OTGHS device OUT endpoint-5 transfer size register"]
    pub doeptsiz5: DOEPTSIZ5,
    #[doc = "0x3b4 - Device OUT Endpoint 5 DMA Address register"]
    pub doepdma5: DOEPDMA5,
    _reserved76: [u8; 0x08],
    #[doc = "0x3c0 - OTGHS device OUT endpoint-6 control register"]
    pub doepctl6: DOEPCTL6,
    _reserved77: [u8; 0x04],
    #[doc = "0x3c8 - OTGHS device OUT endpoint-6 interrupt register"]
    pub doepint6: DOEPINT6,
    _reserved78: [u8; 0x04],
    #[doc = "0x3d0 - OTGHS device OUT endpoint-6 transfer size register"]
    pub doeptsiz6: DOEPTSIZ6,
    #[doc = "0x3d4 - Device OUT Endpoint 6 DMA Address register"]
    pub doepdma6: DOEPDMA6,
    _reserved80: [u8; 0x08],
    #[doc = "0x3e0 - OTGHS device OUT endpoint-7 control register"]
    pub doepctl7: DOEPCTL7,
    _reserved81: [u8; 0x04],
    #[doc = "0x3e8 - OTGHS device OUT endpoint-7 interrupt register"]
    pub doepint7: DOEPINT7,
    _reserved82: [u8; 0x04],
    #[doc = "0x3f0 - OTGHS device OUT endpoint-7 transfer size register"]
    pub doeptsiz7: DOEPTSIZ7,
    #[doc = "0x3f4 - Device OUT Endpoint 7 DMA Address register"]
    pub doepdma7: DOEPDMA7,
}
#[doc = "DCFG (rw) register accessor: OTGHS device configuration register (OTGHS_DCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTGHS device configuration register (OTGHS_DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTGHS device control register (OTGHS_DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTGHS device control register (OTGHS_DCTL)"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: OTGHS device status register (OTGHS_DSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTGHS device status register (OTGHS_DSTS)"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: OTGHS device IN endpoint common interrupt mask register (OTGHS_DIEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTGHS device IN endpoint common interrupt mask register (OTGHS_DIEPMSK)"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: OTGHS device OUT endpoint common interrupt mask register (OTGHS_DOEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTGHS device OUT endpoint common interrupt mask register (OTGHS_DOEPMSK)"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: OTGHS device all endpoints interrupt register (OTGHS_DAINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTGHS device all endpoints interrupt register (OTGHS_DAINT)"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: OTGHS all endpoints interrupt mask register (OTGHS_DAINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTGHS all endpoints interrupt mask register (OTGHS_DAINTMSK)"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK (rw) register accessor: OTGHS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTGHS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT (rw) register accessor: Device Each Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deachint`]
module"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "Device Each Endpoints Interrupt Register"]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: Device Each Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`deachintmsk`]
module"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "Device Each Endpoints Interrupt Mask Register"]
pub mod deachintmsk;
#[doc = "DIEPEACHMSK1 (rw) register accessor: Device Each IN Endpoint 1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepeachmsk1`]
module"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
#[doc = "Device Each IN Endpoint 1 Interrupt Register"]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 (rw) register accessor: Device Each OUT Endpoint 1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepeachmsk1`]
module"]
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>;
#[doc = "Device Each OUT Endpoint 1 Interrupt Register"]
pub mod doepeachmsk1;
#[doc = "DIEPCTL0 (rw) register accessor: OTGHS device control IN endpoint 0 control register (OTGHS_DIEPCTL0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl0`]
module"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "OTGHS device control IN endpoint 0 control register (OTGHS_DIEPCTL0)"]
pub mod diepctl0;
#[doc = "DIEPCTL1 (rw) register accessor: OTGHS device IN endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl1`]
module"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = "OTGHS device IN endpoint-1 control register"]
pub mod diepctl1;
#[doc = "DIEPCTL2 (rw) register accessor: OTGHS device IN endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl2`]
module"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = "OTGHS device IN endpoint-2 control register"]
pub mod diepctl2;
#[doc = "DIEPCTL3 (rw) register accessor: OTGHS device IN endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl3`]
module"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = "OTGHS device IN endpoint-3 control register"]
pub mod diepctl3;
#[doc = "DIEPCTL4 (rw) register accessor: OTGHS device IN endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl4`]
module"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = "OTGHS device IN endpoint-4 control register"]
pub mod diepctl4;
#[doc = "DIEPCTL5 (rw) register accessor: OTGHS device IN endpoint-5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl5`]
module"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
#[doc = "OTGHS device IN endpoint-5 control register"]
pub mod diepctl5;
#[doc = "DIEPCTL6 (rw) register accessor: OTGHS device IN endpoint-6 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl6`]
module"]
pub type DIEPCTL6 = crate::Reg<diepctl6::DIEPCTL6_SPEC>;
#[doc = "OTGHS device IN endpoint-6 control register"]
pub mod diepctl6;
#[doc = "DIEPCTL7 (rw) register accessor: OTGHS device IN endpoint-7 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl7`]
module"]
pub type DIEPCTL7 = crate::Reg<diepctl7::DIEPCTL7_SPEC>;
#[doc = "OTGHS device IN endpoint-7 control register"]
pub mod diepctl7;
#[doc = "DOEPCTL0 (rw) register accessor: OTGHS device OUT endpoint-0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl0`]
module"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "OTGHS device OUT endpoint-0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL1 (rw) register accessor: OTGHS device OUT endpoint-1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl1`]
module"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = "OTGHS device OUT endpoint-1 control register"]
pub mod doepctl1;
#[doc = "DOEPCTL2 (rw) register accessor: OTGHS device OUT endpoint-2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl2`]
module"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = "OTGHS device OUT endpoint-2 control register"]
pub mod doepctl2;
#[doc = "DOEPCTL3 (rw) register accessor: OTGHS device OUT endpoint-3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl3`]
module"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = "OTGHS device OUT endpoint-3 control register"]
pub mod doepctl3;
#[doc = "DOEPCTL4 (rw) register accessor: OTGHS device OUT endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl4`]
module"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
#[doc = "OTGHS device OUT endpoint-4 control register"]
pub mod doepctl4;
#[doc = "DOEPCTL5 (rw) register accessor: OTGHS device OUT endpoint-5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl5`]
module"]
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5_SPEC>;
#[doc = "OTGHS device OUT endpoint-5 control register"]
pub mod doepctl5;
#[doc = "DOEPCTL6 (rw) register accessor: OTGHS device OUT endpoint-6 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl6`]
module"]
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6_SPEC>;
#[doc = "OTGHS device OUT endpoint-6 control register"]
pub mod doepctl6;
#[doc = "DOEPCTL7 (rw) register accessor: OTGHS device OUT endpoint-7 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl7`]
module"]
pub type DOEPCTL7 = crate::Reg<doepctl7::DOEPCTL7_SPEC>;
#[doc = "OTGHS device OUT endpoint-7 control register"]
pub mod doepctl7;
#[doc = "DIEPINT0 (rw) register accessor: OTGHS device IN endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint0`]
module"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "OTGHS device IN endpoint-0 interrupt register"]
pub mod diepint0;
#[doc = "DIEPINT1 (rw) register accessor: OTGHS device IN endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint1`]
module"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = "OTGHS device IN endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "DIEPINT2 (rw) register accessor: OTGHS device IN endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint2`]
module"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = "OTGHS device IN endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "DIEPINT3 (rw) register accessor: OTGHS device IN endpoint-3 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint3`]
module"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = "OTGHS device IN endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "DIEPINT4 (rw) register accessor: OTGHS device IN endpoint-4 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint4`]
module"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = "OTGHS device IN endpoint-4 interrupt register"]
pub mod diepint4;
#[doc = "DIEPINT5 (rw) register accessor: OTGHS device IN endpoint-5 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint5`]
module"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
#[doc = "OTGHS device IN endpoint-5 interrupt register"]
pub mod diepint5;
#[doc = "DIEPINT6 (rw) register accessor: OTGHS device IN endpoint-6 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint6`]
module"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
#[doc = "OTGHS device IN endpoint-6 interrupt register"]
pub mod diepint6;
#[doc = "DIEPINT7 (rw) register accessor: OTGHS device IN endpoint-7 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint7`]
module"]
pub type DIEPINT7 = crate::Reg<diepint7::DIEPINT7_SPEC>;
#[doc = "OTGHS device IN endpoint-7 interrupt register"]
pub mod diepint7;
#[doc = "DOEPINT0 (rw) register accessor: OTGHS device OUT endpoint-0 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint0`]
module"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "OTGHS device OUT endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "DOEPINT1 (rw) register accessor: OTGHS device OUT endpoint-1 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint1`]
module"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = "OTGHS device OUT endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "DOEPINT2 (rw) register accessor: OTGHS device OUT endpoint-2 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint2`]
module"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = "OTGHS device OUT endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "DOEPINT3 (rw) register accessor: OTGHS device OUT endpoint-3 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint3`]
module"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = "OTGHS device OUT endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "DOEPINT4 (rw) register accessor: OTGHS device OUT endpoint-4 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint4`]
module"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = "OTGHS device OUT endpoint-4 interrupt register"]
pub mod doepint4;
#[doc = "DOEPINT5 (rw) register accessor: OTGHS device OUT endpoint-5 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint5`]
module"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
#[doc = "OTGHS device OUT endpoint-5 interrupt register"]
pub mod doepint5;
#[doc = "DOEPINT6 (rw) register accessor: OTGHS device OUT endpoint-6 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint6`]
module"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
#[doc = "OTGHS device OUT endpoint-6 interrupt register"]
pub mod doepint6;
#[doc = "DOEPINT7 (rw) register accessor: OTGHS device OUT endpoint-7 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint7`]
module"]
pub type DOEPINT7 = crate::Reg<doepint7::DOEPINT7_SPEC>;
#[doc = "OTGHS device OUT endpoint-7 interrupt register"]
pub mod doepint7;
#[doc = "DIEPTSIZ0 (rw) register accessor: OTGHS device IN endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz0`]
module"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "OTGHS device IN endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "DOEPTSIZ0 (rw) register accessor: OTGHS device OUT endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz0`]
module"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "OTGHS device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "DIEPTSIZ1 (rw) register accessor: OTGHS device IN endpoint-1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz1`]
module"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = "OTGHS device IN endpoint-1 transfer size register"]
pub mod dieptsiz1;
#[doc = "DIEPTSIZ2 (rw) register accessor: OTGHS device IN endpoint-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz2`]
module"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = "OTGHS device IN endpoint-2 transfer size register"]
pub mod dieptsiz2;
#[doc = "DIEPTSIZ3 (rw) register accessor: OTG device IN endpoint-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz3`]
module"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = "OTG device IN endpoint-3 transfer size register"]
pub mod dieptsiz3;
#[doc = "DIEPTSIZ4 (rw) register accessor: OTG device IN endpoint-4 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz4`]
module"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = "OTG device IN endpoint-4 transfer size register"]
pub mod dieptsiz4;
#[doc = "DIEPTSIZ5 (rw) register accessor: OTG device IN endpoint-5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz5`]
module"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
#[doc = "OTG device IN endpoint-5 transfer size register"]
pub mod dieptsiz5;
#[doc = "DIEPTSIZ6 (rw) register accessor: OTG device IN endpoint-6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz6`]
module"]
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6_SPEC>;
#[doc = "OTG device IN endpoint-6 transfer size register"]
pub mod dieptsiz6;
#[doc = "DIEPTSIZ7 (rw) register accessor: OTG device IN endpoint-7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz7`]
module"]
pub type DIEPTSIZ7 = crate::Reg<dieptsiz7::DIEPTSIZ7_SPEC>;
#[doc = "OTG device IN endpoint-7 transfer size register"]
pub mod dieptsiz7;
#[doc = "DIEPDMA0 (rw) register accessor: Device IN Endpoint 0 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma0`]
module"]
pub type DIEPDMA0 = crate::Reg<diepdma0::DIEPDMA0_SPEC>;
#[doc = "Device IN Endpoint 0 DMA Address register"]
pub mod diepdma0;
#[doc = "DIEPDMA1 (rw) register accessor: Device IN Endpoint 1 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma1`]
module"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
#[doc = "Device IN Endpoint 1 DMA Address register"]
pub mod diepdma1;
#[doc = "DIEPDMA2 (rw) register accessor: Device IN Endpoint 2 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma2`]
module"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
#[doc = "Device IN Endpoint 2 DMA Address register"]
pub mod diepdma2;
#[doc = "DIEPDMA3 (rw) register accessor: Device IN Endpoint 3 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma3`]
module"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
#[doc = "Device IN Endpoint 3 DMA Address register"]
pub mod diepdma3;
#[doc = "DIEPDMA4 (rw) register accessor: Device IN Endpoint 4 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma4`]
module"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
#[doc = "Device IN Endpoint 4 DMA Address register"]
pub mod diepdma4;
#[doc = "DIEPDMA5 (rw) register accessor: Device IN Endpoint 5 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma5`]
module"]
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5_SPEC>;
#[doc = "Device IN Endpoint 5 DMA Address register"]
pub mod diepdma5;
#[doc = "DIEPDMA6 (rw) register accessor: Device IN Endpoint 6 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma6`]
module"]
pub type DIEPDMA6 = crate::Reg<diepdma6::DIEPDMA6_SPEC>;
#[doc = "Device IN Endpoint 6 DMA Address register"]
pub mod diepdma6;
#[doc = "DIEPDMA7 (rw) register accessor: Device IN Endpoint 7 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma7`]
module"]
pub type DIEPDMA7 = crate::Reg<diepdma7::DIEPDMA7_SPEC>;
#[doc = "Device IN Endpoint 7 DMA Address register"]
pub mod diepdma7;
#[doc = "DTXFSTS0 (r) register accessor: OTGHS device IN endpoint-0 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts0`]
module"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "OTGHS device IN endpoint-0 transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "DTXFSTS1 (r) register accessor: OTGHS device IN endpoint-1 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts1`]
module"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = "OTGHS device IN endpoint-1 transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "DTXFSTS2 (r) register accessor: OTGHS device IN endpoint-2 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts2`]
module"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = "OTGHS device IN endpoint-2 transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "DTXFSTS3 (r) register accessor: OTGHS device IN endpoint-3 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts3`]
module"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = "OTGHS device IN endpoint-3 transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "DTXFSTS4 (r) register accessor: OTGHS device IN endpoint-4 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts4`]
module"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = "OTGHS device IN endpoint-4 transmit FIFO status register"]
pub mod dtxfsts4;
#[doc = "DTXFSTS5 (r) register accessor: OTGHS device IN endpoint-5 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts5`]
module"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
#[doc = "OTGHS device IN endpoint-5 transmit FIFO status register"]
pub mod dtxfsts5;
#[doc = "DTXFSTS6 (r) register accessor: OTGHS device IN endpoint-6 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts6`]
module"]
pub type DTXFSTS6 = crate::Reg<dtxfsts6::DTXFSTS6_SPEC>;
#[doc = "OTGHS device IN endpoint-6 transmit FIFO status register"]
pub mod dtxfsts6;
#[doc = "DTXFSTS7 (r) register accessor: OTGHS device IN endpoint-7 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts7`]
module"]
pub type DTXFSTS7 = crate::Reg<dtxfsts7::DTXFSTS7_SPEC>;
#[doc = "OTGHS device IN endpoint-7 transmit FIFO status register"]
pub mod dtxfsts7;
#[doc = "DOEPTSIZ1 (rw) register accessor: OTGHS device OUT endpoint-1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz1`]
module"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = "OTGHS device OUT endpoint-1 transfer size register"]
pub mod doeptsiz1;
#[doc = "DOEPTSIZ2 (rw) register accessor: OTGHS device OUT endpoint-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz2`]
module"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = "OTGHS device OUT endpoint-2 transfer size register"]
pub mod doeptsiz2;
#[doc = "DOEPTSIZ3 (rw) register accessor: OTGHS device OUT endpoint-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz3`]
module"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = "OTGHS device OUT endpoint-3 transfer size register"]
pub mod doeptsiz3;
#[doc = "DOEPTSIZ4 (rw) register accessor: OTGHS device OUT endpoint-4 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz4`]
module"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = "OTGHS device OUT endpoint-4 transfer size register"]
pub mod doeptsiz4;
#[doc = "DOEPTSIZ5 (rw) register accessor: OTGHS device OUT endpoint-5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz5`]
module"]
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5_SPEC>;
#[doc = "OTGHS device OUT endpoint-5 transfer size register"]
pub mod doeptsiz5;
#[doc = "DOEPTSIZ6 (rw) register accessor: OTGHS device OUT endpoint-6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz6`]
module"]
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6_SPEC>;
#[doc = "OTGHS device OUT endpoint-6 transfer size register"]
pub mod doeptsiz6;
#[doc = "DOEPTSIZ7 (rw) register accessor: OTGHS device OUT endpoint-7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz7`]
module"]
pub type DOEPTSIZ7 = crate::Reg<doeptsiz7::DOEPTSIZ7_SPEC>;
#[doc = "OTGHS device OUT endpoint-7 transfer size register"]
pub mod doeptsiz7;
#[doc = "DOEPDMA0 (rw) register accessor: Device OUT Endpoint 0 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma0`]
module"]
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0_SPEC>;
#[doc = "Device OUT Endpoint 0 DMA Address register"]
pub mod doepdma0;
#[doc = "DOEPDMA1 (rw) register accessor: Device OUT Endpoint 1 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma1`]
module"]
pub type DOEPDMA1 = crate::Reg<doepdma1::DOEPDMA1_SPEC>;
#[doc = "Device OUT Endpoint 1 DMA Address register"]
pub mod doepdma1;
#[doc = "DOEPDMA2 (rw) register accessor: Device OUT Endpoint 2 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma2`]
module"]
pub type DOEPDMA2 = crate::Reg<doepdma2::DOEPDMA2_SPEC>;
#[doc = "Device OUT Endpoint 2 DMA Address register"]
pub mod doepdma2;
#[doc = "DOEPDMA3 (rw) register accessor: Device OUT Endpoint 3 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma3`]
module"]
pub type DOEPDMA3 = crate::Reg<doepdma3::DOEPDMA3_SPEC>;
#[doc = "Device OUT Endpoint 3 DMA Address register"]
pub mod doepdma3;
#[doc = "DOEPDMA4 (rw) register accessor: Device OUT Endpoint 4 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma4`]
module"]
pub type DOEPDMA4 = crate::Reg<doepdma4::DOEPDMA4_SPEC>;
#[doc = "Device OUT Endpoint 4 DMA Address register"]
pub mod doepdma4;
#[doc = "DOEPDMA5 (rw) register accessor: Device OUT Endpoint 5 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma5`]
module"]
pub type DOEPDMA5 = crate::Reg<doepdma5::DOEPDMA5_SPEC>;
#[doc = "Device OUT Endpoint 5 DMA Address register"]
pub mod doepdma5;
#[doc = "DOEPDMA6 (rw) register accessor: Device OUT Endpoint 6 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma6`]
module"]
pub type DOEPDMA6 = crate::Reg<doepdma6::DOEPDMA6_SPEC>;
#[doc = "Device OUT Endpoint 6 DMA Address register"]
pub mod doepdma6;
#[doc = "DOEPDMA7 (rw) register accessor: Device OUT Endpoint 7 DMA Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma7`]
module"]
pub type DOEPDMA7 = crate::Reg<doepdma7::DOEPDMA7_SPEC>;
#[doc = "Device OUT Endpoint 7 DMA Address register"]
pub mod doepdma7;
