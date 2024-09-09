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
            .field("d1dmaudrf", &self.d1dmaudrf())
            .field("d2dmaudrf", &self.d2dmaudrf())
            .finish()
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
}
#[doc = "DAC2 status register (DAC_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
