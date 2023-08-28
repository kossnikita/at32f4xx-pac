#[doc = "Register `S1LLP` reader"]
pub type R = crate::R<S1LLP_SPEC>;
#[doc = "Register `S1LLP` writer"]
pub type W = crate::W<S1LLP_SPEC>;
#[doc = "Field `LLP` reader - Link list pointer"]
pub type LLP_R = crate::FieldReader<u32>;
#[doc = "Field `LLP` writer - Link list pointer"]
pub type LLP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    pub fn llp(&self) -> LLP_R {
        LLP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    #[must_use]
    pub fn llp(&mut self) -> LLP_W<S1LLP_SPEC, 0> {
        LLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Stream 1 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1llp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1llp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1LLP_SPEC;
impl crate::RegisterSpec for S1LLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s1llp::R`](R) reader structure"]
impl crate::Readable for S1LLP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s1llp::W`](W) writer structure"]
impl crate::Writable for S1LLP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S1LLP to value 0"]
impl crate::Resettable for S1LLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}