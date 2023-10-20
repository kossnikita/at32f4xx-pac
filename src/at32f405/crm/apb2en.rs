#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `TMR1` reader - Timer1 clock enable"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - Timer1 clock enable"]
pub type TMR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - USART1 clock enable"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 clock enable"]
pub type USART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6` reader - USART6 clock enable"]
pub type USART6_R = crate::BitReader;
#[doc = "Field `USART6` writer - USART6 clock enable"]
pub type USART6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 clock enable"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 clock enable"]
pub type ADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 clock enable"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 clock enable"]
pub type SPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFG` reader - SCFG clock enable"]
pub type SCFG_R = crate::BitReader;
#[doc = "Field `SCFG` writer - SCFG clock enable"]
pub type SCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9` reader - Timer9 clock enable"]
pub type TMR9_R = crate::BitReader;
#[doc = "Field `TMR9` writer - Timer9 clock enable"]
pub type TMR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10` reader - Timer10 clock enable"]
pub type TMR10_R = crate::BitReader;
#[doc = "Field `TMR10` writer - Timer10 clock enable"]
pub type TMR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11` reader - Timer11 clock enable"]
pub type TMR11_R = crate::BitReader;
#[doc = "Field `TMR11` writer - Timer11 clock enable"]
pub type TMR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S5` reader - I2S5 clock enable"]
pub type I2S5_R = crate::BitReader;
#[doc = "Field `I2S5` writer - I2S5 clock enable"]
pub type I2S5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC` reader - ACC clock enable"]
pub type ACC_R = crate::BitReader;
#[doc = "Field `ACC` writer - ACC clock enable"]
pub type ACC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6(&self) -> USART6_R {
        USART6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG clock enable"]
    #[inline(always)]
    pub fn scfg(&self) -> SCFG_R {
        SCFG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 clock enable"]
    #[inline(always)]
    pub fn tmr9(&self) -> TMR9_R {
        TMR9_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 clock enable"]
    #[inline(always)]
    pub fn tmr10(&self) -> TMR10_R {
        TMR10_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer11 clock enable"]
    #[inline(always)]
    pub fn tmr11(&self) -> TMR11_R {
        TMR11_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - I2S5 clock enable"]
    #[inline(always)]
    pub fn i2s5(&self) -> I2S5_R {
        I2S5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC clock enable"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2EN")
            .field("tmr1", &format_args!("{}", self.tmr1().bit()))
            .field("usart1", &format_args!("{}", self.usart1().bit()))
            .field("usart6", &format_args!("{}", self.usart6().bit()))
            .field("adc1", &format_args!("{}", self.adc1().bit()))
            .field("spi1", &format_args!("{}", self.spi1().bit()))
            .field("scfg", &format_args!("{}", self.scfg().bit()))
            .field("tmr9", &format_args!("{}", self.tmr9().bit()))
            .field("tmr10", &format_args!("{}", self.tmr10().bit()))
            .field("tmr11", &format_args!("{}", self.tmr11().bit()))
            .field("i2s5", &format_args!("{}", self.i2s5().bit()))
            .field("acc", &format_args!("{}", self.acc().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB2EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<APB2EN_SPEC> {
        TMR1_W::new(self, 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<APB2EN_SPEC> {
        USART1_W::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6(&mut self) -> USART6_W<APB2EN_SPEC> {
        USART6_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<APB2EN_SPEC> {
        ADC1_W::new(self, 8)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<APB2EN_SPEC> {
        SPI1_W::new(self, 12)
    }
    #[doc = "Bit 14 - SCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfg(&mut self) -> SCFG_W<APB2EN_SPEC> {
        SCFG_W::new(self, 14)
    }
    #[doc = "Bit 16 - Timer9 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9(&mut self) -> TMR9_W<APB2EN_SPEC> {
        TMR9_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer10 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10(&mut self) -> TMR10_W<APB2EN_SPEC> {
        TMR10_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer11 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11(&mut self) -> TMR11_W<APB2EN_SPEC> {
        TMR11_W::new(self, 18)
    }
    #[doc = "Bit 20 - I2S5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2s5(&mut self) -> I2S5_W<APB2EN_SPEC> {
        I2S5_W::new(self, 20)
    }
    #[doc = "Bit 29 - ACC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn acc(&mut self) -> ACC_W<APB2EN_SPEC> {
        ACC_W::new(self, 29)
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
