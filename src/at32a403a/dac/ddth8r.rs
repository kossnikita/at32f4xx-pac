#[doc = "Register `DDTH8R` reader"]
pub type R = crate::R<DDTH8R_SPEC>;
#[doc = "Register `DDTH8R` writer"]
pub type W = crate::W<DDTH8R_SPEC>;
#[doc = "Field `DD1DT8R` reader - DAC1 8-bit right-aligned data"]
pub type DD1DT8R_R = crate::FieldReader;
#[doc = "Field `DD1DT8R` writer - DAC1 8-bit right-aligned data"]
pub type DD1DT8R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DD2DT8R` reader - DAC2 8-bit right-aligned data"]
pub type DD2DT8R_R = crate::FieldReader;
#[doc = "Field `DD2DT8R` writer - DAC2 8-bit right-aligned data"]
pub type DD2DT8R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dd1dt8r(&self) -> DD1DT8R_R {
        DD1DT8R_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dd2dt8r(&self) -> DD2DT8R_R {
        DD2DT8R_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd1dt8r(&mut self) -> DD1DT8R_W<DDTH8R_SPEC, 0> {
        DD1DT8R_W::new(self)
    }
    #[doc = "Bits 8:15 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd2dt8r(&mut self) -> DD2DT8R_W<DDTH8R_SPEC, 8> {
        DD2DT8R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth8r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth8r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDTH8R_SPEC;
impl crate::RegisterSpec for DDTH8R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddth8r::R`](R) reader structure"]
impl crate::Readable for DDTH8R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddth8r::W`](W) writer structure"]
impl crate::Writable for DDTH8R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDTH8R to value 0"]
impl crate::Resettable for DDTH8R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}