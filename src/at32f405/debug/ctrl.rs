#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SLEEP_DEBUG` reader - Debug Sleep mode control bit"]
pub type SLEEP_DEBUG_R = crate::BitReader<SLEEP_DEBUGR_A>;
#[doc = "Debug Sleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_DEBUGR_A {
    #[doc = "0: When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    Disabled = 0,
    #[doc = "1: When entering Sleep mode, all clocks keep running"]
    Enabled = 1,
}
impl From<SLEEP_DEBUGR_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_DEBUGR_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_DEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_DEBUGR_A {
        match self.bits {
            false => SLEEP_DEBUGR_A::Disabled,
            true => SLEEP_DEBUGR_A::Enabled,
        }
    }
    #[doc = "When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLEEP_DEBUGR_A::Disabled
    }
    #[doc = "When entering Sleep mode, all clocks keep running"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEP_DEBUGR_A::Enabled
    }
}
#[doc = "Debug Sleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_DEBUGW_AW {
    #[doc = "0: When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    Disable = 0,
    #[doc = "1: When entering Sleep mode, all clocks keep running"]
    Enable = 1,
}
impl From<SLEEP_DEBUGW_AW> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_DEBUGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP_DEBUG` writer - Debug Sleep mode control bit"]
pub type SLEEP_DEBUG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLEEP_DEBUGW_AW>;
impl<'a, REG, const O: u8> SLEEP_DEBUG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_DEBUGW_AW::Disable)
    }
    #[doc = "When entering Sleep mode, all clocks keep running"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_DEBUGW_AW::Enable)
    }
}
#[doc = "Field `DEEPSLEEP_DEBUG` reader - Debug Deepsleep mode control bit"]
pub type DEEPSLEEP_DEBUG_R = crate::BitReader<DEEPSLEEP_DEBUGR_A>;
#[doc = "Debug Deepsleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPSLEEP_DEBUGR_A {
    #[doc = "0: In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    Disabled = 0,
    #[doc = "1: In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    Enabled = 1,
}
impl From<DEEPSLEEP_DEBUGR_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPSLEEP_DEBUGR_A) -> Self {
        variant as u8 != 0
    }
}
impl DEEPSLEEP_DEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPSLEEP_DEBUGR_A {
        match self.bits {
            false => DEEPSLEEP_DEBUGR_A::Disabled,
            true => DEEPSLEEP_DEBUGR_A::Enabled,
        }
    }
    #[doc = "In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEEPSLEEP_DEBUGR_A::Disabled
    }
    #[doc = "In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEEPSLEEP_DEBUGR_A::Enabled
    }
}
#[doc = "Debug Deepsleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEEPSLEEP_DEBUGW_AW {
    #[doc = "0: In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    Disable = 0,
    #[doc = "1: In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    Enable = 1,
}
impl From<DEEPSLEEP_DEBUGW_AW> for bool {
    #[inline(always)]
    fn from(variant: DEEPSLEEP_DEBUGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPSLEEP_DEBUG` writer - Debug Deepsleep mode control bit"]
pub type DEEPSLEEP_DEBUG_W<'a, REG, const O: u8> =
    crate::BitWriter<'a, REG, O, DEEPSLEEP_DEBUGW_AW>;
impl<'a, REG, const O: u8> DEEPSLEEP_DEBUG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPSLEEP_DEBUGW_AW::Disable)
    }
    #[doc = "In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPSLEEP_DEBUGW_AW::Enable)
    }
}
#[doc = "Field `STANDBY_DEBUG` reader - Debug Standby mode control bit"]
pub type STANDBY_DEBUG_R = crate::BitReader<STANDBY_DEBUGR_A>;
#[doc = "Debug Standby mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STANDBY_DEBUGR_A {
    #[doc = "0: The whole 1.2V digital circuit is unpowered in Standby mode"]
    Disabled = 0,
    #[doc = "1: The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    Enabled = 1,
}
impl From<STANDBY_DEBUGR_A> for bool {
    #[inline(always)]
    fn from(variant: STANDBY_DEBUGR_A) -> Self {
        variant as u8 != 0
    }
}
impl STANDBY_DEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STANDBY_DEBUGR_A {
        match self.bits {
            false => STANDBY_DEBUGR_A::Disabled,
            true => STANDBY_DEBUGR_A::Enabled,
        }
    }
    #[doc = "The whole 1.2V digital circuit is unpowered in Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STANDBY_DEBUGR_A::Disabled
    }
    #[doc = "The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STANDBY_DEBUGR_A::Enabled
    }
}
#[doc = "Debug Standby mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STANDBY_DEBUGW_AW {
    #[doc = "0: The whole 1.2V digital circuit is unpowered in Standby mode"]
    Disable = 0,
    #[doc = "1: The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    Enable = 1,
}
impl From<STANDBY_DEBUGW_AW> for bool {
    #[inline(always)]
    fn from(variant: STANDBY_DEBUGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STANDBY_DEBUG` writer - Debug Standby mode control bit"]
pub type STANDBY_DEBUG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STANDBY_DEBUGW_AW>;
impl<'a, REG, const O: u8> STANDBY_DEBUG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The whole 1.2V digital circuit is unpowered in Standby mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(STANDBY_DEBUGW_AW::Disable)
    }
    #[doc = "The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(STANDBY_DEBUGW_AW::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Sleep mode control bit"]
    #[inline(always)]
    pub fn sleep_debug(&self) -> SLEEP_DEBUG_R {
        SLEEP_DEBUG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Deepsleep mode control bit"]
    #[inline(always)]
    pub fn deepsleep_debug(&self) -> DEEPSLEEP_DEBUG_R {
        DEEPSLEEP_DEBUG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby mode control bit"]
    #[inline(always)]
    pub fn standby_debug(&self) -> STANDBY_DEBUG_R {
        STANDBY_DEBUG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Sleep mode control bit"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_debug(&mut self) -> SLEEP_DEBUG_W<CTRL_SPEC, 0> {
        SLEEP_DEBUG_W::new(self)
    }
    #[doc = "Bit 1 - Debug Deepsleep mode control bit"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_debug(&mut self) -> DEEPSLEEP_DEBUG_W<CTRL_SPEC, 1> {
        DEEPSLEEP_DEBUG_W::new(self)
    }
    #[doc = "Bit 2 - Debug Standby mode control bit"]
    #[inline(always)]
    #[must_use]
    pub fn standby_debug(&mut self) -> STANDBY_DEBUG_W<CTRL_SPEC, 2> {
        STANDBY_DEBUG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DEBUG CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
