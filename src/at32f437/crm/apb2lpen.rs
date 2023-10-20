#[doc = "Register `APB2LPEN` reader"]
pub type R = crate::R<APB2LPEN_SPEC>;
#[doc = "Register `APB2LPEN` writer"]
pub type W = crate::W<APB2LPEN_SPEC>;
#[doc = "Field `TMR1LPEN` reader - Timer1 clock enable during sleep mode"]
pub type TMR1LPEN_R = crate::BitReader;
#[doc = "Field `TMR1LPEN` writer - Timer1 clock enable during sleep mode"]
pub type TMR1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8LPEN` reader - Timer8 clock enable during sleep mode"]
pub type TMR8LPEN_R = crate::BitReader;
#[doc = "Field `TMR8LPEN` writer - Timer8 clock enable during sleep mode"]
pub type TMR8LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during sleep mode"]
pub type USART1LPEN_R = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during sleep mode"]
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during sleep mode"]
pub type USART6LPEN_R = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during sleep mode"]
pub type USART6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1CPEN` reader - ADC1 clock enable during sleep mode"]
pub type ADC1CPEN_R = crate::BitReader;
#[doc = "Field `ADC1CPEN` writer - ADC1 clock enable during sleep mode"]
pub type ADC1CPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2CPEN` reader - ADC2 clock enable during sleep mode"]
pub type ADC2CPEN_R = crate::BitReader;
#[doc = "Field `ADC2CPEN` writer - ADC2 clock enable during sleep mode"]
pub type ADC2CPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable during sleep mode"]
pub type ADC3EN_R = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable during sleep mode"]
pub type ADC3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1LPEN` reader - SPI1 clock enable during sleep mode"]
pub type SPI1LPEN_R = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI1 clock enable during sleep mode"]
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4LPEN` reader - SPI4 clock enable during sleep mode"]
pub type SPI4LPEN_R = crate::BitReader;
#[doc = "Field `SPI4LPEN` writer - SPI4 clock enable during sleep mode"]
pub type SPI4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFGLPEN` reader - SCFG clock enable during sleep mode"]
pub type SCFGLPEN_R = crate::BitReader;
#[doc = "Field `SCFGLPEN` writer - SCFG clock enable during sleep mode"]
pub type SCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9LPEN` reader - Timer9 clock enable during sleep mode"]
pub type TMR9LPEN_R = crate::BitReader;
#[doc = "Field `TMR9LPEN` writer - Timer9 clock enable during sleep mode"]
pub type TMR9LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10LPEN` reader - Timer10 clock enable during sleep mode"]
pub type TMR10LPEN_R = crate::BitReader;
#[doc = "Field `TMR10LPEN` writer - Timer10 clock enable during sleep mode"]
pub type TMR10LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11LPEN` reader - Timer11 clock enable during sleep mode"]
pub type TMR11LPEN_R = crate::BitReader;
#[doc = "Field `TMR11LPEN` writer - Timer11 clock enable during sleep mode"]
pub type TMR11LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20LPEN` reader - Timer20 clock enable during sleep mode"]
pub type TMR20LPEN_R = crate::BitReader;
#[doc = "Field `TMR20LPEN` writer - Timer20 clock enable during sleep mode"]
pub type TMR20LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCLPEN` reader - ACC clock enable during sleep mode"]
pub type ACCLPEN_R = crate::BitReader;
#[doc = "Field `ACCLPEN` writer - ACC clock enable during sleep mode"]
pub type ACCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPEN")
            .field("tmr1lpen", &format_args!("{}", self.tmr1lpen().bit()))
            .field("tmr8lpen", &format_args!("{}", self.tmr8lpen().bit()))
            .field("usart1lpen", &format_args!("{}", self.usart1lpen().bit()))
            .field("usart6lpen", &format_args!("{}", self.usart6lpen().bit()))
            .field("adc1cpen", &format_args!("{}", self.adc1cpen().bit()))
            .field("adc2cpen", &format_args!("{}", self.adc2cpen().bit()))
            .field("adc3en", &format_args!("{}", self.adc3en().bit()))
            .field("spi1lpen", &format_args!("{}", self.spi1lpen().bit()))
            .field("spi4lpen", &format_args!("{}", self.spi4lpen().bit()))
            .field("scfglpen", &format_args!("{}", self.scfglpen().bit()))
            .field("tmr9lpen", &format_args!("{}", self.tmr9lpen().bit()))
            .field("tmr10lpen", &format_args!("{}", self.tmr10lpen().bit()))
            .field("tmr11lpen", &format_args!("{}", self.tmr11lpen().bit()))
            .field("tmr20lpen", &format_args!("{}", self.tmr20lpen().bit()))
            .field("acclpen", &format_args!("{}", self.acclpen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB2LPEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1lpen(&mut self) -> TMR1LPEN_W<APB2LPEN_SPEC> {
        TMR1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer8 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8lpen(&mut self) -> TMR8LPEN_W<APB2LPEN_SPEC> {
        TMR8LPEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<APB2LPEN_SPEC> {
        USART1LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<APB2LPEN_SPEC> {
        USART6LPEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc1cpen(&mut self) -> ADC1CPEN_W<APB2LPEN_SPEC> {
        ADC1CPEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc2cpen(&mut self) -> ADC2CPEN_W<APB2LPEN_SPEC> {
        ADC2CPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> ADC3EN_W<APB2LPEN_SPEC> {
        ADC3EN_W::new(self, 10)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<APB2LPEN_SPEC> {
        SPI1LPEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<APB2LPEN_SPEC> {
        SPI4LPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SCFG clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn scfglpen(&mut self) -> SCFGLPEN_W<APB2LPEN_SPEC> {
        SCFGLPEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9lpen(&mut self) -> TMR9LPEN_W<APB2LPEN_SPEC> {
        TMR9LPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10lpen(&mut self) -> TMR10LPEN_W<APB2LPEN_SPEC> {
        TMR10LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer11 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11lpen(&mut self) -> TMR11LPEN_W<APB2LPEN_SPEC> {
        TMR11LPEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - Timer20 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20lpen(&mut self) -> TMR20LPEN_W<APB2LPEN_SPEC> {
        TMR20LPEN_W::new(self, 20)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn acclpen(&mut self) -> ACCLPEN_W<APB2LPEN_SPEC> {
        ACCLPEN_W::new(self, 29)
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
