#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Debug Sleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SleepDebugr {
    #[doc = "0: When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    Disabled = 0,
    #[doc = "1: When entering Sleep mode, all clocks keep running"]
    Enabled = 1,
}
impl From<SleepDebugr> for bool {
    #[inline(always)]
    fn from(variant: SleepDebugr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP_DEBUG` reader - Debug Sleep mode control bit"]
pub type SLEEP_DEBUG_R = crate::BitReader<SleepDebugr>;
impl SLEEP_DEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SleepDebugr {
        match self.bits {
            false => SleepDebugr::Disabled,
            true => SleepDebugr::Enabled,
        }
    }
    #[doc = "When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SleepDebugr::Disabled
    }
    #[doc = "When entering Sleep mode, all clocks keep running"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SleepDebugr::Enabled
    }
}
#[doc = "Debug Sleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SleepDebugwWO {
    #[doc = "0: When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    Disable = 0,
    #[doc = "1: When entering Sleep mode, all clocks keep running"]
    Enable = 1,
}
impl From<SleepDebugwWO> for bool {
    #[inline(always)]
    fn from(variant: SleepDebugwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP_DEBUG` writer - Debug Sleep mode control bit"]
pub type SLEEP_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG, SleepDebugwWO>;
impl<'a, REG> SLEEP_DEBUG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When entering Sleep mode, CPU HCLK clock is disabled, but other clocks remain active"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SleepDebugwWO::Disable)
    }
    #[doc = "When entering Sleep mode, all clocks keep running"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SleepDebugwWO::Enable)
    }
}
#[doc = "Debug Deepsleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeepsleepDebugr {
    #[doc = "0: In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    Disabled = 0,
    #[doc = "1: In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    Enabled = 1,
}
impl From<DeepsleepDebugr> for bool {
    #[inline(always)]
    fn from(variant: DeepsleepDebugr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPSLEEP_DEBUG` reader - Debug Deepsleep mode control bit"]
pub type DEEPSLEEP_DEBUG_R = crate::BitReader<DeepsleepDebugr>;
impl DEEPSLEEP_DEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DeepsleepDebugr {
        match self.bits {
            false => DeepsleepDebugr::Disabled,
            true => DeepsleepDebugr::Enabled,
        }
    }
    #[doc = "In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DeepsleepDebugr::Disabled
    }
    #[doc = "In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DeepsleepDebugr::Enabled
    }
}
#[doc = "Debug Deepsleep mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeepsleepDebugwWO {
    #[doc = "0: In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    Disable = 0,
    #[doc = "1: In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    Enable = 1,
}
impl From<DeepsleepDebugwWO> for bool {
    #[inline(always)]
    fn from(variant: DeepsleepDebugwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPSLEEP_DEBUG` writer - Debug Deepsleep mode control bit"]
pub type DEEPSLEEP_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG, DeepsleepDebugwWO>;
impl<'a, REG> DEEPSLEEP_DEBUG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In Deepsleep mode, all clocks in the 1.2V domain are disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DeepsleepDebugwWO::Disable)
    }
    #[doc = "In Deepsleep mode, system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DeepsleepDebugwWO::Enable)
    }
}
#[doc = "Debug Standby mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StandbyDebugr {
    #[doc = "0: The whole 1.2V digital circuit is unpowered in Standby mode"]
    Disabled = 0,
    #[doc = "1: The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    Enabled = 1,
}
impl From<StandbyDebugr> for bool {
    #[inline(always)]
    fn from(variant: StandbyDebugr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STANDBY_DEBUG` reader - Debug Standby mode control bit"]
pub type STANDBY_DEBUG_R = crate::BitReader<StandbyDebugr>;
impl STANDBY_DEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StandbyDebugr {
        match self.bits {
            false => StandbyDebugr::Disabled,
            true => StandbyDebugr::Enabled,
        }
    }
    #[doc = "The whole 1.2V digital circuit is unpowered in Standby mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == StandbyDebugr::Disabled
    }
    #[doc = "The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == StandbyDebugr::Enabled
    }
}
#[doc = "Debug Standby mode control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StandbyDebugwWO {
    #[doc = "0: The whole 1.2V digital circuit is unpowered in Standby mode"]
    Disable = 0,
    #[doc = "1: The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    Enable = 1,
}
impl From<StandbyDebugwWO> for bool {
    #[inline(always)]
    fn from(variant: StandbyDebugwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STANDBY_DEBUG` writer - Debug Standby mode control bit"]
pub type STANDBY_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG, StandbyDebugwWO>;
impl<'a, REG> STANDBY_DEBUG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The whole 1.2V digital circuit is unpowered in Standby mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(StandbyDebugwWO::Disable)
    }
    #[doc = "The whole 1.2V digital circuit is not unpowered in Standby mode, and the system clock is provided by the internal RC oscillator (HICK)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(StandbyDebugwWO::Enable)
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sleep_debug", &self.sleep_debug())
            .field("deepsleep_debug", &self.deepsleep_debug())
            .field("standby_debug", &self.standby_debug())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Debug Sleep mode control bit"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_debug(&mut self) -> SLEEP_DEBUG_W<CTRL_SPEC> {
        SLEEP_DEBUG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Deepsleep mode control bit"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_debug(&mut self) -> DEEPSLEEP_DEBUG_W<CTRL_SPEC> {
        DEEPSLEEP_DEBUG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby mode control bit"]
    #[inline(always)]
    #[must_use]
    pub fn standby_debug(&mut self) -> STANDBY_DEBUG_W<CTRL_SPEC> {
        STANDBY_DEBUG_W::new(self, 2)
    }
}
#[doc = "DEBUG CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
