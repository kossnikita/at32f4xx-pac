#[doc = "Register `PDT4` reader"]
pub type R = crate::R<PDT4_SPEC>;
#[doc = "Field `PDT4` reader - Preempted data"]
pub type PDT4_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Preempted data"]
    #[inline(always)]
    pub fn pdt4(&self) -> PDT4_R {
        PDT4_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Preempted data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDT4_SPEC;
impl crate::RegisterSpec for PDT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdt4::R`](R) reader structure"]
impl crate::Readable for PDT4_SPEC {}
#[doc = "`reset()` method sets PDT4 to value 0"]
impl crate::Resettable for PDT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
