#[doc = "Register `REMAP5` reader"]
pub type R = crate::R<REMAP5_SPEC>;
#[doc = "Register `REMAP5` writer"]
pub type W = crate::W<REMAP5_SPEC>;
#[doc = "I2C1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1_GMUX_A {
    #[doc = "0: SCL/PB6, SDA/PB7, SMBA/PB5"]
    Mux0 = 0,
    #[doc = "1: SCL/PB8, SDA/PB9, SMBA/PB5"]
    Mux1 = 1,
    #[doc = "2: SCL/PF6, SDA/PF7, SMBA/PB5"]
    Mux2 = 2,
}
impl From<I2C1_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1_GMUX_A {
    type Ux = u8;
}
impl crate::IsEnum for I2C1_GMUX_A {}
#[doc = "Field `I2C1_GMUX` reader - I2C1 muxing"]
pub type I2C1_GMUX_R = crate::FieldReader<I2C1_GMUX_A>;
impl I2C1_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C1_GMUX_A> {
        match self.bits {
            0 => Some(I2C1_GMUX_A::Mux0),
            1 => Some(I2C1_GMUX_A::Mux1),
            2 => Some(I2C1_GMUX_A::Mux2),
            _ => None,
        }
    }
    #[doc = "SCL/PB6, SDA/PB7, SMBA/PB5"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == I2C1_GMUX_A::Mux0
    }
    #[doc = "SCL/PB8, SDA/PB9, SMBA/PB5"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == I2C1_GMUX_A::Mux1
    }
    #[doc = "SCL/PF6, SDA/PF7, SMBA/PB5"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == I2C1_GMUX_A::Mux2
    }
}
#[doc = "Field `I2C1_GMUX` writer - I2C1 muxing"]
pub type I2C1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, I2C1_GMUX_A>;
impl<'a, REG> I2C1_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCL/PB6, SDA/PB7, SMBA/PB5"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_GMUX_A::Mux0)
    }
    #[doc = "SCL/PB8, SDA/PB9, SMBA/PB5"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_GMUX_A::Mux1)
    }
    #[doc = "SCL/PF6, SDA/PF7, SMBA/PB5"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_GMUX_A::Mux2)
    }
}
#[doc = "I2C2 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C2_GMUX_A {
    #[doc = "0: SCL/PB10, SDA/PB11, SMBA/PB12"]
    Mux0 = 0,
    #[doc = "1: SCL/PA8, SDA/PC9, SMBA/PA9"]
    Mux1 = 1,
    #[doc = "2: SCL/PA8, SDA/PB4, SMBA/PA9"]
    Mux2 = 2,
    #[doc = "3: SCL/PF6, SDA/PF7, SMBA/PA9"]
    Mux3 = 3,
}
impl From<I2C2_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C2_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C2_GMUX_A {
    type Ux = u8;
}
impl crate::IsEnum for I2C2_GMUX_A {}
#[doc = "Field `I2C2_GMUX` reader - I2C2 muxing"]
pub type I2C2_GMUX_R = crate::FieldReader<I2C2_GMUX_A>;
impl I2C2_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<I2C2_GMUX_A> {
        match self.bits {
            0 => Some(I2C2_GMUX_A::Mux0),
            1 => Some(I2C2_GMUX_A::Mux1),
            2 => Some(I2C2_GMUX_A::Mux2),
            3 => Some(I2C2_GMUX_A::Mux3),
            _ => None,
        }
    }
    #[doc = "SCL/PB10, SDA/PB11, SMBA/PB12"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == I2C2_GMUX_A::Mux0
    }
    #[doc = "SCL/PA8, SDA/PC9, SMBA/PA9"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == I2C2_GMUX_A::Mux1
    }
    #[doc = "SCL/PA8, SDA/PB4, SMBA/PA9"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == I2C2_GMUX_A::Mux2
    }
    #[doc = "SCL/PF6, SDA/PF7, SMBA/PA9"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == I2C2_GMUX_A::Mux3
    }
}
#[doc = "Field `I2C2_GMUX` writer - I2C2 muxing"]
pub type I2C2_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, I2C2_GMUX_A>;
impl<'a, REG> I2C2_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCL/PB10, SDA/PB11, SMBA/PB12"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_GMUX_A::Mux0)
    }
    #[doc = "SCL/PA8, SDA/PC9, SMBA/PA9"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_GMUX_A::Mux1)
    }
    #[doc = "SCL/PA8, SDA/PB4, SMBA/PA9"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_GMUX_A::Mux2)
    }
    #[doc = "SCL/PF6, SDA/PF7, SMBA/PA9"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_GMUX_A::Mux3)
    }
}
#[doc = "SPI1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1_GMUX_A {
    #[doc = "0: CS/PA4, SCK/PA5, MISO/PA6, MOSI/PA7, MCK/PB0"]
    Mux0 = 0,
    #[doc = "1: CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5, MCK/PB6"]
    Mux1 = 1,
}
impl From<SPI1_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1_GMUX_A {
    type Ux = u8;
}
impl crate::IsEnum for SPI1_GMUX_A {}
#[doc = "Field `SPI1_GMUX` reader - SPI1 muxing"]
pub type SPI1_GMUX_R = crate::FieldReader<SPI1_GMUX_A>;
impl SPI1_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI1_GMUX_A> {
        match self.bits {
            0 => Some(SPI1_GMUX_A::Mux0),
            1 => Some(SPI1_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "CS/PA4, SCK/PA5, MISO/PA6, MOSI/PA7, MCK/PB0"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == SPI1_GMUX_A::Mux0
    }
    #[doc = "CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5, MCK/PB6"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == SPI1_GMUX_A::Mux1
    }
}
#[doc = "Field `SPI1_GMUX` writer - SPI1 muxing"]
pub type SPI1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SPI1_GMUX_A>;
impl<'a, REG> SPI1_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS/PA4, SCK/PA5, MISO/PA6, MOSI/PA7, MCK/PB0"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_GMUX_A::Mux0)
    }
    #[doc = "CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5, MCK/PB6"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_GMUX_A::Mux1)
    }
}
#[doc = "SPI2 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI2_GMUX_A {
    #[doc = "0: CS/PB12, SCK/PB13, MISO/PB14, MOSI/PB15, MCK/PC6"]
    Mux0 = 0,
    #[doc = "1: CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5, MCK/PC7"]
    Mux1 = 1,
}
impl From<SPI2_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI2_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI2_GMUX_A {
    type Ux = u8;
}
impl crate::IsEnum for SPI2_GMUX_A {}
#[doc = "Field `SPI2_GMUX` reader - SPI2 muxing"]
pub type SPI2_GMUX_R = crate::FieldReader<SPI2_GMUX_A>;
impl SPI2_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI2_GMUX_A> {
        match self.bits {
            0 => Some(SPI2_GMUX_A::Mux0),
            1 => Some(SPI2_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "CS/PB12, SCK/PB13, MISO/PB14, MOSI/PB15, MCK/PC6"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == SPI2_GMUX_A::Mux0
    }
    #[doc = "CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5, MCK/PC7"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == SPI2_GMUX_A::Mux1
    }
}
#[doc = "Field `SPI2_GMUX` writer - SPI2 muxing"]
pub type SPI2_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SPI2_GMUX_A>;
impl<'a, REG> SPI2_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS/PB12, SCK/PB13, MISO/PB14, MOSI/PB15, MCK/PC6"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2_GMUX_A::Mux0)
    }
    #[doc = "CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5, MCK/PC7"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2_GMUX_A::Mux1)
    }
}
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
            .field("spi2_gmux", &self.spi2_gmux())
            .field("spi1_gmux", &self.spi1_gmux())
            .field("i2c2_gmux", &self.i2c2_gmux())
            .field("i2c1_gmux", &self.i2c1_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_gmux(&mut self) -> I2C1_GMUX_W<REMAP5_SPEC> {
        I2C1_GMUX_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - I2C2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_gmux(&mut self) -> I2C2_GMUX_W<REMAP5_SPEC> {
        I2C2_GMUX_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_gmux(&mut self) -> SPI1_GMUX_W<REMAP5_SPEC> {
        SPI1_GMUX_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_gmux(&mut self) -> SPI2_GMUX_W<REMAP5_SPEC> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP5 to value 0"]
impl crate::Resettable for REMAP5_SPEC {
    const RESET_VALUE: u32 = 0;
}
