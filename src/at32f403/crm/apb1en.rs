#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<APB1EN_SPEC>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<APB1EN_SPEC>;
#[doc = "Field `TMR2` reader - Timer2 clock enable"]
pub type TMR2_R = crate::BitReader;
#[doc = "Field `TMR2` writer - Timer2 clock enable"]
pub type TMR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3` reader - Timer3 clock enable"]
pub type TMR3_R = crate::BitReader;
#[doc = "Field `TMR3` writer - Timer3 clock enable"]
pub type TMR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4` reader - Timer4 clock enable"]
pub type TMR4_R = crate::BitReader;
#[doc = "Field `TMR4` writer - Timer4 clock enable"]
pub type TMR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5` reader - Timer5 clock enable"]
pub type TMR5_R = crate::BitReader;
#[doc = "Field `TMR5` writer - Timer5 clock enable"]
pub type TMR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6` reader - Timer6 clock enable"]
pub type TMR6_R = crate::BitReader;
#[doc = "Field `TMR6` writer - Timer6 clock enable"]
pub type TMR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7` reader - Timer7 clock enable"]
pub type TMR7_R = crate::BitReader;
#[doc = "Field `TMR7` writer - Timer7 clock enable"]
pub type TMR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR12` reader - Timer12 clock enable"]
pub type TMR12_R = crate::BitReader;
#[doc = "Field `TMR12` writer - Timer12 clock enable"]
pub type TMR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13` reader - Timer13 clock enable"]
pub type TMR13_R = crate::BitReader;
#[doc = "Field `TMR13` writer - Timer13 clock enable"]
pub type TMR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14` reader - Timer14 clock enable"]
pub type TMR14_R = crate::BitReader;
#[doc = "Field `TMR14` writer - Timer14 clock enable"]
pub type TMR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT` reader - Window watchdog timer clock enable"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - Window watchdog timer clock enable"]
pub type WWDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - SPI2 clock enable"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 clock enable"]
pub type SPI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - SPI3 clock enable"]
pub type SPI3_R = crate::BitReader;
#[doc = "Field `SPI3` writer - SPI3 clock enable"]
pub type SPI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4` reader - SPI4 clock enable"]
pub type SPI4_R = crate::BitReader;
#[doc = "Field `SPI4` writer - SPI4 clock enable"]
pub type SPI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - USART2 clock enable"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - USART2 clock enable"]
pub type USART2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - USART3 clock enable"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - USART3 clock enable"]
pub type USART3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4` reader - UART4 clock enable"]
pub type UART4_R = crate::BitReader;
#[doc = "Field `UART4` writer - UART4 clock enable"]
pub type UART4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5` reader - UART5 clock enable"]
pub type UART5_R = crate::BitReader;
#[doc = "Field `UART5` writer - UART5 clock enable"]
pub type UART5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 clock enable"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 clock enable"]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C2 clock enable"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 clock enable"]
pub type I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - USB clock enable"]
pub type USB_R = crate::BitReader;
#[doc = "Field `USB` writer - USB clock enable"]
pub type USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN1 clock enable"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN1 clock enable"]
pub type CAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3` reader - I2C3 clock enable"]
pub type I2C3_R = crate::BitReader;
#[doc = "Field `I2C3` writer - I2C3 clock enable"]
pub type I2C3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPR` reader - Barrery powered domain register clock enable"]
pub type BPR_R = crate::BitReader;
#[doc = "Field `BPR` writer - Barrery powered domain register clock enable"]
pub type BPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC` reader - Power clock enable"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - Power clock enable"]
pub type PWC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC` reader - DAC clock enable"]
pub type DAC_R = crate::BitReader;
#[doc = "Field `DAC` writer - DAC clock enable"]
pub type DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer5 clock enable"]
    #[inline(always)]
    pub fn tmr5(&self) -> TMR5_R {
        TMR5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    pub fn tmr6(&self) -> TMR6_R {
        TMR6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer7 clock enable"]
    #[inline(always)]
    pub fn tmr7(&self) -> TMR7_R {
        TMR7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer12 clock enable"]
    #[inline(always)]
    pub fn tmr12(&self) -> TMR12_R {
        TMR12_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer13 clock enable"]
    #[inline(always)]
    pub fn tmr13(&self) -> TMR13_R {
        TMR13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    pub fn tmr14(&self) -> TMR14_R {
        TMR14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4(&self) -> SPI4_R {
        SPI4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5(&self) -> UART5_R {
        UART5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Barrery powered domain register clock enable"]
    #[inline(always)]
    pub fn bpr(&self) -> BPR_R {
        BPR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    pub fn pwc(&self) -> PWC_R {
        PWC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1EN")
            .field("tmr2", &format_args!("{}", self.tmr2().bit()))
            .field("tmr3", &format_args!("{}", self.tmr3().bit()))
            .field("tmr4", &format_args!("{}", self.tmr4().bit()))
            .field("tmr5", &format_args!("{}", self.tmr5().bit()))
            .field("tmr6", &format_args!("{}", self.tmr6().bit()))
            .field("tmr7", &format_args!("{}", self.tmr7().bit()))
            .field("tmr12", &format_args!("{}", self.tmr12().bit()))
            .field("tmr13", &format_args!("{}", self.tmr13().bit()))
            .field("tmr14", &format_args!("{}", self.tmr14().bit()))
            .field("wwdt", &format_args!("{}", self.wwdt().bit()))
            .field("spi2", &format_args!("{}", self.spi2().bit()))
            .field("spi3", &format_args!("{}", self.spi3().bit()))
            .field("spi4", &format_args!("{}", self.spi4().bit()))
            .field("usart2", &format_args!("{}", self.usart2().bit()))
            .field("usart3", &format_args!("{}", self.usart3().bit()))
            .field("uart4", &format_args!("{}", self.uart4().bit()))
            .field("uart5", &format_args!("{}", self.uart5().bit()))
            .field("i2c1", &format_args!("{}", self.i2c1().bit()))
            .field("i2c2", &format_args!("{}", self.i2c2().bit()))
            .field("usb", &format_args!("{}", self.usb().bit()))
            .field("can1", &format_args!("{}", self.can1().bit()))
            .field("i2c3", &format_args!("{}", self.i2c3().bit()))
            .field("bpr", &format_args!("{}", self.bpr().bit()))
            .field("pwc", &format_args!("{}", self.pwc().bit()))
            .field("dac", &format_args!("{}", self.dac().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB1EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2(&mut self) -> TMR2_W<APB1EN_SPEC> {
        TMR2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<APB1EN_SPEC> {
        TMR3_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4(&mut self) -> TMR4_W<APB1EN_SPEC> {
        TMR4_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5(&mut self) -> TMR5_W<APB1EN_SPEC> {
        TMR5_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6(&mut self) -> TMR6_W<APB1EN_SPEC> {
        TMR6_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer7 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7(&mut self) -> TMR7_W<APB1EN_SPEC> {
        TMR7_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer12 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr12(&mut self) -> TMR12_W<APB1EN_SPEC> {
        TMR12_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer13 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13(&mut self) -> TMR13_W<APB1EN_SPEC> {
        TMR13_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14(&mut self) -> TMR14_W<APB1EN_SPEC> {
        TMR14_W::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<APB1EN_SPEC> {
        WWDT_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<APB1EN_SPEC> {
        SPI2_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<APB1EN_SPEC> {
        SPI3_W::new(self, 15)
    }
    #[doc = "Bit 16 - SPI4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi4(&mut self) -> SPI4_W<APB1EN_SPEC> {
        SPI4_W::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<APB1EN_SPEC> {
        USART2_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<APB1EN_SPEC> {
        USART3_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> UART4_W<APB1EN_SPEC> {
        UART4_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5(&mut self) -> UART5_W<APB1EN_SPEC> {
        UART5_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<APB1EN_SPEC> {
        I2C1_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<APB1EN_SPEC> {
        I2C2_W::new(self, 22)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<APB1EN_SPEC> {
        USB_W::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<APB1EN_SPEC> {
        CAN1_W::new(self, 25)
    }
    #[doc = "Bit 26 - I2C3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3(&mut self) -> I2C3_W<APB1EN_SPEC> {
        I2C3_W::new(self, 26)
    }
    #[doc = "Bit 27 - Barrery powered domain register clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpr(&mut self) -> BPR_W<APB1EN_SPEC> {
        BPR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwc(&mut self) -> PWC_W<APB1EN_SPEC> {
        PWC_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<APB1EN_SPEC> {
        DAC_W::new(self, 29)
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
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1EN_SPEC;
impl crate::RegisterSpec for APB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for APB1EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for APB1EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for APB1EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
