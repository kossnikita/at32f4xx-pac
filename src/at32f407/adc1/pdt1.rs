#[doc = "Register `PDT1` reader"]
pub type R = crate::R<PDT1_SPEC>;
#[doc = "Field `PDT1` reader - Preempted data"]
pub type PDT1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt1(&self) -> PDT1_R {
        PDT1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Preempted data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDT1_SPEC;
impl crate::RegisterSpec for PDT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt1::R`](R) reader structure"]
impl crate::Readable for PDT1_SPEC {}
#[doc = "`reset()` method sets PDT1 to value 0"]
impl crate::Resettable for PDT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
