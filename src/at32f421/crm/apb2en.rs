#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `SCFG` reader - Syscfg and comparator clock enable"]
pub type SCFG_R = crate::BitReader;
#[doc = "Field `SCFG` writer - Syscfg and comparator clock enable"]
pub type SCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC clock enable"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC clock enable"]
pub type ADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1` reader - Timer1 clock enable"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - Timer1 clock enable"]
pub type TMR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 clock enable"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 clock enable"]
pub type SPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - USART1 clock enable"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 clock enable"]
pub type USART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15` reader - Timer15 clock enable"]
pub type TMR15_R = crate::BitReader;
#[doc = "Field `TMR15` writer - Timer15 clock enable"]
pub type TMR15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR16` reader - Timer16 clock enable"]
pub type TMR16_R = crate::BitReader;
#[doc = "Field `TMR16` writer - Timer16 clock enable"]
pub type TMR16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR17` reader - Timer17 clock enable"]
pub type TMR17_R = crate::BitReader;
#[doc = "Field `TMR17` writer - Timer17 clock enable"]
pub type TMR17_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Syscfg and comparator clock enable"]
    #[inline(always)]
    pub fn scfg(&self) -> SCFG_R {
        SCFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    pub fn tmr15(&self) -> TMR15_R {
        TMR15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer16 clock enable"]
    #[inline(always)]
    pub fn tmr16(&self) -> TMR16_R {
        TMR16_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer17 clock enable"]
    #[inline(always)]
    pub fn tmr17(&self) -> TMR17_R {
        TMR17_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2EN")
            .field("scfg", &self.scfg())
            .field("adc1", &self.adc1())
            .field("tmr1", &self.tmr1())
            .field("spi1", &self.spi1())
            .field("usart1", &self.usart1())
            .field("tmr15", &self.tmr15())
            .field("tmr16", &self.tmr16())
            .field("tmr17", &self.tmr17())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Syscfg and comparator clock enable"]
    #[inline(always)]
    pub fn scfg(&mut self) -> SCFG_W<'_, APB2EN_SPEC> {
        SCFG_W::new(self, 0)
    }
    #[doc = "Bit 9 - ADC clock enable"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W<'_, APB2EN_SPEC> {
        ADC1_W::new(self, 9)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1(&mut self) -> TMR1_W<'_, APB2EN_SPEC> {
        TMR1_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<'_, APB2EN_SPEC> {
        SPI1_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1(&mut self) -> USART1_W<'_, APB2EN_SPEC> {
        USART1_W::new(self, 14)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    pub fn tmr15(&mut self) -> TMR15_W<'_, APB2EN_SPEC> {
        TMR15_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer16 clock enable"]
    #[inline(always)]
    pub fn tmr16(&mut self) -> TMR16_W<'_, APB2EN_SPEC> {
        TMR16_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer17 clock enable"]
    #[inline(always)]
    pub fn tmr17(&mut self) -> TMR17_W<'_, APB2EN_SPEC> {
        TMR17_W::new(self, 18)
    }
}
#[doc = "APB2 peripheral clock enable register (CRM_APB2EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2EN_SPEC;
impl crate::RegisterSpec for APB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for APB2EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for APB2EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for APB2EN_SPEC {}
