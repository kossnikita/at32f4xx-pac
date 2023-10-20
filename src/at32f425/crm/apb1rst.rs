#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<APB1RST_SPEC>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<APB1RST_SPEC>;
#[doc = "Field `TMR2` reader - Timer 2 reset"]
pub type TMR2_R = crate::BitReader;
#[doc = "Field `TMR2` writer - Timer 2 reset"]
pub type TMR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3` reader - Timer 3 reset"]
pub type TMR3_R = crate::BitReader;
#[doc = "Field `TMR3` writer - Timer 3 reset"]
pub type TMR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6` reader - Timer 6 reset"]
pub type TMR6_R = crate::BitReader;
#[doc = "Field `TMR6` writer - Timer 6 reset"]
pub type TMR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7` reader - Timer 7 reset"]
pub type TMR7_R = crate::BitReader;
#[doc = "Field `TMR7` writer - Timer 7 reset"]
pub type TMR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13` reader - Timer 13 reset"]
pub type TMR13_R = crate::BitReader;
#[doc = "Field `TMR13` writer - Timer 13 reset"]
pub type TMR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14` reader - Timer 14 reset"]
pub type TMR14_R = crate::BitReader;
#[doc = "Field `TMR14` writer - Timer 14 reset"]
pub type TMR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT` reader - Window watchdog timer reset"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - Window watchdog timer reset"]
pub type WWDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - SPI2 reset"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 reset"]
pub type SPI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - SPI3 reset"]
pub type SPI3_R = crate::BitReader;
#[doc = "Field `SPI3` writer - SPI3 reset"]
pub type SPI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - USART 2 reset"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - USART 2 reset"]
pub type USART2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - USART 3 reset"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - USART 3 reset"]
pub type USART3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4` reader - USART 4 reset"]
pub type USART4_R = crate::BitReader;
#[doc = "Field `USART4` writer - USART 4 reset"]
pub type USART4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C2 reset"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 reset"]
pub type I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN1 reset"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN1 reset"]
pub type CAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC` reader - ACC reset"]
pub type ACC_R = crate::BitReader;
#[doc = "Field `ACC` writer - ACC reset"]
pub type ACC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC` reader - Power controller reset"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - Power controller reset"]
pub type PWC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    pub fn tmr6(&self) -> TMR6_R {
        TMR6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    pub fn tmr7(&self) -> TMR7_R {
        TMR7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 13 reset"]
    #[inline(always)]
    pub fn tmr13(&self) -> TMR13_R {
        TMR13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline(always)]
    pub fn tmr14(&self) -> TMR14_R {
        TMR14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - ACC reset"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    pub fn pwc(&self) -> PWC_R {
        PWC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RST")
            .field("tmr2", &format_args!("{}", self.tmr2().bit()))
            .field("tmr3", &format_args!("{}", self.tmr3().bit()))
            .field("tmr6", &format_args!("{}", self.tmr6().bit()))
            .field("tmr7", &format_args!("{}", self.tmr7().bit()))
            .field("tmr13", &format_args!("{}", self.tmr13().bit()))
            .field("tmr14", &format_args!("{}", self.tmr14().bit()))
            .field("wwdt", &format_args!("{}", self.wwdt().bit()))
            .field("spi2", &format_args!("{}", self.spi2().bit()))
            .field("spi3", &format_args!("{}", self.spi3().bit()))
            .field("usart2", &format_args!("{}", self.usart2().bit()))
            .field("usart3", &format_args!("{}", self.usart3().bit()))
            .field("usart4", &format_args!("{}", self.usart4().bit()))
            .field("i2c1", &format_args!("{}", self.i2c1().bit()))
            .field("i2c2", &format_args!("{}", self.i2c2().bit()))
            .field("can1", &format_args!("{}", self.can1().bit()))
            .field("acc", &format_args!("{}", self.acc().bit()))
            .field("pwc", &format_args!("{}", self.pwc().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB1RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<APB1RST_SPEC> {
        TMR2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<APB1RST_SPEC> {
        TMR3_W::new(self, 1)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6(&mut self) -> TMR6_W<APB1RST_SPEC> {
        TMR6_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7(&mut self) -> TMR7_W<APB1RST_SPEC> {
        TMR7_W::new(self, 5)
    }
    #[doc = "Bit 7 - Timer 13 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13(&mut self) -> TMR13_W<APB1RST_SPEC> {
        TMR13_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14(&mut self) -> TMR14_W<APB1RST_SPEC> {
        TMR14_W::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<APB1RST_SPEC> {
        WWDT_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<APB1RST_SPEC> {
        SPI2_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<APB1RST_SPEC> {
        SPI3_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<APB1RST_SPEC> {
        USART2_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<APB1RST_SPEC> {
        USART3_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart4(&mut self) -> USART4_W<APB1RST_SPEC> {
        USART4_W::new(self, 19)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<APB1RST_SPEC> {
        I2C1_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<APB1RST_SPEC> {
        I2C2_W::new(self, 22)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<APB1RST_SPEC> {
        CAN1_W::new(self, 25)
    }
    #[doc = "Bit 27 - ACC reset"]
    #[inline(always)]
    #[must_use]
    pub fn acc(&mut self) -> ACC_W<APB1RST_SPEC> {
        ACC_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwc(&mut self) -> PWC_W<APB1RST_SPEC> {
        PWC_W::new(self, 28)
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
#[doc = "APB1 peripheral reset register (CRM_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RST_SPEC;
impl crate::RegisterSpec for APB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rst::R`](R) reader structure"]
impl crate::Readable for APB1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1rst::W`](W) writer structure"]
impl crate::Writable for APB1RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1RST to value 0"]
impl crate::Resettable for APB1RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
