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
    pub const fn variant(&self) -> SLEEP_DEBUGR_A {
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
pub type SLEEP_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG, SLEEP_DEBUGW_AW>;
impl<'a, REG> SLEEP_DEBUG_W<'a, REG>
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
    pub const fn variant(&self) -> DEEPSLEEP_DEBUGR_A {
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
pub type DEEPSLEEP_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG, DEEPSLEEP_DEBUGW_AW>;
impl<'a, REG> DEEPSLEEP_DEBUG_W<'a, REG>
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
    pub const fn variant(&self) -> STANDBY_DEBUGR_A {
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
pub type STANDBY_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG, STANDBY_DEBUGW_AW>;
impl<'a, REG> STANDBY_DEBUG_W<'a, REG>
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
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TRACE_IOEN_R = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TRACE_MODE_R = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDT_PAUSE` reader - WDT_PAUSE"]
pub type WDT_PAUSE_R = crate::BitReader;
#[doc = "Field `WDT_PAUSE` writer - WDT_PAUSE"]
pub type WDT_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT_PAUSE` reader - WWDT_PAUSE"]
pub type WWDT_PAUSE_R = crate::BitReader;
#[doc = "Field `WWDT_PAUSE` writer - WWDT_PAUSE"]
pub type WWDT_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1_PAUSE` reader - TMR1_PAUSE"]
pub type TMR1_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR1_PAUSE` writer - TMR1_PAUSE"]
pub type TMR1_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2_PAUSE` reader - TMR2_PAUSE"]
pub type TMR2_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR2_PAUSE` writer - TMR2_PAUSE"]
pub type TMR2_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3_PAUSE` reader - TMR3_PAUSE"]
pub type TMR3_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR3_PAUSE` writer - TMR3_PAUSE"]
pub type TMR3_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4_PAUSE` reader - TMR4_PAUSE"]
pub type TMR4_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR4_PAUSE` writer - TMR4_PAUSE"]
pub type TMR4_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `TMR8_PAUSE` reader - TMR8_PAUSE"]
pub type TMR8_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR8_PAUSE` writer - TMR8_PAUSE"]
pub type TMR8_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5_PAUSE` reader - TMR5_PAUSE"]
pub type TMR5_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR5_PAUSE` writer - TMR5_PAUSE"]
pub type TMR5_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6_PAUSE` reader - TMR6_PAUSE"]
pub type TMR6_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR6_PAUSE` writer - TMR6_PAUSE"]
pub type TMR6_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7_PAUSE` reader - TMR7_PAUSE"]
pub type TMR7_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR7_PAUSE` writer - TMR7_PAUSE"]
pub type TMR7_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15_PAUSE` reader - TMR15_PAUSE"]
pub type TMR15_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR15_PAUSE` writer - TMR15_PAUSE"]
pub type TMR15_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR12_PAUSE` reader - TMR12_PAUSE"]
pub type TMR12_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR12_PAUSE` writer - TMR12_PAUSE"]
pub type TMR12_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13_PAUSE` reader - TMR13_PAUSE"]
pub type TMR13_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR13_PAUSE` writer - TMR13_PAUSE"]
pub type TMR13_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14_PAUSE` reader - TMR14_PAUSE"]
pub type TMR14_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR14_PAUSE` writer - TMR14_PAUSE"]
pub type TMR14_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9_PAUSE` reader - TMR9_PAUSE"]
pub type TMR9_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR9_PAUSE` writer - TMR9_PAUSE"]
pub type TMR9_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10_PAUSE` reader - TMR10_PAUSE"]
pub type TMR10_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR10_PAUSE` writer - TMR10_PAUSE"]
pub type TMR10_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11_PAUSE` reader - TMR11_PAUSE"]
pub type TMR11_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR11_PAUSE` writer - TMR11_PAUSE"]
pub type TMR11_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` reader - I2C3_SMBUS_TIMEOUT"]
pub type I2C3_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` writer - I2C3_SMBUS_TIMEOUT"]
pub type I2C3_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    pub fn tmr1_pause(&self) -> TMR1_PAUSE_R {
        TMR1_PAUSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TMR2_PAUSE"]
    #[inline(always)]
    pub fn tmr2_pause(&self) -> TMR2_PAUSE_R {
        TMR2_PAUSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
    #[inline(always)]
    pub fn tmr3_pause(&self) -> TMR3_PAUSE_R {
        TMR3_PAUSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TMR4_PAUSE"]
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
    #[doc = "Bit 17 - TMR8_PAUSE"]
    #[inline(always)]
    pub fn tmr8_pause(&self) -> TMR8_PAUSE_R {
        TMR8_PAUSE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TMR5_PAUSE"]
    #[inline(always)]
    pub fn tmr5_pause(&self) -> TMR5_PAUSE_R {
        TMR5_PAUSE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    pub fn tmr6_pause(&self) -> TMR6_PAUSE_R {
        TMR6_PAUSE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TMR7_PAUSE"]
    #[inline(always)]
    pub fn tmr7_pause(&self) -> TMR7_PAUSE_R {
        TMR7_PAUSE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - TMR15_PAUSE"]
    #[inline(always)]
    pub fn tmr15_pause(&self) -> TMR15_PAUSE_R {
        TMR15_PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - TMR12_PAUSE"]
    #[inline(always)]
    pub fn tmr12_pause(&self) -> TMR12_PAUSE_R {
        TMR12_PAUSE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TMR13_PAUSE"]
    #[inline(always)]
    pub fn tmr13_pause(&self) -> TMR13_PAUSE_R {
        TMR13_PAUSE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    pub fn tmr14_pause(&self) -> TMR14_PAUSE_R {
        TMR14_PAUSE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TMR9_PAUSE"]
    #[inline(always)]
    pub fn tmr9_pause(&self) -> TMR9_PAUSE_R {
        TMR9_PAUSE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TMR10_PAUSE"]
    #[inline(always)]
    pub fn tmr10_pause(&self) -> TMR10_PAUSE_R {
        TMR10_PAUSE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TMR11_PAUSE"]
    #[inline(always)]
    pub fn tmr11_pause(&self) -> TMR11_PAUSE_R {
        TMR11_PAUSE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c3_smbus_timeout(&self) -> I2C3_SMBUS_TIMEOUT_R {
        I2C3_SMBUS_TIMEOUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sleep_debug", &format_args!("{}", self.sleep_debug().bit()))
            .field(
                "deepsleep_debug",
                &format_args!("{}", self.deepsleep_debug().bit()),
            )
            .field(
                "standby_debug",
                &format_args!("{}", self.standby_debug().bit()),
            )
            .field("trace_ioen", &format_args!("{}", self.trace_ioen().bit()))
            .field("trace_mode", &format_args!("{}", self.trace_mode().bits()))
            .field("wdt_pause", &format_args!("{}", self.wdt_pause().bit()))
            .field("wwdt_pause", &format_args!("{}", self.wwdt_pause().bit()))
            .field("tmr1_pause", &format_args!("{}", self.tmr1_pause().bit()))
            .field("tmr2_pause", &format_args!("{}", self.tmr2_pause().bit()))
            .field("tmr3_pause", &format_args!("{}", self.tmr3_pause().bit()))
            .field("tmr4_pause", &format_args!("{}", self.tmr4_pause().bit()))
            .field("can1_pause", &format_args!("{}", self.can1_pause().bit()))
            .field(
                "i2c1_smbus_timeout",
                &format_args!("{}", self.i2c1_smbus_timeout().bit()),
            )
            .field(
                "i2c2_smbus_timeout",
                &format_args!("{}", self.i2c2_smbus_timeout().bit()),
            )
            .field("tmr8_pause", &format_args!("{}", self.tmr8_pause().bit()))
            .field("tmr5_pause", &format_args!("{}", self.tmr5_pause().bit()))
            .field("tmr6_pause", &format_args!("{}", self.tmr6_pause().bit()))
            .field("tmr7_pause", &format_args!("{}", self.tmr7_pause().bit()))
            .field("tmr15_pause", &format_args!("{}", self.tmr15_pause().bit()))
            .field("tmr12_pause", &format_args!("{}", self.tmr12_pause().bit()))
            .field("tmr13_pause", &format_args!("{}", self.tmr13_pause().bit()))
            .field("tmr14_pause", &format_args!("{}", self.tmr14_pause().bit()))
            .field("tmr9_pause", &format_args!("{}", self.tmr9_pause().bit()))
            .field("tmr10_pause", &format_args!("{}", self.tmr10_pause().bit()))
            .field("tmr11_pause", &format_args!("{}", self.tmr11_pause().bit()))
            .field(
                "i2c3_smbus_timeout",
                &format_args!("{}", self.i2c3_smbus_timeout().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_pause(&mut self) -> TMR1_PAUSE_W<CTRL_SPEC> {
        TMR1_PAUSE_W::new(self, 10)
    }
    #[doc = "Bit 11 - TMR2_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_pause(&mut self) -> TMR2_PAUSE_W<CTRL_SPEC> {
        TMR2_PAUSE_W::new(self, 11)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_pause(&mut self) -> TMR3_PAUSE_W<CTRL_SPEC> {
        TMR3_PAUSE_W::new(self, 12)
    }
    #[doc = "Bit 13 - TMR4_PAUSE"]
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
    #[doc = "Bit 17 - TMR8_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8_pause(&mut self) -> TMR8_PAUSE_W<CTRL_SPEC> {
        TMR8_PAUSE_W::new(self, 17)
    }
    #[doc = "Bit 18 - TMR5_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5_pause(&mut self) -> TMR5_PAUSE_W<CTRL_SPEC> {
        TMR5_PAUSE_W::new(self, 18)
    }
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6_pause(&mut self) -> TMR6_PAUSE_W<CTRL_SPEC> {
        TMR6_PAUSE_W::new(self, 19)
    }
    #[doc = "Bit 20 - TMR7_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7_pause(&mut self) -> TMR7_PAUSE_W<CTRL_SPEC> {
        TMR7_PAUSE_W::new(self, 20)
    }
    #[doc = "Bit 22 - TMR15_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15_pause(&mut self) -> TMR15_PAUSE_W<CTRL_SPEC> {
        TMR15_PAUSE_W::new(self, 22)
    }
    #[doc = "Bit 25 - TMR12_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr12_pause(&mut self) -> TMR12_PAUSE_W<CTRL_SPEC> {
        TMR12_PAUSE_W::new(self, 25)
    }
    #[doc = "Bit 26 - TMR13_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13_pause(&mut self) -> TMR13_PAUSE_W<CTRL_SPEC> {
        TMR13_PAUSE_W::new(self, 26)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14_pause(&mut self) -> TMR14_PAUSE_W<CTRL_SPEC> {
        TMR14_PAUSE_W::new(self, 27)
    }
    #[doc = "Bit 28 - TMR9_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_pause(&mut self) -> TMR9_PAUSE_W<CTRL_SPEC> {
        TMR9_PAUSE_W::new(self, 28)
    }
    #[doc = "Bit 29 - TMR10_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_pause(&mut self) -> TMR10_PAUSE_W<CTRL_SPEC> {
        TMR10_PAUSE_W::new(self, 29)
    }
    #[doc = "Bit 30 - TMR11_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_pause(&mut self) -> TMR11_PAUSE_W<CTRL_SPEC> {
        TMR11_PAUSE_W::new(self, 30)
    }
    #[doc = "Bit 31 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_smbus_timeout(&mut self) -> I2C3_SMBUS_TIMEOUT_W<CTRL_SPEC> {
        I2C3_SMBUS_TIMEOUT_W::new(self, 31)
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
#[doc = "DEBUG_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
