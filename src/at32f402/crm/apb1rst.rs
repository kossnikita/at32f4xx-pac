#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<APB1RST_SPEC>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<APB1RST_SPEC>;
#[doc = "Field `TMR2` reader - Timer2 reset"]
pub type TMR2_R = crate::BitReader;
#[doc = "Field `TMR2` writer - Timer2 reset"]
pub type TMR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3` reader - Timer3 reset"]
pub type TMR3_R = crate::BitReader;
#[doc = "Field `TMR3` writer - Timer3 reset"]
pub type TMR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4` reader - Timer4 reset"]
pub type TMR4_R = crate::BitReader;
#[doc = "Field `TMR4` writer - Timer4 reset"]
pub type TMR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6` reader - Timer6 reset"]
pub type TMR6_R = crate::BitReader;
#[doc = "Field `TMR6` writer - Timer6 reset"]
pub type TMR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7` reader - Timer7 reset"]
pub type TMR7_R = crate::BitReader;
#[doc = "Field `TMR7` writer - Timer7 reset"]
pub type TMR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13` reader - Timer13 reset"]
pub type TMR13_R = crate::BitReader;
#[doc = "Field `TMR13` writer - Timer13 reset"]
pub type TMR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14` reader - Timer14 reset"]
pub type TMR14_R = crate::BitReader;
#[doc = "Field `TMR14` writer - Timer14 reset"]
pub type TMR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT` reader - Window watchdog reset"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - Window watchdog reset"]
pub type WWDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - SPI2 reset"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 reset"]
pub type SPI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - SPI3 reset"]
pub type SPI3_R = crate::BitReader;
#[doc = "Field `SPI3` writer - SPI3 reset"]
pub type SPI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - USART2 reset"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - USART2 reset"]
pub type USART2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - USART3 reset"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - USART3 reset"]
pub type USART3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4` reader - USART4 reset"]
pub type USART4_R = crate::BitReader;
#[doc = "Field `USART4` writer - USART4 reset"]
pub type USART4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5` reader - USART5 reset"]
pub type USART5_R = crate::BitReader;
#[doc = "Field `USART5` writer - USART5 reset"]
pub type USART5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C2 reset"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 reset"]
pub type I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3` reader - I2C3 reset"]
pub type I2C3_R = crate::BitReader;
#[doc = "Field `I2C3` writer - I2C3 reset"]
pub type I2C3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN1 reset"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN1 reset"]
pub type CAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC` reader - PWC reset"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - PWC reset"]
pub type PWC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7` reader - UART7 reset"]
pub type UART7_R = crate::BitReader;
#[doc = "Field `UART7` writer - UART7 reset"]
pub type UART7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8` reader - UART8 reset"]
pub type UART8_R = crate::BitReader;
#[doc = "Field `UART8` writer - UART8 reset"]
pub type UART8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer2 reset"]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 reset"]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer4 reset"]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer6 reset"]
    #[inline(always)]
    pub fn tmr6(&self) -> TMR6_R {
        TMR6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer7 reset"]
    #[inline(always)]
    pub fn tmr7(&self) -> TMR7_R {
        TMR7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer13 reset"]
    #[inline(always)]
    pub fn tmr13(&self) -> TMR13_R {
        TMR13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer14 reset"]
    #[inline(always)]
    pub fn tmr14(&self) -> TMR14_R {
        TMR14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
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
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 reset"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - PWC reset"]
    #[inline(always)]
    pub fn pwc(&self) -> PWC_R {
        PWC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 reset"]
    #[inline(always)]
    pub fn uart7(&self) -> UART7_R {
        UART7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 reset"]
    #[inline(always)]
    pub fn uart8(&self) -> UART8_R {
        UART8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RST")
            .field("tmr2", &self.tmr2())
            .field("tmr3", &self.tmr3())
            .field("tmr4", &self.tmr4())
            .field("tmr6", &self.tmr6())
            .field("tmr7", &self.tmr7())
            .field("tmr13", &self.tmr13())
            .field("tmr14", &self.tmr14())
            .field("wwdt", &self.wwdt())
            .field("spi2", &self.spi2())
            .field("spi3", &self.spi3())
            .field("usart2", &self.usart2())
            .field("usart3", &self.usart3())
            .field("usart4", &self.usart4())
            .field("usart5", &self.usart5())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("i2c3", &self.i2c3())
            .field("can1", &self.can1())
            .field("pwc", &self.pwc())
            .field("uart7", &self.uart7())
            .field("uart8", &self.uart8())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<APB1RST_SPEC> {
        TMR2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<APB1RST_SPEC> {
        TMR3_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4(&mut self) -> TMR4_W<APB1RST_SPEC> {
        TMR4_W::new(self, 2)
    }
    #[doc = "Bit 4 - Timer6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6(&mut self) -> TMR6_W<APB1RST_SPEC> {
        TMR6_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7(&mut self) -> TMR7_W<APB1RST_SPEC> {
        TMR7_W::new(self, 5)
    }
    #[doc = "Bit 7 - Timer13 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13(&mut self) -> TMR13_W<APB1RST_SPEC> {
        TMR13_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer14 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14(&mut self) -> TMR14_W<APB1RST_SPEC> {
        TMR14_W::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
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
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<APB1RST_SPEC> {
        USART2_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<APB1RST_SPEC> {
        USART3_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart4(&mut self) -> USART4_W<APB1RST_SPEC> {
        USART4_W::new(self, 19)
    }
    #[doc = "Bit 20 - USART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart5(&mut self) -> USART5_W<APB1RST_SPEC> {
        USART5_W::new(self, 20)
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
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3(&mut self) -> I2C3_W<APB1RST_SPEC> {
        I2C3_W::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<APB1RST_SPEC> {
        CAN1_W::new(self, 25)
    }
    #[doc = "Bit 28 - PWC reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwc(&mut self) -> PWC_W<APB1RST_SPEC> {
        PWC_W::new(self, 28)
    }
    #[doc = "Bit 30 - UART7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart7(&mut self) -> UART7_W<APB1RST_SPEC> {
        UART7_W::new(self, 30)
    }
    #[doc = "Bit 31 - UART8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart8(&mut self) -> UART8_W<APB1RST_SPEC> {
        UART8_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral reset register (CRM_APB1RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RST_SPEC;
impl crate::RegisterSpec for APB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rst::R`](R) reader structure"]
impl crate::Readable for APB1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1rst::W`](W) writer structure"]
impl crate::Writable for APB1RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RST to value 0"]
impl crate::Resettable for APB1RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
