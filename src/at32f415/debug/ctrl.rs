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
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TRACE_IOEN_R = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TRACE_MODE_R = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
#[doc = "Field `TMR2_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR2_PAUSE_R;
#[doc = "Field `TMR3_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR3_PAUSE_R;
#[doc = "Field `TMR4_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR4_PAUSE_R;
#[doc = "Field `TMR2_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR2_PAUSE_W;
#[doc = "Field `TMR3_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR3_PAUSE_W;
#[doc = "Field `TMR4_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR4_PAUSE_W;
#[doc = "Field `CAN1_PAUSE` reader - CAN1_PAUSE"]
pub type CAN1_PAUSE_R = crate::BitReader;
#[doc = "Field `CAN1_PAUSE` writer - CAN1_PAUSE"]
pub type CAN1_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR5_PAUSE_R;
#[doc = "Field `TMR9_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR9_PAUSE_R;
#[doc = "Field `TMR10_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR10_PAUSE_R;
#[doc = "Field `TMR11_PAUSE` reader - Timer pause control bit"]
pub use TMR1_PAUSE_R as TMR11_PAUSE_R;
#[doc = "Field `TMR5_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR5_PAUSE_W;
#[doc = "Field `TMR9_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR9_PAUSE_W;
#[doc = "Field `TMR10_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR10_PAUSE_W;
#[doc = "Field `TMR11_PAUSE` writer - Timer pause control bit"]
pub use TMR1_PAUSE_W as TMR11_PAUSE_W;
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
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
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
    #[doc = "Bit 11 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr2_pause(&self) -> TMR2_PAUSE_R {
        TMR2_PAUSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr3_pause(&self) -> TMR3_PAUSE_R {
        TMR3_PAUSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr4_pause(&self) -> TMR4_PAUSE_R {
        TMR4_PAUSE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN1_PAUSE"]
    #[inline(always)]
    pub fn can1_pause(&self) -> CAN1_PAUSE_R {
        CAN1_PAUSE_R::new(((self.bits >> 14) & 1) != 0)
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
    #[doc = "Bit 18 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr5_pause(&self) -> TMR5_PAUSE_R {
        TMR5_PAUSE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr9_pause(&self) -> TMR9_PAUSE_R {
        TMR9_PAUSE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr10_pause(&self) -> TMR10_PAUSE_R {
        TMR10_PAUSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Timer pause control bit"]
    #[inline(always)]
    pub fn tmr11_pause(&self) -> TMR11_PAUSE_R {
        TMR11_PAUSE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sleep_debug", &self.sleep_debug())
            .field("deepsleep_debug", &self.deepsleep_debug())
            .field("standby_debug", &self.standby_debug())
            .field("trace_ioen", &self.trace_ioen())
            .field("trace_mode", &self.trace_mode())
            .field("wdt_pause", &self.wdt_pause())
            .field("wwdt_pause", &self.wwdt_pause())
            .field("tmr1_pause", &self.tmr1_pause())
            .field("tmr2_pause", &self.tmr2_pause())
            .field("tmr3_pause", &self.tmr3_pause())
            .field("tmr4_pause", &self.tmr4_pause())
            .field("can1_pause", &self.can1_pause())
            .field("i2c1_smbus_timeout", &self.i2c1_smbus_timeout())
            .field("i2c2_smbus_timeout", &self.i2c2_smbus_timeout())
            .field("tmr5_pause", &self.tmr5_pause())
            .field("tmr9_pause", &self.tmr9_pause())
            .field("tmr10_pause", &self.tmr10_pause())
            .field("tmr11_pause", &self.tmr11_pause())
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
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CTRL_SPEC> {
        TRACE_IOEN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CTRL_SPEC> {
        TRACE_MODE_W::new(self, 6)
    }
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause(&mut self) -> WDT_PAUSE_W<CTRL_SPEC> {
        WDT_PAUSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt_pause(&mut self) -> WWDT_PAUSE_W<CTRL_SPEC> {
        WWDT_PAUSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_pause(&mut self) -> TMR1_PAUSE_W<CTRL_SPEC> {
        TMR1_PAUSE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_pause(&mut self) -> TMR2_PAUSE_W<CTRL_SPEC> {
        TMR2_PAUSE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_pause(&mut self) -> TMR3_PAUSE_W<CTRL_SPEC> {
        TMR3_PAUSE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4_pause(&mut self) -> TMR4_PAUSE_W<CTRL_SPEC> {
        TMR4_PAUSE_W::new(self, 13)
    }
    #[doc = "Bit 14 - CAN1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn can1_pause(&mut self) -> CAN1_PAUSE_W<CTRL_SPEC> {
        CAN1_PAUSE_W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_smbus_timeout(&mut self) -> I2C1_SMBUS_TIMEOUT_W<CTRL_SPEC> {
        I2C1_SMBUS_TIMEOUT_W::new(self, 15)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_smbus_timeout(&mut self) -> I2C2_SMBUS_TIMEOUT_W<CTRL_SPEC> {
        I2C2_SMBUS_TIMEOUT_W::new(self, 16)
    }
    #[doc = "Bit 18 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5_pause(&mut self) -> TMR5_PAUSE_W<CTRL_SPEC> {
        TMR5_PAUSE_W::new(self, 18)
    }
    #[doc = "Bit 28 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_pause(&mut self) -> TMR9_PAUSE_W<CTRL_SPEC> {
        TMR9_PAUSE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_pause(&mut self) -> TMR10_PAUSE_W<CTRL_SPEC> {
        TMR10_PAUSE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Timer pause control bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_pause(&mut self) -> TMR11_PAUSE_W<CTRL_SPEC> {
        TMR11_PAUSE_W::new(self, 30)
    }
}
#[doc = "MCUDBG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
