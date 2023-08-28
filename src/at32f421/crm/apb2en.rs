#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `SCFGCMPEN` reader - Syscfg and comparator clock enable"]
pub type SCFGCMPEN_R = crate::BitReader;
#[doc = "Field `SCFGCMPEN` writer - Syscfg and comparator clock enable"]
pub type SCFGCMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type ADCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR1EN` reader - Timer1 clock enable"]
pub type TMR1EN_R = crate::BitReader;
#[doc = "Field `TMR1EN` writer - Timer1 clock enable"]
pub type TMR1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR15EN` reader - Timer15 clock enable"]
pub type TMR15EN_R = crate::BitReader;
#[doc = "Field `TMR15EN` writer - Timer15 clock enable"]
pub type TMR15EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR16EN` reader - Timer16 clock enable"]
pub type TMR16EN_R = crate::BitReader;
#[doc = "Field `TMR16EN` writer - Timer16 clock enable"]
pub type TMR16EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR17EN` reader - Timer17 clock enable"]
pub type TMR17EN_R = crate::BitReader;
#[doc = "Field `TMR17EN` writer - Timer17 clock enable"]
pub type TMR17EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Syscfg and comparator clock enable"]
    #[inline(always)]
    pub fn scfgcmpen(&self) -> SCFGCMPEN_R {
        SCFGCMPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    pub fn tmr1en(&self) -> TMR1EN_R {
        TMR1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    pub fn tmr15en(&self) -> TMR15EN_R {
        TMR15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer16 clock enable"]
    #[inline(always)]
    pub fn tmr16en(&self) -> TMR16EN_R {
        TMR16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer17 clock enable"]
    #[inline(always)]
    pub fn tmr17en(&self) -> TMR17EN_R {
        TMR17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Syscfg and comparator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn scfgcmpen(&mut self) -> SCFGCMPEN_W<APB2EN_SPEC, 0> {
        SCFGCMPEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<APB2EN_SPEC, 9> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 11 - Timer1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1en(&mut self) -> TMR1EN_W<APB2EN_SPEC, 11> {
        TMR1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB2EN_SPEC, 12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB2EN_SPEC, 14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 16 - Timer15 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15en(&mut self) -> TMR15EN_W<APB2EN_SPEC, 16> {
        TMR15EN_W::new(self)
    }
    #[doc = "Bit 17 - Timer16 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr16en(&mut self) -> TMR16EN_W<APB2EN_SPEC, 17> {
        TMR16EN_W::new(self)
    }
    #[doc = "Bit 18 - Timer17 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmr17en(&mut self) -> TMR17EN_W<APB2EN_SPEC, 18> {
        TMR17EN_W::new(self)
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
