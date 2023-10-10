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
#[doc = "Field `CMP` reader - Comparator reset"]
pub type CMP_R = crate::BitReader;
#[doc = "Field `CMP` writer - Comparator reset"]
pub type CMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDT` reader - Window watchdog timer reset"]
pub type WWDT_R = crate::BitReader;
#[doc = "Field `WWDT` writer - Window watchdog timer reset"]
pub type WWDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2` reader - SPI2 reset"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI2` writer - SPI2 reset"]
pub type SPI2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2` reader - USART 2 reset"]
pub type USART2_R = crate::BitReader;
#[doc = "Field `USART2` writer - USART 2 reset"]
pub type USART2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3` reader - USART 3 reset"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART3` writer - USART 3 reset"]
pub type USART3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART5` reader - UART 5 reset"]
pub type UART5_R = crate::BitReader;
#[doc = "Field `UART5` writer - UART 5 reset"]
pub type UART5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2C1_R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1` reader - CAN1 reset"]
pub type CAN1_R = crate::BitReader;
#[doc = "Field `CAN1` writer - CAN1 reset"]
pub type CAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWC` reader - Power controller reset"]
pub type PWC_R = crate::BitReader;
#[doc = "Field `PWC` writer - Power controller reset"]
pub type PWC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 9 - Comparator reset"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 25) & 1) != 0)
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
            .field("tmr4", &format_args!("{}", self.tmr4().bit()))
            .field("tmr5", &format_args!("{}", self.tmr5().bit()))
            .field("cmp", &format_args!("{}", self.cmp().bit()))
            .field("wwdt", &format_args!("{}", self.wwdt().bit()))
            .field("spi2", &format_args!("{}", self.spi2().bit()))
            .field("usart2", &format_args!("{}", self.usart2().bit()))
            .field("usart3", &format_args!("{}", self.usart3().bit()))
            .field("uart5", &format_args!("{}", self.uart5().bit()))
            .field("i2c1", &format_args!("{}", self.i2c1().bit()))
            .field("can1", &format_args!("{}", self.can1().bit()))
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
    #[doc = "Bit 9 - Comparator reset"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<APB1RST_SPEC, 9> {
        CMP_W::new(self)
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
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<APB1RST_SPEC, 25> {
        CAN1_W::new(self)
    }
    #[doc = "Bit 28 - Power controller reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwc(&mut self) -> PWC_W<APB1RST_SPEC, 28> {
        PWC_W::new(self)
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
