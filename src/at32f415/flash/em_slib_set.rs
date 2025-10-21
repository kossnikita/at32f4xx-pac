#[doc = "Register `EM_SLIB_SET` writer"]
pub type W = crate::W<EM_SLIB_SET_SPEC>;
#[doc = "Field `EM_SLIB_SET` writer - Extension memory sLib setting"]
pub type EM_SLIB_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EM_SLIB_DSS_SET` writer - Extension memory sLib data start sector setting"]
pub type EM_SLIB_DSS_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<EM_SLIB_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Extension memory sLib setting"]
    #[inline(always)]
    pub fn em_slib_set(&mut self) -> EM_SLIB_SET_W<'_, EM_SLIB_SET_SPEC> {
        EM_SLIB_SET_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Extension memory sLib data start sector setting"]
    #[inline(always)]
    pub fn em_slib_dss_set(&mut self) -> EM_SLIB_DSS_SET_W<'_, EM_SLIB_SET_SPEC> {
        EM_SLIB_DSS_SET_W::new(self, 16)
    }
}
#[doc = "Extension momery slib set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em_slib_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM_SLIB_SET_SPEC;
impl crate::RegisterSpec for EM_SLIB_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`em_slib_set::W`](W) writer structure"]
impl crate::Writable for EM_SLIB_SET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM_SLIB_SET to value 0"]
impl crate::Resettable for EM_SLIB_SET_SPEC {}
