#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Field `TPEFCLR` writer - Tamper event flag clear"]
pub type TPEFCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPIFCLR` writer - Tamper interrupt flag clear"]
pub type TPIFCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPIEN` reader - Tamper pin interrupt enable"]
pub type TPIEN_R = crate::BitReader;
#[doc = "Field `TPIEN` writer - Tamper pin interrupt enable"]
pub type TPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPEF` reader - Tamper event flag"]
pub type TPEF_R = crate::BitReader;
#[doc = "Field `TPEF` writer - Tamper event flag"]
pub type TPEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPIF` reader - Tamper interrupt flag"]
pub type TPIF_R = crate::BitReader;
#[doc = "Field `TPIF` writer - Tamper interrupt flag"]
pub type TPIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Tamper pin interrupt enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TPIEN_R {
        TPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tpef(&self) -> TPEF_R {
        TPEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tpif(&self) -> TPIF_R {
        TPIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpefclr(&mut self) -> TPEFCLR_W<CTRLSTS_SPEC, 0> {
        TPEFCLR_W::new(self)
    }
    #[doc = "Bit 1 - Tamper interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpifclr(&mut self) -> TPIFCLR_W<CTRLSTS_SPEC, 1> {
        TPIFCLR_W::new(self)
    }
    #[doc = "Bit 2 - Tamper pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpien(&mut self) -> TPIEN_W<CTRLSTS_SPEC, 2> {
        TPIEN_W::new(self)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpef(&mut self) -> TPEF_W<CTRLSTS_SPEC, 8> {
        TPEF_W::new(self)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpif(&mut self) -> TPIF_W<CTRLSTS_SPEC, 9> {
        TPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CTRLSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
