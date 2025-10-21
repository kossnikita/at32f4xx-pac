#[doc = "Register `PADDR` reader"]
pub type R = crate::R<PADDR_SPEC>;
#[doc = "Register `PADDR` writer"]
pub type W = crate::W<PADDR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA channel peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADDR_SPEC;
impl crate::RegisterSpec for PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`paddr::R`](R) reader structure"]
impl crate::Readable for PADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`paddr::W`](W) writer structure"]
impl crate::Writable for PADDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PADDR to value 0"]
impl crate::Resettable for PADDR_SPEC {}
