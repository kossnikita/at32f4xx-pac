#[doc = "Register `D1ODT` reader"]
pub type R = crate::R<D1ODT_SPEC>;
#[doc = "Field `D1ODT` reader - DAC1 data output"]
pub type D1ODT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 data output"]
    #[inline(always)]
    pub fn d1odt(&self) -> D1ODT_R {
        D1ODT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC1 data output register (DAC_D1ODT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1odt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1ODT_SPEC;
impl crate::RegisterSpec for D1ODT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1odt::R`](R) reader structure"]
impl crate::Readable for D1ODT_SPEC {}
#[doc = "`reset()` method sets D1ODT to value 0"]
impl crate::Resettable for D1ODT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
