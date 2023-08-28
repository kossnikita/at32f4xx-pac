#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SLEEP_DEBUG` reader - SLEEP_DEBUG"]
pub type SLEEP_DEBUG_R = crate::BitReader;
#[doc = "Field `SLEEP_DEBUG` writer - SLEEP_DEBUG"]
pub type SLEEP_DEBUG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEEPSLEEP_DEBUG` reader - DEEPSLEEP_DEBUG"]
pub type DEEPSLEEP_DEBUG_R = crate::BitReader;
#[doc = "Field `DEEPSLEEP_DEBUG` writer - DEEPSLEEP_DEBUG"]
pub type DEEPSLEEP_DEBUG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STANDBY_DEBUG` reader - STANDBY_DEBUG"]
pub type STANDBY_DEBUG_R = crate::BitReader;
#[doc = "Field `STANDBY_DEBUG` writer - STANDBY_DEBUG"]
pub type STANDBY_DEBUG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_PAUSE` reader - WDT_PAUSE"]
pub type WDT_PAUSE_R = crate::BitReader;
#[doc = "Field `WDT_PAUSE` writer - WDT_PAUSE"]
pub type WDT_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT_PAUSE` reader - WWDT_PAUSE"]
pub type WWDT_PAUSE_R = crate::BitReader;
#[doc = "Field `WWDT_PAUSE` writer - WWDT_PAUSE"]
pub type WWDT_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR1_PAUSE` reader - TMR1_PAUSE"]
pub type TMR1_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR1_PAUSE` writer - TMR1_PAUSE"]
pub type TMR1_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR3_PAUSE` reader - TMR3_PAUSE"]
pub type TMR3_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR3_PAUSE` writer - TMR3_PAUSE"]
pub type TMR3_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERTC_PAUSE` reader - ERTC_PAUSE"]
pub type ERTC_PAUSE_R = crate::BitReader;
#[doc = "Field `ERTC_PAUSE` writer - ERTC_PAUSE"]
pub type ERTC_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR6_PAUSE` reader - TMR6_PAUSE"]
pub type TMR6_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR6_PAUSE` writer - TMR6_PAUSE"]
pub type TMR6_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERTC_512_PAUSE` reader - ERTC_512_PAUSE"]
pub type ERTC_512_PAUSE_R = crate::BitReader;
#[doc = "Field `ERTC_512_PAUSE` writer - ERTC_512_PAUSE"]
pub type ERTC_512_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR15_PAUSE` reader - TMR15_PAUSE"]
pub type TMR15_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR15_PAUSE` writer - TMR15_PAUSE"]
pub type TMR15_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR16_PAUSE` reader - TMR16_PAUSE"]
pub type TMR16_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR16_PAUSE` writer - TMR16_PAUSE"]
pub type TMR16_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR17_PAUSE` reader - TMR17_PAUSE"]
pub type TMR17_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR17_PAUSE` writer - TMR17_PAUSE"]
pub type TMR17_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR14_PAUSE` reader - TMR14_PAUSE"]
pub type TMR14_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR14_PAUSE` writer - TMR14_PAUSE"]
pub type TMR14_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    pub fn sleep_debug(&self) -> SLEEP_DEBUG_R {
        SLEEP_DEBUG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    pub fn deepsleep_debug(&self) -> DEEPSLEEP_DEBUG_R {
        DEEPSLEEP_DEBUG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
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
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    pub fn tmr1_pause(&self) -> TMR1_PAUSE_R {
        TMR1_PAUSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
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
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    pub fn tmr6_pause(&self) -> TMR6_PAUSE_R {
        TMR6_PAUSE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - ERTC_512_PAUSE"]
    #[inline(always)]
    pub fn ertc_512_pause(&self) -> ERTC_512_PAUSE_R {
        ERTC_512_PAUSE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TMR15_PAUSE"]
    #[inline(always)]
    pub fn tmr15_pause(&self) -> TMR15_PAUSE_R {
        TMR15_PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TMR16_PAUSE"]
    #[inline(always)]
    pub fn tmr16_pause(&self) -> TMR16_PAUSE_R {
        TMR16_PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TMR17_PAUSE"]
    #[inline(always)]
    pub fn tmr17_pause(&self) -> TMR17_PAUSE_R {
        TMR17_PAUSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    pub fn tmr14_pause(&self) -> TMR14_PAUSE_R {
        TMR14_PAUSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_debug(&mut self) -> SLEEP_DEBUG_W<CTRL_SPEC, 0> {
        SLEEP_DEBUG_W::new(self)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_debug(&mut self) -> DEEPSLEEP_DEBUG_W<CTRL_SPEC, 1> {
        DEEPSLEEP_DEBUG_W::new(self)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
    #[inline(always)]
    #[must_use]
    pub fn standby_debug(&mut self) -> STANDBY_DEBUG_W<CTRL_SPEC, 2> {
        STANDBY_DEBUG_W::new(self)
    }
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause(&mut self) -> WDT_PAUSE_W<CTRL_SPEC, 8> {
        WDT_PAUSE_W::new(self)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt_pause(&mut self) -> WWDT_PAUSE_W<CTRL_SPEC, 9> {
        WWDT_PAUSE_W::new(self)
    }
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_pause(&mut self) -> TMR1_PAUSE_W<CTRL_SPEC, 10> {
        TMR1_PAUSE_W::new(self)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_pause(&mut self) -> TMR3_PAUSE_W<CTRL_SPEC, 12> {
        TMR3_PAUSE_W::new(self)
    }
    #[doc = "Bit 14 - ERTC_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc_pause(&mut self) -> ERTC_PAUSE_W<CTRL_SPEC, 14> {
        ERTC_PAUSE_W::new(self)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_smbus_timeout(&mut self) -> I2C1_SMBUS_TIMEOUT_W<CTRL_SPEC, 15> {
        I2C1_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_smbus_timeout(&mut self) -> I2C2_SMBUS_TIMEOUT_W<CTRL_SPEC, 16> {
        I2C2_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6_pause(&mut self) -> TMR6_PAUSE_W<CTRL_SPEC, 19> {
        TMR6_PAUSE_W::new(self)
    }
    #[doc = "Bit 21 - ERTC_512_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc_512_pause(&mut self) -> ERTC_512_PAUSE_W<CTRL_SPEC, 21> {
        ERTC_512_PAUSE_W::new(self)
    }
    #[doc = "Bit 22 - TMR15_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15_pause(&mut self) -> TMR15_PAUSE_W<CTRL_SPEC, 22> {
        TMR15_PAUSE_W::new(self)
    }
    #[doc = "Bit 23 - TMR16_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr16_pause(&mut self) -> TMR16_PAUSE_W<CTRL_SPEC, 23> {
        TMR16_PAUSE_W::new(self)
    }
    #[doc = "Bit 24 - TMR17_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr17_pause(&mut self) -> TMR17_PAUSE_W<CTRL_SPEC, 24> {
        TMR17_PAUSE_W::new(self)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14_pause(&mut self) -> TMR14_PAUSE_W<CTRL_SPEC, 27> {
        TMR14_PAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
