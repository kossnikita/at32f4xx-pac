#[doc = "Register `WAT` reader"]
pub type R = crate::R<WAT_SPEC>;
#[doc = "Register `WAT` writer"]
pub type W = crate::W<WAT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAT_SPEC;
impl crate::RegisterSpec for WAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wat::R`](R) reader structure"]
impl crate::Readable for WAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wat::W`](W) writer structure"]
impl crate::Writable for WAT_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WAT to value 0xffff"]
impl crate::Resettable for WAT_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
