#[doc = "Register `RPR` reader"]
pub type R = crate::R<RPR_SPEC>;
#[doc = "Register `RPR` writer"]
pub type W = crate::W<RPR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Repetition of period value\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RPR_SPEC;
impl crate::RegisterSpec for RPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rpr::R`](R) reader structure"]
impl crate::Readable for RPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rpr::W`](W) writer structure"]
impl crate::Writable for RPR_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RPR to value 0"]
impl crate::Resettable for RPR_SPEC {
    const RESET_VALUE: u8 = 0;
}
