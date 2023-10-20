#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `SCFGCMP` reader - System config and comparator reset"]
pub type SCFGCMP_R = crate::BitReader;
#[doc = "Field `SCFGCMP` writer - System config and comparator reset"]
pub type SCFGCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINT` reader - External interrupt reset"]
pub type EXINT_R = crate::BitReader;
#[doc = "Field `EXINT` writer - External interrupt reset"]
pub type EXINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC` reader - ADC reset"]
pub type ADC_R = crate::BitReader;
#[doc = "Field `ADC` writer - ADC reset"]
pub type ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1` reader - TMR1 reset"]
pub type TMR1_R = crate::BitReader;
#[doc = "Field `TMR1` writer - TMR1 reset"]
pub type TMR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub type SPI1_R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub type SPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - USART1 reset"]
pub type USART1_R = crate::BitReader;
#[doc = "Field `USART1` writer - USART1 reset"]
pub type USART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR15` reader - Timer15 reset"]
pub type TMR15_R = crate::BitReader;
#[doc = "Field `TMR15` writer - Timer15 reset"]
pub type TMR15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR16` reader - Timer16 reset"]
pub type TMR16_R = crate::BitReader;
#[doc = "Field `TMR16` writer - Timer16 reset"]
pub type TMR16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR17` reader - Timer17 reset"]
pub type TMR17_R = crate::BitReader;
#[doc = "Field `TMR17` writer - Timer17 reset"]
pub type TMR17_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System config and comparator reset"]
    #[inline(always)]
    pub fn scfgcmp(&self) -> SCFGCMP_R {
        SCFGCMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exint(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TMR1 reset"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer15 reset"]
    #[inline(always)]
    pub fn tmr15(&self) -> TMR15_R {
        TMR15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer16 reset"]
    #[inline(always)]
    pub fn tmr16(&self) -> TMR16_R {
        TMR16_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer17 reset"]
    #[inline(always)]
    pub fn tmr17(&self) -> TMR17_R {
        TMR17_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RST")
            .field("scfgcmp", &format_args!("{}", self.scfgcmp().bit()))
            .field("exint", &format_args!("{}", self.exint().bit()))
            .field("adc", &format_args!("{}", self.adc().bit()))
            .field("tmr1", &format_args!("{}", self.tmr1().bit()))
            .field("spi1", &format_args!("{}", self.spi1().bit()))
            .field("usart1", &format_args!("{}", self.usart1().bit()))
            .field("tmr15", &format_args!("{}", self.tmr15().bit()))
            .field("tmr16", &format_args!("{}", self.tmr16().bit()))
            .field("tmr17", &format_args!("{}", self.tmr17().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APB2RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - System config and comparator reset"]
    #[inline(always)]
    #[must_use]
    pub fn scfgcmp(&mut self) -> SCFGCMP_W<APB2RST_SPEC> {
        SCFGCMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    #[must_use]
    pub fn exint(&mut self) -> EXINT_W<APB2RST_SPEC> {
        EXINT_W::new(self, 1)
    }
    #[doc = "Bit 9 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<APB2RST_SPEC> {
        ADC_W::new(self, 9)
    }
    #[doc = "Bit 11 - TMR1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1(&mut self) -> TMR1_W<APB2RST_SPEC> {
        TMR1_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<APB2RST_SPEC> {
        SPI1_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<APB2RST_SPEC> {
        USART1_W::new(self, 14)
    }
    #[doc = "Bit 16 - Timer15 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15(&mut self) -> TMR15_W<APB2RST_SPEC> {
        TMR15_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer16 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr16(&mut self) -> TMR16_W<APB2RST_SPEC> {
        TMR16_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer17 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr17(&mut self) -> TMR17_W<APB2RST_SPEC> {
        TMR17_W::new(self, 18)
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
