#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    hprt: HPRT,
    _reserved7: [u8; 0xbc],
    hcchar0: HCCHAR0,
    hcsplt0: HCSPLT0,
    hcint0: HCINT0,
    hcintmsk0: HCINTMSK0,
    hctsiz0: HCTSIZ0,
    hcdma0: HCDMA0,
    _reserved13: [u8; 0x08],
    hcchar1: HCCHAR1,
    hcsplt1: HCSPLT1,
    hcint1: HCINT1,
    hcintmsk1: HCINTMSK1,
    hctsiz1: HCTSIZ1,
    hcdma1: HCDMA1,
    _reserved19: [u8; 0x08],
    hcchar2: HCCHAR2,
    hcsplt2: HCSPLT2,
    hcint2: HCINT2,
    hcintmsk2: HCINTMSK2,
    hctsiz2: HCTSIZ2,
    hcdma2: HCDMA2,
    _reserved25: [u8; 0x08],
    hcchar3: HCCHAR3,
    hcsplt3: HCSPLT3,
    hcint3: HCINT3,
    hcintmsk3: HCINTMSK3,
    hctsiz3: HCTSIZ3,
    hcdma3: HCDMA3,
    _reserved31: [u8; 0x08],
    hcchar4: HCCHAR4,
    hcsplt4: HCSPLT4,
    hcint4: HCINT4,
    hcintmsk4: HCINTMSK4,
    hctsiz4: HCTSIZ4,
    hcdma4: HCDMA4,
    _reserved37: [u8; 0x08],
    hcchar5: HCCHAR5,
    hcsplt5: HCSPLT5,
    hcint5: HCINT5,
    hcintmsk5: HCINTMSK5,
    hctsiz5: HCTSIZ5,
    hcdma5: HCDMA5,
    _reserved43: [u8; 0x08],
    hcchar6: HCCHAR6,
    hcsplt6: HCSPLT6,
    hcint6: HCINT6,
    hcintmsk6: HCINTMSK6,
    hctsiz6: HCTSIZ6,
    hcdma6: HCDMA6,
    _reserved49: [u8; 0x08],
    hcchar7: HCCHAR7,
    hcsplt7: HCSPLT7,
    hcint7: HCINT7,
    hcintmsk7: HCINTMSK7,
    hctsiz7: HCTSIZ7,
    hcdma7: HCDMA7,
    _reserved55: [u8; 0x08],
    hcchar8: HCCHAR8,
    hcsplt8: HCSPLT8,
    hcint8: HCINT8,
    hcintmsk8: HCINTMSK8,
    hctsiz8: HCTSIZ8,
    hcdma8: HCDMA8,
    _reserved61: [u8; 0x08],
    hcchar9: HCCHAR9,
    hcsplt9: HCSPLT9,
    hcint9: HCINT9,
    hcintmsk9: HCINTMSK9,
    hctsiz9: HCTSIZ9,
    hcdma9: HCDMA9,
    _reserved67: [u8; 0x08],
    hcchar10: HCCHAR10,
    hcsplt10: HCSPLT10,
    hcint10: HCINT10,
    hcintmsk10: HCINTMSK10,
    hctsiz10: HCTSIZ10,
    hcdma10: HCDMA10,
    _reserved73: [u8; 0x08],
    hcchar11: HCCHAR11,
    hcsplt11: HCSPLT11,
    hcint11: HCINT11,
    hcintmsk11: HCINTMSK11,
    hctsiz11: HCTSIZ11,
    hcdma11: HCDMA11,
    _reserved79: [u8; 0x08],
    hcchar12: HCCHAR12,
    hcsplt12: HCSPLT12,
    hcint12: HCINT12,
    hcintmsk12: HCINTMSK12,
    hctsiz12: HCTSIZ12,
    hcdma12: HCDMA12,
    _reserved85: [u8; 0x08],
    hcchar13: HCCHAR13,
    hcsplt13: HCSPLT13,
    hcint13: HCINT13,
    hcintmsk13: HCINTMSK13,
    hctsiz13: HCTSIZ13,
    hcdma13: HCDMA13,
    _reserved91: [u8; 0x08],
    hcchar14: HCCHAR14,
    hcsplt14: HCSPLT14,
    hcint14: HCINT14,
    hcintmsk14: HCINTMSK14,
    hctsiz14: HCTSIZ14,
    hcdma14: HCDMA14,
    _reserved97: [u8; 0x08],
    hcchar15: HCCHAR15,
    hcsplt15: HCSPLT15,
    hcint15: HCINT15,
    hcintmsk15: HCINTMSK15,
    hctsiz15: HCTSIZ15,
    hcdma15: HCDMA15,
}
impl RegisterBlock {
    #[doc = "0x00 - OTGHS host configuration register (OTGHS_HCFG)"]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0x04 - OTGHS Host frame interval register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x08 - OTGHS host frame number/frame time remaining register (OTGHS_HFNUM)"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0x10 - OTGHS_Host periodic transmit FIFO/queue status register (OTGHS_HPTXSTS)"]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0x14 - OTGHS Host all channels interrupt register"]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x18 - OTGHS host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x40 - OTGHS host port control and status register (OTGHS_HPRT)"]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0x100 - OTGHS host channel-0 characteristics register (OTGHS_HCCHAR0)"]
    #[inline(always)]
    pub const fn hcchar0(&self) -> &HCCHAR0 {
        &self.hcchar0
    }
    #[doc = "0x104 - Host Channel 0 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt0(&self) -> &HCSPLT0 {
        &self.hcsplt0
    }
    #[doc = "0x108 - OTGHS host channel-0 interrupt register (OTGHS_HCINT0)"]
    #[inline(always)]
    pub const fn hcint0(&self) -> &HCINT0 {
        &self.hcint0
    }
    #[doc = "0x10c - OTGHS host channel-0 mask register (OTGHS_HCINTMSK0)"]
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &HCINTMSK0 {
        &self.hcintmsk0
    }
    #[doc = "0x110 - OTGHS host channel-0 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &HCTSIZ0 {
        &self.hctsiz0
    }
    #[doc = "0x114 - Host channel 0 DMA address register"]
    #[inline(always)]
    pub const fn hcdma0(&self) -> &HCDMA0 {
        &self.hcdma0
    }
    #[doc = "0x120 - OTGHS host channel-1 characteristics register (OTGHS_HCCHAR1)"]
    #[inline(always)]
    pub const fn hcchar1(&self) -> &HCCHAR1 {
        &self.hcchar1
    }
    #[doc = "0x124 - Host Channel 1 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt1(&self) -> &HCSPLT1 {
        &self.hcsplt1
    }
    #[doc = "0x128 - OTGHS host channel-1 interrupt register (OTGHS_HCINT1)"]
    #[inline(always)]
    pub const fn hcint1(&self) -> &HCINT1 {
        &self.hcint1
    }
    #[doc = "0x12c - OTGHS host channel-1 mask register (OTGHS_HCINTMSK1)"]
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &HCINTMSK1 {
        &self.hcintmsk1
    }
    #[doc = "0x130 - OTGHS host channel-1 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &HCTSIZ1 {
        &self.hctsiz1
    }
    #[doc = "0x134 - Host channel 1 DMA address register"]
    #[inline(always)]
    pub const fn hcdma1(&self) -> &HCDMA1 {
        &self.hcdma1
    }
    #[doc = "0x140 - OTGHS host channel-2 characteristics register (OTGHS_HCCHAR2)"]
    #[inline(always)]
    pub const fn hcchar2(&self) -> &HCCHAR2 {
        &self.hcchar2
    }
    #[doc = "0x144 - Host Channel 2 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt2(&self) -> &HCSPLT2 {
        &self.hcsplt2
    }
    #[doc = "0x148 - OTGHS host channel-2 interrupt register (OTGHS_HCINT2)"]
    #[inline(always)]
    pub const fn hcint2(&self) -> &HCINT2 {
        &self.hcint2
    }
    #[doc = "0x14c - OTGHS host channel-2 mask register (OTGHS_HCINTMSK2)"]
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &HCINTMSK2 {
        &self.hcintmsk2
    }
    #[doc = "0x150 - OTGHS host channel-2 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &HCTSIZ2 {
        &self.hctsiz2
    }
    #[doc = "0x154 - Host channel 2 DMA address register"]
    #[inline(always)]
    pub const fn hcdma2(&self) -> &HCDMA2 {
        &self.hcdma2
    }
    #[doc = "0x160 - OTGHS host channel-3 characteristics register (OTGHS_HCCHAR3)"]
    #[inline(always)]
    pub const fn hcchar3(&self) -> &HCCHAR3 {
        &self.hcchar3
    }
    #[doc = "0x164 - Host Channel 3 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt3(&self) -> &HCSPLT3 {
        &self.hcsplt3
    }
    #[doc = "0x168 - OTGHS host channel-3 interrupt register (OTGHS_HCINT3)"]
    #[inline(always)]
    pub const fn hcint3(&self) -> &HCINT3 {
        &self.hcint3
    }
    #[doc = "0x16c - OTGHS host channel-3 mask register (OTGHS_HCINTMSK3)"]
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &HCINTMSK3 {
        &self.hcintmsk3
    }
    #[doc = "0x170 - OTGHS host channel-3 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &HCTSIZ3 {
        &self.hctsiz3
    }
    #[doc = "0x174 - Host channel 3 DMA address register"]
    #[inline(always)]
    pub const fn hcdma3(&self) -> &HCDMA3 {
        &self.hcdma3
    }
    #[doc = "0x180 - OTGHS host channel-4 characteristics register (OTGHS_HCCHAR4)"]
    #[inline(always)]
    pub const fn hcchar4(&self) -> &HCCHAR4 {
        &self.hcchar4
    }
    #[doc = "0x184 - Host Channel 1 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt4(&self) -> &HCSPLT4 {
        &self.hcsplt4
    }
    #[doc = "0x188 - OTGHS host channel-4 interrupt register (OTGHS_HCINT4)"]
    #[inline(always)]
    pub const fn hcint4(&self) -> &HCINT4 {
        &self.hcint4
    }
    #[doc = "0x18c - OTGHS host channel-4 mask register (OTGHS_HCINTMSK4)"]
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &HCINTMSK4 {
        &self.hcintmsk4
    }
    #[doc = "0x190 - OTGHS host channel-4 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &HCTSIZ4 {
        &self.hctsiz4
    }
    #[doc = "0x194 - Host channel 4 DMA address register"]
    #[inline(always)]
    pub const fn hcdma4(&self) -> &HCDMA4 {
        &self.hcdma4
    }
    #[doc = "0x1a0 - OTGHS host channel-5 characteristics register (OTGHS_HCCHAR5)"]
    #[inline(always)]
    pub const fn hcchar5(&self) -> &HCCHAR5 {
        &self.hcchar5
    }
    #[doc = "0x1a4 - Host Channel 5 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt5(&self) -> &HCSPLT5 {
        &self.hcsplt5
    }
    #[doc = "0x1a8 - OTGHS host channel-5 interrupt register (OTGHS_HCINT5)"]
    #[inline(always)]
    pub const fn hcint5(&self) -> &HCINT5 {
        &self.hcint5
    }
    #[doc = "0x1ac - OTGHS host channel-5 mask register (OTGHS_HCINTMSK5)"]
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &HCINTMSK5 {
        &self.hcintmsk5
    }
    #[doc = "0x1b0 - OTGHS host channel-5 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &HCTSIZ5 {
        &self.hctsiz5
    }
    #[doc = "0x1b4 - Host channel 5 DMA address register"]
    #[inline(always)]
    pub const fn hcdma5(&self) -> &HCDMA5 {
        &self.hcdma5
    }
    #[doc = "0x1c0 - OTGHS host channel-6 characteristics register (OTGHS_HCCHAR6)"]
    #[inline(always)]
    pub const fn hcchar6(&self) -> &HCCHAR6 {
        &self.hcchar6
    }
    #[doc = "0x1c4 - Host Channel 6 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt6(&self) -> &HCSPLT6 {
        &self.hcsplt6
    }
    #[doc = "0x1c8 - OTGHS host channel-6 interrupt register (OTGHS_HCINT6)"]
    #[inline(always)]
    pub const fn hcint6(&self) -> &HCINT6 {
        &self.hcint6
    }
    #[doc = "0x1cc - OTGHS host channel-6 mask register (OTGHS_HCINTMSK6)"]
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &HCINTMSK6 {
        &self.hcintmsk6
    }
    #[doc = "0x1d0 - OTGHS host channel-6 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &HCTSIZ6 {
        &self.hctsiz6
    }
    #[doc = "0x1d4 - Host channel 6 DMA address register"]
    #[inline(always)]
    pub const fn hcdma6(&self) -> &HCDMA6 {
        &self.hcdma6
    }
    #[doc = "0x1e0 - OTGHS host channel-7 characteristics register (OTGHS_HCCHAR7)"]
    #[inline(always)]
    pub const fn hcchar7(&self) -> &HCCHAR7 {
        &self.hcchar7
    }
    #[doc = "0x1e4 - Host Channel 7 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt7(&self) -> &HCSPLT7 {
        &self.hcsplt7
    }
    #[doc = "0x1e8 - OTGHS host channel-7 interrupt register (OTGHS_HCINT7)"]
    #[inline(always)]
    pub const fn hcint7(&self) -> &HCINT7 {
        &self.hcint7
    }
    #[doc = "0x1ec - OTGHS host channel-7 mask register (OTGHS_HCINTMSK7)"]
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &HCINTMSK7 {
        &self.hcintmsk7
    }
    #[doc = "0x1f0 - OTGHS host channel-7 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &HCTSIZ7 {
        &self.hctsiz7
    }
    #[doc = "0x1f4 - Host channel 7 DMA address register"]
    #[inline(always)]
    pub const fn hcdma7(&self) -> &HCDMA7 {
        &self.hcdma7
    }
    #[doc = "0x200 - OTGHS host channel-8 characteristics register (OTGHS_HCCHAR8)"]
    #[inline(always)]
    pub const fn hcchar8(&self) -> &HCCHAR8 {
        &self.hcchar8
    }
    #[doc = "0x204 - Host Channel 8 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt8(&self) -> &HCSPLT8 {
        &self.hcsplt8
    }
    #[doc = "0x208 - OTGHS host channel-8 interrupt register (OTGHS_HCINT8)"]
    #[inline(always)]
    pub const fn hcint8(&self) -> &HCINT8 {
        &self.hcint8
    }
    #[doc = "0x20c - OTGHS host channel-8 mask register (OTGHS_HCINTMSK8)"]
    #[inline(always)]
    pub const fn hcintmsk8(&self) -> &HCINTMSK8 {
        &self.hcintmsk8
    }
    #[doc = "0x210 - OTGHS host channel-8 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz8(&self) -> &HCTSIZ8 {
        &self.hctsiz8
    }
    #[doc = "0x214 - Host channel 8 DMA address register"]
    #[inline(always)]
    pub const fn hcdma8(&self) -> &HCDMA8 {
        &self.hcdma8
    }
    #[doc = "0x220 - OTGHS host channel-9 characteristics register (OTGHS_HCCHAR9)"]
    #[inline(always)]
    pub const fn hcchar9(&self) -> &HCCHAR9 {
        &self.hcchar9
    }
    #[doc = "0x224 - Host Channel 9 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt9(&self) -> &HCSPLT9 {
        &self.hcsplt9
    }
    #[doc = "0x228 - OTGHS host channel-9 interrupt register (OTGHS_HCINT9)"]
    #[inline(always)]
    pub const fn hcint9(&self) -> &HCINT9 {
        &self.hcint9
    }
    #[doc = "0x22c - OTGHS host channel-9 mask register (OTGHS_HCINTMSK9)"]
    #[inline(always)]
    pub const fn hcintmsk9(&self) -> &HCINTMSK9 {
        &self.hcintmsk9
    }
    #[doc = "0x230 - OTGHS host channel-9 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz9(&self) -> &HCTSIZ9 {
        &self.hctsiz9
    }
    #[doc = "0x234 - Host channel 9 DMA address register"]
    #[inline(always)]
    pub const fn hcdma9(&self) -> &HCDMA9 {
        &self.hcdma9
    }
    #[doc = "0x240 - OTGHS host channel-10 characteristics register (OTGHS_HCCHAR10)"]
    #[inline(always)]
    pub const fn hcchar10(&self) -> &HCCHAR10 {
        &self.hcchar10
    }
    #[doc = "0x244 - Host Channel 10 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt10(&self) -> &HCSPLT10 {
        &self.hcsplt10
    }
    #[doc = "0x248 - OTGHS host channel-10 interrupt register (OTGHS_HCINT10)"]
    #[inline(always)]
    pub const fn hcint10(&self) -> &HCINT10 {
        &self.hcint10
    }
    #[doc = "0x24c - OTGHS host channel-10 mask register (OTGHS_HCINTMSK10)"]
    #[inline(always)]
    pub const fn hcintmsk10(&self) -> &HCINTMSK10 {
        &self.hcintmsk10
    }
    #[doc = "0x250 - OTGHS host channel-10 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz10(&self) -> &HCTSIZ10 {
        &self.hctsiz10
    }
    #[doc = "0x254 - Host channel 10 DMA address register"]
    #[inline(always)]
    pub const fn hcdma10(&self) -> &HCDMA10 {
        &self.hcdma10
    }
    #[doc = "0x260 - OTGHS host channel-7 characteristics register (OTGHS_HCCHAR11)"]
    #[inline(always)]
    pub const fn hcchar11(&self) -> &HCCHAR11 {
        &self.hcchar11
    }
    #[doc = "0x264 - Host Channel 11 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt11(&self) -> &HCSPLT11 {
        &self.hcsplt11
    }
    #[doc = "0x268 - OTGHS host channel-11 interrupt register (OTGHS_HCINT11)"]
    #[inline(always)]
    pub const fn hcint11(&self) -> &HCINT11 {
        &self.hcint11
    }
    #[doc = "0x26c - OTGHS host channel-11 mask register (OTGHS_HCINTMSK11)"]
    #[inline(always)]
    pub const fn hcintmsk11(&self) -> &HCINTMSK11 {
        &self.hcintmsk11
    }
    #[doc = "0x270 - OTGHS host channel-11 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz11(&self) -> &HCTSIZ11 {
        &self.hctsiz11
    }
    #[doc = "0x274 - Host channel 11 DMA address register"]
    #[inline(always)]
    pub const fn hcdma11(&self) -> &HCDMA11 {
        &self.hcdma11
    }
    #[doc = "0x280 - OTGHS host channel-12 characteristics register (OTGHS_HCCHAR12)"]
    #[inline(always)]
    pub const fn hcchar12(&self) -> &HCCHAR12 {
        &self.hcchar12
    }
    #[doc = "0x284 - Host Channel 12 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt12(&self) -> &HCSPLT12 {
        &self.hcsplt12
    }
    #[doc = "0x288 - OTGHS host channel-12 interrupt register (OTGHS_HCINT12)"]
    #[inline(always)]
    pub const fn hcint12(&self) -> &HCINT12 {
        &self.hcint12
    }
    #[doc = "0x28c - OTGHS host channel-12 mask register (OTGHS_HCINTMSK12)"]
    #[inline(always)]
    pub const fn hcintmsk12(&self) -> &HCINTMSK12 {
        &self.hcintmsk12
    }
    #[doc = "0x290 - OTGHS host channel-12 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz12(&self) -> &HCTSIZ12 {
        &self.hctsiz12
    }
    #[doc = "0x294 - Host channel 12 DMA address register"]
    #[inline(always)]
    pub const fn hcdma12(&self) -> &HCDMA12 {
        &self.hcdma12
    }
    #[doc = "0x2a0 - OTGHS host channel-13 characteristics register (OTGHS_HCCHAR13)"]
    #[inline(always)]
    pub const fn hcchar13(&self) -> &HCCHAR13 {
        &self.hcchar13
    }
    #[doc = "0x2a4 - Host Channel 13 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt13(&self) -> &HCSPLT13 {
        &self.hcsplt13
    }
    #[doc = "0x2a8 - OTGHS host channel-13 interrupt register (OTGHS_HCINT13)"]
    #[inline(always)]
    pub const fn hcint13(&self) -> &HCINT13 {
        &self.hcint13
    }
    #[doc = "0x2ac - OTGHS host channel-13 mask register (OTGHS_HCINTMSK13)"]
    #[inline(always)]
    pub const fn hcintmsk13(&self) -> &HCINTMSK13 {
        &self.hcintmsk13
    }
    #[doc = "0x2b0 - OTGHS host channel-13 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz13(&self) -> &HCTSIZ13 {
        &self.hctsiz13
    }
    #[doc = "0x2b4 - Host channel 13 DMA address register"]
    #[inline(always)]
    pub const fn hcdma13(&self) -> &HCDMA13 {
        &self.hcdma13
    }
    #[doc = "0x2c0 - OTGHS host channel-14 characteristics register (OTGHS_HCCHAR14)"]
    #[inline(always)]
    pub const fn hcchar14(&self) -> &HCCHAR14 {
        &self.hcchar14
    }
    #[doc = "0x2c4 - Host Channel 14 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt14(&self) -> &HCSPLT14 {
        &self.hcsplt14
    }
    #[doc = "0x2c8 - OTGHS host channel-14 interrupt register (OTGHS_HCINT14)"]
    #[inline(always)]
    pub const fn hcint14(&self) -> &HCINT14 {
        &self.hcint14
    }
    #[doc = "0x2cc - OTGHS host channel-14 mask register (OTGHS_HCINTMSK14)"]
    #[inline(always)]
    pub const fn hcintmsk14(&self) -> &HCINTMSK14 {
        &self.hcintmsk14
    }
    #[doc = "0x2d0 - OTGHS host channel-14 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz14(&self) -> &HCTSIZ14 {
        &self.hctsiz14
    }
    #[doc = "0x2d4 - Host channel 14 DMA address register"]
    #[inline(always)]
    pub const fn hcdma14(&self) -> &HCDMA14 {
        &self.hcdma14
    }
    #[doc = "0x2e0 - OTGHS host channel-15 characteristics register (OTGHS_HCCHAR15)"]
    #[inline(always)]
    pub const fn hcchar15(&self) -> &HCCHAR15 {
        &self.hcchar15
    }
    #[doc = "0x2e4 - Host Channel 15 Split Control Register"]
    #[inline(always)]
    pub const fn hcsplt15(&self) -> &HCSPLT15 {
        &self.hcsplt15
    }
    #[doc = "0x2e8 - OTGHS host channel-15 interrupt register (OTGHS_HCINT15)"]
    #[inline(always)]
    pub const fn hcint15(&self) -> &HCINT15 {
        &self.hcint15
    }
    #[doc = "0x2ec - OTGHS host channel-15 mask register (OTGHS_HCINTMSK15)"]
    #[inline(always)]
    pub const fn hcintmsk15(&self) -> &HCINTMSK15 {
        &self.hcintmsk15
    }
    #[doc = "0x2f0 - OTGHS host channel-15 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz15(&self) -> &HCTSIZ15 {
        &self.hctsiz15
    }
    #[doc = "0x2f4 - Host channel 15 DMA address register"]
    #[inline(always)]
    pub const fn hcdma15(&self) -> &HCDMA15 {
        &self.hcdma15
    }
}
#[doc = "HCFG (rw) register accessor: OTGHS host configuration register (OTGHS_HCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`] module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTGHS host configuration register (OTGHS_HCFG)"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: OTGHS Host frame interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`] module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTGHS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: OTGHS host frame number/frame time remaining register (OTGHS_HFNUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`] module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTGHS host frame number/frame time remaining register (OTGHS_HFNUM)"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: OTGHS_Host periodic transmit FIFO/queue status register (OTGHS_HPTXSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`] module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "OTGHS_Host periodic transmit FIFO/queue status register (OTGHS_HPTXSTS)"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: OTGHS Host all channels interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`] module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTGHS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: OTGHS host all channels interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`] module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTGHS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: OTGHS host port control and status register (OTGHS_HPRT)\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`] module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTGHS host port control and status register (OTGHS_HPRT)"]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: OTGHS host channel-0 characteristics register (OTGHS_HCCHAR0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar0`] module"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
#[doc = "OTGHS host channel-0 characteristics register (OTGHS_HCCHAR0)"]
pub mod hcchar0;
#[doc = "HCCHAR1 (rw) register accessor: OTGHS host channel-1 characteristics register (OTGHS_HCCHAR1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar1`] module"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
#[doc = "OTGHS host channel-1 characteristics register (OTGHS_HCCHAR1)"]
pub mod hcchar1;
#[doc = "HCCHAR2 (rw) register accessor: OTGHS host channel-2 characteristics register (OTGHS_HCCHAR2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar2`] module"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
#[doc = "OTGHS host channel-2 characteristics register (OTGHS_HCCHAR2)"]
pub mod hcchar2;
#[doc = "HCCHAR3 (rw) register accessor: OTGHS host channel-3 characteristics register (OTGHS_HCCHAR3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar3`] module"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
#[doc = "OTGHS host channel-3 characteristics register (OTGHS_HCCHAR3)"]
pub mod hcchar3;
#[doc = "HCCHAR4 (rw) register accessor: OTGHS host channel-4 characteristics register (OTGHS_HCCHAR4)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar4`] module"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
#[doc = "OTGHS host channel-4 characteristics register (OTGHS_HCCHAR4)"]
pub mod hcchar4;
#[doc = "HCCHAR5 (rw) register accessor: OTGHS host channel-5 characteristics register (OTGHS_HCCHAR5)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar5`] module"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
#[doc = "OTGHS host channel-5 characteristics register (OTGHS_HCCHAR5)"]
pub mod hcchar5;
#[doc = "HCCHAR6 (rw) register accessor: OTGHS host channel-6 characteristics register (OTGHS_HCCHAR6)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar6`] module"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
#[doc = "OTGHS host channel-6 characteristics register (OTGHS_HCCHAR6)"]
pub mod hcchar6;
#[doc = "HCCHAR7 (rw) register accessor: OTGHS host channel-7 characteristics register (OTGHS_HCCHAR7)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar7`] module"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
#[doc = "OTGHS host channel-7 characteristics register (OTGHS_HCCHAR7)"]
pub mod hcchar7;
#[doc = "HCCHAR8 (rw) register accessor: OTGHS host channel-8 characteristics register (OTGHS_HCCHAR8)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar8`] module"]
pub type HCCHAR8 = crate::Reg<hcchar8::HCCHAR8_SPEC>;
#[doc = "OTGHS host channel-8 characteristics register (OTGHS_HCCHAR8)"]
pub mod hcchar8;
#[doc = "HCCHAR9 (rw) register accessor: OTGHS host channel-9 characteristics register (OTGHS_HCCHAR9)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar9`] module"]
pub type HCCHAR9 = crate::Reg<hcchar9::HCCHAR9_SPEC>;
#[doc = "OTGHS host channel-9 characteristics register (OTGHS_HCCHAR9)"]
pub mod hcchar9;
#[doc = "HCCHAR10 (rw) register accessor: OTGHS host channel-10 characteristics register (OTGHS_HCCHAR10)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar10`] module"]
pub type HCCHAR10 = crate::Reg<hcchar10::HCCHAR10_SPEC>;
#[doc = "OTGHS host channel-10 characteristics register (OTGHS_HCCHAR10)"]
pub mod hcchar10;
#[doc = "HCCHAR11 (rw) register accessor: OTGHS host channel-7 characteristics register (OTGHS_HCCHAR11)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar11`] module"]
pub type HCCHAR11 = crate::Reg<hcchar11::HCCHAR11_SPEC>;
#[doc = "OTGHS host channel-7 characteristics register (OTGHS_HCCHAR11)"]
pub mod hcchar11;
#[doc = "HCCHAR12 (rw) register accessor: OTGHS host channel-12 characteristics register (OTGHS_HCCHAR12)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar12`] module"]
pub type HCCHAR12 = crate::Reg<hcchar12::HCCHAR12_SPEC>;
#[doc = "OTGHS host channel-12 characteristics register (OTGHS_HCCHAR12)"]
pub mod hcchar12;
#[doc = "HCCHAR13 (rw) register accessor: OTGHS host channel-13 characteristics register (OTGHS_HCCHAR13)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar13`] module"]
pub type HCCHAR13 = crate::Reg<hcchar13::HCCHAR13_SPEC>;
#[doc = "OTGHS host channel-13 characteristics register (OTGHS_HCCHAR13)"]
pub mod hcchar13;
#[doc = "HCCHAR14 (rw) register accessor: OTGHS host channel-14 characteristics register (OTGHS_HCCHAR14)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar14`] module"]
pub type HCCHAR14 = crate::Reg<hcchar14::HCCHAR14_SPEC>;
#[doc = "OTGHS host channel-14 characteristics register (OTGHS_HCCHAR14)"]
pub mod hcchar14;
#[doc = "HCCHAR15 (rw) register accessor: OTGHS host channel-15 characteristics register (OTGHS_HCCHAR15)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar15`] module"]
pub type HCCHAR15 = crate::Reg<hcchar15::HCCHAR15_SPEC>;
#[doc = "OTGHS host channel-15 characteristics register (OTGHS_HCCHAR15)"]
pub mod hcchar15;
#[doc = "HCSPLT0 (rw) register accessor: Host Channel 0 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt0`] module"]
pub type HCSPLT0 = crate::Reg<hcsplt0::HCSPLT0_SPEC>;
#[doc = "Host Channel 0 Split Control Register"]
pub mod hcsplt0;
#[doc = "HCSPLT1 (rw) register accessor: Host Channel 1 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt1`] module"]
pub type HCSPLT1 = crate::Reg<hcsplt1::HCSPLT1_SPEC>;
#[doc = "Host Channel 1 Split Control Register"]
pub mod hcsplt1;
#[doc = "HCSPLT2 (rw) register accessor: Host Channel 2 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt2`] module"]
pub type HCSPLT2 = crate::Reg<hcsplt2::HCSPLT2_SPEC>;
#[doc = "Host Channel 2 Split Control Register"]
pub mod hcsplt2;
#[doc = "HCSPLT3 (rw) register accessor: Host Channel 3 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt3`] module"]
pub type HCSPLT3 = crate::Reg<hcsplt3::HCSPLT3_SPEC>;
#[doc = "Host Channel 3 Split Control Register"]
pub mod hcsplt3;
#[doc = "HCSPLT4 (rw) register accessor: Host Channel 1 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt4`] module"]
pub type HCSPLT4 = crate::Reg<hcsplt4::HCSPLT4_SPEC>;
#[doc = "Host Channel 1 Split Control Register"]
pub mod hcsplt4;
#[doc = "HCSPLT5 (rw) register accessor: Host Channel 5 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt5`] module"]
pub type HCSPLT5 = crate::Reg<hcsplt5::HCSPLT5_SPEC>;
#[doc = "Host Channel 5 Split Control Register"]
pub mod hcsplt5;
#[doc = "HCSPLT6 (rw) register accessor: Host Channel 6 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt6`] module"]
pub type HCSPLT6 = crate::Reg<hcsplt6::HCSPLT6_SPEC>;
#[doc = "Host Channel 6 Split Control Register"]
pub mod hcsplt6;
#[doc = "HCSPLT7 (rw) register accessor: Host Channel 7 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt7`] module"]
pub type HCSPLT7 = crate::Reg<hcsplt7::HCSPLT7_SPEC>;
#[doc = "Host Channel 7 Split Control Register"]
pub mod hcsplt7;
#[doc = "HCSPLT8 (rw) register accessor: Host Channel 8 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt8`] module"]
pub type HCSPLT8 = crate::Reg<hcsplt8::HCSPLT8_SPEC>;
#[doc = "Host Channel 8 Split Control Register"]
pub mod hcsplt8;
#[doc = "HCSPLT9 (rw) register accessor: Host Channel 9 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt9`] module"]
pub type HCSPLT9 = crate::Reg<hcsplt9::HCSPLT9_SPEC>;
#[doc = "Host Channel 9 Split Control Register"]
pub mod hcsplt9;
#[doc = "HCSPLT10 (rw) register accessor: Host Channel 10 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt10`] module"]
pub type HCSPLT10 = crate::Reg<hcsplt10::HCSPLT10_SPEC>;
#[doc = "Host Channel 10 Split Control Register"]
pub mod hcsplt10;
#[doc = "HCSPLT11 (rw) register accessor: Host Channel 11 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt11`] module"]
pub type HCSPLT11 = crate::Reg<hcsplt11::HCSPLT11_SPEC>;
#[doc = "Host Channel 11 Split Control Register"]
pub mod hcsplt11;
#[doc = "HCSPLT12 (rw) register accessor: Host Channel 12 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt12`] module"]
pub type HCSPLT12 = crate::Reg<hcsplt12::HCSPLT12_SPEC>;
#[doc = "Host Channel 12 Split Control Register"]
pub mod hcsplt12;
#[doc = "HCSPLT13 (rw) register accessor: Host Channel 13 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt13`] module"]
pub type HCSPLT13 = crate::Reg<hcsplt13::HCSPLT13_SPEC>;
#[doc = "Host Channel 13 Split Control Register"]
pub mod hcsplt13;
#[doc = "HCSPLT14 (rw) register accessor: Host Channel 14 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt14`] module"]
pub type HCSPLT14 = crate::Reg<hcsplt14::HCSPLT14_SPEC>;
#[doc = "Host Channel 14 Split Control Register"]
pub mod hcsplt14;
#[doc = "HCSPLT15 (rw) register accessor: Host Channel 15 Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt15`] module"]
pub type HCSPLT15 = crate::Reg<hcsplt15::HCSPLT15_SPEC>;
#[doc = "Host Channel 15 Split Control Register"]
pub mod hcsplt15;
#[doc = "HCINT0 (rw) register accessor: OTGHS host channel-0 interrupt register (OTGHS_HCINT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint0`] module"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
#[doc = "OTGHS host channel-0 interrupt register (OTGHS_HCINT0)"]
pub mod hcint0;
#[doc = "HCINT1 (rw) register accessor: OTGHS host channel-1 interrupt register (OTGHS_HCINT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint1`] module"]
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
#[doc = "OTGHS host channel-1 interrupt register (OTGHS_HCINT1)"]
pub mod hcint1;
#[doc = "HCINT2 (rw) register accessor: OTGHS host channel-2 interrupt register (OTGHS_HCINT2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint2`] module"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
#[doc = "OTGHS host channel-2 interrupt register (OTGHS_HCINT2)"]
pub mod hcint2;
#[doc = "HCINT3 (rw) register accessor: OTGHS host channel-3 interrupt register (OTGHS_HCINT3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint3`] module"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
#[doc = "OTGHS host channel-3 interrupt register (OTGHS_HCINT3)"]
pub mod hcint3;
#[doc = "HCINT4 (rw) register accessor: OTGHS host channel-4 interrupt register (OTGHS_HCINT4)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint4`] module"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
#[doc = "OTGHS host channel-4 interrupt register (OTGHS_HCINT4)"]
pub mod hcint4;
#[doc = "HCINT5 (rw) register accessor: OTGHS host channel-5 interrupt register (OTGHS_HCINT5)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint5`] module"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
#[doc = "OTGHS host channel-5 interrupt register (OTGHS_HCINT5)"]
pub mod hcint5;
#[doc = "HCINT6 (rw) register accessor: OTGHS host channel-6 interrupt register (OTGHS_HCINT6)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint6`] module"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
#[doc = "OTGHS host channel-6 interrupt register (OTGHS_HCINT6)"]
pub mod hcint6;
#[doc = "HCINT7 (rw) register accessor: OTGHS host channel-7 interrupt register (OTGHS_HCINT7)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint7`] module"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
#[doc = "OTGHS host channel-7 interrupt register (OTGHS_HCINT7)"]
pub mod hcint7;
#[doc = "HCINT8 (rw) register accessor: OTGHS host channel-8 interrupt register (OTGHS_HCINT8)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint8`] module"]
pub type HCINT8 = crate::Reg<hcint8::HCINT8_SPEC>;
#[doc = "OTGHS host channel-8 interrupt register (OTGHS_HCINT8)"]
pub mod hcint8;
#[doc = "HCINT9 (rw) register accessor: OTGHS host channel-9 interrupt register (OTGHS_HCINT9)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint9`] module"]
pub type HCINT9 = crate::Reg<hcint9::HCINT9_SPEC>;
#[doc = "OTGHS host channel-9 interrupt register (OTGHS_HCINT9)"]
pub mod hcint9;
#[doc = "HCINT10 (rw) register accessor: OTGHS host channel-10 interrupt register (OTGHS_HCINT10)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint10`] module"]
pub type HCINT10 = crate::Reg<hcint10::HCINT10_SPEC>;
#[doc = "OTGHS host channel-10 interrupt register (OTGHS_HCINT10)"]
pub mod hcint10;
#[doc = "HCINT11 (rw) register accessor: OTGHS host channel-11 interrupt register (OTGHS_HCINT11)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint11`] module"]
pub type HCINT11 = crate::Reg<hcint11::HCINT11_SPEC>;
#[doc = "OTGHS host channel-11 interrupt register (OTGHS_HCINT11)"]
pub mod hcint11;
#[doc = "HCINT12 (rw) register accessor: OTGHS host channel-12 interrupt register (OTGHS_HCINT12)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint12`] module"]
pub type HCINT12 = crate::Reg<hcint12::HCINT12_SPEC>;
#[doc = "OTGHS host channel-12 interrupt register (OTGHS_HCINT12)"]
pub mod hcint12;
#[doc = "HCINT13 (rw) register accessor: OTGHS host channel-13 interrupt register (OTGHS_HCINT13)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint13`] module"]
pub type HCINT13 = crate::Reg<hcint13::HCINT13_SPEC>;
#[doc = "OTGHS host channel-13 interrupt register (OTGHS_HCINT13)"]
pub mod hcint13;
#[doc = "HCINT14 (rw) register accessor: OTGHS host channel-14 interrupt register (OTGHS_HCINT14)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint14`] module"]
pub type HCINT14 = crate::Reg<hcint14::HCINT14_SPEC>;
#[doc = "OTGHS host channel-14 interrupt register (OTGHS_HCINT14)"]
pub mod hcint14;
#[doc = "HCINT15 (rw) register accessor: OTGHS host channel-15 interrupt register (OTGHS_HCINT15)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint15`] module"]
pub type HCINT15 = crate::Reg<hcint15::HCINT15_SPEC>;
#[doc = "OTGHS host channel-15 interrupt register (OTGHS_HCINT15)"]
pub mod hcint15;
#[doc = "HCINTMSK0 (rw) register accessor: OTGHS host channel-0 mask register (OTGHS_HCINTMSK0)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk0`] module"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
#[doc = "OTGHS host channel-0 mask register (OTGHS_HCINTMSK0)"]
pub mod hcintmsk0;
#[doc = "HCINTMSK1 (rw) register accessor: OTGHS host channel-1 mask register (OTGHS_HCINTMSK1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk1`] module"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
#[doc = "OTGHS host channel-1 mask register (OTGHS_HCINTMSK1)"]
pub mod hcintmsk1;
#[doc = "HCINTMSK2 (rw) register accessor: OTGHS host channel-2 mask register (OTGHS_HCINTMSK2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk2`] module"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
#[doc = "OTGHS host channel-2 mask register (OTGHS_HCINTMSK2)"]
pub mod hcintmsk2;
#[doc = "HCINTMSK3 (rw) register accessor: OTGHS host channel-3 mask register (OTGHS_HCINTMSK3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk3`] module"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
#[doc = "OTGHS host channel-3 mask register (OTGHS_HCINTMSK3)"]
pub mod hcintmsk3;
#[doc = "HCINTMSK4 (rw) register accessor: OTGHS host channel-4 mask register (OTGHS_HCINTMSK4)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk4`] module"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
#[doc = "OTGHS host channel-4 mask register (OTGHS_HCINTMSK4)"]
pub mod hcintmsk4;
#[doc = "HCINTMSK5 (rw) register accessor: OTGHS host channel-5 mask register (OTGHS_HCINTMSK5)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk5`] module"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
#[doc = "OTGHS host channel-5 mask register (OTGHS_HCINTMSK5)"]
pub mod hcintmsk5;
#[doc = "HCINTMSK6 (rw) register accessor: OTGHS host channel-6 mask register (OTGHS_HCINTMSK6)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk6`] module"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
#[doc = "OTGHS host channel-6 mask register (OTGHS_HCINTMSK6)"]
pub mod hcintmsk6;
#[doc = "HCINTMSK7 (rw) register accessor: OTGHS host channel-7 mask register (OTGHS_HCINTMSK7)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk7`] module"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
#[doc = "OTGHS host channel-7 mask register (OTGHS_HCINTMSK7)"]
pub mod hcintmsk7;
#[doc = "HCINTMSK8 (rw) register accessor: OTGHS host channel-8 mask register (OTGHS_HCINTMSK8)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk8`] module"]
pub type HCINTMSK8 = crate::Reg<hcintmsk8::HCINTMSK8_SPEC>;
#[doc = "OTGHS host channel-8 mask register (OTGHS_HCINTMSK8)"]
pub mod hcintmsk8;
#[doc = "HCINTMSK9 (rw) register accessor: OTGHS host channel-9 mask register (OTGHS_HCINTMSK9)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk9`] module"]
pub type HCINTMSK9 = crate::Reg<hcintmsk9::HCINTMSK9_SPEC>;
#[doc = "OTGHS host channel-9 mask register (OTGHS_HCINTMSK9)"]
pub mod hcintmsk9;
#[doc = "HCINTMSK10 (rw) register accessor: OTGHS host channel-10 mask register (OTGHS_HCINTMSK10)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk10`] module"]
pub type HCINTMSK10 = crate::Reg<hcintmsk10::HCINTMSK10_SPEC>;
#[doc = "OTGHS host channel-10 mask register (OTGHS_HCINTMSK10)"]
pub mod hcintmsk10;
#[doc = "HCINTMSK11 (rw) register accessor: OTGHS host channel-11 mask register (OTGHS_HCINTMSK11)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk11`] module"]
pub type HCINTMSK11 = crate::Reg<hcintmsk11::HCINTMSK11_SPEC>;
#[doc = "OTGHS host channel-11 mask register (OTGHS_HCINTMSK11)"]
pub mod hcintmsk11;
#[doc = "HCINTMSK12 (rw) register accessor: OTGHS host channel-12 mask register (OTGHS_HCINTMSK12)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk12`] module"]
pub type HCINTMSK12 = crate::Reg<hcintmsk12::HCINTMSK12_SPEC>;
#[doc = "OTGHS host channel-12 mask register (OTGHS_HCINTMSK12)"]
pub mod hcintmsk12;
#[doc = "HCINTMSK13 (rw) register accessor: OTGHS host channel-13 mask register (OTGHS_HCINTMSK13)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk13`] module"]
pub type HCINTMSK13 = crate::Reg<hcintmsk13::HCINTMSK13_SPEC>;
#[doc = "OTGHS host channel-13 mask register (OTGHS_HCINTMSK13)"]
pub mod hcintmsk13;
#[doc = "HCINTMSK14 (rw) register accessor: OTGHS host channel-14 mask register (OTGHS_HCINTMSK14)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk14`] module"]
pub type HCINTMSK14 = crate::Reg<hcintmsk14::HCINTMSK14_SPEC>;
#[doc = "OTGHS host channel-14 mask register (OTGHS_HCINTMSK14)"]
pub mod hcintmsk14;
#[doc = "HCINTMSK15 (rw) register accessor: OTGHS host channel-15 mask register (OTGHS_HCINTMSK15)\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk15`] module"]
pub type HCINTMSK15 = crate::Reg<hcintmsk15::HCINTMSK15_SPEC>;
#[doc = "OTGHS host channel-15 mask register (OTGHS_HCINTMSK15)"]
pub mod hcintmsk15;
#[doc = "HCTSIZ0 (rw) register accessor: OTGHS host channel-0 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz0`] module"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
#[doc = "OTGHS host channel-0 transfer size register"]
pub mod hctsiz0;
#[doc = "HCTSIZ1 (rw) register accessor: OTGHS host channel-1 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz1`] module"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
#[doc = "OTGHS host channel-1 transfer size register"]
pub mod hctsiz1;
#[doc = "HCTSIZ2 (rw) register accessor: OTGHS host channel-2 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz2`] module"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
#[doc = "OTGHS host channel-2 transfer size register"]
pub mod hctsiz2;
#[doc = "HCTSIZ3 (rw) register accessor: OTGHS host channel-3 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz3`] module"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
#[doc = "OTGHS host channel-3 transfer size register"]
pub mod hctsiz3;
#[doc = "HCTSIZ4 (rw) register accessor: OTGHS host channel-4 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz4`] module"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
#[doc = "OTGHS host channel-4 transfer size register"]
pub mod hctsiz4;
#[doc = "HCTSIZ5 (rw) register accessor: OTGHS host channel-5 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz5`] module"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
#[doc = "OTGHS host channel-5 transfer size register"]
pub mod hctsiz5;
#[doc = "HCTSIZ6 (rw) register accessor: OTGHS host channel-6 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz6`] module"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
#[doc = "OTGHS host channel-6 transfer size register"]
pub mod hctsiz6;
#[doc = "HCTSIZ7 (rw) register accessor: OTGHS host channel-7 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz7`] module"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
#[doc = "OTGHS host channel-7 transfer size register"]
pub mod hctsiz7;
#[doc = "HCTSIZ8 (rw) register accessor: OTGHS host channel-8 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz8`] module"]
pub type HCTSIZ8 = crate::Reg<hctsiz8::HCTSIZ8_SPEC>;
#[doc = "OTGHS host channel-8 transfer size register"]
pub mod hctsiz8;
#[doc = "HCTSIZ9 (rw) register accessor: OTGHS host channel-9 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz9`] module"]
pub type HCTSIZ9 = crate::Reg<hctsiz9::HCTSIZ9_SPEC>;
#[doc = "OTGHS host channel-9 transfer size register"]
pub mod hctsiz9;
#[doc = "HCTSIZ10 (rw) register accessor: OTGHS host channel-10 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz10`] module"]
pub type HCTSIZ10 = crate::Reg<hctsiz10::HCTSIZ10_SPEC>;
#[doc = "OTGHS host channel-10 transfer size register"]
pub mod hctsiz10;
#[doc = "HCTSIZ11 (rw) register accessor: OTGHS host channel-11 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz11`] module"]
pub type HCTSIZ11 = crate::Reg<hctsiz11::HCTSIZ11_SPEC>;
#[doc = "OTGHS host channel-11 transfer size register"]
pub mod hctsiz11;
#[doc = "HCTSIZ12 (rw) register accessor: OTGHS host channel-12 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz12`] module"]
pub type HCTSIZ12 = crate::Reg<hctsiz12::HCTSIZ12_SPEC>;
#[doc = "OTGHS host channel-12 transfer size register"]
pub mod hctsiz12;
#[doc = "HCTSIZ13 (rw) register accessor: OTGHS host channel-13 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz13`] module"]
pub type HCTSIZ13 = crate::Reg<hctsiz13::HCTSIZ13_SPEC>;
#[doc = "OTGHS host channel-13 transfer size register"]
pub mod hctsiz13;
#[doc = "HCTSIZ14 (rw) register accessor: OTGHS host channel-14 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz14`] module"]
pub type HCTSIZ14 = crate::Reg<hctsiz14::HCTSIZ14_SPEC>;
#[doc = "OTGHS host channel-14 transfer size register"]
pub mod hctsiz14;
#[doc = "HCTSIZ15 (rw) register accessor: OTGHS host channel-15 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz15`] module"]
pub type HCTSIZ15 = crate::Reg<hctsiz15::HCTSIZ15_SPEC>;
#[doc = "OTGHS host channel-15 transfer size register"]
pub mod hctsiz15;
#[doc = "HCDMA0 (rw) register accessor: Host channel 0 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma0`] module"]
pub type HCDMA0 = crate::Reg<hcdma0::HCDMA0_SPEC>;
#[doc = "Host channel 0 DMA address register"]
pub mod hcdma0;
#[doc = "HCDMA1 (rw) register accessor: Host channel 1 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma1`] module"]
pub type HCDMA1 = crate::Reg<hcdma1::HCDMA1_SPEC>;
#[doc = "Host channel 1 DMA address register"]
pub mod hcdma1;
#[doc = "HCDMA2 (rw) register accessor: Host channel 2 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma2`] module"]
pub type HCDMA2 = crate::Reg<hcdma2::HCDMA2_SPEC>;
#[doc = "Host channel 2 DMA address register"]
pub mod hcdma2;
#[doc = "HCDMA3 (rw) register accessor: Host channel 3 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma3`] module"]
pub type HCDMA3 = crate::Reg<hcdma3::HCDMA3_SPEC>;
#[doc = "Host channel 3 DMA address register"]
pub mod hcdma3;
#[doc = "HCDMA4 (rw) register accessor: Host channel 4 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma4`] module"]
pub type HCDMA4 = crate::Reg<hcdma4::HCDMA4_SPEC>;
#[doc = "Host channel 4 DMA address register"]
pub mod hcdma4;
#[doc = "HCDMA5 (rw) register accessor: Host channel 5 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma5`] module"]
pub type HCDMA5 = crate::Reg<hcdma5::HCDMA5_SPEC>;
#[doc = "Host channel 5 DMA address register"]
pub mod hcdma5;
#[doc = "HCDMA6 (rw) register accessor: Host channel 6 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma6`] module"]
pub type HCDMA6 = crate::Reg<hcdma6::HCDMA6_SPEC>;
#[doc = "Host channel 6 DMA address register"]
pub mod hcdma6;
#[doc = "HCDMA7 (rw) register accessor: Host channel 7 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma7`] module"]
pub type HCDMA7 = crate::Reg<hcdma7::HCDMA7_SPEC>;
#[doc = "Host channel 7 DMA address register"]
pub mod hcdma7;
#[doc = "HCDMA8 (rw) register accessor: Host channel 8 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma8`] module"]
pub type HCDMA8 = crate::Reg<hcdma8::HCDMA8_SPEC>;
#[doc = "Host channel 8 DMA address register"]
pub mod hcdma8;
#[doc = "HCDMA9 (rw) register accessor: Host channel 9 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma9`] module"]
pub type HCDMA9 = crate::Reg<hcdma9::HCDMA9_SPEC>;
#[doc = "Host channel 9 DMA address register"]
pub mod hcdma9;
#[doc = "HCDMA10 (rw) register accessor: Host channel 10 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma10`] module"]
pub type HCDMA10 = crate::Reg<hcdma10::HCDMA10_SPEC>;
#[doc = "Host channel 10 DMA address register"]
pub mod hcdma10;
#[doc = "HCDMA11 (rw) register accessor: Host channel 11 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma11`] module"]
pub type HCDMA11 = crate::Reg<hcdma11::HCDMA11_SPEC>;
#[doc = "Host channel 11 DMA address register"]
pub mod hcdma11;
#[doc = "HCDMA12 (rw) register accessor: Host channel 12 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma12`] module"]
pub type HCDMA12 = crate::Reg<hcdma12::HCDMA12_SPEC>;
#[doc = "Host channel 12 DMA address register"]
pub mod hcdma12;
#[doc = "HCDMA13 (rw) register accessor: Host channel 13 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma13`] module"]
pub type HCDMA13 = crate::Reg<hcdma13::HCDMA13_SPEC>;
#[doc = "Host channel 13 DMA address register"]
pub mod hcdma13;
#[doc = "HCDMA14 (rw) register accessor: Host channel 14 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma14`] module"]
pub type HCDMA14 = crate::Reg<hcdma14::HCDMA14_SPEC>;
#[doc = "Host channel 14 DMA address register"]
pub mod hcdma14;
#[doc = "HCDMA15 (rw) register accessor: Host channel 15 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma15`] module"]
pub type HCDMA15 = crate::Reg<hcdma15::HCDMA15_SPEC>;
#[doc = "Host channel 15 DMA address register"]
pub mod hcdma15;
