#[doc = "Register `APB1RST` reader"]
pub type R = crate::R<APB1RST_SPEC>;
#[doc = "Register `APB1RST` writer"]
pub type W = crate::W<APB1RST_SPEC>;
#[doc = "Field `TMR2` reader - Timer 2 reset"]
pub type TMR2_R = crate::BitReader;
#[doc = "Field `TMR2` writer - Timer 2 reset"]
pub type TMR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR3` reader - Timer 3 reset"]
pub type TMR3_R = crate::BitReader;
#[doc = "Field `TMR3` writer - Timer 3 reset"]
pub type TMR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR4` reader - Timer 4 reset"]
pub type TMR4_R = crate::BitReader;
#[doc = "Field `TMR4` writer - Timer 4 reset"]
pub type TMR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR5` reader - Timer 5 reset"]
pub type TMR5_R = crate::BitReader;
#[doc = "Field `TMR5` writer - Timer 5 reset"]
pub type TMR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR6` reader - Timer 6 reset"]
pub type TMR6_R = crate::BitReader;
#[doc = "Field `TMR6` writer - Timer 6 reset"]
pub type TMR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR7` reader - Timer 7 reset"]
pub type TMR7_R = crate::BitReader;
#[doc = "Field `TMR7` writer - Timer 7 reset"]
pub type TMR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR12` reader - Timer 12 reset"]
pub type TMR12_R = crate::BitReader;
#[doc = "Field `TMR12` writer - Timer 12 reset"]
pub type TMR12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR13` reader - Timer 13 reset"]
pub type TMR13_R = crate::BitReader;
#[doc = "Field `TMR13` writer - Timer 13 reset"]
pub type TMR13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR14` reader - Timer 14 reset"]
pub type TMR14_R = crate::BitReader;
#[doc = "Field `TMR14` writer - Timer 14 reset"]
pub type TMR14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT` reader - Window watchdog timer reset"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - Window watchdog timer reset"]
pub type WWDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2` reader - SPI2 reset"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 reset"]
pub type SPI2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI3` reader - SPI3 reset"]
pub type SPI3_R = crate::BitReader;
#[doc = "Field `SPI3` writer - SPI3 reset"]
pub type SPI3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4` reader - SPI4 reset"]
pub type SPI4_R = crate::BitReader;
#[doc = "Field `SPI4` writer - SPI4 reset"]
pub type SPI4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2` reader - USART 2 reset"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - USART 2 reset"]
pub type USART2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3` reader - USART 3 reset"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - USART 3 reset"]
pub type USART3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART4` reader - UART 4 reset"]
pub type UART4_R = crate::BitReader;
#[doc = "Field `UART4` writer - UART 4 reset"]
pub type UART4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART5` reader - UART 5 reset"]
pub type UART5_R = crate::BitReader;
#[doc = "Field `UART5` writer - UART 5 reset"]
pub type UART5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2` reader - I2C2 reset"]
pub type I2C2_R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 reset"]
pub type I2C2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB` reader - USB reset"]
pub type USB_R = crate::BitReader;
#[doc = "Field `USB` writer - USB reset"]
pub type USB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1` reader - CAN1 reset"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN1 reset"]
pub type CAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2` reader - CAN2 reset"]
pub type CAN2_R = crate::BitReader;
#[doc = "Field `CAN2` writer - CAN2 reset"]
pub type CAN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BPR` reader - Battery powered domain register reset"]
pub type BPR_R = crate::BitReader;
#[doc = "Field `BPR` writer - Battery powered domain register reset"]
pub type BPR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWC` reader - Power controller reset"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - Power controller reset"]
pub type PWC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAC` reader - DAC reset"]
pub type DAC_R = crate::BitReader;
#[doc = "Field `DAC` writer - DAC reset"]
pub type DAC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 6 - Timer 12 reset"]
    #[inline(always)]
    pub fn tmr12(&self) -> TMR12_R {
        TMR12_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 16 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4(&self) -> SPI4_R {
        SPI4_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2(&self) -> CAN2_R {
        CAN2_R::new(((self.bits >> 26) & 1) != 0)
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
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RST")
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
            .field("can2", &format_args!("{}", self.can2().bit()))
            .field("bpr", &format_args!("{}", self.bpr().bit()))
            .field("pwc", &format_args!("{}", self.pwc().bit()))
            .field("dac", &format_args!("{}", self.dac().bit()))
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
    pub fn tmr2(&mut self) -> TMR2_W<APB1RST_SPEC, 0> {
        TMR2_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3(&mut self) -> TMR3_W<APB1RST_SPEC, 1> {
        TMR3_W::new(self)
    }
    #[doc = "Bit 2 - Timer 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4(&mut self) -> TMR4_W<APB1RST_SPEC, 2> {
        TMR4_W::new(self)
    }
    #[doc = "Bit 3 - Timer 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5(&mut self) -> TMR5_W<APB1RST_SPEC, 3> {
        TMR5_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6(&mut self) -> TMR6_W<APB1RST_SPEC, 4> {
        TMR6_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7(&mut self) -> TMR7_W<APB1RST_SPEC, 5> {
        TMR7_W::new(self)
    }
    #[doc = "Bit 6 - Timer 12 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr12(&mut self) -> TMR12_W<APB1RST_SPEC, 6> {
        TMR12_W::new(self)
    }
    #[doc = "Bit 7 - Timer 13 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13(&mut self) -> TMR13_W<APB1RST_SPEC, 7> {
        TMR13_W::new(self)
    }
    #[doc = "Bit 8 - Timer 14 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14(&mut self) -> TMR14_W<APB1RST_SPEC, 8> {
        TMR14_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<APB1RST_SPEC, 11> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<APB1RST_SPEC, 14> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<APB1RST_SPEC, 15> {
        SPI3_W::new(self)
    }
    #[doc = "Bit 16 - SPI4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4(&mut self) -> SPI4_W<APB1RST_SPEC, 16> {
        SPI4_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<APB1RST_SPEC, 17> {
        USART2_W::new(self)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<APB1RST_SPEC, 18> {
        USART3_W::new(self)
    }
    #[doc = "Bit 19 - UART 4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> UART4_W<APB1RST_SPEC, 19> {
        UART4_W::new(self)
    }
    #[doc = "Bit 20 - UART 5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart5(&mut self) -> UART5_W<APB1RST_SPEC, 20> {
        UART5_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<APB1RST_SPEC, 21> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<APB1RST_SPEC, 22> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 23 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<APB1RST_SPEC, 23> {
        USB_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<APB1RST_SPEC, 25> {
        CAN1_W::new(self)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can2(&mut self) -> CAN2_W<APB1RST_SPEC, 26> {
        CAN2_W::new(self)
    }
    #[doc = "Bit 27 - Battery powered domain register reset"]
    #[inline(always)]
    #[must_use]
    pub fn bpr(&mut self) -> BPR_W<APB1RST_SPEC, 27> {
        BPR_W::new(self)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwc(&mut self) -> PWC_W<APB1RST_SPEC, 28> {
        PWC_W::new(self)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<APB1RST_SPEC, 29> {
        DAC_W::new(self)
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
