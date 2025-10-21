#[doc = "Register `D1DTH8R` reader"]
pub type R = crate::R<D1DTH8R_SPEC>;
#[doc = "Register `D1DTH8R` writer"]
pub type W = crate::W<D1DTH8R_SPEC>;
#[doc = "Field `D1DT8R` reader - DAC1 8-bit right-aligned data"]
pub type D1DT8R_R = crate::FieldReader;
#[doc = "Field `D1DT8R` writer - DAC1 8-bit right-aligned data"]
pub type D1DT8R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt8r(&self) -> D1DT8R_R {
        D1DT8R_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D1DTH8R")
            .field("d1dt8r", &self.d1dt8r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt8r(&mut self) -> D1DT8R_W<'_, D1DTH8R_SPEC> {
        D1DT8R_W::new(self, 0)
    }
}
#[doc = "DAC1 8-bit right aligned data holding register (DAC_D1DTH8R)\n\nYou can [`read`](crate::Reg::read) this register and get [`d1dth8r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1dth8r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1DTH8R_SPEC;
impl crate::RegisterSpec for D1DTH8R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1dth8r::R`](R) reader structure"]
impl crate::Readable for D1DTH8R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d1dth8r::W`](W) writer structure"]
impl crate::Writable for D1DTH8R_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1DTH8R to value 0"]
impl crate::Resettable for D1DTH8R_SPEC {}
