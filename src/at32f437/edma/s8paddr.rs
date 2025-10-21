#[doc = "Register `S8PADDR` reader"]
pub type R = crate::R<S8PADDR_SPEC>;
#[doc = "Register `S8PADDR` writer"]
pub type W = crate::W<S8PADDR_SPEC>;
#[doc = "Field `PADDR` reader - Peripheral address"]
pub type PADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral address"]
pub type PADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S8PADDR")
            .field("paddr", &self.paddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PADDR_W<'_, S8PADDR_SPEC> {
        PADDR_W::new(self, 0)
    }
}
#[doc = "stream 8 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s8paddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s8paddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S8PADDR_SPEC;
impl crate::RegisterSpec for S8PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s8paddr::R`](R) reader structure"]
impl crate::Readable for S8PADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s8paddr::W`](W) writer structure"]
impl crate::Writable for S8PADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S8PADDR to value 0"]
impl crate::Resettable for S8PADDR_SPEC {}
