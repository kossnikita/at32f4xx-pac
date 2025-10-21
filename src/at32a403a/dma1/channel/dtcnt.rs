#[doc = "Register `DTCNT` reader"]
pub type R = crate::R<DTCNT_SPEC>;
#[doc = "Register `DTCNT` writer"]
pub type W = crate::W<DTCNT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA channel number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCNT_SPEC;
impl crate::RegisterSpec for DTCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dtcnt::R`](R) reader structure"]
impl crate::Readable for DTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtcnt::W`](W) writer structure"]
impl crate::Writable for DTCNT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DTCNT to value 0"]
impl crate::Resettable for DTCNT_SPEC {}
