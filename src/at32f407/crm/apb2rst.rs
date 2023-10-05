#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `IOMUX` reader - MUX function I/O reset"]
pub type IOMUX_R = crate::BitReader;
#[doc = "Field `IOMUX` writer - MUX function I/O reset"]
pub type IOMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXINT` reader - External interrupt reset"]
pub type EXINT_R = crate::BitReader;
#[doc = "Field `EXINT` writer - External interrupt reset"]
pub type EXINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOA` reader - IO port A reset"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - IO port A reset"]
pub type GPIOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOB` reader - IO port B reset"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - IO port B reset"]
pub type GPIOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOC` reader - IO port C reset"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - IO port C reset"]
pub type GPIOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOD` reader - IO port D reset"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - IO port D reset"]
pub type GPIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOE` reader - IO port E reset"]
pub type GPIOE_R = crate::BitReader;
#[doc = "Field `GPIOE` writer - IO port E reset"]
pub type GPIOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1` reader - ADC1 reset"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 reset"]
pub type ADC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2` reader - ADC2 reset"]
pub type ADC2_R = crate::BitReader;
#[doc = "Field `ADC2` writer - ADC2 reset"]
pub type ADC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR1` reader - Timer1 reset"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - Timer1 reset"]
pub type TMR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub type SPI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR8` reader - Timer8 reset"]
pub type TMR8_R = crate::BitReader;
#[doc = "Field `TMR8` writer - Timer8 reset"]
pub type TMR8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1` reader - USART1 reset"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 reset"]
pub type USART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC3` reader - ADC3 reset"]
pub type ADC3_R = crate::BitReader;
#[doc = "Field `ADC3` writer - ADC3 reset"]
pub type ADC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR9` reader - Timer9 reset"]
pub type TMR9_R = crate::BitReader;
#[doc = "Field `TMR9` writer - Timer9 reset"]
pub type TMR9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR10` reader - Timer10 reset"]
pub type TMR10_R = crate::BitReader;
#[doc = "Field `TMR10` writer - Timer10 reset"]
pub type TMR10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR11` reader - Timer11 reset"]
pub type TMR11_R = crate::BitReader;
#[doc = "Field `TMR11` writer - Timer11 reset"]
pub type TMR11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACC` reader - ACC reset"]
pub type ACC_R = crate::BitReader;
#[doc = "Field `ACC` writer - ACC reset"]
pub type ACC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C3` reader - I2C3 reset"]
pub type I2C3_R = crate::BitReader;
#[doc = "Field `I2C3` writer - I2C3 reset"]
pub type I2C3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6` reader - USART6 reset"]
pub type USART6_R = crate::BitReader;
#[doc = "Field `USART6` writer - USART6 reset"]
pub type USART6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART7` reader - UART7 reset"]
pub type UART7_R = crate::BitReader;
#[doc = "Field `UART7` writer - UART7 reset"]
pub type UART7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART8` reader - UART8 reset"]
pub type UART8_R = crate::BitReader;
#[doc = "Field `UART8` writer - UART8 reset"]
pub type UART8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exint(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 reset"]
    #[inline(always)]
    pub fn adc2(&self) -> ADC2_R {
        ADC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer8 reset"]
    #[inline(always)]
    pub fn tmr8(&self) -> TMR8_R {
        TMR8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC3 reset"]
    #[inline(always)]
    pub fn adc3(&self) -> ADC3_R {
        ADC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9(&self) -> TMR9_R {
        TMR9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10(&self) -> TMR10_R {
        TMR10_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    pub fn tmr11(&self) -> TMR11_R {
        TMR11_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - USART6 reset"]
    #[inline(always)]
    pub fn usart6(&self) -> USART6_R {
        USART6_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UART7 reset"]
    #[inline(always)]
    pub fn uart7(&self) -> UART7_R {
        UART7_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - UART8 reset"]
    #[inline(always)]
    pub fn uart8(&self) -> UART8_R {
        UART8_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn iomux(&mut self) -> IOMUX_W<APB2RST_SPEC, 0> {
        IOMUX_W::new(self)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn exint(&mut self) -> EXINT_W<APB2RST_SPEC, 1> {
        EXINT_W::new(self)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<APB2RST_SPEC, 2> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<APB2RST_SPEC, 3> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<APB2RST_SPEC, 4> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<APB2RST_SPEC, 5> {
        GPIOD_W::new(self)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioe(&mut self) -> GPIOE_W<APB2RST_SPEC, 6> {
        GPIOE_W::new(self)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<APB2RST_SPEC, 9> {
        ADC1_W::new(self)
    }
    #[doc = "Bit 10 - ADC2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc2(&mut self) -> ADC2_W<APB2RST_SPEC, 10> {
        ADC2_W::new(self)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<APB2RST_SPEC, 11> {
        TMR1_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<APB2RST_SPEC, 12> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 13 - Timer8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8(&mut self) -> TMR8_W<APB2RST_SPEC, 13> {
        TMR8_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<APB2RST_SPEC, 14> {
        USART1_W::new(self)
    }
    #[doc = "Bit 15 - ADC3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc3(&mut self) -> ADC3_W<APB2RST_SPEC, 15> {
        ADC3_W::new(self)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9(&mut self) -> TMR9_W<APB2RST_SPEC, 19> {
        TMR9_W::new(self)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10(&mut self) -> TMR10_W<APB2RST_SPEC, 20> {
        TMR10_W::new(self)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11(&mut self) -> TMR11_W<APB2RST_SPEC, 21> {
        TMR11_W::new(self)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    #[must_use]
    pub fn acc(&mut self) -> ACC_W<APB2RST_SPEC, 22> {
        ACC_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3(&mut self) -> I2C3_W<APB2RST_SPEC, 23> {
        I2C3_W::new(self)
    }
    #[doc = "Bit 24 - USART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6(&mut self) -> USART6_W<APB2RST_SPEC, 24> {
        USART6_W::new(self)
    }
    #[doc = "Bit 25 - UART7 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart7(&mut self) -> UART7_W<APB2RST_SPEC, 25> {
        UART7_W::new(self)
    }
    #[doc = "Bit 26 - UART8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart8(&mut self) -> UART8_W<APB2RST_SPEC, 26> {
        UART8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
