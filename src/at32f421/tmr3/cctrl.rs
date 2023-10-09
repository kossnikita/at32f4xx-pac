#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CCTRL_SPEC>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CCTRL_SPEC>;
#[doc = "Field `CEN[1-4]` reader - Channel %s enable"]
pub type CEN_R = crate::BitReader<C1ENR_A>;
#[doc = "Channel %s enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ENR_A {
    #[doc = "0: Channel is disabled"]
    Disabled = 0,
    #[doc = "1: Channel is enabled"]
    Enabled = 1,
}
impl From<C1ENR_A> for bool {
    #[inline(always)]
    fn from(variant: C1ENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1ENR_A {
        match self.bits {
            false => C1ENR_A::Disabled,
            true => C1ENR_A::Enabled,
        }
    }
    #[doc = "Channel is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1ENR_A::Disabled
    }
    #[doc = "Channel is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1ENR_A::Enabled
    }
}
#[doc = "Channel %s enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ENW_AW {
    #[doc = "0: Channel disable"]
    Disable = 0,
    #[doc = "1: Channel enable"]
    Enable = 1,
}
impl From<C1ENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1ENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN[1-4]` writer - Channel %s enable"]
pub type CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1ENW_AW>;
impl<'a, REG, const O: u8> CEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1ENW_AW::Disable)
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1ENW_AW::Enable)
    }
}
#[doc = "Field `CP[1-4]` reader - Channel %s polarity"]
pub type CP_R = crate::BitReader<C1P_A>;
#[doc = "Channel %s polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1P_A {
    #[doc = "0: Output active high or Input active rising edge"]
    High = 0,
    #[doc = "1: Output active low or Input active falling edge"]
    Low = 1,
}
impl From<C1P_A> for bool {
    #[inline(always)]
    fn from(variant: C1P_A) -> Self {
        variant as u8 != 0
    }
}
impl CP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1P_A {
        match self.bits {
            false => C1P_A::High,
            true => C1P_A::Low,
        }
    }
    #[doc = "Output active high or Input active rising edge"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == C1P_A::High
    }
    #[doc = "Output active low or Input active falling edge"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == C1P_A::Low
    }
}
#[doc = "Field `CP[1-4]` writer - Channel %s polarity"]
pub type CP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1P_A>;
impl<'a, REG, const O: u8> CP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output active high or Input active rising edge"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(C1P_A::High)
    }
    #[doc = "Output active low or Input active falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(C1P_A::Low)
    }
}
impl R {
    #[doc = "Channel [1-4]
enable"]
    #[inline(always)]
    pub unsafe fn cen(&self, n: u8) -> CEN_R {
        CEN_R::new(((self.bits >> ((n - 1) * 4)) & 1) != 0)
    }
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    pub fn c2en(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    pub fn c3en(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    pub fn c4en(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Channel [1-4]
polarity"]
    #[inline(always)]
    pub unsafe fn cp(&self, n: u8) -> CP_R {
        CP_R::new(((self.bits >> ((n - 1) * 4 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 polarity"]
    #[inline(always)]
    pub fn c1p(&self) -> CP_R {
        CP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 polarity"]
    #[inline(always)]
    pub fn c2p(&self) -> CP_R {
        CP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 polarity"]
    #[inline(always)]
    pub fn c3p(&self) -> CP_R {
        CP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 polarity"]
    #[inline(always)]
    pub fn c4p(&self) -> CP_R {
        CP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("c1p", &format_args!("{}", self.c1p().bit()))
            .field("c2p", &format_args!("{}", self.c2p().bit()))
            .field("c3p", &format_args!("{}", self.c3p().bit()))
            .field("c4p", &format_args!("{}", self.c4p().bit()))
            .field("c1en", &format_args!("{}", self.c1en().bit()))
            .field("c2en", &format_args!("{}", self.c2en().bit()))
            .field("c3en", &format_args!("{}", self.c3en().bit()))
            .field("c4en", &format_args!("{}", self.c4en().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Channel [1-4]
enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cen<const O: u8>(&mut self) -> CEN_W<CCTRL_SPEC, O> {
        CEN_W::new(self)
    }
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1en(&mut self) -> CEN_W<CCTRL_SPEC, 0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2en(&mut self) -> CEN_W<CCTRL_SPEC, 4> {
        CEN_W::new(self)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3en(&mut self) -> CEN_W<CCTRL_SPEC, 8> {
        CEN_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4en(&mut self) -> CEN_W<CCTRL_SPEC, 12> {
        CEN_W::new(self)
    }
    #[doc = "Channel [1-4]
polarity"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cp<const O: u8>(&mut self) -> CP_W<CCTRL_SPEC, O> {
        CP_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c1p(&mut self) -> CP_W<CCTRL_SPEC, 1> {
        CP_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c2p(&mut self) -> CP_W<CCTRL_SPEC, 5> {
        CP_W::new(self)
    }
    #[doc = "Bit 9 - Channel 3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c3p(&mut self) -> CP_W<CCTRL_SPEC, 9> {
        CP_W::new(self)
    }
    #[doc = "Bit 13 - Channel 4 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c4p(&mut self) -> CP_W<CCTRL_SPEC, 13> {
        CP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCTRL_SPEC;
impl crate::RegisterSpec for CCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
