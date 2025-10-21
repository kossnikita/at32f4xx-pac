#[doc = "Register `HCDMA2` reader"]
pub type R = crate::R<HCDMA2_SPEC>;
#[doc = "Register `HCDMA2` writer"]
pub type W = crate::W<HCDMA2_SPEC>;
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
        f.debug_struct("HCDMA2")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, HCDMA2_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Host channel 2 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA2_SPEC;
impl crate::RegisterSpec for HCDMA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma2::R`](R) reader structure"]
impl crate::Readable for HCDMA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma2::W`](W) writer structure"]
impl crate::Writable for HCDMA2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA2 to value 0"]
impl crate::Resettable for HCDMA2_SPEC {}
