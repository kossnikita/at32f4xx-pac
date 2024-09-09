#[doc = "Register `HCDMA4` reader"]
pub type R = crate::R<HCDMA4_SPEC>;
#[doc = "Register `HCDMA4` writer"]
pub type W = crate::W<HCDMA4_SPEC>;
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
        f.debug_struct("HCDMA4")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA4_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Host channel 4 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA4_SPEC;
impl crate::RegisterSpec for HCDMA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma4::R`](R) reader structure"]
impl crate::Readable for HCDMA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma4::W`](W) writer structure"]
impl crate::Writable for HCDMA4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA4 to value 0"]
impl crate::Resettable for HCDMA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
