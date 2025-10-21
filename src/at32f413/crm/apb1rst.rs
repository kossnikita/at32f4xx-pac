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
#[doc = "Field `TMR4` reader - Timer 4 reset"]
pub type TMR4_R = crate::BitReader;
#[doc = "Field `TMR4` writer - Timer 4 reset"]
pub type TMR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5` reader - Timer 5 reset"]
pub type TMR5_R = crate::BitReader;
#[doc = "Field `TMR5` writer - Timer 5 reset"]
pub type TMR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT` reader - Window watchdog timer reset"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - Window watchdog timer reset"]
pub type WWDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - SPI2 reset"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 reset"]
pub type SPI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - USART 2 reset"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - USART 2 reset"]
pub type USART2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3` reader - USART 3 reset"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - USART 3 reset"]
pub type USART3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4` reader - UART 4 reset"]
pub type UART4_R = crate::BitReader;
#[doc = "Field `UART4` writer - UART 4 reset"]
pub type UART4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5` reader - UART 5 reset"]
pub type UART5_R = crate::BitReader;
#[doc = "Field `UART5` writer - UART 5 reset"]
pub type UART5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C2 reset"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 reset"]
pub type I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB` reader - USB reset"]
pub type USB_R = crate::BitReader;
#[doc = "Field `USB` writer - USB reset"]
pub type USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - CAN1 reset"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN1 reset"]
pub type CAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPR` reader - Battery powered domain register reset"]
pub type BPR_R = crate::BitReader;
#[doc = "Field `BPR` writer - Battery powered domain register reset"]
pub type BPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWC` reader - Power controller reset"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - Power controller reset"]
pub type PWC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2` reader - CAN2 reset"]
pub type CAN2_R = crate::BitReader;
#[doc = "Field `CAN2` writer - CAN2 reset"]
pub type CAN2_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    pub fn tmr5(&self) -> TMR5_R {
        TMR5_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    pub fn uart5(&self) -> UART5_R {
        UART5_R::new(((self.bits >> 20) & 1) != 0)
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
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Battery powered domain register reset"]
    #[inline(always)]
    pub fn bpr(&self) -> BPR_R {
        BPR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    pub fn pwc(&self) -> PWC_R {
        PWC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - CAN2 reset"]
    #[inline(always)]
    pub fn can2(&self) -> CAN2_R {
        CAN2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RST")
            .field("tmr2", &self.tmr2())
            .field("tmr3", &self.tmr3())
            .field("tmr4", &self.tmr4())
            .field("tmr5", &self.tmr5())
            .field("wwdt", &self.wwdt())
            .field("spi2", &self.spi2())
            .field("usart2", &self.usart2())
            .field("usart3", &self.usart3())
            .field("uart4", &self.uart4())
            .field("uart5", &self.uart5())
            .field("i2c1", &self.i2c1())
            .field("i2c2", &self.i2c2())
            .field("usb", &self.usb())
            .field("can1", &self.can1())
            .field("bpr", &self.bpr())
            .field("pwc", &self.pwc())
            .field("can2", &self.can2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 reset"]
    #[inline(always)]
    pub fn tmr2(&mut self) -> TMR2_W<'_, APB1RST_SPEC> {
        TMR2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    pub fn tmr3(&mut self) -> TMR3_W<'_, APB1RST_SPEC> {
        TMR3_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    pub fn tmr4(&mut self) -> TMR4_W<'_, APB1RST_SPEC> {
        TMR4_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    pub fn tmr5(&mut self) -> TMR5_W<'_, APB1RST_SPEC> {
        TMR5_W::new(self, 3)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W<'_, APB1RST_SPEC> {
        WWDT_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W<'_, APB1RST_SPEC> {
        SPI2_W::new(self, 14)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2(&mut self) -> USART2_W<'_, APB1RST_SPEC> {
        USART2_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3(&mut self) -> USART3_W<'_, APB1RST_SPEC> {
        USART3_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    pub fn uart4(&mut self) -> UART4_W<'_, APB1RST_SPEC> {
        UART4_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    pub fn uart5(&mut self) -> UART5_W<'_, APB1RST_SPEC> {
        UART5_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W<'_, APB1RST_SPEC> {
        I2C1_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W<'_, APB1RST_SPEC> {
        I2C2_W::new(self, 22)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W<'_, APB1RST_SPEC> {
        USB_W::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W<'_, APB1RST_SPEC> {
        CAN1_W::new(self, 25)
    }
    #[doc = "Bit 27 - Battery powered domain register reset"]
    #[inline(always)]
    pub fn bpr(&mut self) -> BPR_W<'_, APB1RST_SPEC> {
        BPR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    pub fn pwc(&mut self) -> PWC_W<'_, APB1RST_SPEC> {
        PWC_W::new(self, 28)
    }
    #[doc = "Bit 31 - CAN2 reset"]
    #[inline(always)]
    pub fn can2(&mut self) -> CAN2_W<'_, APB1RST_SPEC> {
        CAN2_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RST_SPEC;
impl crate::RegisterSpec for APB1RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rst::R`](R) reader structure"]
impl crate::Readable for APB1RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1rst::W`](W) writer structure"]
impl crate::Writable for APB1RST_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1RST to value 0"]
impl crate::Resettable for APB1RST_SPEC {}
