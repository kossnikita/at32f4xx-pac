#[doc = "Register `S5M0ADDR` reader"]
pub type R = crate::R<S5M0ADDR_SPEC>;
#[doc = "Register `S5M0ADDR` writer"]
pub type W = crate::W<S5M0ADDR_SPEC>;
#[doc = "Field `M0ADDR` reader - Memory 0 address"]
pub type M0ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `M0ADDR` writer - Memory 0 address"]
pub type M0ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0addr(&self) -> M0ADDR_R {
        M0ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S5M0ADDR")
            .field("m0addr", &self.m0addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0addr(&mut self) -> M0ADDR_W<'_, S5M0ADDR_SPEC> {
        M0ADDR_W::new(self, 0)
    }
}
#[doc = "stream 5 memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s5m0addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s5m0addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S5M0ADDR_SPEC;
impl crate::RegisterSpec for S5M0ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s5m0addr::R`](R) reader structure"]
impl crate::Readable for S5M0ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s5m0addr::W`](W) writer structure"]
impl crate::Writable for S5M0ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S5M0ADDR to value 0"]
impl crate::Resettable for S5M0ADDR_SPEC {}
