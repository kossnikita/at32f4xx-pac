#[doc = "Register `D1DTH12R` reader"]
pub type R = crate::R<D1DTH12R_SPEC>;
#[doc = "Register `D1DTH12R` writer"]
pub type W = crate::W<D1DTH12R_SPEC>;
#[doc = "Field `D1DT12R` reader - DAC1 12-bit right-aligned data"]
pub type D1DT12R_R = crate::FieldReader<u16>;
#[doc = "Field `D1DT12R` writer - DAC1 12-bit right-aligned data"]
pub type D1DT12R_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt12r(&self) -> D1DT12R_R {
        D1DT12R_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1DTH12R")
            .field("d1dt12r", &self.d1dt12r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt12r(&mut self) -> D1DT12R_W<'_, D1DTH12R_SPEC> {
        D1DT12R_W::new(self, 0)
    }
}
#[doc = "DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)\n\nYou can [`read`](crate::Reg::read) this register and get [`d1dth12r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1dth12r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1DTH12R_SPEC;
impl crate::RegisterSpec for D1DTH12R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1dth12r::R`](R) reader structure"]
impl crate::Readable for D1DTH12R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d1dth12r::W`](W) writer structure"]
impl crate::Writable for D1DTH12R_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1DTH12R to value 0"]
impl crate::Resettable for D1DTH12R_SPEC {}
