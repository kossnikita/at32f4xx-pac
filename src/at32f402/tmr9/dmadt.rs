#[doc = "Register `DMADT` reader"]
pub type R = crate::R<DMADT_SPEC>;
#[doc = "Register `DMADT` writer"]
pub type W = crate::W<DMADT_SPEC>;
#[doc = "Field `DMADT` reader - DMA data register"]
pub type DMADT_R = crate::FieldReader<u16>;
#[doc = "Field `DMADT` writer - DMA data register"]
pub type DMADT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA data register"]
    #[inline(always)]
    pub fn dmadt(&self) -> DMADT_R {
        DMADT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMADT")
            .field("dmadt", &self.dmadt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA data register"]
    #[inline(always)]
    pub fn dmadt(&mut self) -> DMADT_W<'_, DMADT_SPEC> {
        DMADT_W::new(self, 0)
    }
}
#[doc = "DMA data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmadt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmadt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMADT_SPEC;
impl crate::RegisterSpec for DMADT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadt::R`](R) reader structure"]
impl crate::Readable for DMADT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmadt::W`](W) writer structure"]
impl crate::Writable for DMADT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMADT to value 0"]
impl crate::Resettable for DMADT_SPEC {}
