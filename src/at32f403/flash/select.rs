#[doc = "Register `SELECT` writer"]
pub type W = crate::W<SELECT_SPEC>;
#[doc = "Field `SELECT` writer - spim type selection"]
pub type SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SELECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - spim type selection"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<SELECT_SPEC> {
        SELECT_W::new(self, 0)
    }
}
#[doc = "Select register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`select::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SELECT_SPEC;
impl crate::RegisterSpec for SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`select::W`](W) writer structure"]
impl crate::Writable for SELECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SELECT to value 0"]
impl crate::Resettable for SELECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
