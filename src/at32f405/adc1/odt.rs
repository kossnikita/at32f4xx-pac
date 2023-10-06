#[doc = "Register `ODT` reader"]
pub type R = crate::R<ODT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ODT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODT_SPEC;
impl crate::RegisterSpec for ODT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for ODT_SPEC {}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for ODT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
