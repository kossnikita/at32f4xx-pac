#[doc = "Register `WP` writer"]
pub type W = crate::W<WP_SPEC>;
#[doc = "Field `CMD` writer - Command register"]
pub type CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Command register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<WP_SPEC, 0> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WP_SPEC;
impl crate::RegisterSpec for WP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wp::W`](W) writer structure"]
impl crate::Writable for WP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
