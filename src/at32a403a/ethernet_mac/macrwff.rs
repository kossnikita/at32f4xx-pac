#[doc = "Register `MACRWFF` reader"]
pub type R = crate::R<MACRWFF_SPEC>;
#[doc = "Register `MACRWFF` writer"]
pub type W = crate::W<MACRWFF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC remote wakeup frame filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrwff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRWFF_SPEC;
impl crate::RegisterSpec for MACRWFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwff::R`](R) reader structure"]
impl crate::Readable for MACRWFF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macrwff::W`](W) writer structure"]
impl crate::Writable for MACRWFF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACRWFF to value 0"]
impl crate::Resettable for MACRWFF_SPEC {
    const RESET_VALUE: u32 = 0;
}
