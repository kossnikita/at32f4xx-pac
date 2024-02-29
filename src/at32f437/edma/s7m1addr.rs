#[doc = "Register `S7M1ADDR` reader"]
pub type R = crate::R<S7M1ADDR_SPEC>;
#[doc = "Register `S7M1ADDR` writer"]
pub type W = crate::W<S7M1ADDR_SPEC>;
#[doc = "Field `M1ADDR` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `M1ADDR` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1addr(&self) -> M1ADDR_R {
        M1ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S7M1ADDR")
            .field("m1addr", &format_args!("{}", self.m1addr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S7M1ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn m1addr(&mut self) -> M1ADDR_W<S7M1ADDR_SPEC> {
        M1ADDR_W::new(self, 0)
    }
}
#[doc = "stream 7 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7m1addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7m1addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7M1ADDR_SPEC;
impl crate::RegisterSpec for S7M1ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7m1addr::R`](R) reader structure"]
impl crate::Readable for S7M1ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s7m1addr::W`](W) writer structure"]
impl crate::Writable for S7M1ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S7M1ADDR to value 0"]
impl crate::Resettable for S7M1ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
