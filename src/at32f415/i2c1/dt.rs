#[doc = "Register `DT` reader"]
pub type R = crate::R<DT_SPEC>;
#[doc = "Register `DT` writer"]
pub type W = crate::W<DT_SPEC>;
#[doc = "Field `DT` reader - data register"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - data register"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - data register"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT").field("dt", &self.dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - data register"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, DT_SPEC> {
        DT_W::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DT to value 0"]
impl crate::Resettable for DT_SPEC {}
