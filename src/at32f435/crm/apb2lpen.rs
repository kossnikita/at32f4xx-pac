#[doc = "Register `APB2LPEN` reader"]
pub type R = crate::R<APB2LPEN_SPEC>;
#[doc = "Register `APB2LPEN` writer"]
pub type W = crate::W<APB2LPEN_SPEC>;
#[doc = "Field `TMR1LPEN` reader - Timer1 clock enable during sleep mode"]
pub type TMR1LPEN_R = crate::BitReader;
#[doc = "Field `TMR1LPEN` writer - Timer1 clock enable during sleep mode"]
pub type TMR1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR8LPEN` reader - Timer8 clock enable during sleep mode"]
pub type TMR8LPEN_R = crate::BitReader;
#[doc = "Field `TMR8LPEN` writer - Timer8 clock enable during sleep mode"]
pub type TMR8LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during sleep mode"]
pub type USART1LPEN_R = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during sleep mode"]
pub type USART1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during sleep mode"]
pub type USART6LPEN_R = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during sleep mode"]
pub type USART6LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1CPEN` reader - ADC1 clock enable during sleep mode"]
pub type ADC1CPEN_R = crate::BitReader;
#[doc = "Field `ADC1CPEN` writer - ADC1 clock enable during sleep mode"]
pub type ADC1CPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2CPEN` reader - ADC2 clock enable during sleep mode"]
pub type ADC2CPEN_R = crate::BitReader;
#[doc = "Field `ADC2CPEN` writer - ADC2 clock enable during sleep mode"]
pub type ADC2CPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable during sleep mode"]
pub type ADC3EN_R = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable during sleep mode"]
pub type ADC3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1LPEN` reader - SPI1 clock enable during sleep mode"]
pub type SPI1LPEN_R = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI1 clock enable during sleep mode"]
pub type SPI1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4LPEN` reader - SPI4 clock enable during sleep mode"]
pub type SPI4LPEN_R = crate::BitReader;
#[doc = "Field `SPI4LPEN` writer - SPI4 clock enable during sleep mode"]
pub type SPI4LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCFGLPEN` reader - SCFG clock enable during sleep mode"]
pub type SCFGLPEN_R = crate::BitReader;
#[doc = "Field `SCFGLPEN` writer - SCFG clock enable during sleep mode"]
pub type SCFGLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR9LPEN` reader - Timer9 clock enable during sleep mode"]
pub type TMR9LPEN_R = crate::BitReader;
#[doc = "Field `TMR9LPEN` writer - Timer9 clock enable during sleep mode"]
pub type TMR9LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR10LPEN` reader - Timer10 clock enable during sleep mode"]
pub type TMR10LPEN_R = crate::BitReader;
#[doc = "Field `TMR10LPEN` writer - Timer10 clock enable during sleep mode"]
pub type TMR10LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR11LPEN` reader - Timer11 clock enable during sleep mode"]
pub type TMR11LPEN_R = crate::BitReader;
#[doc = "Field `TMR11LPEN` writer - Timer11 clock enable during sleep mode"]
pub type TMR11LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR20LPEN` reader - Timer20 clock enable during sleep mode"]
pub type TMR20LPEN_R = crate::BitReader;
#[doc = "Field `TMR20LPEN` writer - Timer20 clock enable during sleep mode"]
pub type TMR20LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCLPEN` reader - ACC clock enable during sleep mode"]
pub type ACCLPEN_R = crate::BitReader;
#[doc = "Field `ACCLPEN` writer - ACC clock enable during sleep mode"]
pub type ACCLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr1lpen(&self) -> TMR1LPEN_R {
        TMR1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer8 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr8lpen(&self) -> TMR8LPEN_R {
        TMR8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn adc1cpen(&self) -> ADC1CPEN_R {
        ADC1CPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn adc2cpen(&self) -> ADC2CPEN_R {
        ADC2CPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG clock enable during sleep mode"]
    #[inline(always)]
    pub fn scfglpen(&self) -> SCFGLPEN_R {
        SCFGLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr9lpen(&self) -> TMR9LPEN_R {
        TMR9LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr10lpen(&self) -> TMR10LPEN_R {
        TMR10LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer11 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr11lpen(&self) -> TMR11LPEN_R {
        TMR11LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer20 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr20lpen(&self) -> TMR20LPEN_R {
        TMR20LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    pub fn acclpen(&self) -> ACCLPEN_R {
        ACCLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1lpen(&mut self) -> TMR1LPEN_W<APB2LPEN_SPEC, 0> {
        TMR1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - Timer8 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8lpen(&mut self) -> TMR8LPEN_W<APB2LPEN_SPEC, 1> {
        TMR8LPEN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<APB2LPEN_SPEC, 4> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<APB2LPEN_SPEC, 5> {
        USART6LPEN_W::new(self)
    }
    #[doc = "Bit 8 - ADC1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc1cpen(&mut self) -> ADC1CPEN_W<APB2LPEN_SPEC, 8> {
        ADC1CPEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc2cpen(&mut self) -> ADC2CPEN_W<APB2LPEN_SPEC, 9> {
        ADC2CPEN_W::new(self)
    }
    #[doc = "Bit 10 - ADC3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> ADC3EN_W<APB2LPEN_SPEC, 10> {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<APB2LPEN_SPEC, 12> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<APB2LPEN_SPEC, 13> {
        SPI4LPEN_W::new(self)
    }
    #[doc = "Bit 14 - SCFG clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn scfglpen(&mut self) -> SCFGLPEN_W<APB2LPEN_SPEC, 14> {
        SCFGLPEN_W::new(self)
    }
    #[doc = "Bit 16 - Timer9 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9lpen(&mut self) -> TMR9LPEN_W<APB2LPEN_SPEC, 16> {
        TMR9LPEN_W::new(self)
    }
    #[doc = "Bit 17 - Timer10 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10lpen(&mut self) -> TMR10LPEN_W<APB2LPEN_SPEC, 17> {
        TMR10LPEN_W::new(self)
    }
    #[doc = "Bit 18 - Timer11 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11lpen(&mut self) -> TMR11LPEN_W<APB2LPEN_SPEC, 18> {
        TMR11LPEN_W::new(self)
    }
    #[doc = "Bit 20 - Timer20 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20lpen(&mut self) -> TMR20LPEN_W<APB2LPEN_SPEC, 20> {
        TMR20LPEN_W::new(self)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn acclpen(&mut self) -> ACCLPEN_W<APB2LPEN_SPEC, 29> {
        ACCLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2LPEN_SPEC;
impl crate::RegisterSpec for APB2LPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpen::R`](R) reader structure"]
impl crate::Readable for APB2LPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2lpen::W`](W) writer structure"]
impl crate::Writable for APB2LPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2LPEN to value 0x2017_7733"]
impl crate::Resettable for APB2LPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x2017_7733;
}
