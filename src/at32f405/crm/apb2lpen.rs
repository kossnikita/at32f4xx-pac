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
#[doc = "Field `I2S5LPEN` reader - I2S5 clock enable during sleep mode"]
pub type I2S5LPEN_R = crate::BitReader;
#[doc = "Field `I2S5LPEN` writer - I2S5 clock enable during sleep mode"]
pub type I2S5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCLPEN` reader - ACC clock enable during sleep mode"]
pub type ACCLPEN_R = crate::BitReader;
#[doc = "Field `ACCLPEN` writer - ACC clock enable during sleep mode"]
pub type ACCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGHSPHYLPEN` reader - OTGHS phy clock enable during sleep mode"]
pub type OTGHSPHYLPEN_R = crate::BitReader;
#[doc = "Field `OTGHSPHYLPEN` writer - OTGHS phy clock enable during sleep mode"]
pub type OTGHSPHYLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 20 - I2S5 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2s5lpen(&self) -> I2S5LPEN_R {
        I2S5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    pub fn acclpen(&self) -> ACCLPEN_R {
        ACCLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - OTGHS phy clock enable during sleep mode"]
    #[inline(always)]
    pub fn otghsphylpen(&self) -> OTGHSPHYLPEN_R {
        OTGHSPHYLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPEN")
            .field("tmr1lpen", &format_args!("{}", self.tmr1lpen().bit()))
            .field("usart1lpen", &format_args!("{}", self.usart1lpen().bit()))
            .field("usart6lpen", &format_args!("{}", self.usart6lpen().bit()))
            .field("spi1lpen", &format_args!("{}", self.spi1lpen().bit()))
            .field("scfglpen", &format_args!("{}", self.scfglpen().bit()))
            .field("tmr9lpen", &format_args!("{}", self.tmr9lpen().bit()))
            .field("tmr10lpen", &format_args!("{}", self.tmr10lpen().bit()))
            .field("tmr11lpen", &format_args!("{}", self.tmr11lpen().bit()))
            .field("i2s5lpen", &format_args!("{}", self.i2s5lpen().bit()))
            .field("acclpen", &format_args!("{}", self.acclpen().bit()))
            .field(
                "otghsphylpen",
                &format_args!("{}", self.otghsphylpen().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB2LPEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1lpen(&mut self) -> TMR1LPEN_W<APB2LPEN_SPEC> {
        TMR1LPEN_W::new(self, 0)
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
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<APB2LPEN_SPEC> {
        SPI1LPEN_W::new(self, 12)
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
    #[doc = "Bit 20 - I2S5 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2s5lpen(&mut self) -> I2S5LPEN_W<APB2LPEN_SPEC> {
        I2S5LPEN_W::new(self, 20)
    }
    #[doc = "Bit 29 - ACC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn acclpen(&mut self) -> ACCLPEN_W<APB2LPEN_SPEC> {
        ACCLPEN_W::new(self, 29)
    }
    #[doc = "Bit 31 - OTGHS phy clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otghsphylpen(&mut self) -> OTGHSPHYLPEN_W<APB2LPEN_SPEC> {
        OTGHSPHYLPEN_W::new(self, 31)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2LPEN to value 0x2017_7733"]
impl crate::Resettable for APB2LPEN_SPEC {
    const RESET_VALUE: u32 = 0x2017_7733;
}
