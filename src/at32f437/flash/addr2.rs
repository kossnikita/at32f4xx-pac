#[doc = "Register `ADDR2` writer"]
pub type W = crate::W<ADDR2_SPEC>;
#[doc = "Field `FA` writer - Flash Address"]
pub type FA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash Address"]
    #[inline(always)]
    #[must_use]
    pub fn fa(&mut self) -> FA_W<ADDR2_SPEC, 0> {
        FA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Address 2 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR2_SPEC;
impl crate::RegisterSpec for ADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`addr2::W`](W) writer structure"]
impl crate::Writable for ADDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR2 to value 0"]
impl crate::Resettable for ADDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
