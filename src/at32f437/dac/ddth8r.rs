#[doc = "Register `DDTH8R` reader"]
pub type R = crate::R<DDTH8R_SPEC>;
#[doc = "Register `DDTH8R` writer"]
pub type W = crate::W<DDTH8R_SPEC>;
#[doc = "Field `DD1DT8R` reader - DAC1 8-bit right-aligned data"]
pub type DD1DT8R_R = crate::FieldReader;
#[doc = "Field `DD1DT8R` writer - DAC1 8-bit right-aligned data"]
pub type DD1DT8R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DD2DT8R` reader - DAC2 8-bit right-aligned data"]
pub type DD2DT8R_R = crate::FieldReader;
#[doc = "Field `DD2DT8R` writer - DAC2 8-bit right-aligned data"]
pub type DD2DT8R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDTH8R")
            .field("dd1dt8r", &format_args!("{}", self.dd1dt8r().bits()))
            .field("dd2dt8r", &format_args!("{}", self.dd2dt8r().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDTH8R_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd1dt8r(&mut self) -> DD1DT8R_W<DDTH8R_SPEC> {
        DD1DT8R_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd2dt8r(&mut self) -> DD2DT8R_W<DDTH8R_SPEC> {
        DD2DT8R_W::new(self, 8)
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
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth8r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth8r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDTH8R_SPEC;
impl crate::RegisterSpec for DDTH8R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddth8r::R`](R) reader structure"]
impl crate::Readable for DDTH8R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddth8r::W`](W) writer structure"]
impl crate::Writable for DDTH8R_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDTH8R to value 0"]
impl crate::Resettable for DDTH8R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
