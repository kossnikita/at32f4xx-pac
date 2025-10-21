#[doc = "Register `S6M1ADDR` reader"]
pub type R = crate::R<S6M1ADDR_SPEC>;
#[doc = "Register `S6M1ADDR` writer"]
pub type W = crate::W<S6M1ADDR_SPEC>;
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
        f.debug_struct("S6M1ADDR")
            .field("m1addr", &self.m1addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1addr(&mut self) -> M1ADDR_W<'_, S6M1ADDR_SPEC> {
        M1ADDR_W::new(self, 0)
    }
}
#[doc = "stream 6 memory 1 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s6m1addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s6m1addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6M1ADDR_SPEC;
impl crate::RegisterSpec for S6M1ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6m1addr::R`](R) reader structure"]
impl crate::Readable for S6M1ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6m1addr::W`](W) writer structure"]
impl crate::Writable for S6M1ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S6M1ADDR to value 0"]
impl crate::Resettable for S6M1ADDR_SPEC {}
