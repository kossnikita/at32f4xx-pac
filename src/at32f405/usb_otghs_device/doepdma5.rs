#[doc = "Register `DOEPDMA5` reader"]
pub type R = crate::R<DOEPDMA5_SPEC>;
#[doc = "Register `DOEPDMA5` writer"]
pub type W = crate::W<DOEPDMA5_SPEC>;
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA5")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DOEPDMA5_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Device OUT Endpoint 5 DMA Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA5_SPEC;
impl crate::RegisterSpec for DOEPDMA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma5::R`](R) reader structure"]
impl crate::Readable for DOEPDMA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma5::W`](W) writer structure"]
impl crate::Writable for DOEPDMA5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMA5 to value 0"]
impl crate::Resettable for DOEPDMA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
