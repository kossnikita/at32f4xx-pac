#[doc = "Register `HCDMA6` reader"]
pub type R = crate::R<HCDMA6_SPEC>;
#[doc = "Register `HCDMA6` writer"]
pub type W = crate::W<HCDMA6_SPEC>;
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DMAADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA6_SPEC, 0> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host channel 6 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA6_SPEC;
impl crate::RegisterSpec for HCDMA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma6::R`](R) reader structure"]
impl crate::Readable for HCDMA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma6::W`](W) writer structure"]
impl crate::Writable for HCDMA6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA6 to value 0"]
impl crate::Resettable for HCDMA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
