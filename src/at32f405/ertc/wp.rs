#[doc = "Register `WP` writer"]
pub type W = crate::W<WP_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<WP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WP_SPEC;
impl crate::RegisterSpec for WP_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`wp::W`](W) writer structure"]
impl crate::Writable for WP_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WP_SPEC {}
