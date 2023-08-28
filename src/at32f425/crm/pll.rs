#[doc = "Register `PLL` reader"]
pub type R = crate::R<PLL_SPEC>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PLL_SPEC>;
#[doc = "Field `PLL_FR` reader - PLL_FR"]
pub type PLL_FR_R = crate::FieldReader;
#[doc = "Field `PLL_FR` writer - PLL_FR"]
pub type PLL_FR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PLL_MS` reader - PLL_MS"]
pub type PLL_MS_R = crate::FieldReader;
#[doc = "Field `PLL_MS` writer - PLL_MS"]
pub type PLL_MS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLL_NS` reader - PLL_NS"]
pub type PLL_NS_R = crate::FieldReader<u16>;
#[doc = "Field `PLL_NS` writer - PLL_NS"]
pub type PLL_NS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `PLL_FREF` reader - PLL entry clock reference frequency"]
pub type PLL_FREF_R = crate::FieldReader;
#[doc = "Field `PLL_FREF` writer - PLL entry clock reference frequency"]
pub type PLL_FREF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PLLCFGEN` reader - PLL config enable"]
pub type PLLCFGEN_R = crate::BitReader;
#[doc = "Field `PLLCFGEN` writer - PLL config enable"]
pub type PLLCFGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - PLL_FR"]
    #[inline(always)]
    pub fn pll_fr(&self) -> PLL_FR_R {
        PLL_FR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - PLL_MS"]
    #[inline(always)]
    pub fn pll_ms(&self) -> PLL_MS_R {
        PLL_MS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:16 - PLL_NS"]
    #[inline(always)]
    pub fn pll_ns(&self) -> PLL_NS_R {
        PLL_NS_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:26 - PLL entry clock reference frequency"]
    #[inline(always)]
    pub fn pll_fref(&self) -> PLL_FREF_R {
        PLL_FREF_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - PLL config enable"]
    #[inline(always)]
    pub fn pllcfgen(&self) -> PLLCFGEN_R {
        PLLCFGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL_FR"]
    #[inline(always)]
    #[must_use]
    pub fn pll_fr(&mut self) -> PLL_FR_W<PLL_SPEC, 0> {
        PLL_FR_W::new(self)
    }
    #[doc = "Bits 4:7 - PLL_MS"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ms(&mut self) -> PLL_MS_W<PLL_SPEC, 4> {
        PLL_MS_W::new(self)
    }
    #[doc = "Bits 8:16 - PLL_NS"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ns(&mut self) -> PLL_NS_W<PLL_SPEC, 8> {
        PLL_NS_W::new(self)
    }
    #[doc = "Bits 24:26 - PLL entry clock reference frequency"]
    #[inline(always)]
    #[must_use]
    pub fn pll_fref(&mut self) -> PLL_FREF_W<PLL_SPEC, 24> {
        PLL_FREF_W::new(self)
    }
    #[doc = "Bit 31 - PLL config enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllcfgen(&mut self) -> PLLCFGEN_W<PLL_SPEC, 31> {
        PLLCFGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PLL configuration register (CRM_PLL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_SPEC;
impl crate::RegisterSpec for PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL to value 0x1f10"]
impl crate::Resettable for PLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f10;
}
