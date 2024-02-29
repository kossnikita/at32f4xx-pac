#[doc = "Register `MADDR` reader"]
pub type R = crate::R<MADDR_SPEC>;
#[doc = "Register `MADDR` writer"]
pub type W = crate::W<MADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "DMA channel memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MADDR_SPEC;
impl crate::RegisterSpec for MADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maddr::R`](R) reader structure"]
impl crate::Readable for MADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maddr::W`](W) writer structure"]
impl crate::Writable for MADDR_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MADDR to value 0"]
impl crate::Resettable for MADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
