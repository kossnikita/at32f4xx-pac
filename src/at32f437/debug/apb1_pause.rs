#[doc = "Register `APB1_PAUSE` reader"]
pub type R = crate::R<APB1_PAUSE_SPEC>;
#[doc = "Register `APB1_PAUSE` writer"]
pub type W = crate::W<APB1_PAUSE_SPEC>;
#[doc = "Field `TMR2_PAUSE` reader - TMR2_PAUSE"]
pub type TMR2_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR2_PAUSE` writer - TMR2_PAUSE"]
pub type TMR2_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR3_PAUSE` reader - TMR3_PAUSE"]
pub type TMR3_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR3_PAUSE` writer - TMR3_PAUSE"]
pub type TMR3_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR4_PAUSE` reader - TMR4_PAUSE"]
pub type TMR4_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR4_PAUSE` writer - TMR4_PAUSE"]
pub type TMR4_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR5_PAUSE` reader - TMR5_PAUSE"]
pub type TMR5_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR5_PAUSE` writer - TMR5_PAUSE"]
pub type TMR5_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR6_PAUSE` reader - TMR6_PAUSE"]
pub type TMR6_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR6_PAUSE` writer - TMR6_PAUSE"]
pub type TMR6_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR7_PAUSE` reader - TMR7_PAUSE"]
pub type TMR7_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR7_PAUSE` writer - TMR7_PAUSE"]
pub type TMR7_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR12_PAUSE` reader - TMR12_PAUSE"]
pub type TMR12_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR12_PAUSE` writer - TMR12_PAUSE"]
pub type TMR12_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR13_PAUSE` reader - TMR13_PAUSE"]
pub type TMR13_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR13_PAUSE` writer - TMR13_PAUSE"]
pub type TMR13_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR14_PAUSE` reader - TMR14_PAUSE"]
pub type TMR14_PAUSE_R = crate::BitReader;
#[doc = "Field `TMR14_PAUSE` writer - TMR14_PAUSE"]
pub type TMR14_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERTC_PAUSE` reader - ERTC_PAUSE"]
pub type ERTC_PAUSE_R = crate::BitReader;
#[doc = "Field `ERTC_PAUSE` writer - ERTC_PAUSE"]
pub type ERTC_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT_PAUSE` reader - WWDT_PAUSE"]
pub type WWDT_PAUSE_R = crate::BitReader;
#[doc = "Field `WWDT_PAUSE` writer - WWDT_PAUSE"]
pub type WWDT_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDT_PAUSE` reader - WDT_PAUSE"]
pub type WDT_PAUSE_R = crate::BitReader;
#[doc = "Field `WDT_PAUSE` writer - WDT_PAUSE"]
pub type WDT_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERTC512_PAUSE` reader - ERTC512_PAUSE"]
pub type ERTC512_PAUSE_R = crate::BitReader;
#[doc = "Field `ERTC512_PAUSE` writer - ERTC512_PAUSE"]
pub type ERTC512_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - I2C1_SMBUS_TIMEOUT"]
pub type I2C1_SMBUS_TIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1_PAUSE` reader - CAN1_PAUSE"]
pub type CAN1_PAUSE_R = crate::BitReader;
#[doc = "Field `CAN1_PAUSE` writer - CAN1_PAUSE"]
pub type CAN1_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2_PAUSE` reader - CAN2_PAUSE"]
pub type CAN2_PAUSE_R = crate::BitReader;
#[doc = "Field `CAN2_PAUSE` writer - CAN2_PAUSE"]
pub type CAN2_PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - I2C2_SMBUS_TIMEOUT"]
pub type I2C2_SMBUS_TIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` reader - I2C3_SMBUS_TIMEOUT"]
pub type I2C3_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` writer - I2C3_SMBUS_TIMEOUT"]
pub type I2C3_SMBUS_TIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TMR2_PAUSE"]
    #[inline(always)]
    pub fn tmr2_pause(&self) -> TMR2_PAUSE_R {
        TMR2_PAUSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR3_PAUSE"]
    #[inline(always)]
    pub fn tmr3_pause(&self) -> TMR3_PAUSE_R {
        TMR3_PAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMR4_PAUSE"]
    #[inline(always)]
    pub fn tmr4_pause(&self) -> TMR4_PAUSE_R {
        TMR4_PAUSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TMR5_PAUSE"]
    #[inline(always)]
    pub fn tmr5_pause(&self) -> TMR5_PAUSE_R {
        TMR5_PAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TMR6_PAUSE"]
    #[inline(always)]
    pub fn tmr6_pause(&self) -> TMR6_PAUSE_R {
        TMR6_PAUSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TMR7_PAUSE"]
    #[inline(always)]
    pub fn tmr7_pause(&self) -> TMR7_PAUSE_R {
        TMR7_PAUSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TMR12_PAUSE"]
    #[inline(always)]
    pub fn tmr12_pause(&self) -> TMR12_PAUSE_R {
        TMR12_PAUSE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TMR13_PAUSE"]
    #[inline(always)]
    pub fn tmr13_pause(&self) -> TMR13_PAUSE_R {
        TMR13_PAUSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TMR14_PAUSE"]
    #[inline(always)]
    pub fn tmr14_pause(&self) -> TMR14_PAUSE_R {
        TMR14_PAUSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - ERTC_PAUSE"]
    #[inline(always)]
    pub fn ertc_pause(&self) -> ERTC_PAUSE_R {
        ERTC_PAUSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDT_PAUSE"]
    #[inline(always)]
    pub fn wwdt_pause(&self) -> WWDT_PAUSE_R {
        WWDT_PAUSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WDT_PAUSE"]
    #[inline(always)]
    pub fn wdt_pause(&self) -> WDT_PAUSE_R {
        WDT_PAUSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - ERTC512_PAUSE"]
    #[inline(always)]
    pub fn ertc512_pause(&self) -> ERTC512_PAUSE_R {
        ERTC512_PAUSE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2C1_SMBUS_TIMEOUT_R {
        I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1_PAUSE"]
    #[inline(always)]
    pub fn can1_pause(&self) -> CAN1_PAUSE_R {
        CAN1_PAUSE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2_PAUSE"]
    #[inline(always)]
    pub fn can2_pause(&self) -> CAN2_PAUSE_R {
        CAN2_PAUSE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2C2_SMBUS_TIMEOUT_R {
        I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c3_smbus_timeout(&self) -> I2C3_SMBUS_TIMEOUT_R {
        I2C3_SMBUS_TIMEOUT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1_PAUSE")
            .field("tmr2_pause", &format_args!("{}", self.tmr2_pause().bit()))
            .field("tmr3_pause", &format_args!("{}", self.tmr3_pause().bit()))
            .field("tmr4_pause", &format_args!("{}", self.tmr4_pause().bit()))
            .field("tmr5_pause", &format_args!("{}", self.tmr5_pause().bit()))
            .field("tmr6_pause", &format_args!("{}", self.tmr6_pause().bit()))
            .field("tmr7_pause", &format_args!("{}", self.tmr7_pause().bit()))
            .field("tmr12_pause", &format_args!("{}", self.tmr12_pause().bit()))
            .field("tmr13_pause", &format_args!("{}", self.tmr13_pause().bit()))
            .field("tmr14_pause", &format_args!("{}", self.tmr14_pause().bit()))
            .field("ertc_pause", &format_args!("{}", self.ertc_pause().bit()))
            .field("wwdt_pause", &format_args!("{}", self.wwdt_pause().bit()))
            .field("wdt_pause", &format_args!("{}", self.wdt_pause().bit()))
            .field(
                "ertc512_pause",
                &format_args!("{}", self.ertc512_pause().bit()),
            )
            .field(
                "i2c1_smbus_timeout",
                &format_args!("{}", self.i2c1_smbus_timeout().bit()),
            )
            .field("can1_pause", &format_args!("{}", self.can1_pause().bit()))
            .field("can2_pause", &format_args!("{}", self.can2_pause().bit()))
            .field(
                "i2c2_smbus_timeout",
                &format_args!("{}", self.i2c2_smbus_timeout().bit()),
            )
            .field(
                "i2c3_smbus_timeout",
                &format_args!("{}", self.i2c3_smbus_timeout().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB1_PAUSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - TMR2_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_pause(&mut self) -> TMR2_PAUSE_W<APB1_PAUSE_SPEC, 0> {
        TMR2_PAUSE_W::new(self)
    }
    #[doc = "Bit 1 - TMR3_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_pause(&mut self) -> TMR3_PAUSE_W<APB1_PAUSE_SPEC, 1> {
        TMR3_PAUSE_W::new(self)
    }
    #[doc = "Bit 2 - TMR4_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4_pause(&mut self) -> TMR4_PAUSE_W<APB1_PAUSE_SPEC, 2> {
        TMR4_PAUSE_W::new(self)
    }
    #[doc = "Bit 3 - TMR5_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5_pause(&mut self) -> TMR5_PAUSE_W<APB1_PAUSE_SPEC, 3> {
        TMR5_PAUSE_W::new(self)
    }
    #[doc = "Bit 4 - TMR6_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6_pause(&mut self) -> TMR6_PAUSE_W<APB1_PAUSE_SPEC, 4> {
        TMR6_PAUSE_W::new(self)
    }
    #[doc = "Bit 5 - TMR7_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7_pause(&mut self) -> TMR7_PAUSE_W<APB1_PAUSE_SPEC, 5> {
        TMR7_PAUSE_W::new(self)
    }
    #[doc = "Bit 6 - TMR12_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr12_pause(&mut self) -> TMR12_PAUSE_W<APB1_PAUSE_SPEC, 6> {
        TMR12_PAUSE_W::new(self)
    }
    #[doc = "Bit 7 - TMR13_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13_pause(&mut self) -> TMR13_PAUSE_W<APB1_PAUSE_SPEC, 7> {
        TMR13_PAUSE_W::new(self)
    }
    #[doc = "Bit 8 - TMR14_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14_pause(&mut self) -> TMR14_PAUSE_W<APB1_PAUSE_SPEC, 8> {
        TMR14_PAUSE_W::new(self)
    }
    #[doc = "Bit 10 - ERTC_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc_pause(&mut self) -> ERTC_PAUSE_W<APB1_PAUSE_SPEC, 10> {
        ERTC_PAUSE_W::new(self)
    }
    #[doc = "Bit 11 - WWDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt_pause(&mut self) -> WWDT_PAUSE_W<APB1_PAUSE_SPEC, 11> {
        WWDT_PAUSE_W::new(self)
    }
    #[doc = "Bit 12 - WDT_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause(&mut self) -> WDT_PAUSE_W<APB1_PAUSE_SPEC, 12> {
        WDT_PAUSE_W::new(self)
    }
    #[doc = "Bit 15 - ERTC512_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn ertc512_pause(&mut self) -> ERTC512_PAUSE_W<APB1_PAUSE_SPEC, 15> {
        ERTC512_PAUSE_W::new(self)
    }
    #[doc = "Bit 24 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_smbus_timeout(&mut self) -> I2C1_SMBUS_TIMEOUT_W<APB1_PAUSE_SPEC, 24> {
        I2C1_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 25 - CAN1_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn can1_pause(&mut self) -> CAN1_PAUSE_W<APB1_PAUSE_SPEC, 25> {
        CAN1_PAUSE_W::new(self)
    }
    #[doc = "Bit 26 - CAN2_PAUSE"]
    #[inline(always)]
    #[must_use]
    pub fn can2_pause(&mut self) -> CAN2_PAUSE_W<APB1_PAUSE_SPEC, 26> {
        CAN2_PAUSE_W::new(self)
    }
    #[doc = "Bit 27 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_smbus_timeout(&mut self) -> I2C2_SMBUS_TIMEOUT_W<APB1_PAUSE_SPEC, 27> {
        I2C2_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 28 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_smbus_timeout(&mut self) -> I2C3_SMBUS_TIMEOUT_W<APB1_PAUSE_SPEC, 28> {
        I2C3_SMBUS_TIMEOUT_W::new(self)
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
#[doc = "DEBUG APB1 PAUSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1_pause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1_pause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1_PAUSE_SPEC;
impl crate::RegisterSpec for APB1_PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1_pause::R`](R) reader structure"]
impl crate::Readable for APB1_PAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1_pause::W`](W) writer structure"]
impl crate::Writable for APB1_PAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1_PAUSE to value 0"]
impl crate::Resettable for APB1_PAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
