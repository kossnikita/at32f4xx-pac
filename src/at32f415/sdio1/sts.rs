#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `CMDFAIL` reader - CCRCFAIL"]
pub type CMDFAIL_R = crate::BitReader;
#[doc = "Field `DTFAIL` reader - DCRCFAIL"]
pub type DTFAIL_R = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` reader - CTIMEOUT"]
pub type CMDTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTTIMEOUT` reader - DTIMEOUT"]
pub type DTTIMEOUT_R = crate::BitReader;
#[doc = "Field `TXERRU` reader - TXUNDERR"]
pub type TXERRU_R = crate::BitReader;
#[doc = "Field `RXERRO` reader - RXOVERR"]
pub type RXERRO_R = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` reader - CMDREND"]
pub type CMDRSPCMPL_R = crate::BitReader;
#[doc = "Field `CMDCMPL` reader - CMDSENT"]
pub type CMDCMPL_R = crate::BitReader;
#[doc = "Field `DTCMPL` reader - DATAEND"]
pub type DTCMPL_R = crate::BitReader;
#[doc = "Field `SBITERR` reader - STBITERR"]
pub type SBITERR_R = crate::BitReader;
#[doc = "Field `DTBLKCMPL` reader - DBCKEND"]
pub type DTBLKCMPL_R = crate::BitReader;
#[doc = "Field `DOCMD` reader - CMDACT"]
pub type DOCMD_R = crate::BitReader;
#[doc = "Field `DOTX` reader - TXACT"]
pub type DOTX_R = crate::BitReader;
#[doc = "Field `DORX` reader - RXACT"]
pub type DORX_R = crate::BitReader;
#[doc = "Field `TXBUF_H` reader - TXFIFOHE"]
pub type TXBUF_H_R = crate::BitReader;
#[doc = "Field `RXBUF_H` reader - RXFIFOHF"]
pub type RXBUF_H_R = crate::BitReader;
#[doc = "Field `TXBUF_F` reader - TXFIFOF"]
pub type TXBUF_F_R = crate::BitReader;
#[doc = "Field `RXBUF_F` reader - RXFIFOF"]
pub type RXBUF_F_R = crate::BitReader;
#[doc = "Field `TXBUF_E` reader - TXFIFOE"]
pub type TXBUF_E_R = crate::BitReader;
#[doc = "Field `RXBUF_E` reader - RXFIFOE"]
pub type RXBUF_E_R = crate::BitReader;
#[doc = "Field `TXBUF` reader - TXDAVL"]
pub type TXBUF_R = crate::BitReader;
#[doc = "Field `RXBUF` reader - RXDAVL"]
pub type RXBUF_R = crate::BitReader;
#[doc = "Field `SDIOIF` reader - SDIOIT"]
pub type SDIOIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CCRCFAIL"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CMDFAIL_R {
        CMDFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL"]
    #[inline(always)]
    pub fn dtfail(&self) -> DTFAIL_R {
        DTFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CMDTIMEOUT_R {
        CMDTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DTTIMEOUT_R {
        DTTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR"]
    #[inline(always)]
    pub fn txerru(&self) -> TXERRU_R {
        TXERRU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERR"]
    #[inline(always)]
    pub fn rxerro(&self) -> RXERRO_R {
        RXERRO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDREND"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CMDRSPCMPL_R {
        CMDRSPCMPL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENT"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CMDCMPL_R {
        CMDCMPL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAEND"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DTCMPL_R {
        DTCMPL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERR"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SBITERR_R {
        SBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKEND"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DTBLKCMPL_R {
        DTBLKCMPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMDACT"]
    #[inline(always)]
    pub fn docmd(&self) -> DOCMD_R {
        DOCMD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACT"]
    #[inline(always)]
    pub fn dotx(&self) -> DOTX_R {
        DOTX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXACT"]
    #[inline(always)]
    pub fn dorx(&self) -> DORX_R {
        DORX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHE"]
    #[inline(always)]
    pub fn txbuf_h(&self) -> TXBUF_H_R {
        TXBUF_H_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHF"]
    #[inline(always)]
    pub fn rxbuf_h(&self) -> RXBUF_H_R {
        RXBUF_H_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIFOF"]
    #[inline(always)]
    pub fn txbuf_f(&self) -> TXBUF_F_R {
        TXBUF_F_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOF"]
    #[inline(always)]
    pub fn rxbuf_f(&self) -> RXBUF_F_R {
        RXBUF_F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOE"]
    #[inline(always)]
    pub fn txbuf_e(&self) -> TXBUF_E_R {
        TXBUF_E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RXFIFOE"]
    #[inline(always)]
    pub fn rxbuf_e(&self) -> RXBUF_E_R {
        RXBUF_E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXDAVL"]
    #[inline(always)]
    pub fn txbuf(&self) -> TXBUF_R {
        TXBUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXDAVL"]
    #[inline(always)]
    pub fn rxbuf(&self) -> RXBUF_R {
        RXBUF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOIT"]
    #[inline(always)]
    pub fn sdioif(&self) -> SDIOIF_R {
        SDIOIF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("cmdfail", &format_args!("{}", self.cmdfail().bit()))
            .field("dtfail", &format_args!("{}", self.dtfail().bit()))
            .field("cmdtimeout", &format_args!("{}", self.cmdtimeout().bit()))
            .field("dttimeout", &format_args!("{}", self.dttimeout().bit()))
            .field("txerru", &format_args!("{}", self.txerru().bit()))
            .field("rxerro", &format_args!("{}", self.rxerro().bit()))
            .field("cmdrspcmpl", &format_args!("{}", self.cmdrspcmpl().bit()))
            .field("cmdcmpl", &format_args!("{}", self.cmdcmpl().bit()))
            .field("dtcmpl", &format_args!("{}", self.dtcmpl().bit()))
            .field("sbiterr", &format_args!("{}", self.sbiterr().bit()))
            .field("dtblkcmpl", &format_args!("{}", self.dtblkcmpl().bit()))
            .field("docmd", &format_args!("{}", self.docmd().bit()))
            .field("dotx", &format_args!("{}", self.dotx().bit()))
            .field("dorx", &format_args!("{}", self.dorx().bit()))
            .field("txbuf_h", &format_args!("{}", self.txbuf_h().bit()))
            .field("rxbuf_h", &format_args!("{}", self.rxbuf_h().bit()))
            .field("txbuf_f", &format_args!("{}", self.txbuf_f().bit()))
            .field("rxbuf_f", &format_args!("{}", self.rxbuf_f().bit()))
            .field("txbuf_e", &format_args!("{}", self.txbuf_e().bit()))
            .field("rxbuf_e", &format_args!("{}", self.rxbuf_e().bit()))
            .field("txbuf", &format_args!("{}", self.txbuf().bit()))
            .field("rxbuf", &format_args!("{}", self.rxbuf().bit()))
            .field("sdioif", &format_args!("{}", self.sdioif().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SDIO status register (SDIO_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
