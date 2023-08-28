#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `SCFGRST` reader - System config reset"]
pub type SCFGRST_R = crate::BitReader;
#[doc = "Field `SCFGRST` writer - System config reset"]
pub type SCFGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXINTRST` reader - External interrupt reset"]
pub type EXINTRST_R = crate::BitReader;
#[doc = "Field `EXINTRST` writer - External interrupt reset"]
pub type EXINTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub type ADC1RST_R = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub type ADC1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR1RST` reader - TMR1 reset"]
pub type TMR1RST_R = crate::BitReader;
#[doc = "Field `TMR1RST` writer - TMR1 reset"]
pub type TMR1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR15RST` reader - Timer15 reset"]
pub type TMR15RST_R = crate::BitReader;
#[doc = "Field `TMR15RST` writer - Timer15 reset"]
pub type TMR15RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR16RST` reader - Timer16 reset"]
pub type TMR16RST_R = crate::BitReader;
#[doc = "Field `TMR16RST` writer - Timer16 reset"]
pub type TMR16RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR17RST` reader - Timer17 reset"]
pub type TMR17RST_R = crate::BitReader;
#[doc = "Field `TMR17RST` writer - Timer17 reset"]
pub type TMR17RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - System config reset"]
    #[inline(always)]
    pub fn scfgrst(&self) -> SCFGRST_R {
        SCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exintrst(&self) -> EXINTRST_R {
        EXINTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TMR1 reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> TMR1RST_R {
        TMR1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer15 reset"]
    #[inline(always)]
    pub fn tmr15rst(&self) -> TMR15RST_R {
        TMR15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer16 reset"]
    #[inline(always)]
    pub fn tmr16rst(&self) -> TMR16RST_R {
        TMR16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer17 reset"]
    #[inline(always)]
    pub fn tmr17rst(&self) -> TMR17RST_R {
        TMR17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System config reset"]
    #[inline(always)]
    #[must_use]
    pub fn scfgrst(&mut self) -> SCFGRST_W<APB2RST_SPEC, 0> {
        SCFGRST_W::new(self)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn exintrst(&mut self) -> EXINTRST_W<APB2RST_SPEC, 1> {
        EXINTRST_W::new(self)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<APB2RST_SPEC, 9> {
        ADC1RST_W::new(self)
    }
    #[doc = "Bit 11 - TMR1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1rst(&mut self) -> TMR1RST_W<APB2RST_SPEC, 11> {
        TMR1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RST_SPEC, 12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RST_SPEC, 14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 16 - Timer15 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15rst(&mut self) -> TMR15RST_W<APB2RST_SPEC, 16> {
        TMR15RST_W::new(self)
    }
    #[doc = "Bit 17 - Timer16 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr16rst(&mut self) -> TMR16RST_W<APB2RST_SPEC, 17> {
        TMR16RST_W::new(self)
    }
    #[doc = "Bit 18 - Timer17 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr17rst(&mut self) -> TMR17RST_W<APB2RST_SPEC, 18> {
        TMR17RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
