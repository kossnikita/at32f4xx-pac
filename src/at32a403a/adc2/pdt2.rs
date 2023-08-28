#[doc = "Register `PDT2` reader"]
pub type R = crate::R<PDT2_SPEC>;
#[doc = "Field `PDT2` reader - Preempted data"]
pub type PDT2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt2(&self) -> PDT2_R {
        PDT2_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Preempted data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDT2_SPEC;
impl crate::RegisterSpec for PDT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt2::R`](R) reader structure"]
impl crate::Readable for PDT2_SPEC {}
#[doc = "`reset()` method sets PDT2 to value 0"]
impl crate::Resettable for PDT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
