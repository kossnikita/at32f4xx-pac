#[doc = "Register `HCDMA15` reader"]
pub type R = crate::R<HCDMA15_SPEC>;
#[doc = "Register `HCDMA15` writer"]
pub type W = crate::W<HCDMA15_SPEC>;
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
        f.debug_struct("HCDMA15")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, HCDMA15_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Host channel 15 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA15_SPEC;
impl crate::RegisterSpec for HCDMA15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma15::R`](R) reader structure"]
impl crate::Readable for HCDMA15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma15::W`](W) writer structure"]
impl crate::Writable for HCDMA15_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDMA15 to value 0"]
impl crate::Resettable for HCDMA15_SPEC {}
