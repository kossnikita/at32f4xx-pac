#[doc = "Register `USD_UNLOCK` writer"]
pub type W = crate::W<USD_UNLOCK_SPEC>;
#[doc = "Field `USD_UKVAL` writer - User system data Unlock key value"]
pub type USD_UKVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl core::fmt::Debug for crate::generic::Reg<USD_UNLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - User system data Unlock key value"]
    #[inline(always)]
    #[must_use]
    pub fn usd_ukval(&mut self) -> USD_UKVAL_W<USD_UNLOCK_SPEC, 0> {
        USD_UKVAL_W::new(self)
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
#[doc = "USD unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usd_unlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USD_UNLOCK_SPEC;
impl crate::RegisterSpec for USD_UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`usd_unlock::W`](W) writer structure"]
impl crate::Writable for USD_UNLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USD_UNLOCK to value 0"]
impl crate::Resettable for USD_UNLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
