#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `D1DMAUDRF` reader - DAC1 DMA underrun flag"]
pub type D1DMAUDRF_R = crate::BitReader;
#[doc = "Field `D1DMAUDRF` writer - DAC1 DMA underrun flag"]
pub type D1DMAUDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2DMAUDRF` reader - DAC2 DMA underrun flag"]
pub type D2DMAUDRF_R = crate::BitReader;
#[doc = "Field `D2DMAUDRF` writer - DAC2 DMA underrun flag"]
pub type D2DMAUDRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - DAC1 DMA underrun flag"]
    #[inline(always)]
    pub fn d1dmaudrf(&self) -> D1DMAUDRF_R {
        D1DMAUDRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun flag"]
    #[inline(always)]
    pub fn d2dmaudrf(&self) -> D2DMAUDRF_R {
        D2DMAUDRF_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("d1dmaudrf", &format_args!("{}", self.d1dmaudrf().bit()))
            .field("d2dmaudrf", &format_args!("{}", self.d2dmaudrf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 13 - DAC1 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn d1dmaudrf(&mut self) -> D1DMAUDRF_W<STS_SPEC> {
        D1DMAUDRF_W::new(self, 13)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn d2dmaudrf(&mut self) -> D2DMAUDRF_W<STS_SPEC> {
        D2DMAUDRF_W::new(self, 29)
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
#[doc = "DAC2 status register (DAC_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
