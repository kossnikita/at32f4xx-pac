#[doc = "Register `STS1` reader"]
pub type R = crate::R<STS1_SPEC>;
#[doc = "Register `STS1` writer"]
pub type W = crate::W<STS1_SPEC>;
#[doc = "Start bit (Master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTF_A {
    #[doc = "0: No Start condition"]
    NoStart = 0,
    #[doc = "1: Start condition generated"]
    Start = 1,
}
impl From<STARTF_A> for bool {
    #[inline(always)]
    fn from(variant: STARTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTF` reader - Start bit (Master mode)"]
pub type STARTF_R = crate::BitReader<STARTF_A>;
impl STARTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STARTF_A {
        match self.bits {
            false => STARTF_A::NoStart,
            true => STARTF_A::Start,
        }
    }
    #[doc = "No Start condition"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == STARTF_A::NoStart
    }
    #[doc = "Start condition generated"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STARTF_A::Start
    }
}
#[doc = "Address sent (master mode)/matched (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR7F_A {
    #[doc = "0: Adress mismatched or not received"]
    NotMatch = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    Match = 1,
}
impl From<ADDR7F_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR7F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR7F` reader - Address sent (master mode)/matched (slave mode)"]
pub type ADDR7F_R = crate::BitReader<ADDR7F_A>;
impl ADDR7F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR7F_A {
        match self.bits {
            false => ADDR7F_A::NotMatch,
            true => ADDR7F_A::Match,
        }
    }
    #[doc = "Adress mismatched or not received"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR7F_A::NotMatch
    }
    #[doc = "Received slave address matched with one of the enabled slave addresses"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDR7F_A::Match
    }
}
#[doc = "Transmit data complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDC_A {
    #[doc = "0: Data byte transfer not done"]
    NotFinished = 0,
    #[doc = "1: Data byte transfer successful"]
    Finished = 1,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transmit data complete"]
pub type TDC_R = crate::BitReader<TDC_A>;
impl TDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDC_A {
        match self.bits {
            false => TDC_A::NotFinished,
            true => TDC_A::Finished,
        }
    }
    #[doc = "Data byte transfer not done"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == TDC_A::NotFinished
    }
    #[doc = "Data byte transfer successful"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == TDC_A::Finished
    }
}
#[doc = "address header match (Master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRHF_A {
    #[doc = "0: Master 9~8 bit address head mismatch"]
    NotMatch = 0,
    #[doc = "1: Master 9~8 bit address head match"]
    Match = 1,
}
impl From<ADDRHF_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRHF` reader - address header match (Master mode)"]
pub type ADDRHF_R = crate::BitReader<ADDRHF_A>;
impl ADDRHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRHF_A {
        match self.bits {
            false => ADDRHF_A::NotMatch,
            true => ADDRHF_A::Match,
        }
    }
    #[doc = "Master 9~8 bit address head mismatch"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDRHF_A::NotMatch
    }
    #[doc = "Master 9~8 bit address head match"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDRHF_A::Match
    }
}
#[doc = "Stop detection (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF_A {
    #[doc = "0: No Stop condition detected"]
    NoStop = 0,
    #[doc = "1: Stop condition detected"]
    Stop = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type STOPF_R = crate::BitReader<STOPF_A>;
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NoStop,
            true => STOPF_A::Stop,
        }
    }
    #[doc = "No Stop condition detected"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NoStop
    }
    #[doc = "Stop condition detected"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::Stop
    }
}
#[doc = "Receive data buffer full (receivers)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBF_A {
    #[doc = "0: Data register empty"]
    Empty = 0,
    #[doc = "1: Data register not empty"]
    NotEmpty = 1,
}
impl From<RDBF_A> for bool {
    #[inline(always)]
    fn from(variant: RDBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBF` reader - Receive data buffer full (receivers)"]
