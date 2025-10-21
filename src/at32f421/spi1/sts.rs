#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
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
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RDBF_R = crate::BitReader<RDBF_A>;
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
#[doc = "Field `TDBE` reader - Transmit data buffer empty"]
pub type TDBE_R = crate::BitReader<TDBE_A>;
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
#[doc = "Audio channel state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACS_A {
    #[doc = "0: Left channel"]
    Left = 0,
    #[doc = "1: Right channel"]
    Right = 1,
}
impl From<ACS_A> for bool {
    #[inline(always)]
    fn from(variant: ACS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACS` reader - Audio channel state"]
pub type ACS_R = crate::BitReader<ACS_A>;
impl ACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACS_A {
        match self.bits {
            false => ACS_A::Left,
            true => ACS_A::Right,
        }
    }
    #[doc = "Left channel"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ACS_A::Left
    }
    #[doc = "Right channel"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ACS_A::Right
    }
}
#[doc = "Transmitter underload error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUERR_A {
    #[doc = "0: No underload error"]
    NoError = 0,
    #[doc = "1: Underload error occurs"]
    Underload = 1,
}
impl From<TUERR_A> for bool {
    #[inline(always)]
    fn from(variant: TUERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TUERR` reader - Transmitter underload error"]
pub type TUERR_R = crate::BitReader<TUERR_A>;
impl TUERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TUERR_A {
        match self.bits {
            false => TUERR_A::NoError,
            true => TUERR_A::Underload,
        }
    }
    #[doc = "No underload error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TUERR_A::NoError
    }
    #[doc = "Underload error occurs"]
    #[inline(always)]
    pub fn is_underload(&self) -> bool {
        *self == TUERR_A::Underload
    }
}
#[doc = "CRC calculation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccerrr {
    #[doc = "0: No CRC error"]
    NoError = 0,
    #[doc = "1: CRC error occurs"]
    Error = 1,
}
impl From<Ccerrr> for bool {
    #[inline(always)]
    fn from(variant: Ccerrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCERR` reader - CRC calculation error"]
pub type CCERR_R = crate::BitReader<Ccerrr>;
impl CCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccerrr {
        match self.bits {
            false => Ccerrr::NoError,
            true => Ccerrr::Error,
        }
    }
    #[doc = "No CRC error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Ccerrr::NoError
    }
    #[doc = "CRC error occurs"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Ccerrr::Error
    }
}
#[doc = "CRC calculation error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcerrwWO {
    #[doc = "0: Clear CRC error"]
    Clear = 0,
}
impl From<CcerrwWO> for bool {
    #[inline(always)]
    fn from(variant: CcerrwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCERR` writer - CRC calculation error"]
pub type CCERR_W<'a, REG> = crate::BitWriter0C<'a, REG, CcerrwWO>;
impl<'a, REG> CCERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CRC error"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CcerrwWO::Clear)
    }
}
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
#[doc = "Field `MMERR` reader - Master mode error"]
pub type MMERR_R = crate::BitReader<MMERR_A>;
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
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type ROERR_R = crate::BitReader<ROERR_A>;
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
#[doc = "Field `BF` reader - Busy flag"]
pub type BF_R = crate::BitReader<BF_A>;
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
    #[doc = "Bit 2 - Audio channel state"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter underload error"]
    #[inline(always)]
    pub fn tuerr(&self) -> TUERR_R {
        TUERR_R::new(((self.bits >> 3) & 1) != 0)
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
            .field("bf", &self.bf())
            .field("roerr", &self.roerr())
            .field("mmerr", &self.mmerr())
            .field("ccerr", &self.ccerr())
            .field("tuerr", &self.tuerr())
            .field("acs", &self.acs())
            .field("tdbe", &self.tdbe())
            .field("rdbf", &self.rdbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - CRC calculation error"]
    #[inline(always)]
    pub fn ccerr(&mut self) -> CCERR_W<'_, STS_SPEC> {
        CCERR_W::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets STS to value 0x02"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
