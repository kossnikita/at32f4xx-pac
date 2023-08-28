#[doc = "Register `REMAP5` reader"]
pub type R = crate::R<REMAP5_SPEC>;
#[doc = "Register `REMAP5` writer"]
pub type W = crate::W<REMAP5_SPEC>;
#[doc = "Field `I2C1_GMUX` reader - I2C1 muxing"]
pub type I2C1_GMUX_R = crate::FieldReader;
#[doc = "Field `I2C1_GMUX` writer - I2C1 muxing"]
pub type I2C1_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SPI2_GMUX` reader - SPI2 muxing"]
pub type SPI2_GMUX_R = crate::FieldReader;
#[doc = "Field `SPI2_GMUX` writer - SPI2 muxing"]
pub type SPI2_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_gmux(&self) -> I2C1_GMUX_R {
        I2C1_GMUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    pub fn spi2_gmux(&self) -> SPI2_GMUX_R {
        SPI2_GMUX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_gmux(&mut self) -> I2C1_GMUX_W<REMAP5_SPEC, 4> {
        I2C1_GMUX_W::new(self)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_gmux(&mut self) -> SPI2_GMUX_W<REMAP5_SPEC, 20> {
        SPI2_GMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IO MUX remap register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP5_SPEC;
impl crate::RegisterSpec for REMAP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap5::R`](R) reader structure"]
impl crate::Readable for REMAP5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap5::W`](W) writer structure"]
impl crate::Writable for REMAP5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP5 to value 0"]
impl crate::Resettable for REMAP5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
