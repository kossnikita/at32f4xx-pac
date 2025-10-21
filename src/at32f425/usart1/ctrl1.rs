#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Send break frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbfr {
    #[doc = "0: No break frame is transmitted"]
    NoTransmission = 0,
    #[doc = "1: Break frame is transmitted"]
    Transmitting = 1,
}
impl From<Sbfr> for bool {
    #[inline(always)]
    fn from(variant: Sbfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` reader - Send break frame"]
pub type SBF_R = crate::BitReader<Sbfr>;
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbfr {
        match self.bits {
            false => Sbfr::NoTransmission,
            true => Sbfr::Transmitting,
        }
    }
    #[doc = "No break frame is transmitted"]
    #[inline(always)]
    pub fn is_no_transmission(&self) -> bool {
        *self == Sbfr::NoTransmission
    }
    #[doc = "Break frame is transmitted"]
    #[inline(always)]
    pub fn is_transmitting(&self) -> bool {
        *self == Sbfr::Transmitting
    }
}
#[doc = "Send break frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SbfwWO {
    #[doc = "0: Clear send break frame"]
    Clear = 0,
    #[doc = "1: Send break frame"]
    Send = 1,
}
impl From<SbfwWO> for bool {
    #[inline(always)]
    fn from(variant: SbfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` writer - Send break frame"]
pub type SBF_W<'a, REG> = crate::BitWriter<'a, REG, SbfwWO>;
impl<'a, REG> SBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear send break frame"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SbfwWO::Clear)
    }
    #[doc = "Send break frame"]
    #[inline(always)]
    pub fn send(self) -> &'a mut crate::W<REG> {
        self.variant(SbfwWO::Send)
    }
}
#[doc = "Receiver mute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RM_A {
    #[doc = "0: Receiver is in active mode"]
    Active = 0,
    #[doc = "1: Receiver is in mute mode"]
    Mute = 1,
}
impl From<RM_A> for bool {
    #[inline(always)]
    fn from(variant: RM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RM` reader - Receiver mute"]
pub type RM_R = crate::BitReader<RM_A>;
impl RM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RM_A {
        match self.bits {
            false => RM_A::Active,
            true => RM_A::Mute,
        }
    }
    #[doc = "Receiver is in active mode"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RM_A::Active
    }
    #[doc = "Receiver is in mute mode"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == RM_A::Mute
    }
}
#[doc = "Field `RM` writer - Receiver mute"]
pub type RM_W<'a, REG> = crate::BitWriter<'a, REG, RM_A>;
impl<'a, REG> RM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver is in active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RM_A::Active)
    }
    #[doc = "Receiver is in mute mode"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(RM_A::Mute)
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Renr {
    #[doc = "0: Receiver is disabled"]
    Disabled = 0,
    #[doc = "1: Receiver is enabled"]
    Enabled = 1,
}
impl From<Renr> for bool {
    #[inline(always)]
    fn from(variant: Renr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader<Renr>;
impl REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Renr {
        match self.bits {
            false => Renr::Disabled,
            true => Renr::Enabled,
        }
    }
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Renr::Disabled
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Renr::Enabled
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenwWO {
    #[doc = "0: Receiver disable"]
    Disable = 0,
    #[doc = "1: Receiver enable"]
    Enable = 1,
}
impl From<RenwWO> for bool {
    #[inline(always)]
    fn from(variant: RenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG, RenwWO>;
impl<'a, REG> REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RenwWO::Disable)
    }
    #[doc = "Receiver enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RenwWO::Enable)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tenr {
    #[doc = "0: Transmitter is disabled"]
    Disabled = 0,
    #[doc = "1: Transmitter is enabled"]
    Enabled = 1,
}
impl From<Tenr> for bool {
    #[inline(always)]
    fn from(variant: Tenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader<Tenr>;
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tenr {
        match self.bits {
            false => Tenr::Disabled,
            true => Tenr::Enabled,
        }
    }
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tenr::Disabled
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tenr::Enabled
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TenwWO {
    #[doc = "0: Transmitter disable"]
    Disable = 0,
    #[doc = "1: Transmitter enable"]
    Enable = 1,
}
impl From<TenwWO> for bool {
    #[inline(always)]
    fn from(variant: TenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG, TenwWO>;
impl<'a, REG> TEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TenwWO::Disable)
    }
    #[doc = "Transmitter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TenwWO::Enable)
    }
}
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idleienr {
    #[doc = "0: IDLE interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: IDLE interrupt is enabled"]
    Enabled = 1,
}
impl From<Idleienr> for bool {
    #[inline(always)]
    fn from(variant: Idleienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIEN` reader - IDLE interrupt enable"]
pub type IDLEIEN_R = crate::BitReader<Idleienr>;
impl IDLEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idleienr {
        match self.bits {
            false => Idleienr::Disabled,
            true => Idleienr::Enabled,
        }
    }
    #[doc = "IDLE interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Idleienr::Disabled
    }
    #[doc = "IDLE interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Idleienr::Enabled
    }
}
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleienwWO {
    #[doc = "0: IDLE interrupt disable"]
    Disable = 0,
    #[doc = "1: IDLE interrupt enable"]
    Enable = 1,
}
impl From<IdleienwWO> for bool {
    #[inline(always)]
    fn from(variant: IdleienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIEN` writer - IDLE interrupt enable"]
pub type IDLEIEN_W<'a, REG> = crate::BitWriter<'a, REG, IdleienwWO>;
impl<'a, REG> IDLEIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IDLE interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IdleienwWO::Disable)
    }
    #[doc = "IDLE interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IdleienwWO::Enable)
    }
}
#[doc = "RDBF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdbfienr {
    #[doc = "0: RDBF interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: RDBF interrupt is enabled"]
    Enabled = 1,
}
impl From<Rdbfienr> for bool {
    #[inline(always)]
    fn from(variant: Rdbfienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBFIEN` reader - RDBF interrupt enable"]
pub type RDBFIEN_R = crate::BitReader<Rdbfienr>;
impl RDBFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdbfienr {
        match self.bits {
            false => Rdbfienr::Disabled,
            true => Rdbfienr::Enabled,
        }
    }
    #[doc = "RDBF interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rdbfienr::Disabled
    }
    #[doc = "RDBF interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rdbfienr::Enabled
    }
}
#[doc = "RDBF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdbfienwWO {
    #[doc = "0: RDBF interrupt disable"]
    Disable = 0,
    #[doc = "1: RDBF interrupt enable"]
    Enable = 1,
}
impl From<RdbfienwWO> for bool {
    #[inline(always)]
    fn from(variant: RdbfienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBFIEN` writer - RDBF interrupt enable"]
pub type RDBFIEN_W<'a, REG> = crate::BitWriter<'a, REG, RdbfienwWO>;
impl<'a, REG> RDBFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RDBF interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RdbfienwWO::Disable)
    }
    #[doc = "RDBF interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RdbfienwWO::Enable)
    }
}
#[doc = "TDC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcienr {
    #[doc = "0: TDC interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: TDC interrupt is enabled"]
    Enabled = 1,
}
impl From<Tdcienr> for bool {
    #[inline(always)]
    fn from(variant: Tdcienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCIEN` reader - TDC interrupt enable"]
pub type TDCIEN_R = crate::BitReader<Tdcienr>;
impl TDCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdcienr {
        match self.bits {
            false => Tdcienr::Disabled,
            true => Tdcienr::Enabled,
        }
    }
    #[doc = "TDC interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tdcienr::Disabled
    }
    #[doc = "TDC interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tdcienr::Enabled
    }
}
#[doc = "TDC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdcienwWO {
    #[doc = "0: TDC interrupt disable"]
    Disable = 0,
    #[doc = "1: TDC interrupt enable"]
    Enable = 1,
}
impl From<TdcienwWO> for bool {
    #[inline(always)]
    fn from(variant: TdcienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCIEN` writer - TDC interrupt enable"]
pub type TDCIEN_W<'a, REG> = crate::BitWriter<'a, REG, TdcienwWO>;
impl<'a, REG> TDCIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDC interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TdcienwWO::Disable)
    }
    #[doc = "TDC interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TdcienwWO::Enable)
    }
}
#[doc = "TDBE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdbeienr {
    #[doc = "0: TDBE interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: TDBE interrupt is enabled"]
    Enabled = 1,
}
impl From<Tdbeienr> for bool {
    #[inline(always)]
    fn from(variant: Tdbeienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBEIEN` reader - TDBE interrupt enable"]
pub type TDBEIEN_R = crate::BitReader<Tdbeienr>;
impl TDBEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdbeienr {
        match self.bits {
            false => Tdbeienr::Disabled,
            true => Tdbeienr::Enabled,
        }
    }
    #[doc = "TDBE interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tdbeienr::Disabled
    }
    #[doc = "TDBE interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tdbeienr::Enabled
    }
}
#[doc = "TDBE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdbeienwWO {
    #[doc = "0: TDBE interrupt disable"]
    Disable = 0,
    #[doc = "1: TDBE interrupt enable"]
    Enable = 1,
}
impl From<TdbeienwWO> for bool {
    #[inline(always)]
    fn from(variant: TdbeienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBEIEN` writer - TDBE interrupt enable"]
pub type TDBEIEN_W<'a, REG> = crate::BitWriter<'a, REG, TdbeienwWO>;
impl<'a, REG> TDBEIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDBE interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TdbeienwWO::Disable)
    }
    #[doc = "TDBE interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TdbeienwWO::Enable)
    }
}
#[doc = "PERR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Perrienr {
    #[doc = "0: PERR interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: PERR interrupt is enabled"]
    Enabled = 1,
}
impl From<Perrienr> for bool {
    #[inline(always)]
    fn from(variant: Perrienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIEN` reader - PERR interrupt enable"]
pub type PERRIEN_R = crate::BitReader<Perrienr>;
impl PERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Perrienr {
        match self.bits {
            false => Perrienr::Disabled,
            true => Perrienr::Enabled,
        }
    }
    #[doc = "PERR interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Perrienr::Disabled
    }
    #[doc = "PERR interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Perrienr::Enabled
    }
}
#[doc = "PERR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerrienwWO {
    #[doc = "0: PERR interrupt disable"]
    Disable = 0,
    #[doc = "1: PERR interrupt enable"]
    Enable = 1,
}
impl From<PerrienwWO> for bool {
    #[inline(always)]
    fn from(variant: PerrienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIEN` writer - PERR interrupt enable"]
pub type PERRIEN_W<'a, REG> = crate::BitWriter<'a, REG, PerrienwWO>;
impl<'a, REG> PERRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PERR interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PerrienwWO::Disable)
    }
    #[doc = "PERR interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PerrienwWO::Enable)
    }
}
#[doc = "Parity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSEL_A {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<PSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSEL` reader - Parity selection"]
pub type PSEL_R = crate::BitReader<PSEL_A>;
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSEL_A {
        match self.bits {
            false => PSEL_A::Even,
            true => PSEL_A::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PSEL_A::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PSEL_A::Odd
    }
}
#[doc = "Field `PSEL` writer - Parity selection"]
pub type PSEL_W<'a, REG> = crate::BitWriter<'a, REG, PSEL_A>;
impl<'a, REG> PSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PSEL_A::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PSEL_A::Odd)
    }
}
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Penr {
    #[doc = "0: Parity is disabled"]
    Disabled = 0,
    #[doc = "1: Parity is enabled"]
    Enabled = 1,
}
impl From<Penr> for bool {
    #[inline(always)]
    fn from(variant: Penr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - Parity enable"]
pub type PEN_R = crate::BitReader<Penr>;
impl PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Penr {
        match self.bits {
            false => Penr::Disabled,
            true => Penr::Enabled,
        }
    }
    #[doc = "Parity is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Penr::Disabled
    }
    #[doc = "Parity is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Penr::Enabled
    }
}
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PenwWO {
    #[doc = "0: Parity disable"]
    Disable = 0,
    #[doc = "1: Parity enable"]
    Enable = 1,
}
impl From<PenwWO> for bool {
    #[inline(always)]
    fn from(variant: PenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` writer - Parity enable"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG, PenwWO>;
impl<'a, REG> PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PenwWO::Disable)
    }
    #[doc = "Parity enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PenwWO::Enable)
    }
}
#[doc = "Wake up mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUM_A {
    #[doc = "0: Waken up by idle line"]
    Idle = 0,
    #[doc = "1: Waken up by ID match"]
    Id = 1,
}
impl From<WUM_A> for bool {
    #[inline(always)]
    fn from(variant: WUM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUM` reader - Wake up mode"]
pub type WUM_R = crate::BitReader<WUM_A>;
impl WUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUM_A {
        match self.bits {
            false => WUM_A::Idle,
            true => WUM_A::Id,
        }
    }
    #[doc = "Waken up by idle line"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WUM_A::Idle
    }
    #[doc = "Waken up by ID match"]
    #[inline(always)]
    pub fn is_id(&self) -> bool {
        *self == WUM_A::Id
    }
}
#[doc = "Field `WUM` writer - Wake up mode"]
pub type WUM_W<'a, REG> = crate::BitWriter<'a, REG, WUM_A>;
impl<'a, REG> WUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Waken up by idle line"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(WUM_A::Idle)
    }
    #[doc = "Waken up by ID match"]
    #[inline(always)]
    pub fn id(self) -> &'a mut crate::W<REG> {
        self.variant(WUM_A::Id)
    }
}
#[doc = "Field `DBN0` reader - low bit for data bit num"]
pub type DBN0_R = crate::BitReader;
#[doc = "Field `DBN0` writer - low bit for data bit num"]
pub type DBN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uenr {
    #[doc = "0: USART is disabled"]
    Disabled = 0,
    #[doc = "1: USART is enabled"]
    Enabled = 1,
}
impl From<Uenr> for bool {
    #[inline(always)]
    fn from(variant: Uenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader<Uenr>;
impl UEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uenr {
        match self.bits {
            false => Uenr::Disabled,
            true => Uenr::Enabled,
        }
    }
    #[doc = "USART is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Uenr::Disabled
    }
    #[doc = "USART is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Uenr::Enabled
    }
}
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UenwWO {
    #[doc = "0: USART disable"]
    Disable = 0,
    #[doc = "1: USART enable"]
    Enable = 1,
}
impl From<UenwWO> for bool {
    #[inline(always)]
    fn from(variant: UenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a, REG> = crate::BitWriter<'a, REG, UenwWO>;
impl<'a, REG> UEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(UenwWO::Disable)
    }
    #[doc = "USART enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(UenwWO::Enable)
    }
}
#[doc = "Field `TCDT` reader - transmit complete delay time"]
pub type TCDT_R = crate::FieldReader;
#[doc = "Field `TCDT` writer - transmit complete delay time"]
pub type TCDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSDT` reader - transmit start delay time"]
pub type TSDT_R = crate::FieldReader;
#[doc = "Field `TSDT` writer - transmit start delay time"]
pub type TSDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBN1` reader - high bit for data bit num"]
pub type DBN1_R = crate::BitReader;
#[doc = "Field `DBN1` writer - high bit for data bit num"]
pub type DBN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleien(&self) -> IDLEIEN_R {
        IDLEIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    pub fn rdbfien(&self) -> RDBFIEN_R {
        RDBFIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&self) -> TDCIEN_R {
        TDCIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    pub fn tdbeien(&self) -> TDBEIEN_R {
        TDBEIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    pub fn perrien(&self) -> PERRIEN_R {
        PERRIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    pub fn wum(&self) -> WUM_R {
        WUM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - low bit for data bit num"]
    #[inline(always)]
    pub fn dbn0(&self) -> DBN0_R {
        DBN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - transmit complete delay time"]
    #[inline(always)]
    pub fn tcdt(&self) -> TCDT_R {
        TCDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - transmit start delay time"]
    #[inline(always)]
    pub fn tsdt(&self) -> TSDT_R {
        TSDT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - high bit for data bit num"]
    #[inline(always)]
    pub fn dbn1(&self) -> DBN1_R {
        DBN1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("dbn1", &self.dbn1())
            .field("tsdt", &self.tsdt())
            .field("tcdt", &self.tcdt())
            .field("uen", &self.uen())
            .field("dbn0", &self.dbn0())
            .field("wum", &self.wum())
            .field("pen", &self.pen())
            .field("psel", &self.psel())
            .field("perrien", &self.perrien())
            .field("tdbeien", &self.tdbeien())
            .field("tdcien", &self.tdcien())
            .field("rdbfien", &self.rdbfien())
            .field("idleien", &self.idleien())
            .field("ten", &self.ten())
            .field("ren", &self.ren())
            .field("rm", &self.rm())
            .field("sbf", &self.sbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    pub fn sbf(&mut self) -> SBF_W<'_, CTRL1_SPEC> {
        SBF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W<'_, CTRL1_SPEC> {
        RM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W<'_, CTRL1_SPEC> {
        REN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W<'_, CTRL1_SPEC> {
        TEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleien(&mut self) -> IDLEIEN_W<'_, CTRL1_SPEC> {
        IDLEIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    pub fn rdbfien(&mut self) -> RDBFIEN_W<'_, CTRL1_SPEC> {
        RDBFIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&mut self) -> TDCIEN_W<'_, CTRL1_SPEC> {
        TDCIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    pub fn tdbeien(&mut self) -> TDBEIEN_W<'_, CTRL1_SPEC> {
        TDBEIEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    pub fn perrien(&mut self) -> PERRIEN_W<'_, CTRL1_SPEC> {
        PERRIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W<'_, CTRL1_SPEC> {
        PSEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W<'_, CTRL1_SPEC> {
        PEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    pub fn wum(&mut self) -> WUM_W<'_, CTRL1_SPEC> {
        WUM_W::new(self, 11)
    }
    #[doc = "Bit 12 - low bit for data bit num"]
    #[inline(always)]
    pub fn dbn0(&mut self) -> DBN0_W<'_, CTRL1_SPEC> {
        DBN0_W::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&mut self) -> UEN_W<'_, CTRL1_SPEC> {
        UEN_W::new(self, 13)
    }
    #[doc = "Bits 16:20 - transmit complete delay time"]
    #[inline(always)]
    pub fn tcdt(&mut self) -> TCDT_W<'_, CTRL1_SPEC> {
        TCDT_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - transmit start delay time"]
    #[inline(always)]
    pub fn tsdt(&mut self) -> TSDT_W<'_, CTRL1_SPEC> {
        TSDT_W::new(self, 21)
    }
    #[doc = "Bit 28 - high bit for data bit num"]
    #[inline(always)]
    pub fn dbn1(&mut self) -> DBN1_W<'_, CTRL1_SPEC> {
        DBN1_W::new(self, 28)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {}
