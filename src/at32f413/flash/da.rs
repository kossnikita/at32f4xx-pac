#[doc = "Register `DA` writer"]
pub type W = crate::W<DA_SPEC>;
#[doc = "Field `FDA` writer - Flash decryption address"]
pub type FDA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<DA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash decryption address"]
    #[inline(always)]
    #[must_use]
    pub fn fda(&mut self) -> FDA_W<DA_SPEC> {
        FDA_W::new(self, 0)
    }
}
#[doc = "Spim decryption address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`da::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DA_SPEC;
impl crate::RegisterSpec for DA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`da::W`](W) writer structure"]
impl crate::Writable for DA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DA to value 0"]
impl crate::Resettable for DA_SPEC {
    const RESET_VALUE: u32 = 0;
}
