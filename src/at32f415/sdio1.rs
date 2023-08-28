#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    pub power: POWER,
    #[doc = "0x04 - SDI clock control register (SDIO_CLKCTRL)"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    pub arg: ARG,
    #[doc = "0x0c - SDIO command register (SDIO_CMD)"]
    pub cmd: CMD,
    #[doc = "0x10 - SDIO command register"]
    pub rspcmd: RSPCMD,
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    pub rsp1: RSP1,
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    pub rsp2: RSP2,
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS3"]
    pub rsp3: RSP3,
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS4"]
    pub rsp4: RSP4,
    #[doc = "0x24 - Bits 31:0 = DATATIME: Data timeout period"]
    pub dttmr: DTTMR,
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    pub dtlen: DTLEN,
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    pub dtctrl: DTCTRL,
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    pub dtcntr: DTCNTR,
    #[doc = "0x34 - SDIO status register (SDIO_STS)"]
    pub sts: STS,
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_INTCLR)"]
    pub intclr: INTCLR,
    #[doc = "0x3c - SDIO interrupt enable register (SDIO_INTEN)"]
    pub inten: INTEN,
    _reserved16: [u8; 0x08],
    #[doc = "0x48 - Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
    pub bufcntr: BUFCNTR,
    _reserved17: [u8; 0x34],
    #[doc = "0x80 - bits 31:0 = FIFOData: Receive and transmit FIFO data"]
    pub buf: BUF,
}
#[doc = "POWER (rw) register accessor: Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`power`]
module"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod power;
#[doc = "CLKCTRL (rw) register accessor: SDI clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkctrl`]
module"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "SDI clock control register (SDIO_CLKCTRL)"]
pub mod clkctrl;
#[doc = "ARG (rw) register accessor: Bits 31:0 = : Command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`arg`]
module"]
pub type ARG = crate::Reg<arg::ARG_SPEC>;
#[doc = "Bits 31:0 = : Command argument"]
pub mod arg;
#[doc = "CMD (rw) register accessor: SDIO command register (SDIO_CMD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SDIO command register (SDIO_CMD)"]
pub mod cmd;
#[doc = "RSPCMD (r) register accessor: SDIO command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspcmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rspcmd`]
module"]
pub type RSPCMD = crate::Reg<rspcmd::RSPCMD_SPEC>;
#[doc = "SDIO command register"]
pub mod rspcmd;
#[doc = "RSP1 (r) register accessor: Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsp1`]
module"]
pub type RSP1 = crate::Reg<rsp1::RSP1_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod rsp1;
#[doc = "RSP2 (r) register accessor: Bits 31:0 = CARDSTATUS2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsp2`]
module"]
pub type RSP2 = crate::Reg<rsp2::RSP2_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod rsp2;
#[doc = "RSP3 (r) register accessor: Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsp3`]
module"]
pub type RSP3 = crate::Reg<rsp3::RSP3_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub mod rsp3;
#[doc = "RSP4 (r) register accessor: Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rsp4`]
module"]
pub type RSP4 = crate::Reg<rsp4::RSP4_SPEC>;
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub mod rsp4;
#[doc = "DTTMR (rw) register accessor: Bits 31:0 = DATATIME: Data timeout period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dttmr`]
module"]
pub type DTTMR = crate::Reg<dttmr::DTTMR_SPEC>;
#[doc = "Bits 31:0 = DATATIME: Data timeout period"]
pub mod dttmr;
#[doc = "DTLEN (rw) register accessor: Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtlen`]
module"]
pub type DTLEN = crate::Reg<dtlen::DTLEN_SPEC>;
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dtlen;
#[doc = "DTCTRL (rw) register accessor: SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtctrl`]
module"]
pub type DTCTRL = crate::Reg<dtctrl::DTCTRL_SPEC>;
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dtctrl;
#[doc = "DTCNTR (r) register accessor: Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtcntr`]
module"]
pub type DTCNTR = crate::Reg<dtcntr::DTCNTR_SPEC>;
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dtcntr;
#[doc = "STS (r) register accessor: SDIO status register (SDIO_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "SDIO status register (SDIO_STS)"]
pub mod sts;
#[doc = "INTCLR (rw) register accessor: SDIO interrupt clear register (SDIO_INTCLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intclr`]
module"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "SDIO interrupt clear register (SDIO_INTCLR)"]
pub mod intclr;
#[doc = "INTEN (rw) register accessor: SDIO interrupt enable register (SDIO_INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "SDIO interrupt enable register (SDIO_INTEN)"]
pub mod inten;
#[doc = "BUFCNTR (r) register accessor: Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufcntr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bufcntr`]
module"]
pub type BUFCNTR = crate::Reg<bufcntr::BUFCNTR_SPEC>;
#[doc = "Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod bufcntr;
#[doc = "BUF (rw) register accessor: bits 31:0 = FIFOData: Receive and transmit FIFO data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buf`]
module"]
pub type BUF = crate::Reg<buf::BUF_SPEC>;
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data"]
pub mod buf;