#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `TMREN` reader - TMR enable"]
pub type TMREN_R = crate::BitReader<TMRENR_A>;
#[doc = "TMR enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMRENR_A {
    #[doc = "0: Timer is disabled"]
    Disabled = 0,
    #[doc = "1: Timer is enabled"]
    Enabled = 1,
}
impl From<TMRENR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMRENR_A {
        match self.bits {
            false => TMRENR_A::Disabled,
            true => TMRENR_A::Enabled,
        }
    }
    #[doc = "Timer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TMRENR_A::Disabled
    }
    #[doc = "Timer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TMRENR_A::Enabled
    }
}
#[doc = "TMR enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMRENW_AW {
    #[doc = "0: Timer disable"]
    Disable = 0,
    #[doc = "1: Timer enable"]
    Enable = 1,
}
impl From<TMRENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TMRENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMREN` writer - TMR enable"]
pub type TMREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMRENW_AW>;
impl<'a, REG, const O: u8> TMREN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TMRENW_AW::Disable)
    }
    #[doc = "Timer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TMRENW_AW::Enable)
    }
}
#[doc = "Field `OVFEN` reader - Overflow event enable"]
pub type OVFEN_R = crate::BitReader<OVFENR_A>;
#[doc = "Overflow event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFENR_A {
    #[doc = "0: Overflow is disabled"]
    Disabled = 0,
    #[doc = "1: Overflow is enabled"]
    Enabled = 1,
}
impl From<OVFENR_A> for bool {
    #[inline(always)]
    fn from(variant: OVFENR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVFENR_A {
        match self.bits {
            false => OVFENR_A::Disabled,
            true => OVFENR_A::Enabled,
        }
    }
    #[doc = "Overflow is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVFENR_A::Disabled
    }
    #[doc = "Overflow is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVFENR_A::Enabled
    }
}
#[doc = "Overflow event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFENW_AW {
    #[doc = "0: Overflow disable"]
    Disable = 0,
    #[doc = "1: Overflow enable"]
    Enable = 1,
}
impl From<OVFENW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFEN` writer - Overflow event enable"]
pub type OVFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OVFENW_AW>;
impl<'a, REG, const O: u8> OVFEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OVFENW_AW::Disable)
    }
    #[doc = "Overflow enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OVFENW_AW::Enable)
    }
}
#[doc = "Field `OVFS` reader - Overflow event source"]
pub type OVFS_R = crate::BitReader<OVFS_A>;
#[doc = "Overflow event source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFS_A {
    #[doc = "0: Counter overflow, setting the OVFSWTR bit or overflow event generated by slave timer controller"]
    Any = 0,
    #[doc = "1: Only counter overflow generates an overflow event"]
    Counter = 1,
}
impl From<OVFS_A> for bool {
    #[inline(always)]
    fn from(variant: OVFS_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVFS_A {
        match self.bits {
            false => OVFS_A::Any,
            true => OVFS_A::Counter,
        }
    }
    #[doc = "Counter overflow, setting the OVFSWTR bit or overflow event generated by slave timer controller"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == OVFS_A::Any
    }
    #[doc = "Only counter overflow generates an overflow event"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == OVFS_A::Counter
    }
}
#[doc = "Field `OVFS` writer - Overflow event source"]
pub type OVFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OVFS_A>;
impl<'a, REG, const O: u8> OVFS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter overflow, setting the OVFSWTR bit or overflow event generated by slave timer controller"]
    #[inline(always)]
    pub fn any(self) -> &'a mut crate::W<REG> {
        self.variant(OVFS_A::Any)
    }
    #[doc = "Only counter overflow generates an overflow event"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(OVFS_A::Counter)
    }
}
#[doc = "Field `OCMEN` reader - One cycle mode enable"]
pub type OCMEN_R = crate::BitReader<OCMEN_A>;
#[doc = "One cycle mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCMEN_A {
    #[doc = "0: The counter does not stop at an update event"]
    Continuous = 0,
    #[doc = "1: The counter stops at an update event"]
    OneCycle = 1,
}
impl From<OCMEN_A> for bool {
    #[inline(always)]
    fn from(variant: OCMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OCMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCMEN_A {
        match self.bits {
            false => OCMEN_A::Continuous,
            true => OCMEN_A::OneCycle,
        }
    }
    #[doc = "The counter does not stop at an update event"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == OCMEN_A::Continuous
    }
    #[doc = "The counter stops at an update event"]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == OCMEN_A::OneCycle
    }
}
#[doc = "Field `OCMEN` writer - One cycle mode enable"]
pub type OCMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OCMEN_A>;
impl<'a, REG, const O: u8> OCMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter does not stop at an update event"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(OCMEN_A::Continuous)
    }
    #[doc = "The counter stops at an update event"]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(OCMEN_A::OneCycle)
    }
}
#[doc = "Field `PRBEN` reader - Period buffer enable"]
pub type PRBEN_R = crate::BitReader<PRBENR_A>;
#[doc = "Period buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRBENR_A {
    #[doc = "0: Period buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Period buffer is enabled"]
    Enabled = 1,
}
impl From<PRBENR_A> for bool {
    #[inline(always)]
    fn from(variant: PRBENR_A) -> Self {
        variant as u8 != 0
    }
}
impl PRBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRBENR_A {
        match self.bits {
            false => PRBENR_A::Disabled,
            true => PRBENR_A::Enabled,
        }
    }
    #[doc = "Period buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRBENR_A::Disabled
    }
    #[doc = "Period buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRBENR_A::Enabled
    }
}
#[doc = "Period buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRBENW_AW {
    #[doc = "0: Period buffer disable"]
    Disable = 0,
    #[doc = "1: Period buffer enable"]
    Enable = 1,
}
impl From<PRBENW_AW> for bool {
    #[inline(always)]
    fn from(variant: PRBENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRBEN` writer - Period buffer enable"]
pub type PRBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRBENW_AW>;
impl<'a, REG, const O: u8> PRBEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Period buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PRBENW_AW::Disable)
    }
    #[doc = "Period buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PRBENW_AW::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - TMR enable"]
    #[inline(always)]
    pub fn tmren(&self) -> TMREN_R {
        TMREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow event enable"]
    #[inline(always)]
    pub fn ovfen(&self) -> OVFEN_R {
        OVFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow event source"]
    #[inline(always)]
    pub fn ovfs(&self) -> OVFS_R {
        OVFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One cycle mode enable"]
    #[inline(always)]
    pub fn ocmen(&self) -> OCMEN_R {
        OCMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    pub fn prben(&self) -> PRBEN_R {
        PRBEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("prben", &format_args!("{}", self.prben().bit()))
            .field("ocmen", &format_args!("{}", self.ocmen().bit()))
            .field("ovfs", &format_args!("{}", self.ovfs().bit()))
            .field("ovfen", &format_args!("{}", self.ovfen().bit()))
            .field("tmren", &format_args!("{}", self.tmren().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - TMR enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmren(&mut self) -> TMREN_W<CTRL1_SPEC, 0> {
        TMREN_W::new(self)
    }
    #[doc = "Bit 1 - Overflow event enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfen(&mut self) -> OVFEN_W<CTRL1_SPEC, 1> {
        OVFEN_W::new(self)
    }
    #[doc = "Bit 2 - Overflow event source"]
    #[inline(always)]
    #[must_use]
    pub fn ovfs(&mut self) -> OVFS_W<CTRL1_SPEC, 2> {
        OVFS_W::new(self)
    }
    #[doc = "Bit 3 - One cycle mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocmen(&mut self) -> OCMEN_W<CTRL1_SPEC, 3> {
        OCMEN_W::new(self)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn prben(&mut self) -> PRBEN_W<CTRL1_SPEC, 7> {
        PRBEN_W::new(self)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
