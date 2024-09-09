#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BAUDR_SPEC>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BAUDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`baudr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUDR_SPEC;
impl crate::RegisterSpec for BAUDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BAUDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BAUDR_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BAUDR_SPEC {
    const RESET_VALUE: u16 = 0;
}
