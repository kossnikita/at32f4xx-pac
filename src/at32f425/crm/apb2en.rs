#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `SCFG` reader - System config clock enable"]
pub type SCFG_R = crate::BitReader;
#[doc = "Field `SCFG` writer - System config clock enable"]
pub type SCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1` reader - ADC1 clock enable"]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 clock enable"]
pub type ADC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR1` reader - Timer1 clock enable"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - Timer1 clock enable"]
pub type TMR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1` reader - SPI1 clock enable"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 clock enable"]
pub type SPI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1` reader - USART1 clock enable"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 clock enable"]
pub type USART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR15` reader - Timer15 clock enable"]
pub type TMR15_R = crate::BitReader;
#[doc = "Field `TMR15` writer - Timer15 clock enable"]
pub type TMR15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR16` reader - Timer16 clock enable"]
pub type TMR16_R = crate::BitReader;
#[doc = "Field `TMR16` writer - Timer16 clock enable"]
pub type TMR16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR17` reader - Timer17 clock enable"]
pub type TMR17_R = crate::BitReader;
#[doc = "Field `TMR17` writer - Timer17 clock enable"]
pub type TMR17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - System config clock enable"]
    #[inline(always)]
    pub fn scfg(&self) -> SCFG_R {
        SCFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
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
impl W {
    #[doc = "Bit 0 - System config clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfg(&mut self) -> SCFG_W<APB2EN_SPEC, 0> {
        SCFG_W::new(self)
    }
    #[doc = "Bit 9 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<APB2EN_SPEC, 9> {
        ADC1_W::new(self)
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
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<APB2EN_SPEC, 14> {
        USART1_W::new(self)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15(&mut self) -> TMR15_W<APB2EN_SPEC, 16> {
        TMR15_W::new(self)
    }
    #[doc = "Bit 17 - Timer16 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr16(&mut self) -> TMR16_W<APB2EN_SPEC, 17> {
        TMR16_W::new(self)
    }
    #[doc = "Bit 18 - Timer17 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr17(&mut self) -> TMR17_W<APB2EN_SPEC, 18> {
        TMR17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
