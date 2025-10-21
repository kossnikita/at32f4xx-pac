#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts1: STS1,
    sts2: STS2,
    clr1: CLR1,
    clr2: CLR2,
    s1ctrl: S1CTRL,
    s1dtcnt: S1DTCNT,
    s1paddr: S1PADDR,
    s1m0addr: S1M0ADDR,
    s1m1addr: S1M1ADDR,
    s1fctrl: S1FCTRL,
    s2ctrl: S2CTRL,
    s2dtcnt: S2DTCNT,
    s2paddr: S2PADDR,
    s2m0addr: S2M0ADDR,
    s2m1addr: S2M1ADDR,
    s2fctrl: S2FCTRL,
    s3ctrl: S3CTRL,
    s3dtcnt: S3DTCNT,
    s3paddr: S3PADDR,
    s3m0addr: S3M0ADDR,
    s3m1addr: S3M1ADDR,
    s3fctrl: S3FCTRL,
    s4ctrl: S4CTRL,
    s4dtcnt: S4DTCNT,
    s4paddr: S4PADDR,
    s4m0addr: S4M0ADDR,
    s4m1addr: S4M1ADDR,
    s4fctrl: S4FCTRL,
    s5ctrl: S5CTRL,
    s5dtcnt: S5DTCNT,
    s5paddr: S5PADDR,
    s5m0addr: S5M0ADDR,
    s5m1addr: S5M1ADDR,
    s5fctrl: S5FCTRL,
    s6ctrl: S6CTRL,
    s6dtcnt: S6DTCNT,
    s6paddr: S6PADDR,
    s6m0addr: S6M0ADDR,
    s6m1addr: S6M1ADDR,
    s6fctrl: S6FCTRL,
    s7ctrl: S7CTRL,
    s7dtcnt: S7DTCNT,
    s7paddr: S7PADDR,
    s7m0addr: S7M0ADDR,
    s7m1addr: S7M1ADDR,
    s7fctrl: S7FCTRL,
    s8ctrl: S8CTRL,
    s8dtcnt: S8DTCNT,
    s8paddr: S8PADDR,
    s8m0addr: S8M0ADDR,
    s8m1addr: S8M1ADDR,
    s8fctrl: S8FCTRL,
    llctrl: LLCTRL,
    s1llp: S1LLP,
    s2llp: S2LLP,
    s3llp: S3LLP,
    s4llp: S4LLP,
    s5llp: S5LLP,
    s6llp: S6LLP,
    s7llp: S7LLP,
    s8llp: S8LLP,
    s2dctrl: S2DCTRL,
    s1_2dcnt: S1_2DCNT,
    s1_stride: S1_STRIDE,
    s2_2dcnt: S2_2DCNT,
    s2_stride: S2_STRIDE,
    s3_2dcnt: S3_2DCNT,
    s3_stride: S3_STRIDE,
    s4_2dcnt: S4_2DCNT,
    s4_stride: S4_STRIDE,
    s5_2dcnt: S5_2DCNT,
    s5_stride: S5_STRIDE,
    s6_2dcnt: S6_2DCNT,
    s6_stride: S6_STRIDE,
    s7_2dcnt: S7_2DCNT,
    s7_stride: S7_STRIDE,
    s8_2dcnt: S8_2DCNT,
    s8_stride: S8_STRIDE,
    syncen: SYNCEN,
    muxsel: MUXSEL,
    muxs1ctrl: MUXS1CTRL,
    muxs2ctrl: MUXS2CTRL,
    muxs3ctrl: MUXS3CTRL,
    muxs4ctrl: MUXS4CTRL,
    muxs5ctrl: MUXS5CTRL,
    muxs6ctrl: MUXS6CTRL,
    muxs7ctrl: MUXS7CTRL,
    muxs8ctrl: MUXS8CTRL,
    muxg1ctrl: MUXG1CTRL,
    muxg2ctrl: MUXG2CTRL,
    muxg3ctrl: MUXG3CTRL,
    muxg4ctrl: MUXG4CTRL,
    muxsyncsts: MUXSYNCSTS,
    muxsyncclr: MUXSYNCCLR,
    muxgsts: MUXGSTS,
    muxgclr: MUXGCLR,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt status register1"]
    #[inline(always)]
    pub const fn sts1(&self) -> &STS1 {
        &self.sts1
    }
    #[doc = "0x04 - Interrupt status register2"]
    #[inline(always)]
    pub const fn sts2(&self) -> &STS2 {
        &self.sts2
    }
    #[doc = "0x08 - Interrupt flag clear register1"]
    #[inline(always)]
    pub const fn clr1(&self) -> &CLR1 {
        &self.clr1
    }
    #[doc = "0x0c - Interrupt flag clear register2"]
    #[inline(always)]
    pub const fn clr2(&self) -> &CLR2 {
        &self.clr2
    }
    #[doc = "0x10 - stream 1 control register"]
    #[inline(always)]
    pub const fn s1ctrl(&self) -> &S1CTRL {
        &self.s1ctrl
    }
    #[doc = "0x14 - stream 1 number of data register"]
    #[inline(always)]
    pub const fn s1dtcnt(&self) -> &S1DTCNT {
        &self.s1dtcnt
    }
    #[doc = "0x18 - stream 1 peripheral address register"]
    #[inline(always)]
    pub const fn s1paddr(&self) -> &S1PADDR {
        &self.s1paddr
    }
    #[doc = "0x1c - stream 1 memory 0 address register"]
    #[inline(always)]
    pub const fn s1m0addr(&self) -> &S1M0ADDR {
        &self.s1m0addr
    }
    #[doc = "0x20 - stream 1 memory 1 address register"]
    #[inline(always)]
    pub const fn s1m1addr(&self) -> &S1M1ADDR {
        &self.s1m1addr
    }
    #[doc = "0x24 - stream 1 FIFO control register"]
    #[inline(always)]
    pub const fn s1fctrl(&self) -> &S1FCTRL {
        &self.s1fctrl
    }
    #[doc = "0x28 - stream 2 control register"]
    #[inline(always)]
    pub const fn s2ctrl(&self) -> &S2CTRL {
        &self.s2ctrl
    }
    #[doc = "0x2c - stream 2 number of data register"]
    #[inline(always)]
    pub const fn s2dtcnt(&self) -> &S2DTCNT {
        &self.s2dtcnt
    }
    #[doc = "0x30 - stream 2 peripheral address register"]
    #[inline(always)]
    pub const fn s2paddr(&self) -> &S2PADDR {
        &self.s2paddr
    }
    #[doc = "0x34 - stream 2 memory 0 address register"]
    #[inline(always)]
    pub const fn s2m0addr(&self) -> &S2M0ADDR {
        &self.s2m0addr
    }
    #[doc = "0x38 - stream 2 memory 1 address register"]
    #[inline(always)]
    pub const fn s2m1addr(&self) -> &S2M1ADDR {
        &self.s2m1addr
    }
    #[doc = "0x3c - stream 2 FIFO control register"]
    #[inline(always)]
    pub const fn s2fctrl(&self) -> &S2FCTRL {
        &self.s2fctrl
    }
    #[doc = "0x40 - stream 3 control register"]
    #[inline(always)]
    pub const fn s3ctrl(&self) -> &S3CTRL {
        &self.s3ctrl
    }
    #[doc = "0x44 - stream 3 number of data register"]
    #[inline(always)]
    pub const fn s3dtcnt(&self) -> &S3DTCNT {
        &self.s3dtcnt
    }
    #[doc = "0x48 - stream 3 peripheral address register"]
    #[inline(always)]
    pub const fn s3paddr(&self) -> &S3PADDR {
        &self.s3paddr
    }
    #[doc = "0x4c - stream 3 memory 0 address register"]
    #[inline(always)]
    pub const fn s3m0addr(&self) -> &S3M0ADDR {
        &self.s3m0addr
    }
    #[doc = "0x50 - stream 3 memory 1 address register"]
    #[inline(always)]
    pub const fn s3m1addr(&self) -> &S3M1ADDR {
        &self.s3m1addr
    }
    #[doc = "0x54 - stream 3 FIFO control register"]
    #[inline(always)]
    pub const fn s3fctrl(&self) -> &S3FCTRL {
        &self.s3fctrl
    }
    #[doc = "0x58 - stream 4 control register"]
    #[inline(always)]
    pub const fn s4ctrl(&self) -> &S4CTRL {
        &self.s4ctrl
    }
    #[doc = "0x5c - stream 4 number of data register"]
    #[inline(always)]
    pub const fn s4dtcnt(&self) -> &S4DTCNT {
        &self.s4dtcnt
    }
    #[doc = "0x60 - stream 4 peripheral address register"]
    #[inline(always)]
    pub const fn s4paddr(&self) -> &S4PADDR {
        &self.s4paddr
    }
    #[doc = "0x64 - stream 4 memory 0 address register"]
    #[inline(always)]
    pub const fn s4m0addr(&self) -> &S4M0ADDR {
        &self.s4m0addr
    }
    #[doc = "0x68 - stream 4 memory 1 address register"]
    #[inline(always)]
    pub const fn s4m1addr(&self) -> &S4M1ADDR {
        &self.s4m1addr
    }
    #[doc = "0x6c - stream 4 FIFO control register"]
    #[inline(always)]
    pub const fn s4fctrl(&self) -> &S4FCTRL {
        &self.s4fctrl
    }
    #[doc = "0x70 - stream 5 control register"]
    #[inline(always)]
    pub const fn s5ctrl(&self) -> &S5CTRL {
        &self.s5ctrl
    }
    #[doc = "0x74 - stream 5 number of data register"]
    #[inline(always)]
    pub const fn s5dtcnt(&self) -> &S5DTCNT {
        &self.s5dtcnt
    }
    #[doc = "0x78 - stream 5 peripheral address register"]
    #[inline(always)]
    pub const fn s5paddr(&self) -> &S5PADDR {
        &self.s5paddr
    }
    #[doc = "0x7c - stream 5 memory 0 address register"]
    #[inline(always)]
    pub const fn s5m0addr(&self) -> &S5M0ADDR {
        &self.s5m0addr
    }
    #[doc = "0x80 - stream 5 memory 1 address register"]
    #[inline(always)]
    pub const fn s5m1addr(&self) -> &S5M1ADDR {
        &self.s5m1addr
    }
    #[doc = "0x84 - stream 5 FIFO control register"]
    #[inline(always)]
    pub const fn s5fctrl(&self) -> &S5FCTRL {
        &self.s5fctrl
    }
    #[doc = "0x88 - stream 6 control register"]
    #[inline(always)]
    pub const fn s6ctrl(&self) -> &S6CTRL {
        &self.s6ctrl
    }
    #[doc = "0x8c - stream 6 number of data register"]
    #[inline(always)]
    pub const fn s6dtcnt(&self) -> &S6DTCNT {
        &self.s6dtcnt
    }
    #[doc = "0x90 - stream 6 peripheral address register"]
    #[inline(always)]
    pub const fn s6paddr(&self) -> &S6PADDR {
        &self.s6paddr
    }
    #[doc = "0x94 - stream 6 memory 0 address register"]
    #[inline(always)]
    pub const fn s6m0addr(&self) -> &S6M0ADDR {
        &self.s6m0addr
    }
    #[doc = "0x98 - stream 6 memory 1 address register"]
    #[inline(always)]
    pub const fn s6m1addr(&self) -> &S6M1ADDR {
        &self.s6m1addr
    }
    #[doc = "0x9c - stream 6 FIFO control register"]
    #[inline(always)]
    pub const fn s6fctrl(&self) -> &S6FCTRL {
        &self.s6fctrl
    }
    #[doc = "0xa0 - stream 7 control register"]
    #[inline(always)]
    pub const fn s7ctrl(&self) -> &S7CTRL {
        &self.s7ctrl
    }
    #[doc = "0xa4 - stream 7 number of data register"]
    #[inline(always)]
    pub const fn s7dtcnt(&self) -> &S7DTCNT {
        &self.s7dtcnt
    }
    #[doc = "0xa8 - stream 7 peripheral address register"]
    #[inline(always)]
    pub const fn s7paddr(&self) -> &S7PADDR {
        &self.s7paddr
    }
    #[doc = "0xac - stream 7 memory 0 address register"]
    #[inline(always)]
    pub const fn s7m0addr(&self) -> &S7M0ADDR {
        &self.s7m0addr
    }
    #[doc = "0xb0 - stream 7 memory 1 address register"]
    #[inline(always)]
    pub const fn s7m1addr(&self) -> &S7M1ADDR {
        &self.s7m1addr
    }
    #[doc = "0xb4 - stream 7 FIFO control register"]
    #[inline(always)]
    pub const fn s7fctrl(&self) -> &S7FCTRL {
        &self.s7fctrl
    }
    #[doc = "0xb8 - stream 8 control register"]
    #[inline(always)]
    pub const fn s8ctrl(&self) -> &S8CTRL {
        &self.s8ctrl
    }
    #[doc = "0xbc - stream 8 number of data register"]
    #[inline(always)]
    pub const fn s8dtcnt(&self) -> &S8DTCNT {
        &self.s8dtcnt
    }
    #[doc = "0xc0 - stream 8 peripheral address register"]
    #[inline(always)]
    pub const fn s8paddr(&self) -> &S8PADDR {
        &self.s8paddr
    }
    #[doc = "0xc4 - stream 8 memory 0 address register"]
    #[inline(always)]
    pub const fn s8m0addr(&self) -> &S8M0ADDR {
        &self.s8m0addr
    }
    #[doc = "0xc8 - stream 8 memory 1 address register"]
    #[inline(always)]
    pub const fn s8m1addr(&self) -> &S8M1ADDR {
        &self.s8m1addr
    }
    #[doc = "0xcc - stream 8 FIFO control register"]
    #[inline(always)]
    pub const fn s8fctrl(&self) -> &S8FCTRL {
        &self.s8fctrl
    }
    #[doc = "0xd0 - DMA Link List Control Register"]
    #[inline(always)]
    pub const fn llctrl(&self) -> &LLCTRL {
        &self.llctrl
    }
    #[doc = "0xd4 - Stream 1 Link List Pointer"]
    #[inline(always)]
    pub const fn s1llp(&self) -> &S1LLP {
        &self.s1llp
    }
    #[doc = "0xd8 - Stream 2 Link List Pointer"]
    #[inline(always)]
    pub const fn s2llp(&self) -> &S2LLP {
        &self.s2llp
    }
    #[doc = "0xdc - Stream 3 Link List Pointer"]
    #[inline(always)]
    pub const fn s3llp(&self) -> &S3LLP {
        &self.s3llp
    }
    #[doc = "0xe0 - Stream 4 Link List Pointer"]
    #[inline(always)]
    pub const fn s4llp(&self) -> &S4LLP {
        &self.s4llp
    }
    #[doc = "0xe4 - Stream 5 Link List Pointer"]
    #[inline(always)]
    pub const fn s5llp(&self) -> &S5LLP {
        &self.s5llp
    }
    #[doc = "0xe8 - Stream 6 Link List Pointer"]
    #[inline(always)]
    pub const fn s6llp(&self) -> &S6LLP {
        &self.s6llp
    }
    #[doc = "0xec - Stream 7 Link List Pointer"]
    #[inline(always)]
    pub const fn s7llp(&self) -> &S7LLP {
        &self.s7llp
    }
    #[doc = "0xf0 - Stream 8 Link List Pointer"]
    #[inline(always)]
    pub const fn s8llp(&self) -> &S8LLP {
        &self.s8llp
    }
    #[doc = "0xf4 - EDMA 2D Transfer Control Register"]
    #[inline(always)]
    pub const fn s2dctrl(&self) -> &S2DCTRL {
        &self.s2dctrl
    }
    #[doc = "0xf8 - Stream 1 2D Transfer Count"]
    #[inline(always)]
    pub const fn s1_2dcnt(&self) -> &S1_2DCNT {
        &self.s1_2dcnt
    }
    #[doc = "0xfc - Stream 1 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s1_stride(&self) -> &S1_STRIDE {
        &self.s1_stride
    }
    #[doc = "0x100 - Stream 2 2D Transfer Count"]
    #[inline(always)]
    pub const fn s2_2dcnt(&self) -> &S2_2DCNT {
        &self.s2_2dcnt
    }
    #[doc = "0x104 - Stream 2 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s2_stride(&self) -> &S2_STRIDE {
        &self.s2_stride
    }
    #[doc = "0x108 - Stream 3 2D Transfer Count"]
    #[inline(always)]
    pub const fn s3_2dcnt(&self) -> &S3_2DCNT {
        &self.s3_2dcnt
    }
    #[doc = "0x10c - Stream 3 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s3_stride(&self) -> &S3_STRIDE {
        &self.s3_stride
    }
    #[doc = "0x110 - Stream 4 2D Transfer Count"]
    #[inline(always)]
    pub const fn s4_2dcnt(&self) -> &S4_2DCNT {
        &self.s4_2dcnt
    }
    #[doc = "0x114 - Stream 4 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s4_stride(&self) -> &S4_STRIDE {
        &self.s4_stride
    }
    #[doc = "0x118 - Stream 5 2D Transfer Count"]
    #[inline(always)]
    pub const fn s5_2dcnt(&self) -> &S5_2DCNT {
        &self.s5_2dcnt
    }
    #[doc = "0x11c - Stream 5 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s5_stride(&self) -> &S5_STRIDE {
        &self.s5_stride
    }
    #[doc = "0x120 - Stream 6 2D Transfer Count"]
    #[inline(always)]
    pub const fn s6_2dcnt(&self) -> &S6_2DCNT {
        &self.s6_2dcnt
    }
    #[doc = "0x124 - Stream 6 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s6_stride(&self) -> &S6_STRIDE {
        &self.s6_stride
    }
    #[doc = "0x128 - Stream 7 2D Transfer Count"]
    #[inline(always)]
    pub const fn s7_2dcnt(&self) -> &S7_2DCNT {
        &self.s7_2dcnt
    }
    #[doc = "0x12c - Stream 7 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s7_stride(&self) -> &S7_STRIDE {
        &self.s7_stride
    }
    #[doc = "0x130 - Stream 8 2D Transfer Count"]
    #[inline(always)]
    pub const fn s8_2dcnt(&self) -> &S8_2DCNT {
        &self.s8_2dcnt
    }
    #[doc = "0x134 - Stream 8 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s8_stride(&self) -> &S8_STRIDE {
        &self.s8_stride
    }
    #[doc = "0x138 - Sync Enable"]
    #[inline(always)]
    pub const fn syncen(&self) -> &SYNCEN {
        &self.syncen
    }
    #[doc = "0x13c - EDMA MUX Table Selection"]
    #[inline(always)]
    pub const fn muxsel(&self) -> &MUXSEL {
        &self.muxsel
    }
    #[doc = "0x140 - Stream 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxs1ctrl(&self) -> &MUXS1CTRL {
        &self.muxs1ctrl
    }
    #[doc = "0x144 - Stream 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxs2ctrl(&self) -> &MUXS2CTRL {
        &self.muxs2ctrl
    }
    #[doc = "0x148 - Stream 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxs3ctrl(&self) -> &MUXS3CTRL {
        &self.muxs3ctrl
    }
    #[doc = "0x14c - Stream 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxs4ctrl(&self) -> &MUXS4CTRL {
        &self.muxs4ctrl
    }
    #[doc = "0x150 - Stream x Configuration Register"]
    #[inline(always)]
    pub const fn muxs5ctrl(&self) -> &MUXS5CTRL {
        &self.muxs5ctrl
    }
    #[doc = "0x154 - Stream 6 Configuration Register"]
    #[inline(always)]
    pub const fn muxs6ctrl(&self) -> &MUXS6CTRL {
        &self.muxs6ctrl
    }
    #[doc = "0x158 - Stream 7 Configuration Register"]
    #[inline(always)]
    pub const fn muxs7ctrl(&self) -> &MUXS7CTRL {
        &self.muxs7ctrl
    }
    #[doc = "0x15c - Stream 8 Configuration Register"]
    #[inline(always)]
    pub const fn muxs8ctrl(&self) -> &MUXS8CTRL {
        &self.muxs8ctrl
    }
    #[doc = "0x160 - Generator 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxg1ctrl(&self) -> &MUXG1CTRL {
        &self.muxg1ctrl
    }
    #[doc = "0x164 - Generator 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxg2ctrl(&self) -> &MUXG2CTRL {
        &self.muxg2ctrl
    }
    #[doc = "0x168 - Generator 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxg3ctrl(&self) -> &MUXG3CTRL {
        &self.muxg3ctrl
    }
    #[doc = "0x16c - Generator 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxg4ctrl(&self) -> &MUXG4CTRL {
        &self.muxg4ctrl
    }
    #[doc = "0x170 - Channel Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxsyncsts(&self) -> &MUXSYNCSTS {
        &self.muxsyncsts
    }
    #[doc = "0x174 - Channel Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxsyncclr(&self) -> &MUXSYNCCLR {
        &self.muxsyncclr
    }
    #[doc = "0x178 - Generator Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxgsts(&self) -> &MUXGSTS {
        &self.muxgsts
    }
    #[doc = "0x17c - Generator Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxgclr(&self) -> &MUXGCLR {
        &self.muxgclr
    }
}
#[doc = "STS1 (r) register accessor: Interrupt status register1\n\nYou can [`read`](crate::Reg::read) this register and get [`sts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts1`] module"]
pub type STS1 = crate::Reg<sts1::STS1_SPEC>;
#[doc = "Interrupt status register1"]
pub mod sts1;
#[doc = "STS2 (r) register accessor: Interrupt status register2\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`] module"]
pub type STS2 = crate::Reg<sts2::STS2_SPEC>;
#[doc = "Interrupt status register2"]
pub mod sts2;
#[doc = "CLR1 (rw) register accessor: Interrupt flag clear register1\n\nYou can [`read`](crate::Reg::read) this register and get [`clr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr1`] module"]
pub type CLR1 = crate::Reg<clr1::CLR1_SPEC>;
#[doc = "Interrupt flag clear register1"]
pub mod clr1;
#[doc = "CLR2 (rw) register accessor: Interrupt flag clear register2\n\nYou can [`read`](crate::Reg::read) this register and get [`clr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr2`] module"]
pub type CLR2 = crate::Reg<clr2::CLR2_SPEC>;
#[doc = "Interrupt flag clear register2"]
pub mod clr2;
#[doc = "S1CTRL (rw) register accessor: stream 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1ctrl`] module"]
pub type S1CTRL = crate::Reg<s1ctrl::S1CTRL_SPEC>;
#[doc = "stream 1 control register"]
pub mod s1ctrl;
#[doc = "S1DTCNT (rw) register accessor: stream 1 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1dtcnt`] module"]
pub type S1DTCNT = crate::Reg<s1dtcnt::S1DTCNT_SPEC>;
#[doc = "stream 1 number of data register"]
pub mod s1dtcnt;
#[doc = "S1PADDR (rw) register accessor: stream 1 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1paddr`] module"]
pub type S1PADDR = crate::Reg<s1paddr::S1PADDR_SPEC>;
#[doc = "stream 1 peripheral address register"]
pub mod s1paddr;
#[doc = "S1M0ADDR (rw) register accessor: stream 1 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1m0addr`] module"]
pub type S1M0ADDR = crate::Reg<s1m0addr::S1M0ADDR_SPEC>;
#[doc = "stream 1 memory 0 address register"]
pub mod s1m0addr;
#[doc = "S1M1ADDR (rw) register accessor: stream 1 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1m1addr`] module"]
pub type S1M1ADDR = crate::Reg<s1m1addr::S1M1ADDR_SPEC>;
#[doc = "stream 1 memory 1 address register"]
pub mod s1m1addr;
#[doc = "S1FCTRL (rw) register accessor: stream 1 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1fctrl`] module"]
pub type S1FCTRL = crate::Reg<s1fctrl::S1FCTRL_SPEC>;
#[doc = "stream 1 FIFO control register"]
pub mod s1fctrl;
#[doc = "S2CTRL (rw) register accessor: stream 2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2ctrl`] module"]
pub type S2CTRL = crate::Reg<s2ctrl::S2CTRL_SPEC>;
#[doc = "stream 2 control register"]
pub mod s2ctrl;
#[doc = "S2DTCNT (rw) register accessor: stream 2 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2dtcnt`] module"]
pub type S2DTCNT = crate::Reg<s2dtcnt::S2DTCNT_SPEC>;
#[doc = "stream 2 number of data register"]
pub mod s2dtcnt;
#[doc = "S2PADDR (rw) register accessor: stream 2 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2paddr`] module"]
pub type S2PADDR = crate::Reg<s2paddr::S2PADDR_SPEC>;
#[doc = "stream 2 peripheral address register"]
pub mod s2paddr;
#[doc = "S2M0ADDR (rw) register accessor: stream 2 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2m0addr`] module"]
pub type S2M0ADDR = crate::Reg<s2m0addr::S2M0ADDR_SPEC>;
#[doc = "stream 2 memory 0 address register"]
pub mod s2m0addr;
#[doc = "S2M1ADDR (rw) register accessor: stream 2 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2m1addr`] module"]
pub type S2M1ADDR = crate::Reg<s2m1addr::S2M1ADDR_SPEC>;
#[doc = "stream 2 memory 1 address register"]
pub mod s2m1addr;
#[doc = "S2FCTRL (rw) register accessor: stream 2 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2fctrl`] module"]
pub type S2FCTRL = crate::Reg<s2fctrl::S2FCTRL_SPEC>;
#[doc = "stream 2 FIFO control register"]
pub mod s2fctrl;
#[doc = "S3CTRL (rw) register accessor: stream 3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3ctrl`] module"]
pub type S3CTRL = crate::Reg<s3ctrl::S3CTRL_SPEC>;
#[doc = "stream 3 control register"]
pub mod s3ctrl;
#[doc = "S3DTCNT (rw) register accessor: stream 3 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3dtcnt`] module"]
pub type S3DTCNT = crate::Reg<s3dtcnt::S3DTCNT_SPEC>;
#[doc = "stream 3 number of data register"]
pub mod s3dtcnt;
#[doc = "S3PADDR (rw) register accessor: stream 3 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3paddr`] module"]
pub type S3PADDR = crate::Reg<s3paddr::S3PADDR_SPEC>;
#[doc = "stream 3 peripheral address register"]
pub mod s3paddr;
#[doc = "S3M0ADDR (rw) register accessor: stream 3 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3m0addr`] module"]
pub type S3M0ADDR = crate::Reg<s3m0addr::S3M0ADDR_SPEC>;
#[doc = "stream 3 memory 0 address register"]
pub mod s3m0addr;
#[doc = "S3M1ADDR (rw) register accessor: stream 3 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3m1addr`] module"]
pub type S3M1ADDR = crate::Reg<s3m1addr::S3M1ADDR_SPEC>;
#[doc = "stream 3 memory 1 address register"]
pub mod s3m1addr;
#[doc = "S3FCTRL (rw) register accessor: stream 3 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3fctrl`] module"]
pub type S3FCTRL = crate::Reg<s3fctrl::S3FCTRL_SPEC>;
#[doc = "stream 3 FIFO control register"]
pub mod s3fctrl;
#[doc = "S4CTRL (rw) register accessor: stream 4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4ctrl`] module"]
pub type S4CTRL = crate::Reg<s4ctrl::S4CTRL_SPEC>;
#[doc = "stream 4 control register"]
pub mod s4ctrl;
#[doc = "S4DTCNT (rw) register accessor: stream 4 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4dtcnt`] module"]
pub type S4DTCNT = crate::Reg<s4dtcnt::S4DTCNT_SPEC>;
#[doc = "stream 4 number of data register"]
pub mod s4dtcnt;
#[doc = "S4PADDR (rw) register accessor: stream 4 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4paddr`] module"]
pub type S4PADDR = crate::Reg<s4paddr::S4PADDR_SPEC>;
#[doc = "stream 4 peripheral address register"]
pub mod s4paddr;
#[doc = "S4M0ADDR (rw) register accessor: stream 4 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4m0addr`] module"]
pub type S4M0ADDR = crate::Reg<s4m0addr::S4M0ADDR_SPEC>;
#[doc = "stream 4 memory 0 address register"]
pub mod s4m0addr;
#[doc = "S4M1ADDR (rw) register accessor: stream 4 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4m1addr`] module"]
pub type S4M1ADDR = crate::Reg<s4m1addr::S4M1ADDR_SPEC>;
#[doc = "stream 4 memory 1 address register"]
pub mod s4m1addr;
#[doc = "S4FCTRL (rw) register accessor: stream 4 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4fctrl`] module"]
pub type S4FCTRL = crate::Reg<s4fctrl::S4FCTRL_SPEC>;
#[doc = "stream 4 FIFO control register"]
pub mod s4fctrl;
#[doc = "S5CTRL (rw) register accessor: stream 5 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5ctrl`] module"]
pub type S5CTRL = crate::Reg<s5ctrl::S5CTRL_SPEC>;
#[doc = "stream 5 control register"]
pub mod s5ctrl;
#[doc = "S5DTCNT (rw) register accessor: stream 5 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5dtcnt`] module"]
pub type S5DTCNT = crate::Reg<s5dtcnt::S5DTCNT_SPEC>;
#[doc = "stream 5 number of data register"]
pub mod s5dtcnt;
#[doc = "S5PADDR (rw) register accessor: stream 5 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5paddr`] module"]
pub type S5PADDR = crate::Reg<s5paddr::S5PADDR_SPEC>;
#[doc = "stream 5 peripheral address register"]
pub mod s5paddr;
#[doc = "S5M0ADDR (rw) register accessor: stream 5 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5m0addr`] module"]
pub type S5M0ADDR = crate::Reg<s5m0addr::S5M0ADDR_SPEC>;
#[doc = "stream 5 memory 0 address register"]
pub mod s5m0addr;
#[doc = "S5M1ADDR (rw) register accessor: stream 5 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5m1addr`] module"]
pub type S5M1ADDR = crate::Reg<s5m1addr::S5M1ADDR_SPEC>;
#[doc = "stream 5 memory 1 address register"]
pub mod s5m1addr;
#[doc = "S5FCTRL (rw) register accessor: stream 5 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5fctrl`] module"]
pub type S5FCTRL = crate::Reg<s5fctrl::S5FCTRL_SPEC>;
#[doc = "stream 5 FIFO control register"]
pub mod s5fctrl;
#[doc = "S6CTRL (rw) register accessor: stream 6 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s6ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6ctrl`] module"]
pub type S6CTRL = crate::Reg<s6ctrl::S6CTRL_SPEC>;
#[doc = "stream 6 control register"]
pub mod s6ctrl;
#[doc = "S6DTCNT (rw) register accessor: stream 6 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s6dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6dtcnt`] module"]
pub type S6DTCNT = crate::Reg<s6dtcnt::S6DTCNT_SPEC>;
#[doc = "stream 6 number of data register"]
pub mod s6dtcnt;
#[doc = "S6PADDR (rw) register accessor: stream 6 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s6paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6paddr`] module"]
pub type S6PADDR = crate::Reg<s6paddr::S6PADDR_SPEC>;
#[doc = "stream 6 peripheral address register"]
pub mod s6paddr;
#[doc = "S6M0ADDR (rw) register accessor: stream 6 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s6m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6m0addr`] module"]
pub type S6M0ADDR = crate::Reg<s6m0addr::S6M0ADDR_SPEC>;
#[doc = "stream 6 memory 0 address register"]
pub mod s6m0addr;
#[doc = "S6M1ADDR (rw) register accessor: stream 6 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s6m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6m1addr`] module"]
pub type S6M1ADDR = crate::Reg<s6m1addr::S6M1ADDR_SPEC>;
#[doc = "stream 6 memory 1 address register"]
pub mod s6m1addr;
#[doc = "S6FCTRL (rw) register accessor: stream 6 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s6fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6fctrl`] module"]
pub type S6FCTRL = crate::Reg<s6fctrl::S6FCTRL_SPEC>;
#[doc = "stream 6 FIFO control register"]
pub mod s6fctrl;
#[doc = "S7CTRL (rw) register accessor: stream 7 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7ctrl`] module"]
pub type S7CTRL = crate::Reg<s7ctrl::S7CTRL_SPEC>;
#[doc = "stream 7 control register"]
pub mod s7ctrl;
#[doc = "S7DTCNT (rw) register accessor: stream 7 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7dtcnt`] module"]
pub type S7DTCNT = crate::Reg<s7dtcnt::S7DTCNT_SPEC>;
#[doc = "stream 7 number of data register"]
pub mod s7dtcnt;
#[doc = "S7PADDR (rw) register accessor: stream 7 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7paddr`] module"]
pub type S7PADDR = crate::Reg<s7paddr::S7PADDR_SPEC>;
#[doc = "stream 7 peripheral address register"]
pub mod s7paddr;
#[doc = "S7M0ADDR (rw) register accessor: stream 7 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7m0addr`] module"]
pub type S7M0ADDR = crate::Reg<s7m0addr::S7M0ADDR_SPEC>;
#[doc = "stream 7 memory 0 address register"]
pub mod s7m0addr;
#[doc = "S7M1ADDR (rw) register accessor: stream 7 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7m1addr`] module"]
pub type S7M1ADDR = crate::Reg<s7m1addr::S7M1ADDR_SPEC>;
#[doc = "stream 7 memory 1 address register"]
pub mod s7m1addr;
#[doc = "S7FCTRL (rw) register accessor: stream 7 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7fctrl`] module"]
pub type S7FCTRL = crate::Reg<s7fctrl::S7FCTRL_SPEC>;
#[doc = "stream 7 FIFO control register"]
pub mod s7fctrl;
#[doc = "S8CTRL (rw) register accessor: stream 8 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s8ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8ctrl`] module"]
pub type S8CTRL = crate::Reg<s8ctrl::S8CTRL_SPEC>;
#[doc = "stream 8 control register"]
pub mod s8ctrl;
#[doc = "S8DTCNT (rw) register accessor: stream 8 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s8dtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8dtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8dtcnt`] module"]
pub type S8DTCNT = crate::Reg<s8dtcnt::S8DTCNT_SPEC>;
#[doc = "stream 8 number of data register"]
pub mod s8dtcnt;
#[doc = "S8PADDR (rw) register accessor: stream 8 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s8paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8paddr`] module"]
pub type S8PADDR = crate::Reg<s8paddr::S8PADDR_SPEC>;
#[doc = "stream 8 peripheral address register"]
pub mod s8paddr;
#[doc = "S8M0ADDR (rw) register accessor: stream 8 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s8m0addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8m0addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8m0addr`] module"]
pub type S8M0ADDR = crate::Reg<s8m0addr::S8M0ADDR_SPEC>;
#[doc = "stream 8 memory 0 address register"]
pub mod s8m0addr;
#[doc = "S8M1ADDR (rw) register accessor: stream 8 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s8m1addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8m1addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8m1addr`] module"]
pub type S8M1ADDR = crate::Reg<s8m1addr::S8M1ADDR_SPEC>;
#[doc = "stream 8 memory 1 address register"]
pub mod s8m1addr;
#[doc = "S8FCTRL (rw) register accessor: stream 8 FIFO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`s8fctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8fctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8fctrl`] module"]
pub type S8FCTRL = crate::Reg<s8fctrl::S8FCTRL_SPEC>;
#[doc = "stream 8 FIFO control register"]
pub mod s8fctrl;
#[doc = "LLCTRL (rw) register accessor: DMA Link List Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`llctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llctrl`] module"]
pub type LLCTRL = crate::Reg<llctrl::LLCTRL_SPEC>;
#[doc = "DMA Link List Control Register"]
pub mod llctrl;
#[doc = "S1LLP (rw) register accessor: Stream 1 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s1llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1llp`] module"]
pub type S1LLP = crate::Reg<s1llp::S1LLP_SPEC>;
#[doc = "Stream 1 Link List Pointer"]
pub mod s1llp;
#[doc = "S2LLP (rw) register accessor: Stream 2 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s2llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2llp`] module"]
pub type S2LLP = crate::Reg<s2llp::S2LLP_SPEC>;
#[doc = "Stream 2 Link List Pointer"]
pub mod s2llp;
#[doc = "S3LLP (rw) register accessor: Stream 3 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s3llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3llp`] module"]
pub type S3LLP = crate::Reg<s3llp::S3LLP_SPEC>;
#[doc = "Stream 3 Link List Pointer"]
pub mod s3llp;
#[doc = "S4LLP (rw) register accessor: Stream 4 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s4llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4llp`] module"]
pub type S4LLP = crate::Reg<s4llp::S4LLP_SPEC>;
#[doc = "Stream 4 Link List Pointer"]
pub mod s4llp;
#[doc = "S5LLP (rw) register accessor: Stream 5 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s5llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5llp`] module"]
pub type S5LLP = crate::Reg<s5llp::S5LLP_SPEC>;
#[doc = "Stream 5 Link List Pointer"]
pub mod s5llp;
#[doc = "S6LLP (rw) register accessor: Stream 6 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s6llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6llp`] module"]
pub type S6LLP = crate::Reg<s6llp::S6LLP_SPEC>;
#[doc = "Stream 6 Link List Pointer"]
pub mod s6llp;
#[doc = "S7LLP (rw) register accessor: Stream 7 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s7llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7llp`] module"]
pub type S7LLP = crate::Reg<s7llp::S7LLP_SPEC>;
#[doc = "Stream 7 Link List Pointer"]
pub mod s7llp;
#[doc = "S8LLP (rw) register accessor: Stream 8 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s8llp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8llp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8llp`] module"]
pub type S8LLP = crate::Reg<s8llp::S8LLP_SPEC>;
#[doc = "Stream 8 Link List Pointer"]
pub mod s8llp;
#[doc = "S2DCTRL (rw) register accessor: EDMA 2D Transfer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2dctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2dctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2dctrl`] module"]
pub type S2DCTRL = crate::Reg<s2dctrl::S2DCTRL_SPEC>;
#[doc = "EDMA 2D Transfer Control Register"]
pub mod s2dctrl;
#[doc = "S1_2DCNT (rw) register accessor: Stream 1 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s1_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1_2dcnt`] module"]
pub type S1_2DCNT = crate::Reg<s1_2dcnt::S1_2DCNT_SPEC>;
#[doc = "Stream 1 2D Transfer Count"]
pub mod s1_2dcnt;
#[doc = "S1_STRIDE (rw) register accessor: Stream 1 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s1_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1_stride`] module"]
pub type S1_STRIDE = crate::Reg<s1_stride::S1_STRIDE_SPEC>;
#[doc = "Stream 1 2D Transfer Stride"]
pub mod s1_stride;
#[doc = "S2_2DCNT (rw) register accessor: Stream 2 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s2_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2_2dcnt`] module"]
pub type S2_2DCNT = crate::Reg<s2_2dcnt::S2_2DCNT_SPEC>;
#[doc = "Stream 2 2D Transfer Count"]
pub mod s2_2dcnt;
#[doc = "S2_STRIDE (rw) register accessor: Stream 2 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s2_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2_stride`] module"]
pub type S2_STRIDE = crate::Reg<s2_stride::S2_STRIDE_SPEC>;
#[doc = "Stream 2 2D Transfer Stride"]
pub mod s2_stride;
#[doc = "S3_2DCNT (rw) register accessor: Stream 3 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s3_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3_2dcnt`] module"]
pub type S3_2DCNT = crate::Reg<s3_2dcnt::S3_2DCNT_SPEC>;
#[doc = "Stream 3 2D Transfer Count"]
pub mod s3_2dcnt;
#[doc = "S3_STRIDE (rw) register accessor: Stream 3 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s3_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3_stride`] module"]
pub type S3_STRIDE = crate::Reg<s3_stride::S3_STRIDE_SPEC>;
#[doc = "Stream 3 2D Transfer Stride"]
pub mod s3_stride;
#[doc = "S4_2DCNT (rw) register accessor: Stream 4 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s4_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4_2dcnt`] module"]
pub type S4_2DCNT = crate::Reg<s4_2dcnt::S4_2DCNT_SPEC>;
#[doc = "Stream 4 2D Transfer Count"]
pub mod s4_2dcnt;
#[doc = "S4_STRIDE (rw) register accessor: Stream 4 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s4_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4_stride`] module"]
pub type S4_STRIDE = crate::Reg<s4_stride::S4_STRIDE_SPEC>;
#[doc = "Stream 4 2D Transfer Stride"]
pub mod s4_stride;
#[doc = "S5_2DCNT (rw) register accessor: Stream 5 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s5_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5_2dcnt`] module"]
pub type S5_2DCNT = crate::Reg<s5_2dcnt::S5_2DCNT_SPEC>;
#[doc = "Stream 5 2D Transfer Count"]
pub mod s5_2dcnt;
#[doc = "S5_STRIDE (rw) register accessor: Stream 5 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s5_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5_stride`] module"]
pub type S5_STRIDE = crate::Reg<s5_stride::S5_STRIDE_SPEC>;
#[doc = "Stream 5 2D Transfer Stride"]
pub mod s5_stride;
#[doc = "S6_2DCNT (rw) register accessor: Stream 6 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s6_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6_2dcnt`] module"]
pub type S6_2DCNT = crate::Reg<s6_2dcnt::S6_2DCNT_SPEC>;
#[doc = "Stream 6 2D Transfer Count"]
pub mod s6_2dcnt;
#[doc = "S6_STRIDE (rw) register accessor: Stream 6 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s6_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6_stride`] module"]
pub type S6_STRIDE = crate::Reg<s6_stride::S6_STRIDE_SPEC>;
#[doc = "Stream 6 2D Transfer Stride"]
pub mod s6_stride;
#[doc = "S7_2DCNT (rw) register accessor: Stream 7 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s7_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7_2dcnt`] module"]
pub type S7_2DCNT = crate::Reg<s7_2dcnt::S7_2DCNT_SPEC>;
#[doc = "Stream 7 2D Transfer Count"]
pub mod s7_2dcnt;
#[doc = "S7_STRIDE (rw) register accessor: Stream 7 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s7_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7_stride`] module"]
pub type S7_STRIDE = crate::Reg<s7_stride::S7_STRIDE_SPEC>;
#[doc = "Stream 7 2D Transfer Stride"]
pub mod s7_stride;
#[doc = "S8_2DCNT (rw) register accessor: Stream 8 2D Transfer Count\n\nYou can [`read`](crate::Reg::read) this register and get [`s8_2dcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8_2dcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8_2dcnt`] module"]
pub type S8_2DCNT = crate::Reg<s8_2dcnt::S8_2DCNT_SPEC>;
#[doc = "Stream 8 2D Transfer Count"]
pub mod s8_2dcnt;
#[doc = "S8_STRIDE (rw) register accessor: Stream 8 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s8_stride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8_stride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8_stride`] module"]
pub type S8_STRIDE = crate::Reg<s8_stride::S8_STRIDE_SPEC>;
#[doc = "Stream 8 2D Transfer Stride"]
pub mod s8_stride;
#[doc = "SYNCEN (rw) register accessor: Sync Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`syncen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncen`] module"]
pub type SYNCEN = crate::Reg<syncen::SYNCEN_SPEC>;
#[doc = "Sync Enable"]
pub mod syncen;
#[doc = "MUXSEL (rw) register accessor: EDMA MUX Table Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsel`] module"]
pub type MUXSEL = crate::Reg<muxsel::MUXSEL_SPEC>;
#[doc = "EDMA MUX Table Selection"]
pub mod muxsel;
#[doc = "MUXS1CTRL (rw) register accessor: Stream 1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs1ctrl`] module"]
pub type MUXS1CTRL = crate::Reg<muxs1ctrl::MUXS1CTRL_SPEC>;
#[doc = "Stream 1 Configuration Register"]
pub mod muxs1ctrl;
#[doc = "MUXS2CTRL (rw) register accessor: Stream 2 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs2ctrl`] module"]
pub type MUXS2CTRL = crate::Reg<muxs2ctrl::MUXS2CTRL_SPEC>;
#[doc = "Stream 2 Configuration Register"]
pub mod muxs2ctrl;
#[doc = "MUXS3CTRL (rw) register accessor: Stream 3 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs3ctrl`] module"]
pub type MUXS3CTRL = crate::Reg<muxs3ctrl::MUXS3CTRL_SPEC>;
#[doc = "Stream 3 Configuration Register"]
pub mod muxs3ctrl;
#[doc = "MUXS4CTRL (rw) register accessor: Stream 4 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs4ctrl`] module"]
pub type MUXS4CTRL = crate::Reg<muxs4ctrl::MUXS4CTRL_SPEC>;
#[doc = "Stream 4 Configuration Register"]
pub mod muxs4ctrl;
#[doc = "MUXS5CTRL (rw) register accessor: Stream x Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs5ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs5ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs5ctrl`] module"]
pub type MUXS5CTRL = crate::Reg<muxs5ctrl::MUXS5CTRL_SPEC>;
#[doc = "Stream x Configuration Register"]
pub mod muxs5ctrl;
#[doc = "MUXS6CTRL (rw) register accessor: Stream 6 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs6ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs6ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs6ctrl`] module"]
pub type MUXS6CTRL = crate::Reg<muxs6ctrl::MUXS6CTRL_SPEC>;
#[doc = "Stream 6 Configuration Register"]
pub mod muxs6ctrl;
#[doc = "MUXS7CTRL (rw) register accessor: Stream 7 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs7ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs7ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs7ctrl`] module"]
pub type MUXS7CTRL = crate::Reg<muxs7ctrl::MUXS7CTRL_SPEC>;
#[doc = "Stream 7 Configuration Register"]
pub mod muxs7ctrl;
#[doc = "MUXS8CTRL (rw) register accessor: Stream 8 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxs8ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxs8ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs8ctrl`] module"]
pub type MUXS8CTRL = crate::Reg<muxs8ctrl::MUXS8CTRL_SPEC>;
#[doc = "Stream 8 Configuration Register"]
pub mod muxs8ctrl;
#[doc = "MUXG1CTRL (rw) register accessor: Generator 1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxg1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxg1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg1ctrl`] module"]
pub type MUXG1CTRL = crate::Reg<muxg1ctrl::MUXG1CTRL_SPEC>;
#[doc = "Generator 1 Configuration Register"]
pub mod muxg1ctrl;
#[doc = "MUXG2CTRL (rw) register accessor: Generator 2 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxg2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxg2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg2ctrl`] module"]
pub type MUXG2CTRL = crate::Reg<muxg2ctrl::MUXG2CTRL_SPEC>;
#[doc = "Generator 2 Configuration Register"]
pub mod muxg2ctrl;
#[doc = "MUXG3CTRL (rw) register accessor: Generator 3 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxg3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxg3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg3ctrl`] module"]
pub type MUXG3CTRL = crate::Reg<muxg3ctrl::MUXG3CTRL_SPEC>;
#[doc = "Generator 3 Configuration Register"]
pub mod muxg3ctrl;
#[doc = "MUXG4CTRL (rw) register accessor: Generator 4 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxg4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxg4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg4ctrl`] module"]
pub type MUXG4CTRL = crate::Reg<muxg4ctrl::MUXG4CTRL_SPEC>;
#[doc = "Generator 4 Configuration Register"]
pub mod muxg4ctrl;
#[doc = "MUXSYNCSTS (rw) register accessor: Channel Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsyncsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsyncsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsyncsts`] module"]
pub type MUXSYNCSTS = crate::Reg<muxsyncsts::MUXSYNCSTS_SPEC>;
#[doc = "Channel Interrupt Status Register"]
pub mod muxsyncsts;
#[doc = "MUXSYNCCLR (rw) register accessor: Channel Interrupt Clear Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxsyncclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxsyncclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsyncclr`] module"]
pub type MUXSYNCCLR = crate::Reg<muxsyncclr::MUXSYNCCLR_SPEC>;
#[doc = "Channel Interrupt Clear Flag Register"]
pub mod muxsyncclr;
#[doc = "MUXGSTS (rw) register accessor: Generator Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxgsts`] module"]
pub type MUXGSTS = crate::Reg<muxgsts::MUXGSTS_SPEC>;
#[doc = "Generator Interrupt Status Register"]
pub mod muxgsts;
#[doc = "MUXGCLR (rw) register accessor: Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxgclr`] module"]
pub type MUXGCLR = crate::Reg<muxgclr::MUXGCLR_SPEC>;
#[doc = "Generator Interrupt Clear Flag Register"]
pub mod muxgclr;
