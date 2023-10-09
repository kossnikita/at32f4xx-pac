#[doc = "Register `S6LLP` reader"]
pub type R = crate::R<S6LLP_SPEC>;
#[doc = "Register `S6LLP` writer"]
pub type W = crate::W<S6LLP_SPEC>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6LLP")
            .field("llp", &format_args!("{}", self.llp().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S6LLP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    #[must_use]
    pub fn llp(&mut self) -> LLP_W<S6LLP_SPEC, 0> {
        LLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Stream 6 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6llp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6llp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6LLP_SPEC;
impl crate::RegisterSpec for S6LLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6llp::R`](R) reader structure"]
impl crate::Readable for S6LLP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6llp::W`](W) writer structure"]
impl crate::Writable for S6LLP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S6LLP to value 0"]
impl crate::Resettable for S6LLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
