#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwrctrl: PWRCTRL,
    clkctrl: CLKCTRL,
    argu: ARGU,
    cmdctrl: CMDCTRL,
    rspcmd: RSPCMD,
    rsp1: RSP1,
    rsp2: RSP2,
    rsp3: RSP3,
    rsp4: RSP4,
    dttmr: DTTMR,
    dtlen: DTLEN,
    dtctrl: DTCTRL,
    dtcnt: DTCNT,
    sts: STS,
    intclr: INTCLR,
    inten: INTEN,
    _reserved16: [u8; 0x08],
    bufcnt: BUFCNT,
    _reserved17: [u8; 0x34],
    buf: BUF,
}
impl RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &PWRCTRL {
        &self.pwrctrl
    }
    #[doc = "0x04 - SD clock control register (SDIO_CLKCTRL)"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &CLKCTRL {
        &self.clkctrl
    }
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    #[inline(always)]
    pub const fn argu(&self) -> &ARGU {
        &self.argu
    }
    #[doc = "0x0c - SDIO command control register (SDIO_CMDCTRL)"]
    #[inline(always)]
    pub const fn cmdctrl(&self) -> &CMDCTRL {
        &self.cmdctrl
    }
    #[doc = "0x10 - SDIO command register"]
    #[inline(always)]
    pub const fn rspcmd(&self) -> &RSPCMD {
        &self.rspcmd
    }
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    #[inline(always)]
    pub const fn rsp1(&self) -> &RSP1 {
        &self.rsp1
    }
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    #[inline(always)]
    pub const fn rsp2(&self) -> &RSP2 {
        &self.rsp2
    }
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS3"]
    #[inline(always)]
    pub const fn rsp3(&self) -> &RSP3 {
        &self.rsp3
    }
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS4"]
    #[inline(always)]
    pub const fn rsp4(&self) -> &RSP4 {
        &self.rsp4
    }
    #[doc = "0x24 - Bits 31:0 = TIMEOUT: Data timeout period"]
    #[inline(always)]
    pub const fn dttmr(&self) -> &DTTMR {
        &self.dttmr
    }
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    #[inline(always)]
    pub const fn dtlen(&self) -> &DTLEN {
        &self.dtlen
    }
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    #[inline(always)]
    pub const fn dtctrl(&self) -> &DTCTRL {
        &self.dtctrl
    }
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    #[inline(always)]
    pub const fn dtcnt(&self) -> &DTCNT {
        &self.dtcnt
    }
    #[doc = "0x34 - SDIO status register (SDIO_STA)"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_INTCLR)"]
    #[inline(always)]
    pub const fn intclr(&self) -> &INTCLR {
        &self.intclr
    }
    #[doc = "0x3c - SDIO mask register (SDIO_MASK)"]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x48 - Bits 23:0 = BUFCOUNT: Remaining number of words to be written to or read from the FIFO"]
    #[inline(always)]
    pub const fn bufcnt(&self) -> &BUFCNT {
        &self.bufcnt
    }
    #[doc = "0x80 - bits 31:0 = Buffer Data: Receive and transmit buffer data"]
    #[inline(always)]
    pub const fn buf(&self) -> &BUF {
        &self.buf
    }
}
#[doc = "PWRCTRL (rw) register accessor: Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctrl`]
module"]
pub type PWRCTRL = crate::Reg<pwrctrl::PWRCTRL_SPEC>;
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod pwrctrl;
#[doc = "CLKCTRL (rw) register accessor: SD clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "SD clock control register (SDIO_CLKCTRL)"]
pub mod clkctrl;
#[doc = "ARGU (rw) register accessor: Bits 31:0 = : Command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argu`]
module"]
pub type ARGU = crate::Reg<argu::ARGU_SPEC>;
#[doc = "Bits 31:0 = : Command argument"]
pub mod argu;
#[doc = "CMDCTRL (rw) register accessor: SDIO command control register (SDIO_CMDCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdctrl`]
module"]
pub type CMDCTRL = crate::Reg<cmdctrl::CMDCTRL_SPEC>;
#[doc = "SDIO command control register (SDIO_CMDCTRL)"]
pub mod cmdctrl;
#[doc = "RSPCMD (r) register accessor: SDIO command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspcmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspcmd`]
module"]
pub type RSPCMD = crate::Reg<rspcmd::RSPCMD_SPEC>;
#[doc = "SDIO command register"]
pub mod rspcmd;
#[doc = "RSP1 (r) register accessor: Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp1`]
module"]
pub type RSP1 = crate::Reg<rsp1::RSP1_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod rsp1;
#[doc = "RSP2 (r) register accessor: Bits 31:0 = CARDSTATUS2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp2`]
module"]
pub type RSP2 = crate::Reg<rsp2::RSP2_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod rsp2;
#[doc = "RSP3 (r) register accessor: Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp3`]
module"]
pub type RSP3 = crate::Reg<rsp3::RSP3_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub mod rsp3;
#[doc = "RSP4 (r) register accessor: Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp4`]
module"]
pub type RSP4 = crate::Reg<rsp4::RSP4_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub mod rsp4;
#[doc = "DTTMR (rw) register accessor: Bits 31:0 = TIMEOUT: Data timeout period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dttmr`]
module"]
pub type DTTMR = crate::Reg<dttmr::DTTMR_SPEC>;
#[doc = "Bits 31:0 = TIMEOUT: Data timeout period"]
pub mod dttmr;
#[doc = "DTLEN (rw) register accessor: Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtlen`]
module"]
pub type DTLEN = crate::Reg<dtlen::DTLEN_SPEC>;
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dtlen;
#[doc = "DTCTRL (rw) register accessor: SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctrl`]
module"]
pub type DTCTRL = crate::Reg<dtctrl::DTCTRL_SPEC>;
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dtctrl;
#[doc = "DTCNT (r) register accessor: Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcnt`]
module"]
pub type DTCNT = crate::Reg<dtcnt::DTCNT_SPEC>;
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dtcnt;
#[doc = "STS (r) register accessor: SDIO status register (SDIO_STA)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "SDIO status register (SDIO_STA)"]
pub mod sts;
#[doc = "INTCLR (rw) register accessor: SDIO interrupt clear register (SDIO_INTCLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "SDIO interrupt clear register (SDIO_INTCLR)"]
pub mod intclr;
#[doc = "INTEN (rw) register accessor: SDIO mask register (SDIO_MASK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "SDIO mask register (SDIO_MASK)"]
pub mod inten;
#[doc = "BUFCNT (r) register accessor: Bits 23:0 = BUFCOUNT: Remaining number of words to be written to or read from the FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufcnt`]
module"]
pub type BUFCNT = crate::Reg<bufcnt::BUFCNT_SPEC>;
#[doc = "Bits 23:0 = BUFCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod bufcnt;
#[doc = "BUF (rw) register accessor: bits 31:0 = Buffer Data: Receive and transmit buffer data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf`]
module"]
pub type BUF = crate::Reg<buf::BUF_SPEC>;
#[doc = "bits 31:0 = Buffer Data: Receive and transmit buffer data"]
pub mod buf;
