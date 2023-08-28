#[doc = "Register `PLLCFG` reader"]
pub type R = crate::R<PLLCFG_SPEC>;
#[doc = "Register `PLLCFG` writer"]
pub type W = crate::W<PLLCFG_SPEC>;
#[doc = "Field `PLL_MS` reader - PLL pre-division"]
pub type PLL_MS_R = crate::FieldReader;
#[doc = "Field `PLL_MS` writer - PLL pre-division"]
pub type PLL_MS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLL_NS` reader - PLL frequency multiplication factor"]
pub type PLL_NS_R = crate::FieldReader<u16>;
#[doc = "Field `PLL_NS` writer - PLL frequency multiplication factor"]
pub type PLL_NS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `PLL_FP` reader - PLLP post-division"]
pub type PLL_FP_R = crate::FieldReader;
#[doc = "Field `PLL_FP` writer - PLLP post-division"]
pub type PLL_FP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PLL_FU` reader - PLLU post-division"]
pub type PLL_FU_R = crate::FieldReader;
#[doc = "Field `PLL_FU` writer - PLLU post-division"]
pub type PLL_FU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PLLU_EN` reader - PLLU enable"]
pub type PLLU_EN_R = crate::BitReader;
#[doc = "Field `PLLU_EN` writer - PLLU enable"]
pub type PLLU_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLRCS` reader - PLL reference clock select"]
pub type PLLRCS_R = crate::BitReader;
#[doc = "Field `PLLRCS` writer - PLL reference clock select"]
pub type PLLRCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - PLL pre-division"]
    #[inline(always)]
    pub fn pll_ms(&self) -> PLL_MS_R {
        PLL_MS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:14 - PLL frequency multiplication factor"]
    #[inline(always)]
    pub fn pll_ns(&self) -> PLL_NS_R {
        PLL_NS_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:19 - PLLP post-division"]
    #[inline(always)]
    pub fn pll_fp(&self) -> PLL_FP_R {
        PLL_FP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - PLLU post-division"]
    #[inline(always)]
    pub fn pll_fu(&self) -> PLL_FU_R {
        PLL_FU_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 29 - PLLU enable"]
    #[inline(always)]
    pub fn pllu_en(&self) -> PLLU_EN_R {
        PLLU_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PLL reference clock select"]
    #[inline(always)]
    pub fn pllrcs(&self) -> PLLRCS_R {
        PLLRCS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL pre-division"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ms(&mut self) -> PLL_MS_W<PLLCFG_SPEC, 0> {
        PLL_MS_W::new(self)
    }
    #[doc = "Bits 6:14 - PLL frequency multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ns(&mut self) -> PLL_NS_W<PLLCFG_SPEC, 6> {
        PLL_NS_W::new(self)
    }
    #[doc = "Bits 16:19 - PLLP post-division"]
    #[inline(always)]
    #[must_use]
    pub fn pll_fp(&mut self) -> PLL_FP_W<PLLCFG_SPEC, 16> {
        PLL_FP_W::new(self)
    }
    #[doc = "Bits 20:22 - PLLU post-division"]
    #[inline(always)]
    #[must_use]
    pub fn pll_fu(&mut self) -> PLL_FU_W<PLLCFG_SPEC, 20> {
        PLL_FU_W::new(self)
    }
    #[doc = "Bit 29 - PLLU enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllu_en(&mut self) -> PLLU_EN_W<PLLCFG_SPEC, 29> {
        PLLU_EN_W::new(self)
    }
    #[doc = "Bit 30 - PLL reference clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllrcs(&mut self) -> PLLRCS_W<PLLCFG_SPEC, 30> {
        PLLRCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PLL configuration register (CRM_PLLCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFG_SPEC;
impl crate::RegisterSpec for PLLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfg::R`](R) reader structure"]
impl crate::Readable for PLLCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcfg::W`](W) writer structure"]
impl crate::Writable for PLLCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFG to value 0x0003_3002"]
impl crate::Resettable for PLLCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_3002;
}
