#[doc = "Register `ODT` reader"]
pub type R = crate::R<ODT_SPEC>;
#[doc = "Field `ODT` reader - Conversion data of ordinary channel"]
pub type ODT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Conversion data of ordinary channel"]
    #[inline(always)]
    pub fn odt(&self) -> ODT_R {
        ODT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODT_SPEC;
impl crate::RegisterSpec for ODT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for ODT_SPEC {}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for ODT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}