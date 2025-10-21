#[doc = "Register `PTPTSAD` reader"]
pub type R = crate::R<PTPTSAD_SPEC>;
#[doc = "Register `PTPTSAD` writer"]
pub type W = crate::W<PTPTSAD_SPEC>;
#[doc = "Field `TAR` reader - Timestamp addend register"]
pub type TAR_R = crate::FieldReader<u32>;
#[doc = "Field `TAR` writer - Timestamp addend register"]
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp addend register"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSAD").field("tar", &self.tar()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp addend register"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<'_, PTPTSAD_SPEC> {
        TAR_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptsad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptsad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSAD_SPEC;
impl crate::RegisterSpec for PTPTSAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsad::R`](R) reader structure"]
impl crate::Readable for PTPTSAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptsad::W`](W) writer structure"]
impl crate::Writable for PTPTSAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PTPTSAD to value 0"]
impl crate::Resettable for PTPTSAD_SPEC {}
