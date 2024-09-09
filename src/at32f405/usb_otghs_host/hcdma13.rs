#[doc = "Register `HCDMA13` reader"]
pub type R = crate::R<HCDMA13_SPEC>;
#[doc = "Register `HCDMA13` writer"]
pub type W = crate::W<HCDMA13_SPEC>;
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
        f.debug_struct("HCDMA13")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA13_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Host channel 13 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA13_SPEC;
impl crate::RegisterSpec for HCDMA13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma13::R`](R) reader structure"]
impl crate::Readable for HCDMA13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma13::W`](W) writer structure"]
impl crate::Writable for HCDMA13_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA13 to value 0"]
impl crate::Resettable for HCDMA13_SPEC {
    const RESET_VALUE: u32 = 0;
}
