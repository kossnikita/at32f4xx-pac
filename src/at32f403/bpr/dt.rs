#[doc = "Register `DT%s` reader"]
pub type R = crate::R<DT_SPEC>;
#[doc = "Register `DT%s` writer"]
pub type W = crate::W<DT_SPEC>;
#[doc = "Field `DT` reader - BPR data"]
pub type DT_R = crate::FieldReader<u16>;
#[doc = "Field `DT` writer - BPR data"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - BPR data"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT").field("dt", &self.dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, DT_SPEC> {
        DT_W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT_SPEC;
impl crate::RegisterSpec for DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt::W`](W) writer structure"]
impl crate::Writable for DT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT%s to value 0"]
impl crate::Resettable for DT_SPEC {}
