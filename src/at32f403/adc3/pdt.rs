#[doc = "Register `PDT%s` reader"]
pub type R = crate::R<PDT_SPEC>;
#[doc = "Field `PDT` reader - Preempted data"]
pub type PDT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt(&self) -> PDT_R {
        PDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDT").field("pdt", &self.pdt()).finish()
    }
}
#[doc = "Preempted data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pdt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDT_SPEC;
impl crate::RegisterSpec for PDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt::R`](R) reader structure"]
impl crate::Readable for PDT_SPEC {}
#[doc = "`reset()` method sets PDT%s to value 0"]
impl crate::Resettable for PDT_SPEC {
    const RESET_VALUE: u32 = 0;
}
