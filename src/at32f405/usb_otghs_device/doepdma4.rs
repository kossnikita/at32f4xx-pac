#[doc = "Register `DOEPDMA4` reader"]
pub type R = crate::R<DOEPDMA4_SPEC>;
#[doc = "Register `DOEPDMA4` writer"]
pub type W = crate::W<DOEPDMA4_SPEC>;
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
        f.debug_struct("DOEPDMA4")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, DOEPDMA4_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Device OUT Endpoint 4 DMA Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMA4_SPEC;
impl crate::RegisterSpec for DOEPDMA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdma4::R`](R) reader structure"]
impl crate::Readable for DOEPDMA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdma4::W`](W) writer structure"]
impl crate::Writable for DOEPDMA4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPDMA4 to value 0"]
impl crate::Resettable for DOEPDMA4_SPEC {}
