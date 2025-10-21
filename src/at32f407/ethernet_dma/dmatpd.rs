#[doc = "Register `DMATPD` reader"]
pub type R = crate::R<DMATPD_SPEC>;
#[doc = "Register `DMATPD` writer"]
pub type W = crate::W<DMATPD_SPEC>;
#[doc = "Field `TPD` reader - Transmit poll demand"]
pub type TPD_R = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATPD").field("tpd", &self.tpd()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<'_, DMATPD_SPEC> {
        TPD_W::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATPD_SPEC;
impl crate::RegisterSpec for DMATPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatpd::R`](R) reader structure"]
impl crate::Readable for DMATPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatpd::W`](W) writer structure"]
impl crate::Writable for DMATPD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATPD to value 0"]
impl crate::Resettable for DMATPD_SPEC {}
