#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `TMR1EN` reader - Timer1 clock enable"]
pub type TMR1EN_R = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Timer1 clock enable"]
pub type TMR1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR8EN` reader - Timer8 clock enable"]
pub type TMR8EN_R = crate::BitReader;
#[doc = "Field `TMR8EN` writer - Timer8 clock enable"]
pub type TMR8EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6EN` reader - USART6 clock enable"]
pub type USART6EN_R = crate::BitReader;
#[doc = "Field `USART6EN` writer - USART6 clock enable"]
pub type USART6EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub type ADC1EN_R = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub type ADC1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2EN` reader - ADC2 clock enable"]
pub type ADC2EN_R = crate::BitReader;
#[doc = "Field `ADC2EN` writer - ADC2 clock enable"]
pub type ADC2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable"]
pub type ADC3EN_R = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable"]
pub type ADC3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable"]
pub type SPI4EN_R = crate::BitReader;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable"]
pub type SPI4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCFGEN` reader - SCFG clock enable"]
pub type SCFGEN_R = crate::BitReader;
#[doc = "Field `SCFGEN` writer - SCFG clock enable"]
pub type SCFGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR9EN` reader - Timer9 clock enable"]
pub type TMR9EN_R = crate::BitReader;
#[doc = "Field `TMR9EN` writer - Timer9 clock enable"]
pub type TMR9EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR10EN` reader - Timer10 clock enable"]
pub type TMR10EN_R = crate::BitReader;
#[doc = "Field `TMR10EN` writer - Timer10 clock enable"]
pub type TMR10EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR11EN` reader - Timer11 clock enable"]
pub type TMR11EN_R = crate::BitReader;
#[doc = "Field `TMR11EN` writer - Timer11 clock enable"]
pub type TMR11EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR20EN` reader - Timer20 clock enable"]
pub type TMR20EN_R = crate::BitReader;
#[doc = "Field `TMR20EN` writer - Timer20 clock enable"]
pub type TMR20EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCEN` reader - ACC clock enable"]
pub type ACCEN_R = crate::BitReader;
#[doc = "Field `ACCEN` writer - ACC clock enable"]
pub type ACCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1en(&self) -> TMR1EN_R {
        TMR1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer8 clock enable"]
    #[inline(always)]
    pub fn tmr8en(&self) -> TMR8EN_R {
        TMR8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG clock enable"]
    #[inline(always)]
    pub fn scfgen(&self) -> SCFGEN_R {
        SCFGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 clock enable"]
    #[inline(always)]
    pub fn tmr9en(&self) -> TMR9EN_R {
        TMR9EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 clock enable"]
    #[inline(always)]
    pub fn tmr10en(&self) -> TMR10EN_R {
        TMR10EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer11 clock enable"]
    #[inline(always)]
    pub fn tmr11en(&self) -> TMR11EN_R {
        TMR11EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer20 clock enable"]
    #[inline(always)]
    pub fn tmr20en(&self) -> TMR20EN_R {
        TMR20EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC clock enable"]
    #[inline(always)]
    pub fn accen(&self) -> ACCEN_R {
        ACCEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1en(&mut self) -> TMR1EN_W<APB2EN_SPEC, 0> {
        TMR1EN_W::new(self)
    }
    #[doc = "Bit 1 - Timer8 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8en(&mut self) -> TMR8EN_W<APB2EN_SPEC, 1> {
        TMR8EN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2EN_SPEC, 4> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<APB2EN_SPEC, 5> {
        USART6EN_W::new(self)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<APB2EN_SPEC, 8> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc2en(&mut self) -> ADC2EN_W<APB2EN_SPEC, 9> {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> ADC3EN_W<APB2EN_SPEC, 10> {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2EN_SPEC, 12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<APB2EN_SPEC, 13> {
        SPI4EN_W::new(self)
    }
    #[doc = "Bit 14 - SCFG clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfgen(&mut self) -> SCFGEN_W<APB2EN_SPEC, 14> {
        SCFGEN_W::new(self)
    }
    #[doc = "Bit 16 - Timer9 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9en(&mut self) -> TMR9EN_W<APB2EN_SPEC, 16> {
        TMR9EN_W::new(self)
    }
    #[doc = "Bit 17 - Timer10 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10en(&mut self) -> TMR10EN_W<APB2EN_SPEC, 17> {
        TMR10EN_W::new(self)
    }
    #[doc = "Bit 18 - Timer11 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11en(&mut self) -> TMR11EN_W<APB2EN_SPEC, 18> {
        TMR11EN_W::new(self)
    }
    #[doc = "Bit 20 - Timer20 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20en(&mut self) -> TMR20EN_W<APB2EN_SPEC, 20> {
        TMR20EN_W::new(self)
    }
    #[doc = "Bit 29 - ACC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn accen(&mut self) -> ACCEN_W<APB2EN_SPEC, 29> {
        ACCEN_W::new(self)
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
