#[doc = "Register `I2SCLKP` reader"]
pub type R = crate::R<I2SCLKP_SPEC>;
#[doc = "Register `I2SCLKP` writer"]
pub type W = crate::W<I2SCLKP_SPEC>;
#[doc = "Field `I2SDIV7_0` reader - I2S division bit7 to bit0"]
pub type I2SDIV7_0_R = crate::FieldReader;
#[doc = "Field `I2SDIV7_0` writer - I2S division bit7 to bit0"]
pub type I2SDIV7_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `I2SODD` reader - Odd result for I2S division"]
pub type I2SODD_R = crate::BitReader;
#[doc = "Field `I2SODD` writer - Odd result for I2S division"]
pub type I2SODD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2SMCLKOE` reader - I2S master clock output enable"]
pub type I2SMCLKOE_R = crate::BitReader;
#[doc = "Field `I2SMCLKOE` writer - I2S master clock output enable"]
pub type I2SMCLKOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2SDIV9_8` reader - I2S division bit9 and bit8"]
pub type I2SDIV9_8_R = crate::FieldReader;
#[doc = "Field `I2SDIV9_8` writer - I2S division bit9 and bit8"]
pub type I2SDIV9_8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:7 - I2S division bit7 to bit0"]
    #[inline(always)]
    pub fn i2sdiv7_0(&self) -> I2SDIV7_0_R {
        I2SDIV7_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd result for I2S division"]
    #[inline(always)]
    pub fn i2sodd(&self) -> I2SODD_R {
        I2SODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2S master clock output enable"]
    #[inline(always)]
    pub fn i2smclkoe(&self) -> I2SMCLKOE_R {
        I2SMCLKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - I2S division bit9 and bit8"]
    #[inline(always)]
    pub fn i2sdiv9_8(&self) -> I2SDIV9_8_R {
        I2SDIV9_8_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S division bit7 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv7_0(&mut self) -> I2SDIV7_0_W<I2SCLKP_SPEC, 0> {
        I2SDIV7_0_W::new(self)
    }
    #[doc = "Bit 8 - Odd result for I2S division"]
    #[inline(always)]
    #[must_use]
    pub fn i2sodd(&mut self) -> I2SODD_W<I2SCLKP_SPEC, 8> {
        I2SODD_W::new(self)
    }
    #[doc = "Bit 9 - I2S master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2smclkoe(&mut self) -> I2SMCLKOE_W<I2SCLKP_SPEC, 9> {
        I2SMCLKOE_W::new(self)
    }
    #[doc = "Bits 10:11 - I2S division bit9 and bit8"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv9_8(&mut self) -> I2SDIV9_8_W<I2SCLKP_SPEC, 10> {
        I2SDIV9_8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclkp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclkp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCLKP_SPEC;
impl crate::RegisterSpec for I2SCLKP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sclkp::R`](R) reader structure"]
impl crate::Readable for I2SCLKP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sclkp::W`](W) writer structure"]
impl crate::Writable for I2SCLKP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCLKP to value 0x0a"]
impl crate::Resettable for I2SCLKP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
