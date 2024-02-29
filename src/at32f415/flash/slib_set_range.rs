#[doc = "Register `SLIB_SET_RANGE` writer"]
pub type W = crate::W<SLIB_SET_RANGE_SPEC>;
#[doc = "Field `SLIB_SS_SET` writer - sLib start sector setting"]
pub type SLIB_SS_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_DSS_SET` writer - sLib data start sector setting"]
pub type SLIB_DSS_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_ES_SET` writer - sLib end sector setting"]
pub type SLIB_ES_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl core::fmt::Debug for crate::generic::Reg<SLIB_SET_RANGE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:10 - sLib start sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ss_set(&mut self) -> SLIB_SS_SET_W<SLIB_SET_RANGE_SPEC> {
        SLIB_SS_SET_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - sLib data start sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_dss_set(&mut self) -> SLIB_DSS_SET_W<SLIB_SET_RANGE_SPEC> {
        SLIB_DSS_SET_W::new(self, 11)
    }
    #[doc = "Bits 22:31 - sLib end sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_es_set(&mut self) -> SLIB_ES_SET_W<SLIB_SET_RANGE_SPEC> {
        SLIB_ES_SET_W::new(self, 22)
    }
}
#[doc = "Configure sLib range register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_SET_RANGE_SPEC;
impl crate::RegisterSpec for SLIB_SET_RANGE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_range::W`](W) writer structure"]
impl crate::Writable for SLIB_SET_RANGE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_SET_RANGE to value 0"]
impl crate::Resettable for SLIB_SET_RANGE_SPEC {
    const RESET_VALUE: u32 = 0;
}
