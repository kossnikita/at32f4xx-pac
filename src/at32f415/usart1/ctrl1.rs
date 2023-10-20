#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `SBF` reader - Send break frame"]
pub type SBF_R = crate::BitReader<SBFR_A>;
#[doc = "Send break frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBFR_A {
    #[doc = "0: No break frame is transmitted"]
    NoTransmission = 0,
    #[doc = "1: Break frame is transmitted"]
    Transmitting = 1,
}
impl From<SBFR_A> for bool {
    #[inline(always)]
    fn from(variant: SBFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBFR_A {
        match self.bits {
            false => SBFR_A::NoTransmission,
            true => SBFR_A::Transmitting,
        }
    }
    #[doc = "No break frame is transmitted"]
    #[inline(always)]
    pub fn is_no_transmission(&self) -> bool {
        *self == SBFR_A::NoTransmission
    }
    #[doc = "Break frame is transmitted"]
    #[inline(always)]
    pub fn is_transmitting(&self) -> bool {
        *self == SBFR_A::Transmitting
    }
}
#[doc = "Send break frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBFW_AW {
    #[doc = "0: Clear send break frame"]
    Clear = 0,
    #[doc = "1: Send break frame"]
    Send = 1,
}
impl From<SBFW_AW> for bool {
    #[inline(always)]
    fn from(variant: SBFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` writer - Send break frame"]
pub type SBF_W<'a, REG> = crate::BitWriter<'a, REG, SBFW_AW>;
impl<'a, REG> SBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear send break frame"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SBFW_AW::Clear)
    }
    #[doc = "Send break frame"]
    #[inline(always)]
    pub fn send(self) -> &'a mut crate::W<REG> {
        self.variant(SBFW_AW::Send)
    }
}
#[doc = "Field `RM` reader - Receiver mute"]
pub type RM_R = crate::BitReader<RM_A>;
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
#[doc = "Field `REN` reader - Receiver enable"]
pub type REN_R = crate::BitReader<RENR_A>;
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RENR_A {
    #[doc = "0: Receiver is disabled"]
    Disabled = 0,
    #[doc = "1: Receiver is enabled"]
    Enabled = 1,
}
impl From<RENR_A> for bool {
    #[inline(always)]
    fn from(variant: RENR_A) -> Self {
        variant as u8 != 0
    }
}
impl REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RENR_A {
        match self.bits {
            false => RENR_A::Disabled,
            true => RENR_A::Enabled,
        }
    }
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RENR_A::Disabled
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RENR_A::Enabled
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RENW_AW {
    #[doc = "0: Receiver disable"]
    Disable = 0,
    #[doc = "1: Receiver enable"]
    Enable = 1,
}
impl From<RENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REN` writer - Receiver enable"]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG, RENW_AW>;
impl<'a, REG> REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RENW_AW::Disable)
    }
    #[doc = "Receiver enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RENW_AW::Enable)
    }
}
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TEN_R = crate::BitReader<TENR_A>;
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TENR_A {
    #[doc = "0: Transmitter is disabled"]
    Disabled = 0,
    #[doc = "1: Transmitter is enabled"]
    Enabled = 1,
}
impl From<TENR_A> for bool {
    #[inline(always)]
    fn from(variant: TENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TENR_A {
        match self.bits {
            false => TENR_A::Disabled,
            true => TENR_A::Enabled,
        }
    }
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TENR_A::Disabled
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TENR_A::Enabled
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TENW_AW {
    #[doc = "0: Transmitter disable"]
    Disable = 0,
    #[doc = "1: Transmitter enable"]
    Enable = 1,
}
impl From<TENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG, TENW_AW>;
impl<'a, REG> TEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TENW_AW::Disable)
    }
    #[doc = "Transmitter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TENW_AW::Enable)
    }
}
#[doc = "Field `IDLEIEN` reader - IDLE interrupt enable"]
pub type IDLEIEN_R = crate::BitReader<IDLEIENR_A>;
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIENR_A {
    #[doc = "0: IDLE interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: IDLE interrupt is enabled"]
    Enabled = 1,
}
impl From<IDLEIENR_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLEIENR_A {
        match self.bits {
            false => IDLEIENR_A::Disabled,
            true => IDLEIENR_A::Enabled,
        }
    }
    #[doc = "IDLE interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIENR_A::Disabled
    }
    #[doc = "IDLE interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIENR_A::Enabled
    }
}
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIENW_AW {
    #[doc = "0: IDLE interrupt disable"]
    Disable = 0,
    #[doc = "1: IDLE interrupt enable"]
    Enable = 1,
}
impl From<IDLEIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: IDLEIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIEN` writer - IDLE interrupt enable"]
pub type IDLEIEN_W<'a, REG> = crate::BitWriter<'a, REG, IDLEIENW_AW>;
impl<'a, REG> IDLEIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IDLE interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIENW_AW::Disable)
    }
    #[doc = "IDLE interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIENW_AW::Enable)
    }
}
#[doc = "Field `RDBFIEN` reader - RDBF interrupt enable"]
pub type RDBFIEN_R = crate::BitReader<RDBFIENR_A>;
#[doc = "RDBF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFIENR_A {
    #[doc = "0: RDBF interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: RDBF interrupt is enabled"]
    Enabled = 1,
}
impl From<RDBFIENR_A> for bool {
    #[inline(always)]
    fn from(variant: RDBFIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDBFIENR_A {
        match self.bits {
            false => RDBFIENR_A::Disabled,
            true => RDBFIENR_A::Enabled,
        }
    }
    #[doc = "RDBF interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDBFIENR_A::Disabled
    }
    #[doc = "RDBF interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDBFIENR_A::Enabled
    }
}
#[doc = "RDBF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFIENW_AW {
    #[doc = "0: RDBF interrupt disable"]
    Disable = 0,
    #[doc = "1: RDBF interrupt enable"]
    Enable = 1,
}
impl From<RDBFIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RDBFIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBFIEN` writer - RDBF interrupt enable"]
pub type RDBFIEN_W<'a, REG> = crate::BitWriter<'a, REG, RDBFIENW_AW>;
impl<'a, REG> RDBFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RDBF interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RDBFIENW_AW::Disable)
    }
    #[doc = "RDBF interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RDBFIENW_AW::Enable)
    }
}
#[doc = "Field `TDCIEN` reader - TDC interrupt enable"]
pub type TDCIEN_R = crate::BitReader<TDCIENR_A>;
#[doc = "TDC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCIENR_A {
    #[doc = "0: TDC interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: TDC interrupt is enabled"]
    Enabled = 1,
}
impl From<TDCIENR_A> for bool {
    #[inline(always)]
    fn from(variant: TDCIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TDCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDCIENR_A {
        match self.bits {
            false => TDCIENR_A::Disabled,
            true => TDCIENR_A::Enabled,
        }
    }
    #[doc = "TDC interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDCIENR_A::Disabled
    }
    #[doc = "TDC interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDCIENR_A::Enabled
    }
}
#[doc = "TDC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDCIENW_AW {
    #[doc = "0: TDC interrupt disable"]
    Disable = 0,
    #[doc = "1: TDC interrupt enable"]
    Enable = 1,
}
impl From<TDCIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TDCIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCIEN` writer - TDC interrupt enable"]
pub type TDCIEN_W<'a, REG> = crate::BitWriter<'a, REG, TDCIENW_AW>;
impl<'a, REG> TDCIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDC interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TDCIENW_AW::Disable)
    }
    #[doc = "TDC interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TDCIENW_AW::Enable)
    }
}
#[doc = "Field `TDBEIEN` reader - TDBE interrupt enable"]
pub type TDBEIEN_R = crate::BitReader<TDBEIENR_A>;
#[doc = "TDBE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEIENR_A {
    #[doc = "0: TDBE interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: TDBE interrupt is enabled"]
    Enabled = 1,
}
impl From<TDBEIENR_A> for bool {
    #[inline(always)]
    fn from(variant: TDBEIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDBEIENR_A {
        match self.bits {
            false => TDBEIENR_A::Disabled,
            true => TDBEIENR_A::Enabled,
        }
    }
    #[doc = "TDBE interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDBEIENR_A::Disabled
    }
    #[doc = "TDBE interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDBEIENR_A::Enabled
    }
}
#[doc = "TDBE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEIENW_AW {
    #[doc = "0: TDBE interrupt disable"]
    Disable = 0,
    #[doc = "1: TDBE interrupt enable"]
    Enable = 1,
}
impl From<TDBEIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TDBEIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBEIEN` writer - TDBE interrupt enable"]
pub type TDBEIEN_W<'a, REG> = crate::BitWriter<'a, REG, TDBEIENW_AW>;
impl<'a, REG> TDBEIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDBE interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TDBEIENW_AW::Disable)
    }
    #[doc = "TDBE interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TDBEIENW_AW::Enable)
    }
}
#[doc = "Field `PERRIEN` reader - PERR interrupt enable"]
pub type PERRIEN_R = crate::BitReader<PERRIENR_A>;
#[doc = "PERR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERRIENR_A {
    #[doc = "0: PERR interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: PERR interrupt is enabled"]
    Enabled = 1,
}
impl From<PERRIENR_A> for bool {
    #[inline(always)]
    fn from(variant: PERRIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl PERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PERRIENR_A {
        match self.bits {
            false => PERRIENR_A::Disabled,
            true => PERRIENR_A::Enabled,
        }
    }
    #[doc = "PERR interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERRIENR_A::Disabled
    }
    #[doc = "PERR interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERRIENR_A::Enabled
    }
}
#[doc = "PERR interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERRIENW_AW {
    #[doc = "0: PERR interrupt disable"]
    Disable = 0,
    #[doc = "1: PERR interrupt enable"]
    Enable = 1,
}
impl From<PERRIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: PERRIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERRIEN` writer - PERR interrupt enable"]
pub type PERRIEN_W<'a, REG> = crate::BitWriter<'a, REG, PERRIENW_AW>;
impl<'a, REG> PERRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PERR interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PERRIENW_AW::Disable)
    }
    #[doc = "PERR interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PERRIENW_AW::Enable)
    }
}
#[doc = "Field `PSEL` reader - Parity selection"]
pub type PSEL_R = crate::BitReader<PSEL_A>;
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
#[doc = "Field `PEN` reader - Parity enable"]
pub type PEN_R = crate::BitReader<PENR_A>;
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENR_A {
    #[doc = "0: Parity is disabled"]
    Disabled = 0,
    #[doc = "1: Parity is enabled"]
    Enabled = 1,
}
impl From<PENR_A> for bool {
    #[inline(always)]
    fn from(variant: PENR_A) -> Self {
        variant as u8 != 0
    }
}
impl PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PENR_A {
        match self.bits {
            false => PENR_A::Disabled,
            true => PENR_A::Enabled,
        }
    }
    #[doc = "Parity is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PENR_A::Disabled
    }
    #[doc = "Parity is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PENR_A::Enabled
    }
}
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PENW_AW {
    #[doc = "0: Parity disable"]
    Disable = 0,
    #[doc = "1: Parity enable"]
    Enable = 1,
}
impl From<PENW_AW> for bool {
    #[inline(always)]
    fn from(variant: PENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` writer - Parity enable"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG, PENW_AW>;
