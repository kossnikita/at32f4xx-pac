#[doc = "Register `USD_UNLOCK` writer"]
pub type W = crate::W<USD_UNLOCK_SPEC>;
#[doc = "Field `USD_UKVAL` writer - User system data Unlock key value"]
pub type USD_UKVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<USD_UNLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - User system data Unlock key value"]
    #[inline(always)]
    pub fn usd_ukval(&mut self) -> USD_UKVAL_W<'_, USD_UNLOCK_SPEC> {
        USD_UKVAL_W::new(self, 0)
    }
}
#[doc = "USD unlock register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usd_unlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USD_UNLOCK_SPEC;
impl crate::RegisterSpec for USD_UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`usd_unlock::W`](W) writer structure"]
impl crate::Writable for USD_UNLOCK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USD_UNLOCK to value 0"]
impl crate::Resettable for USD_UNLOCK_SPEC {}
