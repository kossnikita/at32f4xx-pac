#[doc = "Register `DDTH12R` reader"]
pub type R = crate::R<DDTH12R_SPEC>;
#[doc = "Register `DDTH12R` writer"]
pub type W = crate::W<DDTH12R_SPEC>;
#[doc = "Field `DD1DT12R` reader - DAC1 12-bit right-aligned data"]
pub type DD1DT12R_R = crate::FieldReader<u16>;
#[doc = "Field `DD1DT12R` writer - DAC1 12-bit right-aligned data"]
pub type DD1DT12R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DD2DT12R` reader - DAC2 12-bit right-aligned data"]
pub type DD2DT12R_R = crate::FieldReader<u16>;
#[doc = "Field `DD2DT12R` writer - DAC2 12-bit right-aligned data"]
pub type DD2DT12R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dd1dt12r(&self) -> DD1DT12R_R {
        DD1DT12R_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dd2dt12r(&self) -> DD2DT12R_R {
        DD2DT12R_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDTH12R")
            .field("dd1dt12r", &format_args!("{}", self.dd1dt12r().bits()))
            .field("dd2dt12r", &format_args!("{}", self.dd2dt12r().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDTH12R_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd1dt12r(&mut self) -> DD1DT12R_W<DDTH12R_SPEC, 0> {
        DD1DT12R_W::new(self)
    }
    #[doc = "Bits 16:27 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd2dt12r(&mut self) -> DD2DT12R_W<DDTH12R_SPEC, 16> {
        DD2DT12R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DDTH12R), Bits 31:28 Reserved, Bits 15:12 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth12r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth12r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDTH12R_SPEC;
impl crate::RegisterSpec for DDTH12R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddth12r::R`](R) reader structure"]
impl crate::Readable for DDTH12R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddth12r::W`](W) writer structure"]
impl crate::Writable for DDTH12R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDTH12R to value 0"]
impl crate::Resettable for DDTH12R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
