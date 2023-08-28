#[doc = "Register `SLIB_SET_RANGE` writer"]
pub type W = crate::W<SLIB_SET_RANGE_SPEC>;
#[doc = "Field `SLIB_SS_SET` writer - sLib start sector setting,valid input: 0~511"]
pub type SLIB_SS_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `SLIB_DSS_SET` writer - sLib data start sector setting,valid input: 0~511, 0 means no data area"]
pub type SLIB_DSS_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `SLIB_ES_SET` writer - sLib end sector setting,valid input: 0~511"]
pub type SLIB_ES_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl W {
    #[doc = "Bits 0:10 - sLib start sector setting,valid input: 0~511"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ss_set(&mut self) -> SLIB_SS_SET_W<SLIB_SET_RANGE_SPEC, 0> {
        SLIB_SS_SET_W::new(self)
    }
    #[doc = "Bits 11:21 - sLib data start sector setting,valid input: 0~511, 0 means no data area"]
    #[inline(always)]
    #[must_use]
    pub fn slib_dss_set(&mut self) -> SLIB_DSS_SET_W<SLIB_SET_RANGE_SPEC, 11> {
        SLIB_DSS_SET_W::new(self)
    }
    #[doc = "Bits 22:31 - sLib end sector setting,valid input: 0~511"]
    #[inline(always)]
    #[must_use]
    pub fn slib_es_set(&mut self) -> SLIB_ES_SET_W<SLIB_SET_RANGE_SPEC, 22> {
        SLIB_ES_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure sLib range register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_SET_RANGE_SPEC;
impl crate::RegisterSpec for SLIB_SET_RANGE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_range::W`](W) writer structure"]
impl crate::Writable for SLIB_SET_RANGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_SET_RANGE to value 0"]
impl crate::Resettable for SLIB_SET_RANGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
