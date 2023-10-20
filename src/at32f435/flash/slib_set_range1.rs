#[doc = "Register `SLIB_SET_RANGE1` writer"]
pub type W = crate::W<SLIB_SET_RANGE1_SPEC>;
#[doc = "Field `SLIB_ISS_SET` writer - sLib instruction start sector setting"]
pub type SLIB_ISS_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SET_SLIB_STRT` writer - sLib start setting"]
pub type SET_SLIB_STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SLIB_SET_RANGE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - sLib instruction start sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_iss_set(&mut self) -> SLIB_ISS_SET_W<SLIB_SET_RANGE1_SPEC> {
        SLIB_ISS_SET_W::new(self, 0)
    }
    #[doc = "Bit 31 - sLib start setting"]
    #[inline(always)]
    #[must_use]
    pub fn set_slib_strt(&mut self) -> SET_SLIB_STRT_W<SLIB_SET_RANGE1_SPEC> {
        SET_SLIB_STRT_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
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
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_SET_RANGE1 to value 0"]
impl crate::Resettable for SLIB_SET_RANGE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
