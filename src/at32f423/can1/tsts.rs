#[doc = "Register `TSTS` reader"]
pub type R = crate::R<TSTS_SPEC>;
#[doc = "Register `TSTS` writer"]
pub type W = crate::W<TSTS_SPEC>;
#[doc = "Transmit mailbox %s transmission complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0tcfr {
    #[doc = "0: Transmission in progress"]
    InProgress = 0,
    #[doc = "1: Transmission completed"]
    Completed = 1,
}
impl From<Tm0tcfr> for bool {
    #[inline(always)]
    fn from(variant: Tm0tcfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTCF(0-2)` reader - Transmit mailbox %s transmission complete flag"]
pub type TMTCF_R = crate::BitReader<Tm0tcfr>;
impl TMTCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tm0tcfr {
        match self.bits {
            false => Tm0tcfr::InProgress,
            true => Tm0tcfr::Completed,
        }
    }
    #[doc = "Transmission in progress"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Tm0tcfr::InProgress
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == Tm0tcfr::Completed
    }
}
#[doc = "Transmit mailbox %s transmission complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0tcfwWO {
    #[doc = "1: Clear transmission completed flag"]
    Clear = 1,
}
impl From<Tm0tcfwWO> for bool {
    #[inline(always)]
    fn from(variant: Tm0tcfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTCF(0-2)` writer - Transmit mailbox %s transmission complete flag"]
pub type TMTCF_W<'a, REG> = crate::BitWriter1C<'a, REG, Tm0tcfwWO>;
impl<'a, REG> TMTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmission completed flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tm0tcfwWO::Clear)
    }
}
#[doc = "Transmit mailbox %s transmission success flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0tsfr {
    #[doc = "0: Transmission in progress"]
    Failed = 0,
    #[doc = "1: Transmission was successful"]
    Successful = 1,
}
impl From<Tm0tsfr> for bool {
    #[inline(always)]
    fn from(variant: Tm0tsfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTSF(0-2)` reader - Transmit mailbox %s transmission success flag"]
pub type TMTSF_R = crate::BitReader<Tm0tsfr>;
impl TMTSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tm0tsfr {
        match self.bits {
            false => Tm0tsfr::Failed,
            true => Tm0tsfr::Successful,
        }
    }
    #[doc = "Transmission in progress"]
    #[inline(always)]
    pub fn is_failed(&self) -> bool {
        *self == Tm0tsfr::Failed
    }
    #[doc = "Transmission was successful"]
    #[inline(always)]
    pub fn is_successful(&self) -> bool {
        *self == Tm0tsfr::Successful
    }
}
#[doc = "Transmit mailbox %s transmission success flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0tsfwWO {
    #[doc = "1: Clear transmission success flag"]
    Clear = 1,
}
impl From<Tm0tsfwWO> for bool {
    #[inline(always)]
    fn from(variant: Tm0tsfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTSF(0-2)` writer - Transmit mailbox %s transmission success flag"]
pub type TMTSF_W<'a, REG> = crate::BitWriter1C<'a, REG, Tm0tsfwWO>;
impl<'a, REG> TMTSF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmission success flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tm0tsfwWO::Clear)
    }
}
#[doc = "Transmit mailbox %s arbitration lost flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0alfr {
    #[doc = "0: No arbitration lost"]
    NoLost = 0,
    #[doc = "1: Transmit mailbox arbitration lost"]
    Lost = 1,
}
impl From<Tm0alfr> for bool {
    #[inline(always)]
    fn from(variant: Tm0alfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMALF(0-2)` reader - Transmit mailbox %s arbitration lost flag"]
pub type TMALF_R = crate::BitReader<Tm0alfr>;
impl TMALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tm0alfr {
        match self.bits {
            false => Tm0alfr::NoLost,
            true => Tm0alfr::Lost,
        }
    }
    #[doc = "No arbitration lost"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == Tm0alfr::NoLost
    }
    #[doc = "Transmit mailbox arbitration lost"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == Tm0alfr::Lost
    }
}
#[doc = "Transmit mailbox %s arbitration lost flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0alfwWO {
    #[doc = "1: Clear arbitration lost flag"]
    Clear = 1,
}
impl From<Tm0alfwWO> for bool {
    #[inline(always)]
    fn from(variant: Tm0alfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMALF(0-2)` writer - Transmit mailbox %s arbitration lost flag"]
pub type TMALF_W<'a, REG> = crate::BitWriter1C<'a, REG, Tm0alfwWO>;
impl<'a, REG> TMALF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear arbitration lost flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tm0alfwWO::Clear)
    }
}
#[doc = "Transmit mailbox %s transmission error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0tefr {
    #[doc = "0: No error"]
    NoError = 0,
    #[doc = "1: Mailbox transmission error"]
    Error = 1,
}
impl From<Tm0tefr> for bool {
    #[inline(always)]
    fn from(variant: Tm0tefr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTEF(0-2)` reader - Transmit mailbox %s transmission error flag"]
pub type TMTEF_R = crate::BitReader<Tm0tefr>;
impl TMTEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tm0tefr {
        match self.bits {
            false => Tm0tefr::NoError,
            true => Tm0tefr::Error,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Tm0tefr::NoError
    }
    #[doc = "Mailbox transmission error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Tm0tefr::Error
    }
}
#[doc = "Transmit mailbox %s transmission error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0tefwWO {
    #[doc = "1: Clear transmission error flag"]
    Clear = 1,
}
impl From<Tm0tefwWO> for bool {
    #[inline(always)]
    fn from(variant: Tm0tefwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMTEF(0-2)` writer - Transmit mailbox %s transmission error flag"]
pub type TMTEF_W<'a, REG> = crate::BitWriter1C<'a, REG, Tm0tefwWO>;
impl<'a, REG> TMTEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmission error flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Tm0tefwWO::Clear)
    }
}
#[doc = "Transmit mailbox %s cancel transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0ctr {
    #[doc = "0: No effect"]
    NoCancelling = 0,
    #[doc = "1: Mailbox cancel transmit"]
    Cancelling = 1,
}
impl From<Tm0ctr> for bool {
    #[inline(always)]
    fn from(variant: Tm0ctr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMCT(0-2)` reader - Transmit mailbox %s cancel transmission"]
pub type TMCT_R = crate::BitReader<Tm0ctr>;
impl TMCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tm0ctr {
        match self.bits {
            false => Tm0ctr::NoCancelling,
            true => Tm0ctr::Cancelling,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_cancelling(&self) -> bool {
        *self == Tm0ctr::NoCancelling
    }
    #[doc = "Mailbox cancel transmit"]
    #[inline(always)]
    pub fn is_cancelling(&self) -> bool {
        *self == Tm0ctr::Cancelling
    }
}
#[doc = "Transmit mailbox %s cancel transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm0ctwWO {
    #[doc = "1: Mailbox cancel transmit"]
    Abort = 1,
}
impl From<Tm0ctwWO> for bool {
    #[inline(always)]
    fn from(variant: Tm0ctwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMCT(0-2)` writer - Transmit mailbox %s cancel transmission"]
pub type TMCT_W<'a, REG> = crate::BitWriter1S<'a, REG, Tm0ctwWO>;
impl<'a, REG> TMCT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mailbox cancel transmit"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(Tm0ctwWO::Abort)
    }
}
#[doc = "Field `TMNR` reader - Transmit Mailbox number record"]
pub type TMNR_R = crate::FieldReader;
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
#[doc = "Field `TMEF(0-2)` reader - Transmit mailbox %s empty flag"]
pub type TMEF_R = crate::BitReader<TM0EF_A>;
impl TMEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TM0EF_A {
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
#[doc = "Field `TMLPF(0-2)` reader - Transmit mailbox %s lowest priority flag"]
pub type TMLPF_R = crate::BitReader<TM0LPF_A>;
impl TMLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TM0LPF_A {
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
    #[doc = "Transmit mailbox (0-2) transmission complete flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0TCF` field.</div>"]
    #[inline(always)]
    pub fn tmtcf(&self, n: u8) -> TMTCF_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMTCF_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox (0-2) transmission complete flag"]
    #[inline(always)]
    pub fn tmtcf_iter(&self) -> impl Iterator<Item = TMTCF_R> + '_ {
        (0..3).map(move |n| TMTCF_R::new(((self.bits >> (n * 8)) & 1) != 0))
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
    #[doc = "Transmit mailbox (0-2) transmission success flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0TSF` field.</div>"]
    #[inline(always)]
    pub fn tmtsf(&self, n: u8) -> TMTSF_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMTSF_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox (0-2) transmission success flag"]
    #[inline(always)]
    pub fn tmtsf_iter(&self) -> impl Iterator<Item = TMTSF_R> + '_ {
        (0..3).map(move |n| TMTSF_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0))
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
    #[doc = "Transmit mailbox (0-2) arbitration lost flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0ALF` field.</div>"]
    #[inline(always)]
    pub fn tmalf(&self, n: u8) -> TMALF_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMALF_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox (0-2) arbitration lost flag"]
    #[inline(always)]
    pub fn tmalf_iter(&self) -> impl Iterator<Item = TMALF_R> + '_ {
        (0..3).map(move |n| TMALF_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
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
    #[doc = "Transmit mailbox (0-2) transmission error flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0TEF` field.</div>"]
    #[inline(always)]
    pub fn tmtef(&self, n: u8) -> TMTEF_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMTEF_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox (0-2) transmission error flag"]
    #[inline(always)]
    pub fn tmtef_iter(&self) -> impl Iterator<Item = TMTEF_R> + '_ {
        (0..3).map(move |n| TMTEF_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
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
    #[doc = "Transmit mailbox (0-2) cancel transmission"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0CT` field.</div>"]
    #[inline(always)]
    pub fn tmct(&self, n: u8) -> TMCT_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMCT_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox (0-2) cancel transmission"]
    #[inline(always)]
    pub fn tmct_iter(&self) -> impl Iterator<Item = TMCT_R> + '_ {
        (0..3).map(move |n| TMCT_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
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
    #[doc = "Transmit mailbox (0-2) empty flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0EF` field.</div>"]
    #[inline(always)]
    pub fn tmef(&self, n: u8) -> TMEF_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMEF_R::new(((self.bits >> (n + 26)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox (0-2) empty flag"]
    #[inline(always)]
    pub fn tmef_iter(&self) -> impl Iterator<Item = TMEF_R> + '_ {
        (0..3).map(move |n| TMEF_R::new(((self.bits >> (n + 26)) & 1) != 0))
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
    #[doc = "Transmit mailbox (0-2) lowest priority flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0LPF` field.</div>"]
    #[inline(always)]
    pub fn tmlpf(&self, n: u8) -> TMLPF_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMLPF_R::new(((self.bits >> (n + 29)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox (0-2) lowest priority flag"]
    #[inline(always)]
    pub fn tmlpf_iter(&self) -> impl Iterator<Item = TMLPF_R> + '_ {
        (0..3).map(move |n| TMLPF_R::new(((self.bits >> (n + 29)) & 1) != 0))
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
            .field("tm0lpf", &self.tm0lpf())
            .field("tm1lpf", &self.tm1lpf())
            .field("tm2lpf", &self.tm2lpf())
            .field("tm0ef", &self.tm0ef())
            .field("tm1ef", &self.tm1ef())
            .field("tm2ef", &self.tm2ef())
            .field("tmnr", &self.tmnr())
            .field("tm0ct", &self.tm0ct())
            .field("tm1ct", &self.tm1ct())
            .field("tm2ct", &self.tm2ct())
            .field("tm0tef", &self.tm0tef())
            .field("tm1tef", &self.tm1tef())
            .field("tm2tef", &self.tm2tef())
            .field("tm0alf", &self.tm0alf())
            .field("tm1alf", &self.tm1alf())
            .field("tm2alf", &self.tm2alf())
            .field("tm0tsf", &self.tm0tsf())
            .field("tm1tsf", &self.tm1tsf())
            .field("tm2tsf", &self.tm2tsf())
            .field("tm0tcf", &self.tm0tcf())
            .field("tm1tcf", &self.tm1tcf())
            .field("tm2tcf", &self.tm2tcf())
            .finish()
    }
}
impl W {
    #[doc = "Transmit mailbox (0-2) transmission complete flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0TCF` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn tmtcf(&mut self, n: u8) -> TMTCF_W<TSTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMTCF_W::new(self, n * 8)
    }
    #[doc = "Bit 0 - Transmit mailbox 0 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tcf(&mut self) -> TMTCF_W<TSTS_SPEC> {
        TMTCF_W::new(self, 0)
    }
    #[doc = "Bit 8 - Transmit mailbox 1 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tcf(&mut self) -> TMTCF_W<TSTS_SPEC> {
        TMTCF_W::new(self, 8)
    }
    #[doc = "Bit 16 - Transmit mailbox 2 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tcf(&mut self) -> TMTCF_W<TSTS_SPEC> {
        TMTCF_W::new(self, 16)
    }
    #[doc = "Transmit mailbox (0-2) transmission success flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0TSF` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn tmtsf(&mut self, n: u8) -> TMTSF_W<TSTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMTSF_W::new(self, n * 8 + 1)
    }
    #[doc = "Bit 1 - Transmit mailbox 0 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tsf(&mut self) -> TMTSF_W<TSTS_SPEC> {
        TMTSF_W::new(self, 1)
    }
    #[doc = "Bit 9 - Transmit mailbox 1 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tsf(&mut self) -> TMTSF_W<TSTS_SPEC> {
        TMTSF_W::new(self, 9)
    }
    #[doc = "Bit 17 - Transmit mailbox 2 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tsf(&mut self) -> TMTSF_W<TSTS_SPEC> {
        TMTSF_W::new(self, 17)
    }
    #[doc = "Transmit mailbox (0-2) arbitration lost flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0ALF` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn tmalf(&mut self, n: u8) -> TMALF_W<TSTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMALF_W::new(self, n * 8 + 2)
    }
    #[doc = "Bit 2 - Transmit mailbox 0 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0alf(&mut self) -> TMALF_W<TSTS_SPEC> {
        TMALF_W::new(self, 2)
    }
    #[doc = "Bit 10 - Transmit mailbox 1 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1alf(&mut self) -> TMALF_W<TSTS_SPEC> {
        TMALF_W::new(self, 10)
    }
    #[doc = "Bit 18 - Transmit mailbox 2 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2alf(&mut self) -> TMALF_W<TSTS_SPEC> {
        TMALF_W::new(self, 18)
    }
    #[doc = "Transmit mailbox (0-2) transmission error flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0TEF` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn tmtef(&mut self, n: u8) -> TMTEF_W<TSTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMTEF_W::new(self, n * 8 + 3)
    }
    #[doc = "Bit 3 - Transmit mailbox 0 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tef(&mut self) -> TMTEF_W<TSTS_SPEC> {
        TMTEF_W::new(self, 3)
    }
    #[doc = "Bit 11 - Transmit mailbox 1 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tef(&mut self) -> TMTEF_W<TSTS_SPEC> {
        TMTEF_W::new(self, 11)
    }
    #[doc = "Bit 19 - Transmit mailbox 2 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tef(&mut self) -> TMTEF_W<TSTS_SPEC> {
        TMTEF_W::new(self, 19)
    }
    #[doc = "Transmit mailbox (0-2) cancel transmission"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TM0CT` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn tmct(&mut self, n: u8) -> TMCT_W<TSTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TMCT_W::new(self, n * 8 + 7)
    }
    #[doc = "Bit 7 - Transmit mailbox 0 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm0ct(&mut self) -> TMCT_W<TSTS_SPEC> {
        TMCT_W::new(self, 7)
    }
    #[doc = "Bit 15 - Transmit mailbox 1 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm1ct(&mut self) -> TMCT_W<TSTS_SPEC> {
        TMCT_W::new(self, 15)
    }
    #[doc = "Bit 23 - Transmit mailbox 2 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm2ct(&mut self) -> TMCT_W<TSTS_SPEC> {
        TMCT_W::new(self, 23)
    }
}
#[doc = "Transmit status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSTS_SPEC;
impl crate::RegisterSpec for TSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsts::R`](R) reader structure"]
impl crate::Readable for TSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsts::W`](W) writer structure"]
impl crate::Writable for TSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8f;
}
#[doc = "`reset()` method sets TSTS to value 0x1c00_0000"]
impl crate::Resettable for TSTS_SPEC {
    const RESET_VALUE: u32 = 0x1c00_0000;
}
