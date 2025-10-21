#[doc = "Register `DIEPDMA1` reader"]
pub type R = crate::R<DIEPDMA1_SPEC>;
#[doc = "Register `DIEPDMA1` writer"]
pub type W = crate::W<DIEPDMA1_SPEC>;
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
        f.debug_struct("DIEPDMA1")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, DIEPDMA1_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Device IN Endpoint 1 DMA Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMA1_SPEC;
impl crate::RegisterSpec for DIEPDMA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma1::R`](R) reader structure"]
impl crate::Readable for DIEPDMA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepdma1::W`](W) writer structure"]
impl crate::Writable for DIEPDMA1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPDMA1 to value 0"]
impl crate::Resettable for DIEPDMA1_SPEC {}
