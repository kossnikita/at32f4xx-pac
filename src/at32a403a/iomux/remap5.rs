#[doc = "Register `REMAP5` reader"]
pub type R = crate::R<REMAP5_SPEC>;
#[doc = "Register `REMAP5` writer"]
pub type W = crate::W<REMAP5_SPEC>;
#[doc = "Field `USART5_GMUX` reader - USART5 muxing"]
pub type USART5_GMUX_R = crate::FieldReader;
#[doc = "Field `USART5_GMUX` writer - USART5 muxing"]
pub type USART5_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `I2C1_GMUX` reader - I2C1 muxing"]
pub type I2C1_GMUX_R = crate::FieldReader;
#[doc = "Field `I2C1_GMUX` writer - I2C1 muxing"]
pub type I2C1_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `I2C3_GMUX` reader - I2C3 muxing"]
pub type I2C3_GMUX_R = crate::FieldReader;
#[doc = "Field `I2C3_GMUX` writer - I2C3 muxing"]
pub type I2C3_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SPI1_GMUX` reader - SPI1 muxing"]
pub type SPI1_GMUX_R = crate::FieldReader;
#[doc = "Field `SPI1_GMUX` writer - SPI1 muxing"]
pub type SPI1_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SPI2_GMUX` reader - SPI2 muxing"]
pub type SPI2_GMUX_R = crate::FieldReader;
#[doc = "Field `SPI2_GMUX` writer - SPI2 muxing"]
pub type SPI2_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SPI3_GMUX` reader - SPI3 muxing"]
pub type SPI3_GMUX_R = crate::FieldReader;
#[doc = "Field `SPI3_GMUX` writer - SPI3 muxing"]
pub type SPI3_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SPI4_GMUX` reader - SPI4 muxing"]
pub type SPI4_GMUX_R = crate::FieldReader;
#[doc = "Field `SPI4_GMUX` writer - SPI4 muxing"]
pub type SPI4_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - USART5 muxing"]
    #[inline(always)]
    pub fn usart5_gmux(&self) -> USART5_GMUX_R {
        USART5_GMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_gmux(&self) -> I2C1_GMUX_R {
        I2C1_GMUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I2C3 muxing"]
    #[inline(always)]
    pub fn i2c3_gmux(&self) -> I2C3_GMUX_R {
        I2C3_GMUX_R::new(((self.bits >> 12) & 0x0f) as u8)
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
    #[doc = "Bits 24:27 - SPI3 muxing"]
    #[inline(always)]
    pub fn spi3_gmux(&self) -> SPI3_GMUX_R {
        SPI3_GMUX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SPI4 muxing"]
    #[inline(always)]
    pub fn spi4_gmux(&self) -> SPI4_GMUX_R {
        SPI4_GMUX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP5")
            .field(
                "usart5_gmux",
                &format_args!("{}", self.usart5_gmux().bits()),
            )
            .field("i2c1_gmux", &format_args!("{}", self.i2c1_gmux().bits()))
            .field("i2c3_gmux", &format_args!("{}", self.i2c3_gmux().bits()))
            .field("spi1_gmux", &format_args!("{}", self.spi1_gmux().bits()))
            .field("spi2_gmux", &format_args!("{}", self.spi2_gmux().bits()))
            .field("spi3_gmux", &format_args!("{}", self.spi3_gmux().bits()))
            .field("spi4_gmux", &format_args!("{}", self.spi4_gmux().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART5 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart5_gmux(&mut self) -> USART5_GMUX_W<REMAP5_SPEC, 0> {
        USART5_GMUX_W::new(self)
    }
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_gmux(&mut self) -> I2C1_GMUX_W<REMAP5_SPEC, 4> {
        I2C1_GMUX_W::new(self)
    }
    #[doc = "Bits 12:15 - I2C3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_gmux(&mut self) -> I2C3_GMUX_W<REMAP5_SPEC, 12> {
        I2C3_GMUX_W::new(self)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_gmux(&mut self) -> SPI1_GMUX_W<REMAP5_SPEC, 16> {
        SPI1_GMUX_W::new(self)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_gmux(&mut self) -> SPI2_GMUX_W<REMAP5_SPEC, 20> {
        SPI2_GMUX_W::new(self)
    }
    #[doc = "Bits 24:27 - SPI3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_gmux(&mut self) -> SPI3_GMUX_W<REMAP5_SPEC, 24> {
        SPI3_GMUX_W::new(self)
    }
    #[doc = "Bits 28:31 - SPI4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi4_gmux(&mut self) -> SPI4_GMUX_W<REMAP5_SPEC, 28> {
        SPI4_GMUX_W::new(self)
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
#[doc = "IO MUX remap register 5 (IOMUX_REMAP5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
