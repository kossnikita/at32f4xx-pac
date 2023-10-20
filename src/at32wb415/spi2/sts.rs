#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RDBF_R = crate::BitReader<RDBF_A>;
#[doc = "Receive data buffer full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBF_A {
    #[doc = "0: Transmit data buffer is not full"]
    NotFull = 0,
    #[doc = "1: Transmit data buffer is full"]
    Full = 1,
}
impl From<RDBF_A> for bool {
    #[inline(always)]
    fn from(variant: RDBF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDBF_A {
        match self.bits {
            false => RDBF_A::NotFull,
            true => RDBF_A::Full,
        }
    }
    #[doc = "Transmit data buffer is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RDBF_A::NotFull
    }
    #[doc = "Transmit data buffer is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RDBF_A::Full
    }
}
#[doc = "Field `TDBE` reader - Transmit data buffer empty"]
pub type TDBE_R = crate::BitReader<TDBE_A>;
#[doc = "Transmit data buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBE_A {
    #[doc = "0: Transmit data buffer is not empty"]
    NotEmpty = 0,
    #[doc = "1: Transmit data buffer is empty"]
    Empty = 1,
}
impl From<TDBE_A> for bool {
    #[inline(always)]
    fn from(variant: TDBE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDBE_A {
        match self.bits {
            false => TDBE_A::NotEmpty,
            true => TDBE_A::Empty,
        }
    }
    #[doc = "Transmit data buffer is not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TDBE_A::NotEmpty
    }
    #[doc = "Transmit data buffer is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TDBE_A::Empty
    }
}
#[doc = "Field `CCERR` reader - CRC calculation error"]
pub type CCERR_R = crate::BitReader<CCERRR_A>;
#[doc = "CRC calculation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCERRR_A {
    #[doc = "0: No CRC error"]
    NoError = 0,
    #[doc = "1: CRC error occurs"]
    Error = 1,
}
impl From<CCERRR_A> for bool {
    #[inline(always)]
    fn from(variant: CCERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCERRR_A {
        match self.bits {
            false => CCERRR_A::NoError,
            true => CCERRR_A::Error,
        }
    }
    #[doc = "No CRC error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == CCERRR_A::NoError
    }
    #[doc = "CRC error occurs"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == CCERRR_A::Error
    }
}
#[doc = "CRC calculation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCERRW_AW {
    #[doc = "0: Clear CRC error"]
    Clear = 0,
}
impl From<CCERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: CCERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCERR` writer - CRC calculation error"]
pub type CCERR_W<'a, REG> = crate::BitWriter0C<'a, REG, CCERRW_AW>;
impl<'a, REG> CCERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CRC error"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCERRW_AW::Clear)
    }
}
#[doc = "Field `MMERR` reader - Master mode error"]
pub type MMERR_R = crate::BitReader<MMERR_A>;
#[doc = "Master mode error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMERR_A {
    #[doc = "0: No mode error"]
    NoError = 0,
    #[doc = "1: Mode error occurs"]
    Error = 1,
}
impl From<MMERR_A> for bool {
    #[inline(always)]
    fn from(variant: MMERR_A) -> Self {
        variant as u8 != 0
    }
}
impl MMERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMERR_A {
        match self.bits {
            false => MMERR_A::NoError,
            true => MMERR_A::Error,
        }
    }
    #[doc = "No mode error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MMERR_A::NoError
    }
    #[doc = "Mode error occurs"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MMERR_A::Error
    }
}
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type ROERR_R = crate::BitReader<ROERR_A>;
#[doc = "Receiver overflow error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROERR_A {
    #[doc = "0: No overflow error"]
    NoError = 0,
    #[doc = "1: Overflow error occurs"]
    Overflow = 1,
}
impl From<ROERR_A> for bool {
    #[inline(always)]
    fn from(variant: ROERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ROERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROERR_A {
        match self.bits {
            false => ROERR_A::NoError,
            true => ROERR_A::Overflow,
        }
    }
    #[doc = "No overflow error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ROERR_A::NoError
    }
    #[doc = "Overflow error occurs"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == ROERR_A::Overflow
    }
}
#[doc = "Field `BF` reader - Busy flag"]
pub type BF_R = crate::BitReader<BF_A>;
#[doc = "Busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BF_A {
    #[doc = "0: SPI is not busy"]
    Idle = 0,
    #[doc = "1: SPI is busy"]
    Busy = 1,
}
impl From<BF_A> for bool {
    #[inline(always)]
    fn from(variant: BF_A) -> Self {
        variant as u8 != 0
    }
}
impl BF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BF_A {
        match self.bits {
            false => BF_A::Idle,
            true => BF_A::Busy,
        }
    }
    #[doc = "SPI is not busy"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BF_A::Idle
    }
    #[doc = "SPI is busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BF_A::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Receive data buffer full"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC calculation error"]
    #[inline(always)]
    pub fn ccerr(&self) -> CCERR_R {
        CCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master mode error"]
    #[inline(always)]
    pub fn mmerr(&self) -> MMERR_R {
        MMERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receiver overflow error"]
    #[inline(always)]
    pub fn roerr(&self) -> ROERR_R {
        ROERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bf(&self) -> BF_R {
        BF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("bf", &format_args!("{}", self.bf().bit()))
            .field("roerr", &format_args!("{}", self.roerr().bit()))
            .field("mmerr", &format_args!("{}", self.mmerr().bit()))
            .field("ccerr", &format_args!("{}", self.ccerr().bit()))
            .field("tdbe", &format_args!("{}", self.tdbe().bit()))
            .field("rdbf", &format_args!("{}", self.rdbf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - CRC calculation error"]
    #[inline(always)]
    #[must_use]
    pub fn ccerr(&mut self) -> CCERR_W<STS_SPEC> {
        CCERR_W::new(self, 4)
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
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0x10;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0x02"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
