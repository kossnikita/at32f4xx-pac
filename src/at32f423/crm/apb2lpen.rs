#[doc = "Register `APB2LPEN` reader"]
pub type R = crate::R<APB2LPEN_SPEC>;
#[doc = "Register `APB2LPEN` writer"]
pub type W = crate::W<APB2LPEN_SPEC>;
#[doc = "Field `TMR1LPEN` reader - Timer1 clock enable during sleep mode"]
pub type TMR1LPEN_R = crate::BitReader;
#[doc = "Field `TMR1LPEN` writer - Timer1 clock enable during sleep mode"]
pub type TMR1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during sleep mode"]
pub type USART1LPEN_R = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during sleep mode"]
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during sleep mode"]
pub type USART6LPEN_R = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during sleep mode"]
pub type USART6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCLPEN` reader - ADC clock enable during sleep mode"]
pub type ADCLPEN_R = crate::BitReader;
#[doc = "Field `ADCLPEN` writer - ADC clock enable during sleep mode"]
pub type ADCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1LPEN` reader - SPI1 clock enable during sleep mode"]
pub type SPI1LPEN_R = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI1 clock enable during sleep mode"]
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - ADC clock enable during sleep mode"]
    #[inline(always)]
    pub fn adclpen(&self) -> ADCLPEN_R {
        ADCLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
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
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    pub fn acclpen(&self) -> ACCLPEN_R {
        ACCLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPEN")
            .field("tmr1lpen", &self.tmr1lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("usart6lpen", &self.usart6lpen())
            .field("adclpen", &self.adclpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("scfglpen", &self.scfglpen())
            .field("tmr9lpen", &self.tmr9lpen())
            .field("tmr10lpen", &self.tmr10lpen())
            .field("tmr11lpen", &self.tmr11lpen())
            .field("acclpen", &self.acclpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr1lpen(&mut self) -> TMR1LPEN_W<'_, APB2LPEN_SPEC> {
        TMR1LPEN_W::new(self, 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<'_, APB2LPEN_SPEC> {
        USART1LPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<'_, APB2LPEN_SPEC> {
        USART6LPEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC clock enable during sleep mode"]
    #[inline(always)]
    pub fn adclpen(&mut self) -> ADCLPEN_W<'_, APB2LPEN_SPEC> {
        ADCLPEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<'_, APB2LPEN_SPEC> {
        SPI1LPEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - SCFG clock enable during sleep mode"]
    #[inline(always)]
    pub fn scfglpen(&mut self) -> SCFGLPEN_W<'_, APB2LPEN_SPEC> {
        SCFGLPEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr9lpen(&mut self) -> TMR9LPEN_W<'_, APB2LPEN_SPEC> {
        TMR9LPEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr10lpen(&mut self) -> TMR10LPEN_W<'_, APB2LPEN_SPEC> {
        TMR10LPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer11 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr11lpen(&mut self) -> TMR11LPEN_W<'_, APB2LPEN_SPEC> {
        TMR11LPEN_W::new(self, 18)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    pub fn acclpen(&mut self) -> ACCLPEN_W<'_, APB2LPEN_SPEC> {
        ACCLPEN_W::new(self, 29)
    }
}
#[doc = "APB2 peripheral Low-power clock enable register (CRM_APB2LPEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2lpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2LPEN_SPEC;
impl crate::RegisterSpec for APB2LPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpen::R`](R) reader structure"]
impl crate::Readable for APB2LPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2lpen::W`](W) writer structure"]
impl crate::Writable for APB2LPEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2LPEN to value 0x2017_7733"]
impl crate::Resettable for APB2LPEN_SPEC {
    const RESET_VALUE: u32 = 0x2017_7733;
}
