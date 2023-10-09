#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `PERR` reader - Parity error"]
pub type PERR_R = crate::BitReader<PERR_A>;
#[doc = "Parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERR_A {
    #[doc = "0: No parity error occurs"]
    NoError = 0,
    #[doc = "1: Parity error occurs"]
    Error = 1,
}
impl From<PERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERR_A {
        match self.bits {
            false => PERR_A::NoError,
            true => PERR_A::Error,
        }
    }
    #[doc = "No parity error occurs"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PERR_A::NoError
    }
    #[doc = "Parity error occurs"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PERR_A::Error
    }
}
#[doc = "Field `FERR` reader - Framing error"]
pub type FERR_R = crate::BitReader<FERR_A>;
#[doc = "Framing error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FERR_A {
    #[doc = "0: No framing error is detected"]
    NoError = 0,
    #[doc = "1: Framing error is detected"]
    Error = 1,
}
impl From<FERR_A> for bool {
    #[inline(always)]
    fn from(variant: FERR_A) -> Self {
        variant as u8 != 0
    }
}
impl FERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FERR_A {
        match self.bits {
            false => FERR_A::NoError,
            true => FERR_A::Error,
        }
    }
    #[doc = "No framing error is detected"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FERR_A::NoError
    }
    #[doc = "Framing error is detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FERR_A::Error
    }
}
#[doc = "Field `NERR` reader - Noise error"]
pub type NERR_R = crate::BitReader<NERR_A>;
#[doc = "Noise error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NERR_A {
    #[doc = "0: No noise is detected"]
    NoNoise = 0,
    #[doc = "1: Noise is detected"]
    Noise = 1,
}
impl From<NERR_A> for bool {
    #[inline(always)]
    fn from(variant: NERR_A) -> Self {
        variant as u8 != 0
    }
}
impl NERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NERR_A {
        match self.bits {
            false => NERR_A::NoNoise,
            true => NERR_A::Noise,
        }
    }
    #[doc = "No noise is detected"]
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == NERR_A::NoNoise
    }
    #[doc = "Noise is detected"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NERR_A::Noise
    }
}
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type ROERR_R = crate::BitReader<ROERR_A>;
#[doc = "Receiver overflow error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROERR_A {
    #[doc = "0: No overflow error"]
    NoError = 0,
    #[doc = "1: Overflow error is detected"]
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
    pub fn variant(&self) -> ROERR_A {
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
    #[doc = "Overflow error is detected"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == ROERR_A::Overflow
    }
}
#[doc = "Field `IDLEF` reader - IDLE flag"]
pub type IDLEF_R = crate::BitReader<IDLEF_A>;
#[doc = "IDLE flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEF_A {
    #[doc = "0: No idle line is detected"]
    NoIdle = 0,
    #[doc = "1: Idle line is detected"]
    Idle = 1,
}
impl From<IDLEF_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEF_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLEF_A {
        match self.bits {
            false => IDLEF_A::NoIdle,
            true => IDLEF_A::Idle,
        }
    }
    #[doc = "No idle line is detected"]
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == IDLEF_A::NoIdle
    }
    #[doc = "Idle line is detected"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLEF_A::Idle
    }
}
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RDBF_R = crate::BitReader<RDBFR_A>;
#[doc = "Receive data buffer full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFR_A {
    #[doc = "0: Data is not received"]
    NotFull = 0,
    #[doc = "1: Data is received"]
    Full = 1,
}
impl From<RDBFR_A> for bool {
    #[inline(always)]
    fn from(variant: RDBFR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDBFR_A {
        match self.bits {
            false => RDBFR_A::NotFull,
            true => RDBFR_A::Full,
        }
    }
    #[doc = "Data is not received"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RDBFR_A::NotFull
    }
    #[doc = "Data is received"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RDBFR_A::Full
    }
}
#[doc = "Receive data buffer full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFW_AW {
    #[doc = "0: Clear receive buffer full flag"]
    Clear = 0,
}
impl From<RDBFW_AW> for bool {
    #[inline(always)]
    fn from(variant: RDBFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBF` writer - Receive data buffer full"]
pub type RDBF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, RDBFW_AW>;
impl<'a, REG, const O: u8> RDBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear receive buffer full flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RDBFW_AW::Clear)
    }
}
#[doc = "Field `TDC` reader - Transmit data complete"]
pub type TDC_R = crate::BitReader<TDCR_A>;
#[doc = "Transmit data complete\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCR_A {
    #[doc = "0: Transmission is not completed"]
    NotCompleted = 0,
    #[doc = "1: Transmission is completed"]
    Completed = 1,
}
impl From<TDCR_A> for bool {
    #[inline(always)]
    fn from(variant: TDCR_A) -> Self {
        variant as u8 != 0
    }
}
impl TDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCR_A {
        match self.bits {
            false => TDCR_A::NotCompleted,
            true => TDCR_A::Completed,
        }
    }
    #[doc = "Transmission is not completed"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TDCR_A::NotCompleted
    }
    #[doc = "Transmission is completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TDCR_A::Completed
    }
}
#[doc = "Transmit data complete\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCW_AW {
    #[doc = "0: Clear transmit data complete flag"]
    Clear = 0,
}
impl From<TDCW_AW> for bool {
    #[inline(always)]
    fn from(variant: TDCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` writer - Transmit data complete"]
pub type TDC_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TDCW_AW>;
impl<'a, REG, const O: u8> TDC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmit data complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TDCW_AW::Clear)
    }
}
#[doc = "Field `TDBE` reader - Transmit data buffer empty"]
pub type TDBE_R = crate::BitReader<TDBE_A>;
#[doc = "Transmit data buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBE_A {
    #[doc = "0: Data is not transferred to the shift register"]
    NotEmpty = 0,
    #[doc = "1: Data is transferred to the shift register"]
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
    pub fn variant(&self) -> TDBE_A {
        match self.bits {
            false => TDBE_A::NotEmpty,
            true => TDBE_A::Empty,
        }
    }
    #[doc = "Data is not transferred to the shift register"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TDBE_A::NotEmpty
    }
    #[doc = "Data is transferred to the shift register"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TDBE_A::Empty
    }
}
#[doc = "Field `BFF` reader - Break frame flag"]
pub type BFF_R = crate::BitReader<BFFR_A>;
#[doc = "Break frame flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFFR_A {
    #[doc = "0: Break frame is not detected"]
    NoBreak = 0,
    #[doc = "1: Break frame is detected"]
    Break = 1,
}
impl From<BFFR_A> for bool {
    #[inline(always)]
    fn from(variant: BFFR_A) -> Self {
        variant as u8 != 0
    }
}
impl BFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFFR_A {
        match self.bits {
            false => BFFR_A::NoBreak,
            true => BFFR_A::Break,
        }
    }
    #[doc = "Break frame is not detected"]
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == BFFR_A::NoBreak
    }
    #[doc = "Break frame is detected"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == BFFR_A::Break
    }
}
#[doc = "Break frame flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFFW_AW {
    #[doc = "0: Clear break frame flag"]
    Clear = 0,
}
impl From<BFFW_AW> for bool {
    #[inline(always)]
    fn from(variant: BFFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFF` writer - Break frame flag"]
pub type BFF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, BFFW_AW>;
impl<'a, REG, const O: u8> BFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear break frame flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BFFW_AW::Clear)
    }
}
#[doc = "Field `CTSCF` reader - CTS change flag"]
pub type CTSCF_R = crate::BitReader<CTSCFR_A>;
#[doc = "CTS change flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCFR_A {
    #[doc = "0: No change on the CTS status line"]
    NoChange = 0,
    #[doc = "1: A change occurs on the CTS status line"]
    Change = 1,
}
impl From<CTSCFR_A> for bool {
    #[inline(always)]
    fn from(variant: CTSCFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSCFR_A {
        match self.bits {
            false => CTSCFR_A::NoChange,
            true => CTSCFR_A::Change,
        }
    }
    #[doc = "No change on the CTS status line"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CTSCFR_A::NoChange
    }
    #[doc = "A change occurs on the CTS status line"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == CTSCFR_A::Change
    }
}
#[doc = "CTS change flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCFW_AW {
    #[doc = "0: Clear CTS change flag"]
    Clear = 0,
}
impl From<CTSCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCF` writer - CTS change flag"]
pub type CTSCF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, CTSCFW_AW>;
impl<'a, REG, const O: u8> CTSCF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CTS change flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTSCFW_AW::Clear)
    }
}
#[doc = "Field `RTODF` reader - Reiceiver time out detection flag"]
pub type RTODF_R = crate::BitReader;
#[doc = "Field `RTODF` writer - Reiceiver time out detection flag"]
pub type RTODF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDF` reader - Character match detection flag"]
pub type CMDF_R = crate::BitReader;
#[doc = "Field `CMDF` writer - Character match detection flag"]
pub type CMDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error"]
    #[inline(always)]
    pub fn nerr(&self) -> NERR_R {
        NERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver overflow error"]
    #[inline(always)]
    pub fn roerr(&self) -> ROERR_R {
        ROERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE flag"]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive data buffer full"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit data complete"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Break frame flag"]
    #[inline(always)]
    pub fn bff(&self) -> BFF_R {
        BFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    pub fn ctscf(&self) -> CTSCF_R {
        CTSCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Reiceiver time out detection flag"]
    #[inline(always)]
    pub fn rtodf(&self) -> RTODF_R {
        RTODF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match detection flag"]
    #[inline(always)]
    pub fn cmdf(&self) -> CMDF_R {
        CMDF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("cmdf", &format_args!("{}", self.cmdf().bit()))
            .field("rtodf", &format_args!("{}", self.rtodf().bit()))
            .field("ctscf", &format_args!("{}", self.ctscf().bit()))
            .field("bff", &format_args!("{}", self.bff().bit()))
            .field("tdbe", &format_args!("{}", self.tdbe().bit()))
            .field("tdc", &format_args!("{}", self.tdc().bit()))
            .field("rdbf", &format_args!("{}", self.rdbf().bit()))
            .field("idlef", &format_args!("{}", self.idlef().bit()))
            .field("roerr", &format_args!("{}", self.roerr().bit()))
            .field("nerr", &format_args!("{}", self.nerr().bit()))
            .field("ferr", &format_args!("{}", self.ferr().bit()))
            .field("perr", &format_args!("{}", self.perr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 5 - Receive data buffer full"]
    #[inline(always)]
    #[must_use]
    pub fn rdbf(&mut self) -> RDBF_W<STS_SPEC, 5> {
        RDBF_W::new(self)
    }
    #[doc = "Bit 6 - Transmit data complete"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TDC_W<STS_SPEC, 6> {
        TDC_W::new(self)
    }
    #[doc = "Bit 8 - Break frame flag"]
    #[inline(always)]
    #[must_use]
    pub fn bff(&mut self) -> BFF_W<STS_SPEC, 8> {
        BFF_W::new(self)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<STS_SPEC, 9> {
        CTSCF_W::new(self)
    }
    #[doc = "Bit 11 - Reiceiver time out detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtodf(&mut self) -> RTODF_W<STS_SPEC, 11> {
        RTODF_W::new(self)
    }
    #[doc = "Bit 17 - Character match detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmdf(&mut self) -> CMDF_W<STS_SPEC, 17> {
        CMDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0360;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0xc0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
