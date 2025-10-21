#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<INTCLR_SPEC>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<INTCLR_SPEC>;
#[doc = "Field `CMDFAIL` reader - Command crc fail flag clear"]
pub type CMDFAIL_R = crate::BitReader;
#[doc = "Field `CMDFAIL` writer - Command crc fail flag clear"]
pub type CMDFAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFAIL` reader - Data crc fail flag clear"]
pub type DTFAIL_R = crate::BitReader;
#[doc = "Field `DTFAIL` writer - Data crc fail flag clear"]
pub type DTFAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUT` reader - Command timeout flag clear"]
pub type CMDTIMEOUT_R = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` writer - Command timeout flag clear"]
pub type CMDTIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTIMEOUT` reader - Data timeout flag clear"]
pub type DTTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTTIMEOUT` writer - Data timeout flag clear"]
pub type DTTIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRU` reader - Tx under run error flag clear"]
pub type TXERRU_R = crate::BitReader;
#[doc = "Field `TXERRU` writer - Tx under run error flag clear"]
pub type TXERRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRU` reader - Rx over run error flag clear"]
pub type RXERRU_R = crate::BitReader;
#[doc = "Field `RXERRU` writer - Rx over run error flag clear"]
pub type RXERRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRSPCMPL` reader - Command response complete flag clear"]
pub type CMDRSPCMPL_R = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` writer - Command response complete flag clear"]
pub type CMDRSPCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCMPL` reader - Command sent flag clear"]
pub type CMDCMPL_R = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - Command sent flag clear"]
pub type CMDCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMPL` reader - Data sent flag clear"]
pub type DTCMPL_R = crate::BitReader;
#[doc = "Field `DTCMPL` writer - Data sent flag clear"]
pub type DTCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBITERR` reader - Start bit error flag clear"]
pub type SBITERR_R = crate::BitReader;
#[doc = "Field `SBITERR` writer - Start bit error flag clear"]
pub type SBITERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBLKCMPL` reader - Data block sent clear"]
pub type DTBLKCMPL_R = crate::BitReader;
#[doc = "Field `DTBLKCMPL` writer - Data block sent clear"]
pub type DTBLKCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOIF` reader - SD I/O interrupt flag clear"]
pub type IOIF_R = crate::BitReader;
#[doc = "Field `IOIF` writer - SD I/O interrupt flag clear"]
pub type IOIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command crc fail flag clear"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CMDFAIL_R {
        CMDFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data crc fail flag clear"]
    #[inline(always)]
    pub fn dtfail(&self) -> DTFAIL_R {
        DTFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout flag clear"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CMDTIMEOUT_R {
        CMDTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout flag clear"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DTTIMEOUT_R {
        DTTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx under run error flag clear"]
    #[inline(always)]
    pub fn txerru(&self) -> TXERRU_R {
        TXERRU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx over run error flag clear"]
    #[inline(always)]
    pub fn rxerru(&self) -> RXERRU_R {
        RXERRU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response complete flag clear"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CMDRSPCMPL_R {
        CMDRSPCMPL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent flag clear"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CMDCMPL_R {
        CMDCMPL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data sent flag clear"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DTCMPL_R {
        DTCMPL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error flag clear"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SBITERR_R {
        SBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent clear"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DTBLKCMPL_R {
        DTBLKCMPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt flag clear"]
    #[inline(always)]
    pub fn ioif(&self) -> IOIF_R {
        IOIF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTCLR")
            .field("cmdfail", &self.cmdfail())
            .field("dtfail", &self.dtfail())
            .field("cmdtimeout", &self.cmdtimeout())
            .field("dttimeout", &self.dttimeout())
            .field("txerru", &self.txerru())
            .field("rxerru", &self.rxerru())
            .field("cmdrspcmpl", &self.cmdrspcmpl())
            .field("cmdcmpl", &self.cmdcmpl())
            .field("dtcmpl", &self.dtcmpl())
            .field("sbiterr", &self.sbiterr())
            .field("dtblkcmpl", &self.dtblkcmpl())
            .field("ioif", &self.ioif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Command crc fail flag clear"]
    #[inline(always)]
    pub fn cmdfail(&mut self) -> CMDFAIL_W<'_, INTCLR_SPEC> {
        CMDFAIL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data crc fail flag clear"]
    #[inline(always)]
    pub fn dtfail(&mut self) -> DTFAIL_W<'_, INTCLR_SPEC> {
        DTFAIL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Command timeout flag clear"]
    #[inline(always)]
    pub fn cmdtimeout(&mut self) -> CMDTIMEOUT_W<'_, INTCLR_SPEC> {
        CMDTIMEOUT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data timeout flag clear"]
    #[inline(always)]
    pub fn dttimeout(&mut self) -> DTTIMEOUT_W<'_, INTCLR_SPEC> {
        DTTIMEOUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Tx under run error flag clear"]
    #[inline(always)]
    pub fn txerru(&mut self) -> TXERRU_W<'_, INTCLR_SPEC> {
        TXERRU_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx over run error flag clear"]
    #[inline(always)]
    pub fn rxerru(&mut self) -> RXERRU_W<'_, INTCLR_SPEC> {
        RXERRU_W::new(self, 5)
    }
    #[doc = "Bit 6 - Command response complete flag clear"]
    #[inline(always)]
    pub fn cmdrspcmpl(&mut self) -> CMDRSPCMPL_W<'_, INTCLR_SPEC> {
        CMDRSPCMPL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Command sent flag clear"]
    #[inline(always)]
    pub fn cmdcmpl(&mut self) -> CMDCMPL_W<'_, INTCLR_SPEC> {
        CMDCMPL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Data sent flag clear"]
    #[inline(always)]
    pub fn dtcmpl(&mut self) -> DTCMPL_W<'_, INTCLR_SPEC> {
        DTCMPL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start bit error flag clear"]
    #[inline(always)]
    pub fn sbiterr(&mut self) -> SBITERR_W<'_, INTCLR_SPEC> {
        SBITERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data block sent clear"]
    #[inline(always)]
    pub fn dtblkcmpl(&mut self) -> DTBLKCMPL_W<'_, INTCLR_SPEC> {
        DTBLKCMPL_W::new(self, 10)
    }
    #[doc = "Bit 22 - SD I/O interrupt flag clear"]
    #[inline(always)]
    pub fn ioif(&mut self) -> IOIF_W<'_, INTCLR_SPEC> {
        IOIF_W::new(self, 22)
    }
}
#[doc = "SDIO interrupt clear register (SDIO_INTCLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for INTCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {}
