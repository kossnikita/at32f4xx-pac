#[doc = "Register `D2DTH12R` reader"]
pub type R = crate::R<D2DTH12R_SPEC>;
#[doc = "Register `D2DTH12R` writer"]
pub type W = crate::W<D2DTH12R_SPEC>;
#[doc = "Field `D2DT12R` reader - DAC2 12-bit right-aligned data"]
pub type D2DT12R_R = crate::FieldReader<u16>;
#[doc = "Field `D2DT12R` writer - DAC2 12-bit right-aligned data"]
pub type D2DT12R_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn d2dt12r(&self) -> D2DT12R_R {
        D2DT12R_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D2DTH12R")
            .field("d2dt12r", &format_args!("{}", self.d2dt12r().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<D2DTH12R_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn d2dt12r(&mut self) -> D2DT12R_W<D2DTH12R_SPEC> {
        D2DT12R_W::new(self, 0)
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
#[doc = "DAC2 12-bit right aligned data holding register (DAC_D2DTH12R)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2dth12r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2dth12r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2DTH12R_SPEC;
impl crate::RegisterSpec for D2DTH12R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2dth12r::R`](R) reader structure"]
impl crate::Readable for D2DTH12R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d2dth12r::W`](W) writer structure"]
impl crate::Writable for D2DTH12R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D2DTH12R to value 0"]
impl crate::Resettable for D2DTH12R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
