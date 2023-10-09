#[doc = "Register `FCTRL` reader"]
pub type R = crate::R<FCTRL_SPEC>;
#[doc = "Register `FCTRL` writer"]
pub type W = crate::W<FCTRL_SPEC>;
#[doc = "Field `FCS` reader - Filters configure switch"]
pub type FCS_R = crate::BitReader<FCS_A>;
#[doc = "Filters configure switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCS_A {
    #[doc = "0: Filter bank is active"]
    Active = 0,
    #[doc = "1: Filter bank is in configuration mode"]
    Configuration = 1,
}
impl From<FCS_A> for bool {
    #[inline(always)]
    fn from(variant: FCS_A) -> Self {
        variant as u8 != 0
    }
}
impl FCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCS_A {
        match self.bits {
            false => FCS_A::Active,
            true => FCS_A::Configuration,
        }
    }
    #[doc = "Filter bank is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FCS_A::Active
    }
    #[doc = "Filter bank is in configuration mode"]
    #[inline(always)]
    pub fn is_configuration(&self) -> bool {
        *self == FCS_A::Configuration
    }
}
#[doc = "Field `FCS` writer - Filters configure switch"]
pub type FCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FCS_A>;
impl<'a, REG, const O: u8> FCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter bank is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(FCS_A::Active)
    }
    #[doc = "Filter bank is in configuration mode"]
    #[inline(always)]
    pub fn configuration(self) -> &'a mut crate::W<REG> {
        self.variant(FCS_A::Configuration)
    }
}
impl R {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCTRL")
            .field("fcs", &format_args!("{}", self.fcs().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<FCTRL_SPEC, 0> {
        FCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTRL_SPEC;
impl crate::RegisterSpec for FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRL to value 0"]
impl crate::Resettable for FCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
