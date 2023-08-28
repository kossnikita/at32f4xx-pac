#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<INTCLR_SPEC>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<INTCLR_SPEC>;
#[doc = "Field `CMDFAIL` reader - Command crc fail flag clear"]
pub type CMDFAIL_R = crate::BitReader;
#[doc = "Field `CMDFAIL` writer - Command crc fail flag clear"]
pub type CMDFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTFAIL` reader - Data crc fail flag clear"]
pub type DTFAIL_R = crate::BitReader;
#[doc = "Field `DTFAIL` writer - Data crc fail flag clear"]
pub type DTFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDTIMEOUT` reader - Command timeout flag clear"]
pub type CMDTIMEOUT_R = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` writer - Command timeout flag clear"]
pub type CMDTIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTTIMEOUT` reader - Data timeout flag clear"]
pub type DTTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTTIMEOUT` writer - Data timeout flag clear"]
pub type DTTIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXERRU` reader - Tx under run error flag clear"]
pub type TXERRU_R = crate::BitReader;
#[doc = "Field `TXERRU` writer - Tx under run error flag clear"]
pub type TXERRU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXERRU` reader - Rx over run error flag clear"]
pub type RXERRU_R = crate::BitReader;
#[doc = "Field `RXERRU` writer - Rx over run error flag clear"]
pub type RXERRU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDRSPCMPL` reader - Command response complete flag clear"]
pub type CMDRSPCMPL_R = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` writer - Command response complete flag clear"]
pub type CMDRSPCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDCMPL` reader - Command sent flag clear"]
pub type CMDCMPL_R = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - Command sent flag clear"]
pub type CMDCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTCMPL` reader - Data sent flag clear"]
pub type DTCMPL_R = crate::BitReader;
#[doc = "Field `DTCMPL` writer - Data sent flag clear"]
pub type DTCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SBITERR` reader - Start bit error flag clear"]
pub type SBITERR_R = crate::BitReader;
#[doc = "Field `SBITERR` writer - Start bit error flag clear"]
pub type SBITERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTBLKCMPL` reader - Data block sent clear"]
pub type DTBLKCMPL_R = crate::BitReader;
#[doc = "Field `DTBLKCMPL` writer - Data block sent clear"]
pub type DTBLKCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOIF` reader - SD I/O interrupt flag clear"]
pub type IOIF_R = crate::BitReader;
#[doc = "Field `IOIF` writer - SD I/O interrupt flag clear"]
pub type IOIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bit 0 - Command crc fail flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfail(&mut self) -> CMDFAIL_W<INTCLR_SPEC, 0> {
        CMDFAIL_W::new(self)
    }
    #[doc = "Bit 1 - Data crc fail flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtfail(&mut self) -> DTFAIL_W<INTCLR_SPEC, 1> {
        DTFAIL_W::new(self)
    }
    #[doc = "Bit 2 - Command timeout flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtimeout(&mut self) -> CMDTIMEOUT_W<INTCLR_SPEC, 2> {
        CMDTIMEOUT_W::new(self)
    }
    #[doc = "Bit 3 - Data timeout flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn dttimeout(&mut self) -> DTTIMEOUT_W<INTCLR_SPEC, 3> {
        DTTIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - Tx under run error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn txerru(&mut self) -> TXERRU_W<INTCLR_SPEC, 4> {
        TXERRU_W::new(self)
    }
    #[doc = "Bit 5 - Rx over run error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxerru(&mut self) -> RXERRU_W<INTCLR_SPEC, 5> {
        RXERRU_W::new(self)
    }
    #[doc = "Bit 6 - Command response complete flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrspcmpl(&mut self) -> CMDRSPCMPL_W<INTCLR_SPEC, 6> {
        CMDRSPCMPL_W::new(self)
    }
    #[doc = "Bit 7 - Command sent flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmpl(&mut self) -> CMDCMPL_W<INTCLR_SPEC, 7> {
        CMDCMPL_W::new(self)
    }
    #[doc = "Bit 8 - Data sent flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmpl(&mut self) -> DTCMPL_W<INTCLR_SPEC, 8> {
        DTCMPL_W::new(self)
    }
    #[doc = "Bit 9 - Start bit error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn sbiterr(&mut self) -> SBITERR_W<INTCLR_SPEC, 9> {
        SBITERR_W::new(self)
    }
    #[doc = "Bit 10 - Data block sent clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtblkcmpl(&mut self) -> DTBLKCMPL_W<INTCLR_SPEC, 10> {
        DTBLKCMPL_W::new(self)
    }
    #[doc = "Bit 22 - SD I/O interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ioif(&mut self) -> IOIF_W<INTCLR_SPEC, 22> {
        IOIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDIO interrupt clear register (SDIO_INTCLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for INTCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
