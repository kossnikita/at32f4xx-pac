#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `SCLKSEL` reader - System clock select"]
pub type SCLKSEL_R = crate::FieldReader;
#[doc = "Field `SCLKSEL` writer - System clock select"]
pub type SCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SCLKSTS` reader - System Clock select Status"]
pub type SCLKSTS_R = crate::FieldReader;
#[doc = "Field `AHBDIV` reader - AHB division"]
pub type AHBDIV_R = crate::FieldReader;
#[doc = "Field `AHBDIV` writer - AHB division"]
pub type AHBDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `APB1DIV` reader - APB1 division"]
pub type APB1DIV_R = crate::FieldReader;
#[doc = "Field `APB1DIV` writer - APB1 division"]
pub type APB1DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `APB2DIV` reader - APB2 division"]
pub type APB2DIV_R = crate::FieldReader;
#[doc = "Field `APB2DIV` writer - APB2 division"]
pub type APB2DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADCDIV1_0` reader - ADC division bit1 and bit0"]
pub type ADCDIV1_0_R = crate::FieldReader;
#[doc = "Field `ADCDIV1_0` writer - ADC division bit1 and bit0"]
pub type ADCDIV1_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PLLRCS` reader - PLL reference clock select"]
pub type PLLRCS_R = crate::BitReader;
#[doc = "Field `PLLRCS` writer - PLL reference clock select"]
pub type PLLRCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLHEXTDIV` reader - HEXT division selection for PLL entry clock"]
pub type PLLHEXTDIV_R = crate::BitReader;
#[doc = "Field `PLLHEXTDIV` writer - HEXT division selection for PLL entry clock"]
pub type PLLHEXTDIV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLMULT3_0` reader - PLL Multiplication Factor bit3 to bit0"]
pub type PLLMULT3_0_R = crate::FieldReader;
#[doc = "Field `PLLMULT3_0` writer - PLL Multiplication Factor bit3 to bit0"]
pub type PLLMULT3_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLKOUT_SEL` reader - Clock output selection bit2 to bit0"]
pub type CLKOUT_SEL_R = crate::FieldReader;
#[doc = "Field `CLKOUT_SEL` writer - Clock output selection bit2 to bit0"]
pub type CLKOUT_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADCDIV2` reader - ADC division bit2"]
pub type ADCDIV2_R = crate::BitReader;
#[doc = "Field `ADCDIV2` writer - ADC division bit2"]
pub type ADCDIV2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLMULT5_4` reader - PLL Multiplication Factor bit5 and bit4"]
pub type PLLMULT5_4_R = crate::FieldReader;
#[doc = "Field `PLLMULT5_4` writer - PLL Multiplication Factor bit5 and bit4"]
pub type PLLMULT5_4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    pub fn sclksel(&self) -> SCLKSEL_R {
        SCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock select Status"]
    #[inline(always)]
    pub fn sclksts(&self) -> SCLKSTS_R {
        SCLKSTS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    pub fn ahbdiv(&self) -> AHBDIV_R {
        AHBDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC division bit1 and bit0"]
    #[inline(always)]
    pub fn adcdiv1_0(&self) -> ADCDIV1_0_R {
        ADCDIV1_0_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL reference clock select"]
    #[inline(always)]
    pub fn pllrcs(&self) -> PLLRCS_R {
        PLLRCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HEXT division selection for PLL entry clock"]
    #[inline(always)]
    pub fn pllhextdiv(&self) -> PLLHEXTDIV_R {
        PLLHEXTDIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor bit3 to bit0"]
    #[inline(always)]
    pub fn pllmult3_0(&self) -> PLLMULT3_0_R {
        PLLMULT3_0_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Clock output selection bit2 to bit0"]
    #[inline(always)]
    pub fn clkout_sel(&self) -> CLKOUT_SEL_R {
        CLKOUT_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - ADC division bit2"]
    #[inline(always)]
    pub fn adcdiv2(&self) -> ADCDIV2_R {
        ADCDIV2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - PLL Multiplication Factor bit5 and bit4"]
    #[inline(always)]
    pub fn pllmult5_4(&self) -> PLLMULT5_4_R {
        PLLMULT5_4_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    #[must_use]
    pub fn sclksel(&mut self) -> SCLKSEL_W<CFG_SPEC, 0> {
        SCLKSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdiv(&mut self) -> AHBDIV_W<CFG_SPEC, 4> {
        AHBDIV_W::new(self)
    }
    #[doc = "Bits 8:10 - APB1 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> APB1DIV_W<CFG_SPEC, 8> {
        APB1DIV_W::new(self)
    }
    #[doc = "Bits 11:13 - APB2 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> APB2DIV_W<CFG_SPEC, 11> {
        APB2DIV_W::new(self)
    }
    #[doc = "Bits 14:15 - ADC division bit1 and bit0"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv1_0(&mut self) -> ADCDIV1_0_W<CFG_SPEC, 14> {
        ADCDIV1_0_W::new(self)
    }
    #[doc = "Bit 16 - PLL reference clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllrcs(&mut self) -> PLLRCS_W<CFG_SPEC, 16> {
        PLLRCS_W::new(self)
    }
    #[doc = "Bit 17 - HEXT division selection for PLL entry clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllhextdiv(&mut self) -> PLLHEXTDIV_W<CFG_SPEC, 17> {
        PLLHEXTDIV_W::new(self)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor bit3 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn pllmult3_0(&mut self) -> PLLMULT3_0_W<CFG_SPEC, 18> {
        PLLMULT3_0_W::new(self)
    }
    #[doc = "Bits 24:26 - Clock output selection bit2 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_sel(&mut self) -> CLKOUT_SEL_W<CFG_SPEC, 24> {
        CLKOUT_SEL_W::new(self)
    }
    #[doc = "Bit 28 - ADC division bit2"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv2(&mut self) -> ADCDIV2_W<CFG_SPEC, 28> {
        ADCDIV2_W::new(self)
    }
    #[doc = "Bits 29:30 - PLL Multiplication Factor bit5 and bit4"]
    #[inline(always)]
    #[must_use]
    pub fn pllmult5_4(&mut self) -> PLLMULT5_4_W<CFG_SPEC, 29> {
        PLLMULT5_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock configuration register (CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
