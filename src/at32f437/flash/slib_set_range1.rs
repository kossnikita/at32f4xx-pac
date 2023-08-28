#[doc = "Register `SLIB_SET_RANGE1` writer"]
pub type W = crate::W<SLIB_SET_RANGE1_SPEC>;
#[doc = "Field `SLIB_ISS_SET` writer - sLib instruction start sector setting"]
pub type SLIB_ISS_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `SET_SLIB_STRT` writer - sLib start setting"]
pub type SET_SLIB_STRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:15 - sLib instruction start sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_iss_set(&mut self) -> SLIB_ISS_SET_W<SLIB_SET_RANGE1_SPEC, 0> {
        SLIB_ISS_SET_W::new(self)
    }
    #[doc = "Bit 31 - sLib start setting"]
    #[inline(always)]
    #[must_use]
    pub fn set_slib_strt(&mut self) -> SET_SLIB_STRT_W<SLIB_SET_RANGE1_SPEC, 31> {
        SET_SLIB_STRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure sLib range register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_SET_RANGE1_SPEC;
impl crate::RegisterSpec for SLIB_SET_RANGE1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_range1::W`](W) writer structure"]
impl crate::Writable for SLIB_SET_RANGE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_SET_RANGE1 to value 0"]
impl crate::Resettable for SLIB_SET_RANGE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
