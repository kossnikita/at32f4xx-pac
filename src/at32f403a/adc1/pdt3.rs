#[doc = "Register `PDT3` reader"]
pub type R = crate::R<PDT3_SPEC>;
#[doc = "Field `PDT3` reader - Preempted data"]
pub type PDT3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt3(&self) -> PDT3_R {
        PDT3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Preempted data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDT3_SPEC;
impl crate::RegisterSpec for PDT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt3::R`](R) reader structure"]
impl crate::Readable for PDT3_SPEC {}
#[doc = "`reset()` method sets PDT3 to value 0"]
impl crate::Resettable for PDT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
