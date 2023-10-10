#[doc = "Register `CTRLL` reader"]
pub type R = crate::R<CTRLL_SPEC>;
#[doc = "Register `CTRLL` writer"]
pub type W = crate::W<CTRLL_SPEC>;
#[doc = "Field `TSF` reader - Time second flag"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Time second flag"]
pub type TSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAF` reader - Time alarm flag"]
pub type TAF_R = crate::BitReader;
#[doc = "Field `TAF` writer - Time alarm flag"]
pub type TAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVFF` reader - Overflow Flag"]
pub type OVFF_R = crate::BitReader;
#[doc = "Field `OVFF` writer - Overflow Flag"]
pub type OVFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPDF` reader - RTC update finish"]
pub type UPDF_R = crate::BitReader;
#[doc = "Field `UPDF` writer - RTC update finish"]
pub type UPDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFGEN` reader - RTC configuration enable"]
pub type CFGEN_R = crate::BitReader;
#[doc = "Field `CFGEN` writer - RTC configuration enable"]
pub type CFGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFGF` reader - RTC configuration finish"]
pub type CFGF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Time second flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time alarm flag"]
    #[inline(always)]
    pub fn taf(&self) -> TAF_R {
        TAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC update finish"]
    #[inline(always)]
    pub fn updf(&self) -> UPDF_R {
        UPDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC configuration enable"]
    #[inline(always)]
    pub fn cfgen(&self) -> CFGEN_R {
        CFGEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC configuration finish"]
    #[inline(always)]
    pub fn cfgf(&self) -> CFGF_R {
        CFGF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLL")
            .field("tsf", &format_args!("{}", self.tsf().bit()))
            .field("taf", &format_args!("{}", self.taf().bit()))
            .field("ovff", &format_args!("{}", self.ovff().bit()))
            .field("updf", &format_args!("{}", self.updf().bit()))
            .field("cfgen", &format_args!("{}", self.cfgen().bit()))
            .field("cfgf", &format_args!("{}", self.cfgf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRLL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Time second flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<CTRLL_SPEC, 0> {
        TSF_W::new(self)
    }
    #[doc = "Bit 1 - Time alarm flag"]
    #[inline(always)]
    #[must_use]
    pub fn taf(&mut self) -> TAF_W<CTRLL_SPEC, 1> {
        TAF_W::new(self)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovff(&mut self) -> OVFF_W<CTRLL_SPEC, 2> {
        OVFF_W::new(self)
    }
    #[doc = "Bit 3 - RTC update finish"]
    #[inline(always)]
    #[must_use]
    pub fn updf(&mut self) -> UPDF_W<CTRLL_SPEC, 3> {
        UPDF_W::new(self)
    }
    #[doc = "Bit 4 - RTC configuration enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfgen(&mut self) -> CFGEN_W<CTRLL_SPEC, 4> {
        CFGEN_W::new(self)
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
#[doc = "RTC Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLL_SPEC;
impl crate::RegisterSpec for CTRLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrll::R`](R) reader structure"]
impl crate::Readable for CTRLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrll::W`](W) writer structure"]
impl crate::Writable for CTRLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLL to value 0x20"]
impl crate::Resettable for CTRLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
