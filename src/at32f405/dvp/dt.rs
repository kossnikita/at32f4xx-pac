#[doc = "Register `DT` reader"]
pub type R = crate::R<DT_SPEC>;
#[doc = "Field `DT` reader - Data Port"]
pub type DT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Port"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(self.bits)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT_SPEC;
impl crate::RegisterSpec for DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DT_SPEC {}
#[doc = "`reset()` method sets DT to value 0"]
impl crate::Resettable for DT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
