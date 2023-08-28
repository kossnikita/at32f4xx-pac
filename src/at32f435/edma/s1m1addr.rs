#[doc = "Register `S1M1ADDR` reader"]
pub type R = crate::R<S1M1ADDR_SPEC>;
#[doc = "Register `S1M1ADDR` writer"]
pub type W = crate::W<S1M1ADDR_SPEC>;
#[doc = "Field `M1ADDR` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `M1ADDR` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1addr(&self) -> M1ADDR_R {
        M1ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn m1addr(&mut self) -> M1ADDR_W<S1M1ADDR_SPEC, 0> {
        M1ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "stream 1 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1m1addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1m1addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1M1ADDR_SPEC;
impl crate::RegisterSpec for S1M1ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s1m1addr::R`](R) reader structure"]
impl crate::Readable for S1M1ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s1m1addr::W`](W) writer structure"]
impl crate::Writable for S1M1ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S1M1ADDR to value 0"]
impl crate::Resettable for S1M1ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}