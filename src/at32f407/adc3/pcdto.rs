#[doc = "Register `PCDTO%s` reader"]
pub type R = crate::R<PCDTO_SPEC>;
#[doc = "Register `PCDTO%s` writer"]
pub type W = crate::W<PCDTO_SPEC>;
#[doc = "Field `DTO` reader - Data offset for Preempted channel 1"]
pub type DTO_R = crate::FieldReader<u16>;
#[doc = "Field `DTO` writer - Data offset for Preempted channel 1"]
pub type DTO_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCDTO").field("dto", &self.dto()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn dto(&mut self) -> DTO_W<PCDTO_SPEC> {
        DTO_W::new(self, 0)
    }
}
#[doc = "Data offset for Preempted channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pcdto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDTO_SPEC;
impl crate::RegisterSpec for PCDTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto::R`](R) reader structure"]
impl crate::Readable for PCDTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcdto::W`](W) writer structure"]
impl crate::Writable for PCDTO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCDTO%s to value 0"]
impl crate::Resettable for PCDTO_SPEC {
    const RESET_VALUE: u32 = 0;
}
