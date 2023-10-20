#[doc = "Register `SLIB_SET_RANGE0` writer"]
pub type W = crate::W<SLIB_SET_RANGE0_SPEC>;
#[doc = "Field `SLIB_SS_SET` writer - sLib start sector setting"]
pub type SLIB_SS_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SLIB_ES_SET` writer - sLib end sector setting"]
pub type SLIB_ES_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<SLIB_SET_RANGE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - sLib start sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ss_set(&mut self) -> SLIB_SS_SET_W<SLIB_SET_RANGE0_SPEC> {
        SLIB_SS_SET_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - sLib end sector setting"]
    #[inline(always)]
    #[must_use]
    pub fn slib_es_set(&mut self) -> SLIB_ES_SET_W<SLIB_SET_RANGE0_SPEC> {
        SLIB_ES_SET_W::new(self, 16)
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
#[doc = "Configure sLib range register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_SET_RANGE0_SPEC;
impl crate::RegisterSpec for SLIB_SET_RANGE0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_range0::W`](W) writer structure"]
impl crate::Writable for SLIB_SET_RANGE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_SET_RANGE0 to value 0"]
impl crate::Resettable for SLIB_SET_RANGE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
