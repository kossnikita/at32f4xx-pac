#[doc = "Register `REMAP5` reader"]
pub type R = crate::R<REMAP5_SPEC>;
#[doc = "Register `REMAP5` writer"]
pub type W = crate::W<REMAP5_SPEC>;
#[doc = "Field `I2C1_GMUX` reader - I2C1 muxing"]
pub type I2C1_GMUX_R = crate::FieldReader;
#[doc = "Field `I2C1_GMUX` writer - I2C1 muxing"]
pub type I2C1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2C2_GMUX` reader - I2C2 muxing"]
pub type I2C2_GMUX_R = crate::FieldReader;
#[doc = "Field `I2C2_GMUX` writer - I2C2 muxing"]
pub type I2C2_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI1_GMUX` reader - SPI1 muxing"]
pub type SPI1_GMUX_R = crate::FieldReader;
#[doc = "Field `SPI1_GMUX` writer - SPI1 muxing"]
pub type SPI1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI2_GMUX` reader - SPI2 muxing"]
pub type SPI2_GMUX_R = crate::FieldReader;
#[doc = "Field `SPI2_GMUX` writer - SPI2 muxing"]
pub type SPI2_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_gmux(&self) -> I2C1_GMUX_R {
        I2C1_GMUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - I2C2 muxing"]
    #[inline(always)]
    pub fn i2c2_gmux(&self) -> I2C2_GMUX_R {
        I2C2_GMUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    pub fn spi1_gmux(&self) -> SPI1_GMUX_R {
        SPI1_GMUX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    pub fn spi2_gmux(&self) -> SPI2_GMUX_R {
        SPI2_GMUX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP5")
            .field("i2c1_gmux", &self.i2c1_gmux())
            .field("i2c2_gmux", &self.i2c2_gmux())
            .field("spi1_gmux", &self.spi1_gmux())
            .field("spi2_gmux", &self.spi2_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_gmux(&mut self) -> I2C1_GMUX_W<'_, REMAP5_SPEC> {
        I2C1_GMUX_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - I2C2 muxing"]
    #[inline(always)]
    pub fn i2c2_gmux(&mut self) -> I2C2_GMUX_W<'_, REMAP5_SPEC> {
        I2C2_GMUX_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    pub fn spi1_gmux(&mut self) -> SPI1_GMUX_W<'_, REMAP5_SPEC> {
        SPI1_GMUX_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    pub fn spi2_gmux(&mut self) -> SPI2_GMUX_W<'_, REMAP5_SPEC> {
        SPI2_GMUX_W::new(self, 20)
    }
}
#[doc = "IO MUX remap register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`remap5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP5_SPEC;
impl crate::RegisterSpec for REMAP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap5::R`](R) reader structure"]
impl crate::Readable for REMAP5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap5::W`](W) writer structure"]
impl crate::Writable for REMAP5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP5 to value 0"]
impl crate::Resettable for REMAP5_SPEC {}
