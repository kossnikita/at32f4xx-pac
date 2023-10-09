#[doc = "Register `TSTS` reader"]
pub type R = crate::R<TSTS_SPEC>;
#[doc = "Register `TSTS` writer"]
pub type W = crate::W<TSTS_SPEC>;
#[doc = "Field `TMTCF[0-2]` reader - Transmit mailbox %s transmission complete flag"]
pub type TMTCF_R = crate::BitReader<TM0TCFR_A>;
#[doc = "Transmit mailbox %s transmission complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0TCFR_A {
    #[doc = "0: Transmission in progress"]
    InProgress = 0,
    #[doc = "1: Transmission completed"]
    Completed = 1,
}
impl From<TM0TCFR_A> for bool {
    #[inline(always)]
    fn from(variant: TM0TCFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM0TCFR_A {
        match self.bits {
            false => TM0TCFR_A::InProgress,
            true => TM0TCFR_A::Completed,
        }
    }
    #[doc = "Transmission in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == TM0TCFR_A::InProgress
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TM0TCFR_A::Completed
    }
}
#[doc = "Transmit mailbox %s transmission complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0TCFW_AW {
    #[doc = "1: Clear transmission completed flag"]
    Clear = 1,
}
impl From<TM0TCFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TM0TCFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTCF[0-2]` writer - Transmit mailbox %s transmission complete flag"]
pub type TMTCF_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TM0TCFW_AW>;
impl<'a, REG, const O: u8> TMTCF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmission completed flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TM0TCFW_AW::Clear)
    }
}
#[doc = "Field `TMTSF[0-2]` reader - Transmit mailbox %s transmission success flag"]
pub type TMTSF_R = crate::BitReader<TM0TSFR_A>;
#[doc = "Transmit mailbox %s transmission success flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0TSFR_A {
    #[doc = "0: Transmission in progress"]
    Failed = 0,
    #[doc = "1: Transmission was successful"]
    Successful = 1,
}
impl From<TM0TSFR_A> for bool {
    #[inline(always)]
    fn from(variant: TM0TSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM0TSFR_A {
        match self.bits {
            false => TM0TSFR_A::Failed,
            true => TM0TSFR_A::Successful,
        }
    }
    #[doc = "Transmission in progress"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == TM0TSFR_A::Failed
    }
    #[doc = "Transmission was successful"]
    #[inline(always)]
    pub fn is_successful(&self) -> bool {
        *self == TM0TSFR_A::Successful
    }
}
#[doc = "Transmit mailbox %s transmission success flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0TSFW_AW {
    #[doc = "1: Clear transmission success flag"]
    Clear = 1,
}
impl From<TM0TSFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TM0TSFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTSF[0-2]` writer - Transmit mailbox %s transmission success flag"]
pub type TMTSF_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TM0TSFW_AW>;
impl<'a, REG, const O: u8> TMTSF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmission success flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TM0TSFW_AW::Clear)
    }
}
#[doc = "Field `TMALF[0-2]` reader - Transmit mailbox %s arbitration lost flag"]
pub type TMALF_R = crate::BitReader<TM0ALFR_A>;
#[doc = "Transmit mailbox %s arbitration lost flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0ALFR_A {
    #[doc = "0: No arbitration lost"]
    NoLost = 0,
    #[doc = "1: Transmit mailbox arbitration lost"]
    Lost = 1,
}
impl From<TM0ALFR_A> for bool {
    #[inline(always)]
    fn from(variant: TM0ALFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM0ALFR_A {
        match self.bits {
            false => TM0ALFR_A::NoLost,
            true => TM0ALFR_A::Lost,
        }
    }
    #[doc = "No arbitration lost"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == TM0ALFR_A::NoLost
    }
    #[doc = "Transmit mailbox arbitration lost"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == TM0ALFR_A::Lost
    }
}
#[doc = "Transmit mailbox %s arbitration lost flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0ALFW_AW {
    #[doc = "1: Clear arbitration lost flag"]
    Clear = 1,
}
impl From<TM0ALFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TM0ALFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMALF[0-2]` writer - Transmit mailbox %s arbitration lost flag"]
pub type TMALF_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TM0ALFW_AW>;
impl<'a, REG, const O: u8> TMALF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear arbitration lost flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TM0ALFW_AW::Clear)
    }
}
#[doc = "Field `TMTEF[0-2]` reader - Transmit mailbox %s transmission error flag"]
pub type TMTEF_R = crate::BitReader<TM0TEFR_A>;
#[doc = "Transmit mailbox %s transmission error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0TEFR_A {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Mailbox transmission error"]
    Error = 1,
}
impl From<TM0TEFR_A> for bool {
    #[inline(always)]
    fn from(variant: TM0TEFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMTEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM0TEFR_A {
        match self.bits {
            false => TM0TEFR_A::NoError,
            true => TM0TEFR_A::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TM0TEFR_A::NoError
    }
    #[doc = "Mailbox transmission error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TM0TEFR_A::Error
    }
}
#[doc = "Transmit mailbox %s transmission error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0TEFW_AW {
    #[doc = "1: Clear transmission error flag"]
    Clear = 1,
}
impl From<TM0TEFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TM0TEFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTEF[0-2]` writer - Transmit mailbox %s transmission error flag"]
pub type TMTEF_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TM0TEFW_AW>;
impl<'a, REG, const O: u8> TMTEF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmission error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TM0TEFW_AW::Clear)
    }
}
#[doc = "Field `TMCT[0-2]` reader - Transmit mailbox %s cancel transmission"]
pub type TMCT_R = crate::BitReader<TM0CTR_A>;
#[doc = "Transmit mailbox %s cancel transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0CTR_A {
    #[doc = "0: No effect"]
    NoCancelling = 0,
    #[doc = "1: Mailbox cancel transmit"]
    Cancelling = 1,
}
impl From<TM0CTR_A> for bool {
    #[inline(always)]
    fn from(variant: TM0CTR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM0CTR_A {
        match self.bits {
            false => TM0CTR_A::NoCancelling,
            true => TM0CTR_A::Cancelling,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_cancelling(&self) -> bool {
        *self == TM0CTR_A::NoCancelling
    }
    #[doc = "Mailbox cancel transmit"]
    #[inline(always)]
    pub fn is_cancelling(&self) -> bool {
        *self == TM0CTR_A::Cancelling
    }
}
#[doc = "Transmit mailbox %s cancel transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0CTW_AW {
    #[doc = "1: Mailbox cancel transmit"]
    Abort = 1,
}
impl From<TM0CTW_AW> for bool {
    #[inline(always)]
    fn from(variant: TM0CTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMCT[0-2]` writer - Transmit mailbox %s cancel transmission"]
pub type TMCT_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, TM0CTW_AW>;
impl<'a, REG, const O: u8> TMCT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mailbox cancel transmit"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(TM0CTW_AW::Abort)
    }
}
#[doc = "Field `TMNR` reader - Transmit Mailbox number record"]
pub type TMNR_R = crate::FieldReader;
#[doc = "Field `TMEF[0-2]` reader - Transmit mailbox %s empty flag"]
pub type TMEF_R = crate::BitReader<TM0EF_A>;
#[doc = "Transmit mailbox %s empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0EF_A {
    #[doc = "0: Transmission is pending in the mailbox"]
    NotEmpty = 0,
    #[doc = "1: No transmission is pending in the mailbox"]
    Empty = 1,
}
impl From<TM0EF_A> for bool {
    #[inline(always)]
    fn from(variant: TM0EF_A) -> Self {
        variant as u8 != 0
    }
}
impl TMEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM0EF_A {
        match self.bits {
            false => TM0EF_A::NotEmpty,
            true => TM0EF_A::Empty,
        }
    }
    #[doc = "Transmission is pending in the mailbox"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TM0EF_A::NotEmpty
    }
    #[doc = "No transmission is pending in the mailbox"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TM0EF_A::Empty
    }
}
#[doc = "Field `TMLPF[0-2]` reader - Transmit mailbox %s lowest priority flag"]
pub type TMLPF_R = crate::BitReader<TM0LPF_A>;
#[doc = "Transmit mailbox %s lowest priority flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM0LPF_A {
    #[doc = "0: Mailbox is not given the lowest priority"]
    NotLowest = 0,
    #[doc = "1: Mailbox is given the lowest priority"]
    Lowest = 1,
}
impl From<TM0LPF_A> for bool {
    #[inline(always)]
    fn from(variant: TM0LPF_A) -> Self {
        variant as u8 != 0
    }
}
impl TMLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM0LPF_A {
        match self.bits {
            false => TM0LPF_A::NotLowest,
            true => TM0LPF_A::Lowest,
        }
    }
    #[doc = "Mailbox is not given the lowest priority"]
    #[inline(always)]
    pub fn is_not_lowest(&self) -> bool {
        *self == TM0LPF_A::NotLowest
    }
    #[doc = "Mailbox is given the lowest priority"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == TM0LPF_A::Lowest
    }
}
impl R {
    #[doc = "Transmit mailbox [0-2]
transmission complete flag"]
    #[inline(always)]
    pub unsafe fn tmtcf(&self, n: u8) -> TMTCF_R {
        TMTCF_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit mailbox 0 transmission complete flag"]
    #[inline(always)]
    pub fn tm0tcf(&self) -> TMTCF_R {
        TMTCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit mailbox 1 transmission complete flag"]
    #[inline(always)]
    pub fn tm1tcf(&self) -> TMTCF_R {
        TMTCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit mailbox 2 transmission complete flag"]
    #[inline(always)]
    pub fn tm2tcf(&self) -> TMTCF_R {
        TMTCF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Transmit mailbox [0-2]
transmission success flag"]
    #[inline(always)]
    pub unsafe fn tmtsf(&self, n: u8) -> TMTSF_R {
        TMTSF_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mailbox 0 transmission success flag"]
    #[inline(always)]
    pub fn tm0tsf(&self) -> TMTSF_R {
        TMTSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit mailbox 1 transmission success flag"]
    #[inline(always)]
    pub fn tm1tsf(&self) -> TMTSF_R {
        TMTSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit mailbox 2 transmission success flag"]
    #[inline(always)]
    pub fn tm2tsf(&self) -> TMTSF_R {
        TMTSF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Transmit mailbox [0-2]
arbitration lost flag"]
    #[inline(always)]
    pub unsafe fn tmalf(&self, n: u8) -> TMALF_R {
        TMALF_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit mailbox 0 arbitration lost flag"]
    #[inline(always)]
    pub fn tm0alf(&self) -> TMALF_R {
        TMALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit mailbox 1 arbitration lost flag"]
    #[inline(always)]
    pub fn tm1alf(&self) -> TMALF_R {
        TMALF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit mailbox 2 arbitration lost flag"]
    #[inline(always)]
    pub fn tm2alf(&self) -> TMALF_R {
        TMALF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Transmit mailbox [0-2]
transmission error flag"]
    #[inline(always)]
    pub unsafe fn tmtef(&self, n: u8) -> TMTEF_R {
        TMTEF_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit mailbox 0 transmission error flag"]
    #[inline(always)]
    pub fn tm0tef(&self) -> TMTEF_R {
        TMTEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit mailbox 1 transmission error flag"]
    #[inline(always)]
    pub fn tm1tef(&self) -> TMTEF_R {
        TMTEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit mailbox 2 transmission error flag"]
    #[inline(always)]
    pub fn tm2tef(&self) -> TMTEF_R {
        TMTEF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Transmit mailbox [0-2]
cancel transmission"]
    #[inline(always)]
    pub unsafe fn tmct(&self, n: u8) -> TMCT_R {
        TMCT_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit mailbox 0 cancel transmission"]
    #[inline(always)]
    pub fn tm0ct(&self) -> TMCT_R {
        TMCT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit mailbox 1 cancel transmission"]
    #[inline(always)]
    pub fn tm1ct(&self) -> TMCT_R {
        TMCT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit mailbox 2 cancel transmission"]
    #[inline(always)]
    pub fn tm2ct(&self) -> TMCT_R {
        TMCT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transmit Mailbox number record"]
    #[inline(always)]
    pub fn tmnr(&self) -> TMNR_R {
        TMNR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Transmit mailbox [0-2]
empty flag"]
    #[inline(always)]
    pub unsafe fn tmef(&self, n: u8) -> TMEF_R {
        TMEF_R::new(((self.bits >> (n + 26)) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty flag"]
    #[inline(always)]
    pub fn tm0ef(&self) -> TMEF_R {
        TMEF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty flag"]
    #[inline(always)]
    pub fn tm1ef(&self) -> TMEF_R {
        TMEF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty flag"]
    #[inline(always)]
    pub fn tm2ef(&self) -> TMEF_R {
        TMEF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Transmit mailbox [0-2]
lowest priority flag"]
    #[inline(always)]
    pub unsafe fn tmlpf(&self, n: u8) -> TMLPF_R {
        TMLPF_R::new(((self.bits >> (n + 29)) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 lowest priority flag"]
    #[inline(always)]
    pub fn tm0lpf(&self) -> TMLPF_R {
        TMLPF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 lowest priority flag"]
    #[inline(always)]
    pub fn tm1lpf(&self) -> TMLPF_R {
        TMLPF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit mailbox 2 lowest priority flag"]
    #[inline(always)]
    pub fn tm2lpf(&self) -> TMLPF_R {
        TMLPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSTS")
            .field("tm0lpf", &format_args!("{}", self.tm0lpf().bit()))
            .field("tm1lpf", &format_args!("{}", self.tm1lpf().bit()))
            .field("tm2lpf", &format_args!("{}", self.tm2lpf().bit()))
            .field("tm0ef", &format_args!("{}", self.tm0ef().bit()))
            .field("tm1ef", &format_args!("{}", self.tm1ef().bit()))
            .field("tm2ef", &format_args!("{}", self.tm2ef().bit()))
            .field("tmnr", &format_args!("{}", self.tmnr().bits()))
            .field("tm0ct", &format_args!("{}", self.tm0ct().bit()))
            .field("tm1ct", &format_args!("{}", self.tm1ct().bit()))
            .field("tm2ct", &format_args!("{}", self.tm2ct().bit()))
            .field("tm0tef", &format_args!("{}", self.tm0tef().bit()))
            .field("tm1tef", &format_args!("{}", self.tm1tef().bit()))
            .field("tm2tef", &format_args!("{}", self.tm2tef().bit()))
            .field("tm0alf", &format_args!("{}", self.tm0alf().bit()))
            .field("tm1alf", &format_args!("{}", self.tm1alf().bit()))
            .field("tm2alf", &format_args!("{}", self.tm2alf().bit()))
            .field("tm0tsf", &format_args!("{}", self.tm0tsf().bit()))
            .field("tm1tsf", &format_args!("{}", self.tm1tsf().bit()))
            .field("tm2tsf", &format_args!("{}", self.tm2tsf().bit()))
            .field("tm0tcf", &format_args!("{}", self.tm0tcf().bit()))
            .field("tm1tcf", &format_args!("{}", self.tm1tcf().bit()))
            .field("tm2tcf", &format_args!("{}", self.tm2tcf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Transmit mailbox [0-2]
transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn tmtcf<const O: u8>(&mut self) -> TMTCF_W<TSTS_SPEC, O> {
        TMTCF_W::new(self)
    }
    #[doc = "Bit 0 - Transmit mailbox 0 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tcf(&mut self) -> TMTCF_W<TSTS_SPEC, 0> {
        TMTCF_W::new(self)
    }
    #[doc = "Bit 8 - Transmit mailbox 1 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tcf(&mut self) -> TMTCF_W<TSTS_SPEC, 8> {
        TMTCF_W::new(self)
    }
    #[doc = "Bit 16 - Transmit mailbox 2 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tcf(&mut self) -> TMTCF_W<TSTS_SPEC, 16> {
        TMTCF_W::new(self)
    }
    #[doc = "Transmit mailbox [0-2]
transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn tmtsf<const O: u8>(&mut self) -> TMTSF_W<TSTS_SPEC, O> {
        TMTSF_W::new(self)
    }
    #[doc = "Bit 1 - Transmit mailbox 0 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tsf(&mut self) -> TMTSF_W<TSTS_SPEC, 1> {
        TMTSF_W::new(self)
    }
    #[doc = "Bit 9 - Transmit mailbox 1 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tsf(&mut self) -> TMTSF_W<TSTS_SPEC, 9> {
        TMTSF_W::new(self)
    }
    #[doc = "Bit 17 - Transmit mailbox 2 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tsf(&mut self) -> TMTSF_W<TSTS_SPEC, 17> {
        TMTSF_W::new(self)
    }
    #[doc = "Transmit mailbox [0-2]
arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn tmalf<const O: u8>(&mut self) -> TMALF_W<TSTS_SPEC, O> {
        TMALF_W::new(self)
    }
    #[doc = "Bit 2 - Transmit mailbox 0 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0alf(&mut self) -> TMALF_W<TSTS_SPEC, 2> {
        TMALF_W::new(self)
    }
    #[doc = "Bit 10 - Transmit mailbox 1 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1alf(&mut self) -> TMALF_W<TSTS_SPEC, 10> {
        TMALF_W::new(self)
    }
    #[doc = "Bit 18 - Transmit mailbox 2 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2alf(&mut self) -> TMALF_W<TSTS_SPEC, 18> {
        TMALF_W::new(self)
    }
    #[doc = "Transmit mailbox [0-2]
transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn tmtef<const O: u8>(&mut self) -> TMTEF_W<TSTS_SPEC, O> {
        TMTEF_W::new(self)
    }
    #[doc = "Bit 3 - Transmit mailbox 0 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tef(&mut self) -> TMTEF_W<TSTS_SPEC, 3> {
        TMTEF_W::new(self)
    }
    #[doc = "Bit 11 - Transmit mailbox 1 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tef(&mut self) -> TMTEF_W<TSTS_SPEC, 11> {
        TMTEF_W::new(self)
    }
    #[doc = "Bit 19 - Transmit mailbox 2 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tef(&mut self) -> TMTEF_W<TSTS_SPEC, 19> {
        TMTEF_W::new(self)
    }
    #[doc = "Transmit mailbox [0-2]
cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn tmct<const O: u8>(&mut self) -> TMCT_W<TSTS_SPEC, O> {
        TMCT_W::new(self)
    }
    #[doc = "Bit 7 - Transmit mailbox 0 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm0ct(&mut self) -> TMCT_W<TSTS_SPEC, 7> {
        TMCT_W::new(self)
    }
    #[doc = "Bit 15 - Transmit mailbox 1 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm1ct(&mut self) -> TMCT_W<TSTS_SPEC, 15> {
        TMCT_W::new(self)
    }
    #[doc = "Bit 23 - Transmit mailbox 2 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm2ct(&mut self) -> TMCT_W<TSTS_SPEC, 23> {
        TMCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSTS_SPEC;
impl crate::RegisterSpec for TSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsts::R`](R) reader structure"]
impl crate::Readable for TSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsts::W`](W) writer structure"]
impl crate::Writable for TSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8f;
}
#[doc = "`reset()` method sets TSTS to value 0x1c00_0000"]
impl crate::Resettable for TSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c00_0000;
}
