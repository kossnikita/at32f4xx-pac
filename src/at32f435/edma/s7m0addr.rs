#[doc = "Register `S7M0ADDR` reader"]
pub type R = crate::R<S7M0ADDR_SPEC>;
#[doc = "Register `S7M0ADDR` writer"]
pub type W = crate::W<S7M0ADDR_SPEC>;
#[doc = "Field `M0ADDR` reader - Memory 0 address"]
pub type M0ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `M0ADDR` writer - Memory 0 address"]
pub type M0ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0addr(&self) -> M0ADDR_R {
        M0ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    #[must_use]
    pub fn m0addr(&mut self) -> M0ADDR_W<S7M0ADDR_SPEC, 0> {
        M0ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "stream 7 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7m0addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7m0addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7M0ADDR_SPEC;
impl crate::RegisterSpec for S7M0ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7m0addr::R`](R) reader structure"]
impl crate::Readable for S7M0ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s7m0addr::W`](W) writer structure"]
impl crate::Writable for S7M0ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S7M0ADDR to value 0"]
impl crate::Resettable for S7M0ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