impl<'a, REG> PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PENW_AW::Disable)
    }
    #[doc = "Parity enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PENW_AW::Enable)
    }
}
#[doc = "Field `WUM` reader - Wake up mode"]
pub type WUM_R = crate::BitReader<WUM_A>;
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
#[doc = "Field `DBN` reader - Data bit num"]
pub type DBN_R = crate::BitReader;
#[doc = "Field `DBN` writer - Data bit num"]
pub type DBN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UEN_R = crate::BitReader<UENR_A>;
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UENR_A {
    #[doc = "0: USART is disabled"]
    Disabled = 0,
    #[doc = "1: USART is enabled"]
    Enabled = 1,
}
impl From<UENR_A> for bool {
    #[inline(always)]
    fn from(variant: UENR_A) -> Self {
        variant as u8 != 0
    }
}
impl UEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UENR_A {
        match self.bits {
            false => UENR_A::Disabled,
            true => UENR_A::Enabled,
        }
    }
    #[doc = "USART is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UENR_A::Disabled
    }
    #[doc = "USART is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UENR_A::Enabled
    }
}
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UENW_AW {
    #[doc = "0: USART disable"]
    Disable = 0,
    #[doc = "1: USART enable"]
    Enable = 1,
}
impl From<UENW_AW> for bool {
    #[inline(always)]
    fn from(variant: UENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UEN` writer - USART enable"]
pub type UEN_W<'a, REG> = crate::BitWriter<'a, REG, UENW_AW>;
impl<'a, REG> UEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(UENW_AW::Disable)
    }
    #[doc = "USART enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(UENW_AW::Enable)
    }
}
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
    #[doc = "Bit 12 - Data bit num"]
    #[inline(always)]
    pub fn dbn(&self) -> DBN_R {
        DBN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("uen", &format_args!("{}", self.uen().bit()))
            .field("dbn", &format_args!("{}", self.dbn().bit()))
            .field("wum", &format_args!("{}", self.wum().bit()))
            .field("pen", &format_args!("{}", self.pen().bit()))
            .field("psel", &format_args!("{}", self.psel().bit()))
            .field("perrien", &format_args!("{}", self.perrien().bit()))
            .field("tdbeien", &format_args!("{}", self.tdbeien().bit()))
            .field("tdcien", &format_args!("{}", self.tdcien().bit()))
            .field("rdbfien", &format_args!("{}", self.rdbfien().bit()))
            .field("idleien", &format_args!("{}", self.idleien().bit()))
            .field("ten", &format_args!("{}", self.ten().bit()))
            .field("ren", &format_args!("{}", self.ren().bit()))
            .field("rm", &format_args!("{}", self.rm().bit()))
            .field("sbf", &format_args!("{}", self.sbf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SBF_W<CTRL1_SPEC> {
        SBF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RM_W<CTRL1_SPEC> {
        RM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CTRL1_SPEC> {
        REN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CTRL1_SPEC> {
        TEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleien(&mut self) -> IDLEIEN_W<CTRL1_SPEC> {
        IDLEIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfien(&mut self) -> RDBFIEN_W<CTRL1_SPEC> {
        RDBFIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcien(&mut self) -> TDCIEN_W<CTRL1_SPEC> {
        TDCIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeien(&mut self) -> TDBEIEN_W<CTRL1_SPEC> {
        TDBEIEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrien(&mut self) -> PERRIEN_W<CTRL1_SPEC> {
        PERRIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<CTRL1_SPEC> {
        PSEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<CTRL1_SPEC> {
        PEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    #[must_use]
    pub fn wum(&mut self) -> WUM_W<CTRL1_SPEC> {
        WUM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data bit num"]
    #[inline(always)]
    #[must_use]
    pub fn dbn(&mut self) -> DBN_W<CTRL1_SPEC> {
        DBN_W::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UEN_W<CTRL1_SPEC> {
        UEN_W::new(self, 13)
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
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
