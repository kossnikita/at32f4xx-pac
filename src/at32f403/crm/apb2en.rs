#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `IOMUX` reader - MUX function I/O clock enable"]
pub type IOMUX_R = crate::BitReader;
#[doc = "Field `IOMUX` writer - MUX function I/O clock enable"]
pub type IOMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOA` reader - I/O port A clock enable"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - I/O port A clock enable"]
pub type GPIOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOB` reader - I/O port B clock enable"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - I/O port B clock enable"]
pub type GPIOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOC` reader - I/O port C clock enable"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - I/O port C clock enable"]
pub type GPIOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOD` reader - I/O port D clock enable"]
pub type GPIOD_R = crate::BitReader;
#[doc = "Field `GPIOD` writer - I/O port D clock enable"]
pub type GPIOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOE` reader - I/O port E clock enable"]
pub type GPIOE_R = crate::BitReader;
#[doc = "Field `GPIOE` writer - I/O port E clock enable"]
pub type GPIOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOF` reader - I/O port F clock enable"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - I/O port F clock enable"]
pub type GPIOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOG` reader - I/O port G clock enable"]
pub type GPIOG_R = crate::BitReader;
#[doc = "Field `GPIOG` writer - I/O port G clock enable"]
pub type GPIOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1` reader - ADC1 clock enable"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 clock enable"]
pub type ADC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2` reader - ADC2 clock enable"]
pub type ADC2_R = crate::BitReader;
#[doc = "Field `ADC2` writer - ADC2 clock enable"]
pub type ADC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR1` reader - Timer1 clock enable"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - Timer1 clock enable"]
pub type TMR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1` reader - SPI1 clock enable"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 clock enable"]
pub type SPI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR8` reader - Timer8 clock enable"]
pub type TMR8_R = crate::BitReader;
#[doc = "Field `TMR8` writer - Timer8 clock enable"]
pub type TMR8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1` reader - USART1 clock enable"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 clock enable"]
pub type USART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC3` reader - ADC3 clock enable"]
pub type ADC3_R = crate::BitReader;
#[doc = "Field `ADC3` writer - ADC3 clock enable"]
pub type ADC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR15` reader - Timer15 clock enable"]
pub type TMR15_R = crate::BitReader;
#[doc = "Field `TMR15` writer - Timer15 clock enable"]
pub type TMR15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR9` reader - Timer9 clock enable"]
pub type TMR9_R = crate::BitReader;
#[doc = "Field `TMR9` writer - Timer9 clock enable"]
pub type TMR9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR10` reader - Timer10 clock enable"]
pub type TMR10_R = crate::BitReader;
#[doc = "Field `TMR10` writer - Timer10 clock enable"]
pub type TMR10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR11` reader - Timer11 clock enable"]
pub type TMR11_R = crate::BitReader;
#[doc = "Field `TMR11` writer - Timer11 clock enable"]
pub type TMR11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - MUX function I/O clock enable"]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2(&self) -> ADC2_R {
        ADC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer8 clock enable"]
    #[inline(always)]
    pub fn tmr8(&self) -> TMR8_R {
        TMR8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3(&self) -> ADC3_R {
        ADC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    pub fn tmr15(&self) -> TMR15_R {
        TMR15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer9 clock enable"]
    #[inline(always)]
    pub fn tmr9(&self) -> TMR9_R {
        TMR9_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer10 clock enable"]
    #[inline(always)]
    pub fn tmr10(&self) -> TMR10_R {
        TMR10_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer11 clock enable"]
    #[inline(always)]
    pub fn tmr11(&self) -> TMR11_R {
        TMR11_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2EN")
            .field("iomux", &format_args!("{}", self.iomux().bit()))
            .field("gpioa", &format_args!("{}", self.gpioa().bit()))
            .field("gpiob", &format_args!("{}", self.gpiob().bit()))
            .field("gpioc", &format_args!("{}", self.gpioc().bit()))
            .field("gpiod", &format_args!("{}", self.gpiod().bit()))
            .field("gpioe", &format_args!("{}", self.gpioe().bit()))
            .field("gpiof", &format_args!("{}", self.gpiof().bit()))
            .field("gpiog", &format_args!("{}", self.gpiog().bit()))
            .field("adc1", &format_args!("{}", self.adc1().bit()))
            .field("adc2", &format_args!("{}", self.adc2().bit()))
            .field("tmr1", &format_args!("{}", self.tmr1().bit()))
            .field("spi1", &format_args!("{}", self.spi1().bit()))
            .field("tmr8", &format_args!("{}", self.tmr8().bit()))
            .field("usart1", &format_args!("{}", self.usart1().bit()))
            .field("adc3", &format_args!("{}", self.adc3().bit()))
            .field("tmr15", &format_args!("{}", self.tmr15().bit()))
            .field("tmr9", &format_args!("{}", self.tmr9().bit()))
            .field("tmr10", &format_args!("{}", self.tmr10().bit()))
            .field("tmr11", &format_args!("{}", self.tmr11().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB2EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - MUX function I/O clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn iomux(&mut self) -> IOMUX_W<APB2EN_SPEC, 0> {
        IOMUX_W::new(self)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<APB2EN_SPEC, 2> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<APB2EN_SPEC, 3> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<APB2EN_SPEC, 4> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiod(&mut self) -> GPIOD_W<APB2EN_SPEC, 5> {
        GPIOD_W::new(self)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioe(&mut self) -> GPIOE_W<APB2EN_SPEC, 6> {
        GPIOE_W::new(self)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<APB2EN_SPEC, 7> {
        GPIOF_W::new(self)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiog(&mut self) -> GPIOG_W<APB2EN_SPEC, 8> {
        GPIOG_W::new(self)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<APB2EN_SPEC, 9> {
        ADC1_W::new(self)
    }
    #[doc = "Bit 10 - ADC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2(&mut self) -> ADC2_W<APB2EN_SPEC, 10> {
        ADC2_W::new(self)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<APB2EN_SPEC, 11> {
        TMR1_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<APB2EN_SPEC, 12> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 13 - Timer8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8(&mut self) -> TMR8_W<APB2EN_SPEC, 13> {
        TMR8_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<APB2EN_SPEC, 14> {
        USART1_W::new(self)
    }
    #[doc = "Bit 15 - ADC3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3(&mut self) -> ADC3_W<APB2EN_SPEC, 15> {
        ADC3_W::new(self)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15(&mut self) -> TMR15_W<APB2EN_SPEC, 16> {
        TMR15_W::new(self)
    }
    #[doc = "Bit 19 - Timer9 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9(&mut self) -> TMR9_W<APB2EN_SPEC, 19> {
        TMR9_W::new(self)
    }
    #[doc = "Bit 20 - Timer10 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10(&mut self) -> TMR10_W<APB2EN_SPEC, 20> {
        TMR10_W::new(self)
    }
    #[doc = "Bit 21 - Timer11 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11(&mut self) -> TMR11_W<APB2EN_SPEC, 21> {
        TMR11_W::new(self)
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
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2EN_SPEC;
impl crate::RegisterSpec for APB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for APB2EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for APB2EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for APB2EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
