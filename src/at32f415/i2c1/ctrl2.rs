#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CLKFREQ` reader - Input clock frequency"]
pub type CLKFREQ_R = crate::FieldReader;
#[doc = "Field `CLKFREQ` writer - Input clock frequency"]
pub type CLKFREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ERRIEN_R = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVTIEN` reader - Event interrupt enable"]
pub type EVTIEN_R = crate::BitReader;
#[doc = "Field `EVTIEN` writer - Event interrupt enable"]
pub type EVTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATAIEN` reader - Data transmission interrupt enable"]
pub type DATAIEN_R = crate::BitReader;
#[doc = "Field `DATAIEN` writer - Data transmission interrupt enable"]
pub type DATAIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAEN` reader - DMA transfer enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA transfer enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAEND` reader - DMA transfer end indication"]
pub type DMAEND_R = crate::BitReader;
#[doc = "Field `DMAEND` writer - DMA transfer end indication"]
pub type DMAEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Input clock frequency"]
    #[inline(always)]
    pub fn clkfreq(&self) -> CLKFREQ_R {
        CLKFREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evtien(&self) -> EVTIEN_R {
        EVTIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmission interrupt enable"]
    #[inline(always)]
    pub fn dataien(&self) -> DATAIEN_R {
        DATAIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA transfer enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA transfer end indication"]
    #[inline(always)]
    pub fn dmaend(&self) -> DMAEND_R {
        DMAEND_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("dmaend", &format_args!("{}", self.dmaend().bit()))
            .field("dmaen", &format_args!("{}", self.dmaen().bit()))
            .field("dataien", &format_args!("{}", self.dataien().bit()))
            .field("evtien", &format_args!("{}", self.evtien().bit()))
            .field("errien", &format_args!("{}", self.errien().bit()))
            .field("clkfreq", &format_args!("{}", self.clkfreq().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn clkfreq(&mut self) -> CLKFREQ_W<CTRL2_SPEC, 0> {
        CLKFREQ_W::new(self)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ERRIEN_W<CTRL2_SPEC, 8> {
        ERRIEN_W::new(self)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn evtien(&mut self) -> EVTIEN_W<CTRL2_SPEC, 9> {
        EVTIEN_W::new(self)
    }
    #[doc = "Bit 10 - Data transmission interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataien(&mut self) -> DATAIEN_W<CTRL2_SPEC, 10> {
        DATAIEN_W::new(self)
    }
    #[doc = "Bit 11 - DMA transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CTRL2_SPEC, 11> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 12 - DMA transfer end indication"]
    #[inline(always)]
    #[must_use]
    pub fn dmaend(&mut self) -> DMAEND_W<CTRL2_SPEC, 12> {
        DMAEND_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
