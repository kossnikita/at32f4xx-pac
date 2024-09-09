#[doc = "Register `C%sDT` reader"]
pub type R = crate::R<CDT_SPEC>;
#[doc = "Register `C%sDT` writer"]
pub type W = crate::W<CDT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Channel data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDT_SPEC;
impl crate::RegisterSpec for CDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdt::R`](R) reader structure"]
impl crate::Readable for CDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdt::W`](W) writer structure"]
impl crate::Writable for CDT_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C%sDT to value 0"]
impl crate::Resettable for CDT_SPEC {
    const RESET_VALUE: u32 = 0;
}
