#[doc = "Register `MADDR` reader"]
pub type R = crate::R<MADDR_SPEC>;
#[doc = "Register `MADDR` writer"]
pub type W = crate::W<MADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA channel memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MADDR_SPEC;
impl crate::RegisterSpec for MADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maddr::R`](R) reader structure"]
impl crate::Readable for MADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maddr::W`](W) writer structure"]
impl crate::Writable for MADDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MADDR to value 0"]
impl crate::Resettable for MADDR_SPEC {}
