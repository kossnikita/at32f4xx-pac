#[doc = "Register `SELECT` writer"]
pub type W = crate::W<SELECT_SPEC>;
#[doc = "Field `SELECT` writer - spim type selection"]
pub type SELECT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - spim type selection"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<SELECT_SPEC, 0> {
        SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Select register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`select::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SELECT_SPEC;
impl crate::RegisterSpec for SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`select::W`](W) writer structure"]
impl crate::Writable for SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SELECT to value 0"]
impl crate::Resettable for SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
