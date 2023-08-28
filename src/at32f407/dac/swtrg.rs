#[doc = "Register `SWTRG` writer"]
pub type W = crate::W<SWTRG_SPEC>;
#[doc = "Field `D1SWTRG` writer - DAC1 software trigger"]
pub type D1SWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D2SWTRG` writer - DAC2 software trigger"]
pub type D2SWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - DAC1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn d1swtrg(&mut self) -> D1SWTRG_W<SWTRG_SPEC, 0> {
        D1SWTRG_W::new(self)
    }
    #[doc = "Bit 1 - DAC2 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn d2swtrg(&mut self) -> D2SWTRG_W<SWTRG_SPEC, 1> {
        D2SWTRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC software trigger register(DAC_SWTRIGR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRG_SPEC;
impl crate::RegisterSpec for SWTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrg::W`](W) writer structure"]
impl crate::Writable for SWTRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRG to value 0"]
impl crate::Resettable for SWTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
