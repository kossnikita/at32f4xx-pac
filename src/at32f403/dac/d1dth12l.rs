#[doc = "Register `D1DTH12L` reader"]
pub type R = crate::R<D1DTH12L_SPEC>;
#[doc = "Register `D1DTH12L` writer"]
pub type W = crate::W<D1DTH12L_SPEC>;
#[doc = "Field `D1DT12L` reader - DAC1 12-bit left-aligned data"]
pub type D1DT12L_R = crate::FieldReader<u16>;
#[doc = "Field `D1DT12L` writer - DAC1 12-bit left-aligned data"]
pub type D1DT12L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn d1dt12l(&self) -> D1DT12L_R {
        D1DT12L_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn d1dt12l(&mut self) -> D1DT12L_W<D1DTH12L_SPEC, 4> {
        D1DT12L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC1 12-bit left aligned data holding register (DAC_D1DTH12L)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1dth12l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1dth12l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1DTH12L_SPEC;
impl crate::RegisterSpec for D1DTH12L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1dth12l::R`](R) reader structure"]
impl crate::Readable for D1DTH12L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d1dth12l::W`](W) writer structure"]
impl crate::Writable for D1DTH12L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D1DTH12L to value 0"]
impl crate::Resettable for D1DTH12L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}