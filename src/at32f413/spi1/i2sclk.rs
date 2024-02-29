#[doc = "Register `I2SCLK` reader"]
pub type R = crate::R<I2SCLK_SPEC>;
#[doc = "Register `I2SCLK` writer"]
pub type W = crate::W<I2SCLK_SPEC>;
#[doc = "Field `DIV7_0` reader - I2S division bit7 to bit0"]
pub type DIV7_0_R = crate::FieldReader;
#[doc = "Field `DIV7_0` writer - I2S division bit7 to bit0"]
pub type DIV7_0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Odd result for I2S division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD_A {
    #[doc = "0: Actual divider factor = DIV*2"]
    Double = 0,
    #[doc = "1: Actual divider factor = (DIV*2)+1"]
    DoublePlusOne = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - Odd result for I2S division"]
pub type ODD_R = crate::BitReader<ODD_A>;
impl ODD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::Double,
            true => ODD_A::DoublePlusOne,
        }
    }
    #[doc = "Actual divider factor = DIV*2"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == ODD_A::Double
    }
    #[doc = "Actual divider factor = (DIV*2)+1"]
    #[inline(always)]
    pub fn is_double_plus_one(&self) -> bool {
        *self == ODD_A::DoublePlusOne
    }
}
#[doc = "Field `ODD` writer - Odd result for I2S division"]
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD_A>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Actual divider factor = DIV*2"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_A::Double)
    }
    #[doc = "Actual divider factor = (DIV*2)+1"]
    #[inline(always)]
    pub fn double_plus_one(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_A::DoublePlusOne)
    }
}
#[doc = "I2S master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkoer {
    #[doc = "0: Master clock output is disabled"]
    Disabled = 0,
    #[doc = "1: Master clock output is enabled"]
    Enabled = 1,
}
impl From<Mclkoer> for bool {
    #[inline(always)]
    fn from(variant: Mclkoer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKOE` reader - I2S master clock output enable"]
pub type MCLKOE_R = crate::BitReader<Mclkoer>;
impl MCLKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkoer {
        match self.bits {
            false => Mclkoer::Disabled,
            true => Mclkoer::Enabled,
        }
    }
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mclkoer::Disabled
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mclkoer::Enabled
    }
}
#[doc = "I2S master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MclkoewWO {
    #[doc = "0: Master clock output disable"]
    Disable = 0,
    #[doc = "1: Master clock output enable"]
    Enable = 1,
}
impl From<MclkoewWO> for bool {
    #[inline(always)]
    fn from(variant: MclkoewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKOE` writer - I2S master clock output enable"]
pub type MCLKOE_W<'a, REG> = crate::BitWriter<'a, REG, MclkoewWO>;
impl<'a, REG> MCLKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MclkoewWO::Disable)
    }
    #[doc = "Master clock output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MclkoewWO::Enable)
    }
}
#[doc = "Field `DIV9_8` reader - I2S division bit9 and bit8"]
pub type DIV9_8_R = crate::FieldReader;
#[doc = "Field `DIV9_8` writer - I2S division bit9 and bit8"]
pub type DIV9_8_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - I2S division bit7 to bit0"]
    #[inline(always)]
    pub fn div7_0(&self) -> DIV7_0_R {
        DIV7_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd result for I2S division"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2S master clock output enable"]
    #[inline(always)]
    pub fn mclkoe(&self) -> MCLKOE_R {
        MCLKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - I2S division bit9 and bit8"]
    #[inline(always)]
    pub fn div9_8(&self) -> DIV9_8_R {
        DIV9_8_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCLK")
            .field("div9_8", &format_args!("{}", self.div9_8().bits()))
            .field("mclkoe", &format_args!("{}", self.mclkoe().bit()))
            .field("odd", &format_args!("{}", self.odd().bit()))
            .field("div7_0", &format_args!("{}", self.div7_0().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<I2SCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S division bit7 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn div7_0(&mut self) -> DIV7_0_W<I2SCLK_SPEC> {
        DIV7_0_W::new(self, 0)
    }
    #[doc = "Bit 8 - Odd result for I2S division"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<I2SCLK_SPEC> {
        ODD_W::new(self, 8)
    }
    #[doc = "Bit 9 - I2S master clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclkoe(&mut self) -> MCLKOE_W<I2SCLK_SPEC> {
        MCLKOE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - I2S division bit9 and bit8"]
    #[inline(always)]
    #[must_use]
    pub fn div9_8(&mut self) -> DIV9_8_W<I2SCLK_SPEC> {
        DIV9_8_W::new(self, 10)
    }
}
#[doc = "I2S clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCLK_SPEC;
impl crate::RegisterSpec for I2SCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sclk::R`](R) reader structure"]
impl crate::Readable for I2SCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sclk::W`](W) writer structure"]
impl crate::Writable for I2SCLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCLK to value 0x0a"]
impl crate::Resettable for I2SCLK_SPEC {
    const RESET_VALUE: u32 = 0x0a;
}
