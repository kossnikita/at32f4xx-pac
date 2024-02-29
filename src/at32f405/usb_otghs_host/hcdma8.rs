#[doc = "Register `HCDMA8` reader"]
pub type R = crate::R<HCDMA8_SPEC>;
#[doc = "Register `HCDMA8` writer"]
pub type W = crate::W<HCDMA8_SPEC>;
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
        f.debug_struct("HCDMA8")
            .field("dmaaddr", &format_args!("{}", self.dmaaddr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HCDMA8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA8_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "Host channel 8 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA8_SPEC;
impl crate::RegisterSpec for HCDMA8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma8::R`](R) reader structure"]
impl crate::Readable for HCDMA8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma8::W`](W) writer structure"]
impl crate::Writable for HCDMA8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA8 to value 0"]
impl crate::Resettable for HCDMA8_SPEC {
    const RESET_VALUE: u32 = 0;
}
