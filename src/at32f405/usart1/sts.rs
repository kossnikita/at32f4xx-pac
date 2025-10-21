#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
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
#[doc = "Field `PERR` reader - Parity error"]
pub type PERR_R = crate::BitReader<PERR_A>;
impl PERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PERR_A {
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
#[doc = "Field `FERR` reader - Framing error"]
pub type FERR_R = crate::BitReader<FERR_A>;
impl FERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FERR_A {
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
#[doc = "Field `NERR` reader - Noise error"]
pub type NERR_R = crate::BitReader<NERR_A>;
impl NERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NERR_A {
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
    #[doc = "Overflow error is detected"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == ROERR_A::Overflow
    }
}
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
#[doc = "Field `IDLEF` reader - IDLE flag"]
pub type IDLEF_R = crate::BitReader<IDLEF_A>;
impl IDLEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLEF_A {
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
#[doc = "Receive data buffer full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdbfr {
    #[doc = "0: Data is not received"]
    NotFull = 0,
    #[doc = "1: Data is received"]
    Full = 1,
}
impl From<Rdbfr> for bool {
    #[inline(always)]
    fn from(variant: Rdbfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RDBF_R = crate::BitReader<Rdbfr>;
impl RDBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdbfr {
        match self.bits {
            false => Rdbfr::NotFull,
            true => Rdbfr::Full,
        }
    }
    #[doc = "Data is not received"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Rdbfr::NotFull
    }
    #[doc = "Data is received"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rdbfr::Full
    }
}
#[doc = "Receive data buffer full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdbfwWO {
    #[doc = "0: Clear receive buffer full flag"]
    Clear = 0,
}
impl From<RdbfwWO> for bool {
    #[inline(always)]
    fn from(variant: RdbfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBF` writer - Receive data buffer full"]
pub type RDBF_W<'a, REG> = crate::BitWriter0C<'a, REG, RdbfwWO>;
impl<'a, REG> RDBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear receive buffer full flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RdbfwWO::Clear)
    }
}
#[doc = "Transmit data complete\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcr {
    #[doc = "0: Transmission is not completed"]
    NotCompleted = 0,
    #[doc = "1: Transmission is completed"]
    Completed = 1,
}
impl From<Tdcr> for bool {
    #[inline(always)]
    fn from(variant: Tdcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transmit data complete"]
pub type TDC_R = crate::BitReader<Tdcr>;
impl TDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdcr {
        match self.bits {
            false => Tdcr::NotCompleted,
            true => Tdcr::Completed,
        }
    }
    #[doc = "Transmission is not completed"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == Tdcr::NotCompleted
    }
    #[doc = "Transmission is completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == Tdcr::Completed
    }
}
#[doc = "Transmit data complete\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdcwWO {
    #[doc = "0: Clear transmit data complete flag"]
    Clear = 0,
}
impl From<TdcwWO> for bool {
    #[inline(always)]
    fn from(variant: TdcwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` writer - Transmit data complete"]
pub type TDC_W<'a, REG> = crate::BitWriter0C<'a, REG, TdcwWO>;
impl<'a, REG> TDC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear transmit data complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TdcwWO::Clear)
    }
}
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
#[doc = "Break frame flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bffr {
    #[doc = "0: Break frame is not detected"]
    NoBreak = 0,
    #[doc = "1: Break frame is detected"]
    Break = 1,
}
impl From<Bffr> for bool {
    #[inline(always)]
    fn from(variant: Bffr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFF` reader - Break frame flag"]
pub type BFF_R = crate::BitReader<Bffr>;
impl BFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bffr {
        match self.bits {
            false => Bffr::NoBreak,
            true => Bffr::Break,
        }
    }
    #[doc = "Break frame is not detected"]
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == Bffr::NoBreak
    }
    #[doc = "Break frame is detected"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == Bffr::Break
    }
}
#[doc = "Break frame flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BffwWO {
    #[doc = "0: Clear break frame flag"]
    Clear = 0,
}
impl From<BffwWO> for bool {
    #[inline(always)]
    fn from(variant: BffwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFF` writer - Break frame flag"]
pub type BFF_W<'a, REG> = crate::BitWriter0C<'a, REG, BffwWO>;
impl<'a, REG> BFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear break frame flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BffwWO::Clear)
    }
}
#[doc = "CTS change flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctscfr {
    #[doc = "0: No change on the CTS status line"]
    NoChange = 0,
    #[doc = "1: A change occurs on the CTS status line"]
    Change = 1,
}
impl From<Ctscfr> for bool {
    #[inline(always)]
    fn from(variant: Ctscfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCF` reader - CTS change flag"]
pub type CTSCF_R = crate::BitReader<Ctscfr>;
impl CTSCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctscfr {
        match self.bits {
            false => Ctscfr::NoChange,
            true => Ctscfr::Change,
        }
    }
    #[doc = "No change on the CTS status line"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == Ctscfr::NoChange
    }
    #[doc = "A change occurs on the CTS status line"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == Ctscfr::Change
    }
}
#[doc = "CTS change flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtscfwWO {
    #[doc = "0: Clear CTS change flag"]
    Clear = 0,
}
impl From<CtscfwWO> for bool {
    #[inline(always)]
    fn from(variant: CtscfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCF` writer - CTS change flag"]
pub type CTSCF_W<'a, REG> = crate::BitWriter0C<'a, REG, CtscfwWO>;
impl<'a, REG> CTSCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CTS change flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CtscfwWO::Clear)
    }
}
#[doc = "Field `RTODF` reader - Reiceiver time out detection flag"]
pub type RTODF_R = crate::BitReader;
#[doc = "Field `RTODF` writer - Reiceiver time out detection flag"]
pub type RTODF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDF` reader - Character match detection flag"]
pub type CMDF_R = crate::BitReader;
#[doc = "Field `CMDF` writer - Character match detection flag"]
pub type CMDF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("cmdf", &self.cmdf())
            .field("rtodf", &self.rtodf())
            .field("ctscf", &self.ctscf())
            .field("bff", &self.bff())
            .field("tdbe", &self.tdbe())
            .field("tdc", &self.tdc())
            .field("rdbf", &self.rdbf())
            .field("idlef", &self.idlef())
            .field("roerr", &self.roerr())
            .field("nerr", &self.nerr())
            .field("ferr", &self.ferr())
            .field("perr", &self.perr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Receive data buffer full"]
    #[inline(always)]
    pub fn rdbf(&mut self) -> RDBF_W<'_, STS_SPEC> {
        RDBF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit data complete"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W<'_, STS_SPEC> {
        TDC_W::new(self, 6)
    }
    #[doc = "Bit 8 - Break frame flag"]
    #[inline(always)]
    pub fn bff(&mut self) -> BFF_W<'_, STS_SPEC> {
        BFF_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<'_, STS_SPEC> {
        CTSCF_W::new(self, 9)
    }
    #[doc = "Bit 11 - Reiceiver time out detection flag"]
    #[inline(always)]
    pub fn rtodf(&mut self) -> RTODF_W<'_, STS_SPEC> {
        RTODF_W::new(self, 11)
    }
    #[doc = "Bit 17 - Character match detection flag"]
    #[inline(always)]
    pub fn cmdf(&mut self) -> CMDF_W<'_, STS_SPEC> {
        CMDF_W::new(self, 17)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0360;
}
#[doc = "`reset()` method sets STS to value 0xc0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0xc0;
}
