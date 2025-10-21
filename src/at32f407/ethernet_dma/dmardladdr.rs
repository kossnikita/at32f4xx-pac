#[doc = "Register `DMARDLADDR` reader"]
pub type R = crate::R<DMARDLADDR_SPEC>;
#[doc = "Register `DMARDLADDR` writer"]
pub type W = crate::W<DMARDLADDR_SPEC>;
#[doc = "Field `SRL` reader - Start of receive list"]
pub type SRL_R = crate::FieldReader<u32>;
#[doc = "Field `SRL` writer - Start of receive list"]
pub type SRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARDLADDR")
            .field("srl", &self.srl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W<'_, DMARDLADDR_SPEC> {
        SRL_W::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmardladdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardladdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARDLADDR_SPEC;
impl crate::RegisterSpec for DMARDLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardladdr::R`](R) reader structure"]
impl crate::Readable for DMARDLADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmardladdr::W`](W) writer structure"]
impl crate::Writable for DMARDLADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARDLADDR to value 0"]
impl crate::Resettable for DMARDLADDR_SPEC {}
