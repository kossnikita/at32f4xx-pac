#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `CMDFAIL` reader - Command crc fail"]
pub type CMDFAIL_R = crate::BitReader;
#[doc = "Field `DTFAIL` reader - Data crc fail"]
pub type DTFAIL_R = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` reader - Command timeout"]
pub type CMDTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTTIMEOUT` reader - Data timeout"]
pub type DTTIMEOUT_R = crate::BitReader;
#[doc = "Field `TXERRU` reader - Tx under run error"]
pub type TXERRU_R = crate::BitReader;
#[doc = "Field `RXERRO` reader - Rx over run error"]
pub type RXERRO_R = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` reader - Command response complete"]
pub type CMDRSPCMPL_R = crate::BitReader;
#[doc = "Field `CMDCMPL` reader - Command sent"]
pub type CMDCMPL_R = crate::BitReader;
#[doc = "Field `DTCMPL` reader - Data sent"]
pub type DTCMPL_R = crate::BitReader;
#[doc = "Field `SBITERR` reader - Start bit error"]
pub type SBITERR_R = crate::BitReader;
#[doc = "Field `DTBLKCMPL` reader - Data block sent"]
pub type DTBLKCMPL_R = crate::BitReader;
#[doc = "Field `DOCMD` reader - Command transfer in progress"]
pub type DOCMD_R = crate::BitReader;
#[doc = "Field `DOTX` reader - Data transmit in progress"]
pub type DOTX_R = crate::BitReader;
#[doc = "Field `DORX` reader - Data receive in progress"]
pub type DORX_R = crate::BitReader;
#[doc = "Field `TXBUFH` reader - Tx buffer half empty"]
pub type TXBUFH_R = crate::BitReader;
#[doc = "Field `RXBUFH` reader - Rx buffer half empty"]
pub type RXBUFH_R = crate::BitReader;
#[doc = "Field `TXBUFF` reader - Tx buffer full"]
pub type TXBUFF_R = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Rx buffer full"]
pub type RXBUFF_R = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Tx buffer empty"]
pub type TXBUFE_R = crate::BitReader;
#[doc = "Field `RXBUFE` reader - Rx buffer empty"]
pub type RXBUFE_R = crate::BitReader;
#[doc = "Field `TXBUF` reader - Tx data vaild"]
pub type TXBUF_R = crate::BitReader;
#[doc = "Field `RXBUF` reader - Rx data vaild"]
pub type RXBUF_R = crate::BitReader;
#[doc = "Field `IOIF` reader - SD I/O interrupt"]
pub type IOIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command crc fail"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CMDFAIL_R {
        CMDFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data crc fail"]
    #[inline(always)]
    pub fn dtfail(&self) -> DTFAIL_R {
        DTFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CMDTIMEOUT_R {
        CMDTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DTTIMEOUT_R {
        DTTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx under run error"]
    #[inline(always)]
    pub fn txerru(&self) -> TXERRU_R {
        TXERRU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx over run error"]
    #[inline(always)]
    pub fn rxerro(&self) -> RXERRO_R {
        RXERRO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response complete"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CMDRSPCMPL_R {
        CMDRSPCMPL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CMDCMPL_R {
        CMDCMPL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data sent"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DTCMPL_R {
        DTCMPL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SBITERR_R {
        SBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DTBLKCMPL_R {
        DTBLKCMPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn docmd(&self) -> DOCMD_R {
        DOCMD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn dotx(&self) -> DOTX_R {
        DOTX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn dorx(&self) -> DORX_R {
        DORX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx buffer half empty"]
    #[inline(always)]
    pub fn txbufh(&self) -> TXBUFH_R {
        TXBUFH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx buffer half empty"]
    #[inline(always)]
    pub fn rxbufh(&self) -> RXBUFH_R {
        RXBUFH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx buffer full"]
    #[inline(always)]
    pub fn txbuff(&self) -> TXBUFF_R {
        TXBUFF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx buffer full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx buffer empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx buffer empty"]
    #[inline(always)]
    pub fn rxbufe(&self) -> RXBUFE_R {
        RXBUFE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tx data vaild"]
    #[inline(always)]
    pub fn txbuf(&self) -> TXBUF_R {
        TXBUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx data vaild"]
    #[inline(always)]
    pub fn rxbuf(&self) -> RXBUF_R {
        RXBUF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt"]
    #[inline(always)]
    pub fn ioif(&self) -> IOIF_R {
        IOIF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("cmdfail", &self.cmdfail())
            .field("dtfail", &self.dtfail())
            .field("cmdtimeout", &self.cmdtimeout())
            .field("dttimeout", &self.dttimeout())
            .field("txerru", &self.txerru())
            .field("rxerro", &self.rxerro())
            .field("cmdrspcmpl", &self.cmdrspcmpl())
            .field("cmdcmpl", &self.cmdcmpl())
            .field("dtcmpl", &self.dtcmpl())
            .field("sbiterr", &self.sbiterr())
            .field("dtblkcmpl", &self.dtblkcmpl())
            .field("docmd", &self.docmd())
            .field("dotx", &self.dotx())
            .field("dorx", &self.dorx())
            .field("txbufh", &self.txbufh())
            .field("rxbufh", &self.rxbufh())
            .field("txbuff", &self.txbuff())
            .field("rxbuff", &self.rxbuff())
            .field("txbufe", &self.txbufe())
            .field("rxbufe", &self.rxbufe())
            .field("txbuf", &self.txbuf())
            .field("rxbuf", &self.rxbuf())
            .field("ioif", &self.ioif())
            .finish()
    }
}
#[doc = "SDIO status register (SDIO_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
