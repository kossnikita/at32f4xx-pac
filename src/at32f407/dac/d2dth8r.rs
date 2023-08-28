#[doc = "Register `D2DTH8R` reader"]
pub type R = crate::R<D2DTH8R_SPEC>;
#[doc = "Register `D2DTH8R` writer"]
pub type W = crate::W<D2DTH8R_SPEC>;
#[doc = "Field `D2DT8R` reader - DAC2 8-bit right-aligned data"]
pub type D2DT8R_R = crate::FieldReader;
#[doc = "Field `D2DT8R` writer - DAC2 8-bit right-aligned data"]
pub type D2DT8R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn d2dt8r(&self) -> D2DT8R_R {
        D2DT8R_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn d2dt8r(&mut self) -> D2DT8R_W<D2DTH8R_SPEC, 0> {
        D2DT8R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC2 8-bit right-aligned data holding register (DAC_D2DTH8R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2dth8r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2dth8r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2DTH8R_SPEC;
impl crate::RegisterSpec for D2DTH8R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2dth8r::R`](R) reader structure"]
impl crate::Readable for D2DTH8R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d2dth8r::W`](W) writer structure"]
impl crate::Writable for D2DTH8R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D2DTH8R to value 0"]
impl crate::Resettable for D2DTH8R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
