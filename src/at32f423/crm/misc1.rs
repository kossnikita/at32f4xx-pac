#[doc = "Register `MISC1` reader"]
pub type R = crate::R<MISC1_SPEC>;
#[doc = "Register `MISC1` writer"]
pub type W = crate::W<MISC1_SPEC>;
#[doc = "Field `HICKCAL_KEY` reader - HICKCAL write key value"]
pub type HICKCAL_KEY_R = crate::FieldReader;
#[doc = "Field `HICKCAL_KEY` writer - HICKCAL write key value"]
pub type HICKCAL_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HICKDIV` reader - HICK 6 divider selection"]
pub type HICKDIV_R = crate::BitReader;
#[doc = "Field `HICKDIV` writer - HICK 6 divider selection"]
pub type HICKDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICK_TO_USB` reader - HICK to usb clock"]
pub type HICK_TO_USB_R = crate::BitReader;
#[doc = "Field `HICK_TO_USB` writer - HICK to usb clock"]
pub type HICK_TO_USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICK_TO_SCLK` reader - HICK to system clock"]
pub type HICK_TO_SCLK_R = crate::BitReader;
#[doc = "Field `HICK_TO_SCLK` writer - HICK to system clock"]
pub type HICK_TO_SCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLCLK_TO_ADC` reader - ADC clock source select"]
pub type PLLCLK_TO_ADC_R = crate::BitReader;
#[doc = "Field `PLLCLK_TO_ADC` writer - ADC clock source select"]
pub type PLLCLK_TO_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT_SEL2` reader - Clock output select2"]
pub type CLKOUT_SEL2_R = crate::FieldReader;
#[doc = "Field `CLKOUT_SEL2` writer - Clock output select2"]
pub type CLKOUT_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKOUTDIV2` reader - Clock output division2"]
pub type CLKOUTDIV2_R = crate::FieldReader;
#[doc = "Field `CLKOUTDIV2` writer - Clock output division2"]
pub type CLKOUTDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    pub fn hickcal_key(&self) -> HICKCAL_KEY_R {
        HICKCAL_KEY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - HICK 6 divider selection"]
    #[inline(always)]
    pub fn hickdiv(&self) -> HICKDIV_R {
        HICKDIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HICK to usb clock"]
    #[inline(always)]
    pub fn hick_to_usb(&self) -> HICK_TO_USB_R {
        HICK_TO_USB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HICK to system clock"]
    #[inline(always)]
    pub fn hick_to_sclk(&self) -> HICK_TO_SCLK_R {
        HICK_TO_SCLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC clock source select"]
    #[inline(always)]
    pub fn pllclk_to_adc(&self) -> PLLCLK_TO_ADC_R {
        PLLCLK_TO_ADC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Clock output select2"]
    #[inline(always)]
    pub fn clkout_sel2(&self) -> CLKOUT_SEL2_R {
        CLKOUT_SEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Clock output division2"]
    #[inline(always)]
    pub fn clkoutdiv2(&self) -> CLKOUTDIV2_R {
        CLKOUTDIV2_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC1")
            .field(
                "hickcal_key",
                &format_args!("{}", self.hickcal_key().bits()),
            )
            .field("hickdiv", &format_args!("{}", self.hickdiv().bit()))
            .field("hick_to_usb", &format_args!("{}", self.hick_to_usb().bit()))
            .field(
                "hick_to_sclk",
                &format_args!("{}", self.hick_to_sclk().bit()),
            )
            .field(
                "pllclk_to_adc",
                &format_args!("{}", self.pllclk_to_adc().bit()),
            )
            .field(
                "clkout_sel2",
                &format_args!("{}", self.clkout_sel2().bits()),
            )
            .field("clkoutdiv2", &format_args!("{}", self.clkoutdiv2().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MISC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    #[must_use]
    pub fn hickcal_key(&mut self) -> HICKCAL_KEY_W<MISC1_SPEC> {
        HICKCAL_KEY_W::new(self, 0)
    }
    #[doc = "Bit 12 - HICK 6 divider selection"]
    #[inline(always)]
    #[must_use]
    pub fn hickdiv(&mut self) -> HICKDIV_W<MISC1_SPEC> {
        HICKDIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - HICK to usb clock"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_usb(&mut self) -> HICK_TO_USB_W<MISC1_SPEC> {
        HICK_TO_USB_W::new(self, 13)
    }
    #[doc = "Bit 14 - HICK to system clock"]
    #[inline(always)]
    #[must_use]
    pub fn hick_to_sclk(&mut self) -> HICK_TO_SCLK_W<MISC1_SPEC> {
        HICK_TO_SCLK_W::new(self, 14)
    }
    #[doc = "Bit 15 - ADC clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn pllclk_to_adc(&mut self) -> PLLCLK_TO_ADC_W<MISC1_SPEC> {
        PLLCLK_TO_ADC_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Clock output select2"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_sel2(&mut self) -> CLKOUT_SEL2_W<MISC1_SPEC> {
        CLKOUT_SEL2_W::new(self, 16)
    }
    #[doc = "Bits 28:31 - Clock output division2"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutdiv2(&mut self) -> CLKOUTDIV2_W<MISC1_SPEC> {
        CLKOUTDIV2_W::new(self, 28)
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
#[doc = "Miscellaneous register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC1_SPEC;
impl crate::RegisterSpec for MISC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc1::R`](R) reader structure"]
impl crate::Readable for MISC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc1::W`](W) writer structure"]
impl crate::Writable for MISC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC1 to value 0"]
impl crate::Resettable for MISC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
