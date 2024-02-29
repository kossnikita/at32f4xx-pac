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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODT")
            .field("odt", &format_args!("{}", self.odt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ODT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
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
    const RESET_VALUE: u32 = 0;
}