pub type RDBF_R = crate::BitReader<RDBF_A>;
impl RDBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDBF_A {
        match self.bits {
            false => RDBF_A::Empty,
            true => RDBF_A::NotEmpty,
        }
    }
    #[doc = "Data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RDBF_A::Empty
    }
    #[doc = "Data register not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RDBF_A::NotEmpty
    }
}
#[doc = "Transmit data buffer empty (transmitters)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBE_A {
    #[doc = "0: Data register not empty"]
    NotEmpty = 0,
    #[doc = "1: Data register empty"]
    Empty = 1,
}
impl From<TDBE_A> for bool {
    #[inline(always)]
    fn from(variant: TDBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBE` reader - Transmit data buffer empty (transmitters)"]
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
    #[doc = "Data register not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TDBE_A::NotEmpty
    }
    #[doc = "Data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TDBE_A::Empty
    }
}
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buserrr {
    #[doc = "0: No misplaced Start or Stop condition"]
    NoError = 0,
    #[doc = "1: Misplaced Start or Stop condition"]
    Error = 1,
}
impl From<Buserrr> for bool {
    #[inline(always)]
    fn from(variant: Buserrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERR` reader - Bus error"]
pub type BUSERR_R = crate::BitReader<Buserrr>;
impl BUSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buserrr {
        match self.bits {
            false => Buserrr::NoError,
            true => Buserrr::Error,
        }
    }
    #[doc = "No misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Buserrr::NoError
    }
    #[doc = "Misplaced Start or Stop condition"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Buserrr::Error
    }
}
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuserrwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<BuserrwWO> for bool {
    #[inline(always)]
    fn from(variant: BuserrwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSERR` writer - Bus error"]
pub type BUSERR_W<'a, REG> = crate::BitWriter0C<'a, REG, BuserrwWO>;
impl<'a, REG> BUSERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BuserrwWO::Clear)
    }
}
#[doc = "Arbitration lost (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arlostr {
    #[doc = "0: No Arbitration Lost detected"]
    NoLost = 0,
    #[doc = "1: Arbitration Lost detected"]
    Lost = 1,
}
impl From<Arlostr> for bool {
    #[inline(always)]
    fn from(variant: Arlostr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLOST` reader - Arbitration lost (master mode)"]
pub type ARLOST_R = crate::BitReader<Arlostr>;
impl ARLOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arlostr {
        match self.bits {
            false => Arlostr::NoLost,
            true => Arlostr::Lost,
        }
    }
    #[doc = "No Arbitration Lost detected"]
    #[inline(always)]
    pub fn is_no_lost(&self) -> bool {
        *self == Arlostr::NoLost
    }
    #[doc = "Arbitration Lost detected"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == Arlostr::Lost
    }
}
#[doc = "Arbitration lost (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArlostwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<ArlostwWO> for bool {
    #[inline(always)]
    fn from(variant: ArlostwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLOST` writer - Arbitration lost (master mode)"]
pub type ARLOST_W<'a, REG> = crate::BitWriter0C<'a, REG, ArlostwWO>;
impl<'a, REG> ARLOST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ArlostwWO::Clear)
    }
}
#[doc = "Acknowledge failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackfailr {
    #[doc = "0: No acknowledge failure"]
    NoFailure = 0,
    #[doc = "1: Acknowledge failure"]
    Failure = 1,
}
impl From<Ackfailr> for bool {
    #[inline(always)]
    fn from(variant: Ackfailr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKFAIL` reader - Acknowledge failure"]
pub type ACKFAIL_R = crate::BitReader<Ackfailr>;
impl ACKFAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackfailr {
        match self.bits {
            false => Ackfailr::NoFailure,
            true => Ackfailr::Failure,
        }
    }
    #[doc = "No acknowledge failure"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == Ackfailr::NoFailure
    }
    #[doc = "Acknowledge failure"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == Ackfailr::Failure
    }
}
#[doc = "Acknowledge failure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckfailwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<AckfailwWO> for bool {
    #[inline(always)]
    fn from(variant: AckfailwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKFAIL` writer - Acknowledge failure"]
pub type ACKFAIL_W<'a, REG> = crate::BitWriter0C<'a, REG, AckfailwWO>;
impl<'a, REG> ACKFAIL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AckfailwWO::Clear)
    }
}
#[doc = "Overflow or underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oufr {
    #[doc = "0: No overrun/underrun occured"]
    NoOverrun = 0,
    #[doc = "1: Overrun/underrun occured"]
    Overrun = 1,
}
impl From<Oufr> for bool {
    #[inline(always)]
    fn from(variant: Oufr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUF` reader - Overflow or underflow"]
pub type OUF_R = crate::BitReader<Oufr>;
impl OUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oufr {
        match self.bits {
            false => Oufr::NoOverrun,
            true => Oufr::Overrun,
        }
    }
    #[doc = "No overrun/underrun occured"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Oufr::NoOverrun
    }
    #[doc = "Overrun/underrun occured"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Oufr::Overrun
    }
}
#[doc = "Overflow or underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OufwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<OufwWO> for bool {
    #[inline(always)]
    fn from(variant: OufwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUF` writer - Overflow or underflow"]
pub type OUF_W<'a, REG> = crate::BitWriter0C<'a, REG, OufwWO>;
impl<'a, REG> OUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OufwWO::Clear)
    }
}
#[doc = "PEC receive error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecerrr {
    #[doc = "0: no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    NoError = 0,
    #[doc = "1: PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    Error = 1,
}
impl From<Pecerrr> for bool {
    #[inline(always)]
    fn from(variant: Pecerrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` reader - PEC receive error"]
pub type PECERR_R = crate::BitReader<Pecerrr>;
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecerrr {
        match self.bits {
            false => Pecerrr::NoError,
            true => Pecerrr::Error,
        }
    }
    #[doc = "no PEC error: receiver returns ACK after PEC reception (if ACK=1)"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Pecerrr::NoError
    }
    #[doc = "PEC error: receiver returns NACK after PEC reception (whatever ACK)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Pecerrr::Error
    }
}
#[doc = "PEC receive error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PecerrwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<PecerrwWO> for bool {
    #[inline(always)]
    fn from(variant: PecerrwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECERR` writer - PEC receive error"]
pub type PECERR_W<'a, REG> = crate::BitWriter0C<'a, REG, PecerrwWO>;
impl<'a, REG> PECERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PecerrwWO::Clear)
    }
}
#[doc = "Timeout error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmoutr {
    #[doc = "0: No Timeout error"]
    NoTimeout = 0,
    #[doc = "1: SCL remained LOW for 25 ms"]
    Timeout = 1,
}
impl From<Tmoutr> for bool {
    #[inline(always)]
    fn from(variant: Tmoutr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOUT` reader - Timeout error"]
pub type TMOUT_R = crate::BitReader<Tmoutr>;
impl TMOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmoutr {
        match self.bits {
            false => Tmoutr::NoTimeout,
            true => Tmoutr::Timeout,
        }
    }
    #[doc = "No Timeout error"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == Tmoutr::NoTimeout
    }
    #[doc = "SCL remained LOW for 25 ms"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == Tmoutr::Timeout
    }
}
#[doc = "Timeout error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TmoutwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<TmoutwWO> for bool {
    #[inline(always)]
    fn from(variant: TmoutwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOUT` writer - Timeout error"]
pub type TMOUT_W<'a, REG> = crate::BitWriter0C<'a, REG, TmoutwWO>;
impl<'a, REG> TMOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TmoutwWO::Clear)
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alertfr {
    #[doc = "0: No SMBALERT occured"]
    NoAlert = 0,
    #[doc = "1: SMBALERT occurred"]
    Alert = 1,
}
impl From<Alertfr> for bool {
    #[inline(always)]
    fn from(variant: Alertfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTF` reader - SMBus alert"]
pub type ALERTF_R = crate::BitReader<Alertfr>;
impl ALERTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alertfr {
        match self.bits {
            false => Alertfr::NoAlert,
            true => Alertfr::Alert,
        }
    }
    #[doc = "No SMBALERT occured"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == Alertfr::NoAlert
    }
    #[doc = "SMBALERT occurred"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == Alertfr::Alert
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertfwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<AlertfwWO> for bool {
    #[inline(always)]
    fn from(variant: AlertfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTF` writer - SMBus alert"]
pub type ALERTF_W<'a, REG> = crate::BitWriter0C<'a, REG, AlertfwWO>;
impl<'a, REG> ALERTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AlertfwWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr7f(&self) -> ADDR7F_R {
        ADDR7F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit data complete"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - address header match (Master mode)"]
    #[inline(always)]
    pub fn addrhf(&self) -> ADDRHF_R {
        ADDRHF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive data buffer full (receivers)"]
    #[inline(always)]
    pub fn rdbf(&self) -> RDBF_R {
        RDBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty (transmitters)"]
    #[inline(always)]
    pub fn tdbe(&self) -> TDBE_R {
        TDBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlost(&self) -> ARLOST_R {
        ARLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overflow or underflow"]
    #[inline(always)]
    pub fn ouf(&self) -> OUF_R {
        OUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC receive error"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout error"]
    #[inline(always)]
    pub fn tmout(&self) -> TMOUT_R {
        TMOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn alertf(&self) -> ALERTF_R {
        ALERTF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS1")
            .field("alertf", &self.alertf())
            .field("tmout", &self.tmout())
            .field("pecerr", &self.pecerr())
            .field("ouf", &self.ouf())
            .field("ackfail", &self.ackfail())
            .field("arlost", &self.arlost())
            .field("buserr", &self.buserr())
            .field("tdbe", &self.tdbe())
            .field("rdbf", &self.rdbf())
            .field("stopf", &self.stopf())
            .field("addrhf", &self.addrhf())
            .field("tdc", &self.tdc())
            .field("addr7f", &self.addr7f())
            .field("startf", &self.startf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W<'_, STS1_SPEC> {
        BUSERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlost(&mut self) -> ARLOST_W<'_, STS1_SPEC> {
        ARLOST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn ackfail(&mut self) -> ACKFAIL_W<'_, STS1_SPEC> {
        ACKFAIL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Overflow or underflow"]
    #[inline(always)]
    pub fn ouf(&mut self) -> OUF_W<'_, STS1_SPEC> {
        OUF_W::new(self, 11)
    }
    #[doc = "Bit 12 - PEC receive error"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W<'_, STS1_SPEC> {
        PECERR_W::new(self, 12)
    }
    #[doc = "Bit 14 - Timeout error"]
    #[inline(always)]
    pub fn tmout(&mut self) -> TMOUT_W<'_, STS1_SPEC> {
        TMOUT_W::new(self, 14)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn alertf(&mut self) -> ALERTF_W<'_, STS1_SPEC> {
        ALERTF_W::new(self, 15)
    }
}
#[doc = "Status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS1_SPEC;
impl crate::RegisterSpec for STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts1::R`](R) reader structure"]
impl crate::Readable for STS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts1::W`](W) writer structure"]
impl crate::Writable for STS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0xdf00;
}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for STS1_SPEC {}
