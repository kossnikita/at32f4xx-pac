#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `RST` reader - Reset bit"]
pub type RST_R = crate::BitReader<RSTW_A>;
#[doc = "Reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTW_A {
    #[doc = "1: Reset CRC calculation unit, the data register is set as 0xFFFF FFFF"]
    Reset = 1,
}
impl From<RSTW_A> for bool {
    #[inline(always)]
    fn from(variant: RSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSTW_A> {
        match self.bits {
            true => Some(RSTW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Reset CRC calculation unit, the data register is set as 0xFFFF FFFF"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RSTW_A::Reset
    }
}
#[doc = "Field `RST` writer - Reset bit"]
pub type RST_W<'a, REG> = crate::BitWriter1S<'a, REG, RSTW_A>;
impl<'a, REG> RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset CRC calculation unit, the data register is set as 0xFFFF FFFF"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RSTW_A::Reset)
    }
}
#[doc = "Field `POLY_SIZE` reader - Polynomial size"]
pub type POLY_SIZE_R = crate::FieldReader<POLY_SIZE_A>;
#[doc = "Polynomial size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLY_SIZE_A {
    #[doc = "0: 32 bits"]
    Bits32 = 0,
    #[doc = "1: 16 bits"]
    Bits16 = 1,
    #[doc = "2: 8 bits"]
    Bits8 = 2,
    #[doc = "3: 7 bits"]
    Bits7 = 3,
}
impl From<POLY_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLY_SIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POLY_SIZE_A {
    type Ux = u8;
}
impl POLY_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLY_SIZE_A {
        match self.bits {
            0 => POLY_SIZE_A::Bits32,
            1 => POLY_SIZE_A::Bits16,
            2 => POLY_SIZE_A::Bits8,
            3 => POLY_SIZE_A::Bits7,
            _ => unreachable!(),
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == POLY_SIZE_A::Bits32
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == POLY_SIZE_A::Bits16
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == POLY_SIZE_A::Bits8
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn is_bits7(&self) -> bool {
        *self == POLY_SIZE_A::Bits7
    }
}
#[doc = "Field `POLY_SIZE` writer - Polynomial size"]
pub type POLY_SIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, POLY_SIZE_A>;
impl<'a, REG> POLY_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(POLY_SIZE_A::Bits32)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(POLY_SIZE_A::Bits16)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(POLY_SIZE_A::Bits8)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn bits7(self) -> &'a mut crate::W<REG> {
        self.variant(POLY_SIZE_A::Bits7)
    }
}
#[doc = "Field `REVID` reader - Reverse input data"]
pub type REVID_R = crate::FieldReader<REVID_A>;
#[doc = "Reverse input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REVID_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Byte reverse"]
    ByteReverse = 1,
    #[doc = "2: Half-word reverse"]
    HalfWordReverse = 2,
    #[doc = "3: Word reverse"]
    WordReverse = 3,
}
impl From<REVID_A> for u8 {
    #[inline(always)]
    fn from(variant: REVID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REVID_A {
    type Ux = u8;
}
impl REVID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REVID_A {
        match self.bits {
            0 => REVID_A::NoEffect,
            1 => REVID_A::ByteReverse,
            2 => REVID_A::HalfWordReverse,
            3 => REVID_A::WordReverse,
            _ => unreachable!(),
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == REVID_A::NoEffect
    }
    #[doc = "Byte reverse"]
    #[inline(always)]
    pub fn is_byte_reverse(&self) -> bool {
        *self == REVID_A::ByteReverse
    }
    #[doc = "Half-word reverse"]
    #[inline(always)]
    pub fn is_half_word_reverse(&self) -> bool {
        *self == REVID_A::HalfWordReverse
    }
    #[doc = "Word reverse"]
    #[inline(always)]
    pub fn is_word_reverse(&self) -> bool {
        *self == REVID_A::WordReverse
    }
}
#[doc = "Field `REVID` writer - Reverse input data"]
pub type REVID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, REVID_A>;
impl<'a, REG> REVID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(REVID_A::NoEffect)
    }
    #[doc = "Byte reverse"]
    #[inline(always)]
    pub fn byte_reverse(self) -> &'a mut crate::W<REG> {
        self.variant(REVID_A::ByteReverse)
    }
    #[doc = "Half-word reverse"]
    #[inline(always)]
    pub fn half_word_reverse(self) -> &'a mut crate::W<REG> {
        self.variant(REVID_A::HalfWordReverse)
    }
    #[doc = "Word reverse"]
    #[inline(always)]
    pub fn word_reverse(self) -> &'a mut crate::W<REG> {
        self.variant(REVID_A::WordReverse)
    }
}
#[doc = "Field `REVOD` reader - Reverse output data"]
pub type REVOD_R = crate::BitReader<REVOD_A>;
#[doc = "Reverse output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REVOD_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Word reverse"]
    WordReverse = 1,
}
impl From<REVOD_A> for bool {
    #[inline(always)]
    fn from(variant: REVOD_A) -> Self {
        variant as u8 != 0
    }
}
impl REVOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REVOD_A {
        match self.bits {
            false => REVOD_A::NoEffect,
            true => REVOD_A::WordReverse,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == REVOD_A::NoEffect
    }
    #[doc = "Word reverse"]
    #[inline(always)]
    pub fn is_word_reverse(&self) -> bool {
        *self == REVOD_A::WordReverse
    }
}
impl R {
    #[doc = "Bit 0 - Reset bit"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    pub fn poly_size(&self) -> POLY_SIZE_R {
        POLY_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn revod(&self) -> REVOD_R {
        REVOD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("rst", &format_args!("{}", self.rst().bit()))
            .field("poly_size", &format_args!("{}", self.poly_size().bits()))
            .field("revid", &format_args!("{}", self.revid().bits()))
            .field("revod", &format_args!("{}", self.revod().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<CTRL_SPEC> {
        RST_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    #[must_use]
    pub fn poly_size(&mut self) -> POLY_SIZE_W<CTRL_SPEC> {
        POLY_SIZE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    #[must_use]
    pub fn revid(&mut self) -> REVID_W<CTRL_SPEC> {
        REVID_W::new(self, 5)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
