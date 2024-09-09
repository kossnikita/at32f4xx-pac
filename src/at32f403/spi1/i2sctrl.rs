#[doc = "Register `I2SCTRL` reader"]
pub type R = crate::R<I2SCTRL_SPEC>;
#[doc = "Register `I2SCTRL` writer"]
pub type W = crate::W<I2SCTRL_SPEC>;
#[doc = "Field `I2SCBN` reader - I2S channel bit num"]
pub type I2SCBN_R = crate::BitReader;
#[doc = "Field `I2SCBN` writer - I2S channel bit num"]
pub type I2SCBN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SDBN` reader - I2S data bit num"]
pub type I2SDBN_R = crate::FieldReader;
#[doc = "Field `I2SDBN` writer - I2S data bit num"]
pub type I2SDBN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SCLKPOL` reader - I2S clock polarity"]
pub type I2SCLKPOL_R = crate::BitReader;
#[doc = "Field `I2SCLKPOL` writer - I2S clock polarity"]
pub type I2SCLKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STDSEL` reader - I2S standard select"]
pub type STDSEL_R = crate::FieldReader;
#[doc = "Field `STDSEL` writer - I2S standard select"]
pub type STDSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMFSSEL` reader - PCM frame synchronization select"]
pub type PCMFSSEL_R = crate::BitReader;
#[doc = "Field `PCMFSSEL` writer - PCM frame synchronization select"]
pub type PCMFSSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERSEL` reader - I2S operation select"]
pub type OPERSEL_R = crate::FieldReader;
#[doc = "Field `OPERSEL` writer - I2S operation select"]
pub type OPERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SEN` reader - I2S Enable"]
pub type I2SEN_R = crate::BitReader;
#[doc = "Field `I2SEN` writer - I2S Enable"]
pub type I2SEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SMSEL` reader - I2S mode select"]
pub type I2SMSEL_R = crate::BitReader;
#[doc = "Field `I2SMSEL` writer - I2S mode select"]
pub type I2SMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2S channel bit num"]
    #[inline(always)]
    pub fn i2scbn(&self) -> I2SCBN_R {
        I2SCBN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - I2S data bit num"]
    #[inline(always)]
    pub fn i2sdbn(&self) -> I2SDBN_R {
        I2SDBN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - I2S clock polarity"]
    #[inline(always)]
    pub fn i2sclkpol(&self) -> I2SCLKPOL_R {
        I2SCLKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard select"]
    #[inline(always)]
    pub fn stdsel(&self) -> STDSEL_R {
        STDSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization select"]
    #[inline(always)]
    pub fn pcmfssel(&self) -> PCMFSSEL_R {
        PCMFSSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S operation select"]
    #[inline(always)]
    pub fn opersel(&self) -> OPERSEL_R {
        OPERSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode select"]
    #[inline(always)]
    pub fn i2smsel(&self) -> I2SMSEL_R {
        I2SMSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCTRL")
            .field("i2smsel", &self.i2smsel())
            .field("i2sen", &self.i2sen())
            .field("opersel", &self.opersel())
            .field("pcmfssel", &self.pcmfssel())
            .field("stdsel", &self.stdsel())
            .field("i2sclkpol", &self.i2sclkpol())
            .field("i2sdbn", &self.i2sdbn())
            .field("i2scbn", &self.i2scbn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I2S channel bit num"]
    #[inline(always)]
    #[must_use]
    pub fn i2scbn(&mut self) -> I2SCBN_W<I2SCTRL_SPEC> {
        I2SCBN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - I2S data bit num"]
    #[inline(always)]
    #[must_use]
    pub fn i2sdbn(&mut self) -> I2SDBN_W<I2SCTRL_SPEC> {
        I2SDBN_W::new(self, 1)
    }
    #[doc = "Bit 3 - I2S clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn i2sclkpol(&mut self) -> I2SCLKPOL_W<I2SCTRL_SPEC> {
        I2SCLKPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard select"]
    #[inline(always)]
    #[must_use]
    pub fn stdsel(&mut self) -> STDSEL_W<I2SCTRL_SPEC> {
        STDSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization select"]
    #[inline(always)]
    #[must_use]
    pub fn pcmfssel(&mut self) -> PCMFSSEL_W<I2SCTRL_SPEC> {
        PCMFSSEL_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S operation select"]
    #[inline(always)]
    #[must_use]
    pub fn opersel(&mut self) -> OPERSEL_W<I2SCTRL_SPEC> {
        OPERSEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2sen(&mut self) -> I2SEN_W<I2SCTRL_SPEC> {
        I2SEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode select"]
    #[inline(always)]
    #[must_use]
    pub fn i2smsel(&mut self) -> I2SMSEL_W<I2SCTRL_SPEC> {
        I2SMSEL_W::new(self, 11)
    }
}
#[doc = "I2S control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCTRL_SPEC;
impl crate::RegisterSpec for I2SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sctrl::R`](R) reader structure"]
impl crate::Readable for I2SCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sctrl::W`](W) writer structure"]
impl crate::Writable for I2SCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCTRL to value 0"]
impl crate::Resettable for I2SCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
