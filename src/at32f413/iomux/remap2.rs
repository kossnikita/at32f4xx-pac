#[doc = "Register `REMAP2` reader"]
pub type R = crate::R<REMAP2_SPEC>;
#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<REMAP2_SPEC>;
#[doc = "Field `EXT_SPIM_EN_MUX` reader - SPIM enable"]
pub type EXT_SPIM_EN_MUX_R = crate::BitReader;
#[doc = "Field `EXT_SPIM_EN_MUX` writer - SPIM enable"]
pub type EXT_SPIM_EN_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 21 - SPIM enable"]
    #[inline(always)]
    pub fn ext_spim_en_mux(&self) -> EXT_SPIM_EN_MUX_R {
        EXT_SPIM_EN_MUX_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP2")
            .field(
                "ext_spim_en_mux",
                &format_args!("{}", self.ext_spim_en_mux().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 21 - SPIM enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_spim_en_mux(&mut self) -> EXT_SPIM_EN_MUX_W<REMAP2_SPEC, 21> {
        EXT_SPIM_EN_MUX_W::new(self)
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
#[doc = "IO MUX remap register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP2_SPEC;
impl crate::RegisterSpec for REMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap2::R`](R) reader structure"]
impl crate::Readable for REMAP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap2::W`](W) writer structure"]
impl crate::Writable for REMAP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP2 to value 0"]
impl crate::Resettable for REMAP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
