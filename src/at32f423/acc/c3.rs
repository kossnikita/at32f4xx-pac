#[doc = "Register `C3` reader"]
pub type R = crate::R<C3_SPEC>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "compare value 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3_SPEC;
impl crate::RegisterSpec for C3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`c3::R`](R) reader structure"]
impl crate::Readable for C3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c3::W`](W) writer structure"]
impl crate::Writable for C3_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets C3 to value 0x1f54"]
impl crate::Resettable for C3_SPEC {
    const RESET_VALUE: u16 = 0x1f54;
}
