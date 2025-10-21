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
#[doc = "WDT_PAUSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_PAUSE_A {
    #[doc = "0: The counter clock of TMRx is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The counter clock of TMRx is stopped when the core is halted"]
    Pause = 1,
}
impl From<WDT_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_PAUSE` reader - WDT_PAUSE"]
pub type WDT_PAUSE_R = crate::BitReader<WDT_PAUSE_A>;
impl WDT_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_PAUSE_A {
        match self.bits {
            false => WDT_PAUSE_A::Continue,
            true => WDT_PAUSE_A::Pause,
        }
    }
    #[doc = "The counter clock of TMRx is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == WDT_PAUSE_A::Continue
    }
    #[doc = "The counter clock of TMRx is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == WDT_PAUSE_A::Pause
    }
}
#[doc = "Field `WDT_PAUSE` writer - WDT_PAUSE"]
pub type WDT_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG, WDT_PAUSE_A>;
impl<'a, REG> WDT_PAUSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter clock of TMRx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_PAUSE_A::Continue)
    }
    #[doc = "The counter clock of TMRx is stopped when the core is halted"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_PAUSE_A::Pause)
    }
}
#[doc = "WWDT_PAUSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT_PAUSE_A {
    #[doc = "0: The counter clock of TMRx is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The counter clock of TMRx is stopped when the core is halted"]
    Pause = 1,
}
impl From<WWDT_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT_PAUSE` reader - WWDT_PAUSE"]
pub type WWDT_PAUSE_R = crate::BitReader<WWDT_PAUSE_A>;
impl WWDT_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDT_PAUSE_A {
        match self.bits {
            false => WWDT_PAUSE_A::Continue,
            true => WWDT_PAUSE_A::Pause,
        }
    }
    #[doc = "The counter clock of TMRx is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == WWDT_PAUSE_A::Continue
    }
    #[doc = "The counter clock of TMRx is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == WWDT_PAUSE_A::Pause
    }
}
#[doc = "Field `WWDT_PAUSE` writer - WWDT_PAUSE"]
pub type WWDT_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG, WWDT_PAUSE_A>;
impl<'a, REG> WWDT_PAUSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter clock of TMRx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(WWDT_PAUSE_A::Continue)
    }
    #[doc = "The counter clock of TMRx is stopped when the core is halted"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut crate::W<REG> {
        self.variant(WWDT_PAUSE_A::Pause)
    }
}
#[doc = "Timer pause control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR1_PAUSE_A {
    #[doc = "0: The counter clock of TMRx is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The counter clock of TMRx is stopped when the core is halted"]
    Pause = 1,
}
impl From<TMR1_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMR1_PAUSE` reader - Timer pause control bit"]
pub type TMR1_PAUSE_R = crate::BitReader<TMR1_PAUSE_A>;
impl TMR1_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR1_PAUSE_A {
        match self.bits {
            false => TMR1_PAUSE_A::Continue,
            true => TMR1_PAUSE_A::Pause,
        }
    }
    #[doc = "The counter clock of TMRx is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == TMR1_PAUSE_A::Continue
    }
    #[doc = "The counter clock of TMRx is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == TMR1_PAUSE_A::Pause
    }
}
#[doc = "Field `TMR1_PAUSE` writer - Timer pause control bit"]
pub type TMR1_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG, TMR1_PAUSE_A>;
impl<'a, REG> TMR1_PAUSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter clock of TMRx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_PAUSE_A::Continue)
    }
    #[doc = "The counter clock of TMRx is stopped when the core is halted"]
    #[inline(always)]
    pub fn pause(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_PAUSE_A::Pause)
    }
}
#[doc = "Field `TMR3_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR3_PAUSE_R;
#[doc = "Field `TMR3_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR3_PAUSE_W;
#[doc = "Field `ERTC_PAUSE` reader - ERTC_PAUSE"]
pub type ERTC_PAUSE_R = crate::BitReader;
#[doc = "Field `ERTC_PAUSE` writer - ERTC_PAUSE"]
pub type ERTC_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR6_PAUSE_R;
#[doc = "Field `TMR6_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR6_PAUSE_W;
#[doc = "Field `ERTC_512_PAUSE` reader - ERTC_512_PAUSE"]
pub type ERTC_512_PAUSE_R = crate::BitReader;
#[doc = "Field `ERTC_512_PAUSE` writer - ERTC_512_PAUSE"]
pub type ERTC_512_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR15_PAUSE_R;
#[doc = "Field `TMR16_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR16_PAUSE_R;
#[doc = "Field `TMR17_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR17_PAUSE_R;
#[doc = "Field `TMR14_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR14_PAUSE_R;
#[doc = "Field `TMR15_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR15_PAUSE_W;
#[doc = "Field `TMR16_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR16_PAUSE_W;
#[doc = "Field `TMR17_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR17_PAUSE_W;
#[doc = "Field `TMR14_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR14_PAUSE_W;
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
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    pub fn wdt_pause(&self) -> WDT_PAUSE_R {
        WDT_PAUSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    pub fn wwdt_pause(&self) -> WWDT_PAUSE_R {
        WWDT_PAUSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr1_pause(&self) -> TMR1_PAUSE_R {
        TMR1_PAUSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr3_pause(&self) -> TMR3_PAUSE_R {
        TMR3_PAUSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - ERTC_PAUSE"]
    #[inline(always)]
    pub fn ertc_pause(&self) -> ERTC_PAUSE_R {
        ERTC_PAUSE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2C1_SMBUS_TIMEOUT_R {
        I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2C2_SMBUS_TIMEOUT_R {
        I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr6_pause(&self) -> TMR6_PAUSE_R {
        TMR6_PAUSE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - ERTC_512_PAUSE"]
    #[inline(always)]
    pub fn ertc_512_pause(&self) -> ERTC_512_PAUSE_R {
        ERTC_512_PAUSE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr15_pause(&self) -> TMR15_PAUSE_R {
        TMR15_PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr16_pause(&self) -> TMR16_PAUSE_R {
        TMR16_PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr17_pause(&self) -> TMR17_PAUSE_R {
        TMR17_PAUSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr14_pause(&self) -> TMR14_PAUSE_R {
        TMR14_PAUSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sleep_debug", &self.sleep_debug())
            .field("deepsleep_debug", &self.deepsleep_debug())
            .field("standby_debug", &self.standby_debug())
            .field("wdt_pause", &self.wdt_pause())
            .field("wwdt_pause", &self.wwdt_pause())
            .field("tmr1_pause", &self.tmr1_pause())
            .field("tmr3_pause", &self.tmr3_pause())
            .field("ertc_pause", &self.ertc_pause())
            .field("i2c1_smbus_timeout", &self.i2c1_smbus_timeout())
            .field("i2c2_smbus_timeout", &self.i2c2_smbus_timeout())
            .field("tmr6_pause", &self.tmr6_pause())
            .field("ertc_512_pause", &self.ertc_512_pause())
            .field("tmr15_pause", &self.tmr15_pause())
            .field("tmr16_pause", &self.tmr16_pause())
            .field("tmr17_pause", &self.tmr17_pause())
            .field("tmr14_pause", &self.tmr14_pause())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Debug Sleep mode control bit"]
    #[inline(always)]
    pub fn sleep_debug(&mut self) -> SLEEP_DEBUG_W<'_, CTRL_SPEC> {
        SLEEP_DEBUG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Deepsleep mode control bit"]
    #[inline(always)]
    pub fn deepsleep_debug(&mut self) -> DEEPSLEEP_DEBUG_W<'_, CTRL_SPEC> {
        DEEPSLEEP_DEBUG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby mode control bit"]
    #[inline(always)]
    pub fn standby_debug(&mut self) -> STANDBY_DEBUG_W<'_, CTRL_SPEC> {
        STANDBY_DEBUG_W::new(self, 2)
    }
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    pub fn wdt_pause(&mut self) -> WDT_PAUSE_W<'_, CTRL_SPEC> {
        WDT_PAUSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    pub fn wwdt_pause(&mut self) -> WWDT_PAUSE_W<'_, CTRL_SPEC> {
        WWDT_PAUSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr1_pause(&mut self) -> TMR1_PAUSE_W<'_, CTRL_SPEC> {
        TMR1_PAUSE_W::new(self, 10)
    }
    #[doc = "Bit 12 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr3_pause(&mut self) -> TMR3_PAUSE_W<'_, CTRL_SPEC> {
        TMR3_PAUSE_W::new(self, 12)
    }
    #[doc = "Bit 14 - ERTC_PAUSE"]
    #[inline(always)]
    pub fn ertc_pause(&mut self) -> ERTC_PAUSE_W<'_, CTRL_SPEC> {
        ERTC_PAUSE_W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&mut self) -> I2C1_SMBUS_TIMEOUT_W<'_, CTRL_SPEC> {
        I2C1_SMBUS_TIMEOUT_W::new(self, 15)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&mut self) -> I2C2_SMBUS_TIMEOUT_W<'_, CTRL_SPEC> {
        I2C2_SMBUS_TIMEOUT_W::new(self, 16)
    }
    #[doc = "Bit 19 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr6_pause(&mut self) -> TMR6_PAUSE_W<'_, CTRL_SPEC> {
        TMR6_PAUSE_W::new(self, 19)
    }
    #[doc = "Bit 21 - ERTC_512_PAUSE"]
    #[inline(always)]
    pub fn ertc_512_pause(&mut self) -> ERTC_512_PAUSE_W<'_, CTRL_SPEC> {
        ERTC_512_PAUSE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr15_pause(&mut self) -> TMR15_PAUSE_W<'_, CTRL_SPEC> {
        TMR15_PAUSE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr16_pause(&mut self) -> TMR16_PAUSE_W<'_, CTRL_SPEC> {
        TMR16_PAUSE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr17_pause(&mut self) -> TMR17_PAUSE_W<'_, CTRL_SPEC> {
        TMR17_PAUSE_W::new(self, 24)
    }
    #[doc = "Bit 27 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr14_pause(&mut self) -> TMR14_PAUSE_W<'_, CTRL_SPEC> {
        TMR14_PAUSE_W::new(self, 27)
    }
}
#[doc = "DEBUG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {}
