#[doc = "Register `RF%s` reader"]
pub type R = crate::R<RF_SPEC>;
#[doc = "Register `RF%s` writer"]
pub type W = crate::W<RF_SPEC>;
#[doc = "Field `MN` reader - Receive FIFO message num"]
pub type MN_R = crate::FieldReader;
#[doc = "Field `FF` reader - Receive FIFO full flag"]
pub type FF_R = crate::BitReader<FFR_A>;
#[doc = "Receive FIFO full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFR_A {
    #[doc = "0: Receive FIFO is not full"]
    NotFull = 0,
    #[doc = "1: Receive FIFO is full"]
    Full = 1,
}
impl From<FFR_A> for bool {
    #[inline(always)]
    fn from(variant: FFR_A) -> Self {
        variant as u8 != 0
    }
}
impl FF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFR_A {
        match self.bits {
            false => FFR_A::NotFull,
            true => FFR_A::Full,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FFR_A::NotFull
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FFR_A::Full
    }
}
#[doc = "Receive FIFO full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFW_AW {
    #[doc = "1: Clear full flag"]
    Clear = 1,
}
impl From<FFW_AW> for bool {
    #[inline(always)]
    fn from(variant: FFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FF` writer - Receive FIFO full flag"]
pub type FF_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, FFW_AW>;
impl<'a, REG, const O: u8> FF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear full flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FFW_AW::Clear)
    }
}
#[doc = "Field `OF` reader - Receive FIFO overflow flag"]
pub type OF_R = crate::BitReader<OFR_A>;
#[doc = "Receive FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFR_A {
    #[doc = "0: No overflow"]
    NoOverflow = 0,
    #[doc = "1: Receive FIFO overflow"]
    Overflow = 1,
}
impl From<OFR_A> for bool {
    #[inline(always)]
    fn from(variant: OFR_A) -> Self {
        variant as u8 != 0
    }
}
impl OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFR_A {
        match self.bits {
            false => OFR_A::NoOverflow,
            true => OFR_A::Overflow,
        }
    }
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == OFR_A::NoOverflow
    }
    #[doc = "Receive FIFO overflow"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OFR_A::Overflow
    }
}
#[doc = "Receive FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFW_AW {
    #[doc = "1: Clear overflow flag"]
    Clear = 1,
}
impl From<OFW_AW> for bool {
    #[inline(always)]
    fn from(variant: OFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OF` writer - Receive FIFO overflow flag"]
pub type OF_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, OFW_AW>;
impl<'a, REG, const O: u8> OF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear overflow flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OFW_AW::Clear)
    }
}
#[doc = "Field `R` reader - Receive FIFO release"]
pub type R_R = crate::BitReader<RR_A>;
#[doc = "Receive FIFO release\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR_A {
    #[doc = "0: FIFO is released"]
    Released = 0,
    #[doc = "1: FIFO release in progress"]
    Releasing = 1,
}
impl From<RR_A> for bool {
    #[inline(always)]
    fn from(variant: RR_A) -> Self {
        variant as u8 != 0
    }
}
impl R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR_A {
        match self.bits {
            false => RR_A::Released,
            true => RR_A::Releasing,
        }
    }
    #[doc = "FIFO is released"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RR_A::Released
    }
    #[doc = "FIFO release in progress"]
    #[inline(always)]
    pub fn is_releasing(&self) -> bool {
        *self == RR_A::Releasing
    }
}
#[doc = "Receive FIFO release\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RW_AW {
    #[doc = "1: Release FIFO"]
    Release = 1,
}
impl From<RW_AW> for bool {
    #[inline(always)]
    fn from(variant: RW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R` writer - Receive FIFO release"]
pub type R_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, RW_AW>;
impl<'a, REG, const O: u8> R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Release FIFO"]
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(RW_AW::Release)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive FIFO message num"]
    #[inline(always)]
    pub fn mn(&self) -> MN_R {
        MN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO full flag"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overflow flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO release"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF")
            .field("r", &format_args!("{}", self.r().bit()))
            .field("of", &format_args!("{}", self.of().bit()))
            .field("ff", &format_args!("{}", self.ff().bit()))
            .field("mn", &format_args!("{}", self.mn().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO full flag"]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<RF_SPEC, 3> {
        FF_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<RF_SPEC, 4> {
        OF_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO release"]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<RF_SPEC, 5> {
        R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive FIFO %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF_SPEC;
impl crate::RegisterSpec for RF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf::R`](R) reader structure"]
impl crate::Readable for RF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf::W`](W) writer structure"]
impl crate::Writable for RF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x38;
}
#[doc = "`reset()` method sets RF%s to value 0"]
impl crate::Resettable for RF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
