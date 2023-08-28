#[doc = "Register `DTLEN` reader"]
pub type R = crate::R<DTLEN_SPEC>;
#[doc = "Register `DTLEN` writer"]
pub type W = crate::W<DTLEN_SPEC>;
#[doc = "Field `DTLEN` reader - Data length value"]
pub type DTLEN_R = crate::FieldReader<u32>;
#[doc = "Field `DTLEN` writer - Data length value"]
pub type DTLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 25, O, u32>;
impl R {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn dtlen(&self) -> DTLEN_R {
        DTLEN_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    #[must_use]
    pub fn dtlen(&mut self) -> DTLEN_W<DTLEN_SPEC, 0> {
        DTLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTLEN_SPEC;
impl crate::RegisterSpec for DTLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtlen::R`](R) reader structure"]
impl crate::Readable for DTLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtlen::W`](W) writer structure"]
impl crate::Writable for DTLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTLEN to value 0"]
impl crate::Resettable for DTLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}