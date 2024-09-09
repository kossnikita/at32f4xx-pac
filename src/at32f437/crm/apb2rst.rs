#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `TMR1` reader - Timer1 reset"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - Timer1 reset"]
pub type TMR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8` reader - Timer8 reset"]
pub type TMR8_R = crate::BitReader;
#[doc = "Field `TMR8` writer - Timer8 reset"]
pub type TMR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - USART1 reset"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 reset"]
pub type USART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6` reader - USART6 reset"]
pub type USART6_R = crate::BitReader;
#[doc = "Field `USART6` writer - USART6 reset"]
pub type USART6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC` reader - ADC reset"]
pub type ADC_R = crate::BitReader;
#[doc = "Field `ADC` writer - ADC reset"]
pub type ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub type SPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4` reader - SPI4 reset"]
pub type SPI4_R = crate::BitReader;
#[doc = "Field `SPI4` writer - SPI4 reset"]
pub type SPI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFG` reader - SCFG reset"]
pub type SCFG_R = crate::BitReader;
#[doc = "Field `SCFG` writer - SCFG reset"]
pub type SCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9` reader - Timer9 reset"]
pub type TMR9_R = crate::BitReader;
#[doc = "Field `TMR9` writer - Timer9 reset"]
pub type TMR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10` reader - Timer10 reset"]
pub type TMR10_R = crate::BitReader;
#[doc = "Field `TMR10` writer - Timer10 reset"]
pub type TMR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11` reader - Timer 11 reset"]
pub type TMR11_R = crate::BitReader;
#[doc = "Field `TMR11` writer - Timer 11 reset"]
pub type TMR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR20` reader - Timer20 reset"]
pub type TMR20_R = crate::BitReader;
#[doc = "Field `TMR20` writer - Timer20 reset"]
pub type TMR20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC` reader - ACC reset"]
pub type ACC_R = crate::BitReader;
#[doc = "Field `ACC` writer - ACC reset"]
pub type ACC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer8 reset"]
    #[inline(always)]
    pub fn tmr8(&self) -> TMR8_R {
        TMR8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6(&self) -> USART6_R {
        USART6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4(&self) -> SPI4_R {
        SPI4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    pub fn scfg(&self) -> SCFG_R {
        SCFG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9(&self) -> TMR9_R {
        TMR9_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10(&self) -> TMR10_R {
        TMR10_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    pub fn tmr11(&self) -> TMR11_R {
        TMR11_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer20 reset"]
    #[inline(always)]
    pub fn tmr20(&self) -> TMR20_R {
        TMR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RST")
            .field("tmr1", &self.tmr1())
            .field("tmr8", &self.tmr8())
            .field("usart1", &self.usart1())
            .field("usart6", &self.usart6())
            .field("adc", &self.adc())
            .field("spi1", &self.spi1())
            .field("spi4", &self.spi4())
            .field("scfg", &self.scfg())
            .field("tmr9", &self.tmr9())
            .field("tmr10", &self.tmr10())
            .field("tmr11", &self.tmr11())
            .field("tmr20", &self.tmr20())
            .field("acc", &self.acc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<APB2RST_SPEC> {
        TMR1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8(&mut self) -> TMR8_W<APB2RST_SPEC> {
        TMR8_W::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<APB2RST_SPEC> {
        USART1_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6(&mut self) -> USART6_W<APB2RST_SPEC> {
        USART6_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<APB2RST_SPEC> {
        ADC_W::new(self, 8)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<APB2RST_SPEC> {
        SPI1_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4(&mut self) -> SPI4_W<APB2RST_SPEC> {
        SPI4_W::new(self, 13)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    #[must_use]
    pub fn scfg(&mut self) -> SCFG_W<APB2RST_SPEC> {
        SCFG_W::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9(&mut self) -> TMR9_W<APB2RST_SPEC> {
        TMR9_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10(&mut self) -> TMR10_W<APB2RST_SPEC> {
        TMR10_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11(&mut self) -> TMR11_W<APB2RST_SPEC> {
        TMR11_W::new(self, 18)
    }
    #[doc = "Bit 20 - Timer20 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20(&mut self) -> TMR20_W<APB2RST_SPEC> {
        TMR20_W::new(self, 20)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    #[must_use]
    pub fn acc(&mut self) -> ACC_W<APB2RST_SPEC> {
        ACC_W::new(self, 29)
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
