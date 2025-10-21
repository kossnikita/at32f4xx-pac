#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<CTRL3_SPEC>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<CTRL3_SPEC>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errienr {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt is enabled"]
    Enabled = 1,
}
impl From<Errienr> for bool {
    #[inline(always)]
    fn from(variant: Errienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ERRIEN_R = crate::BitReader<Errienr>;
impl ERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errienr {
        match self.bits {
            false => Errienr::Disabled,
            true => Errienr::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errienr::Disabled
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errienr::Enabled
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrienwWO {
    #[doc = "0: Error interrupt disable"]
    Disable = 0,
    #[doc = "1: Error interrupt enable"]
    Enable = 1,
}
impl From<ErrienwWO> for bool {
    #[inline(always)]
    fn from(variant: ErrienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ERRIEN_W<'a, REG> = crate::BitWriter<'a, REG, ErrienwWO>;
impl<'a, REG> ERRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrienwWO::Disable)
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrienwWO::Enable)
    }
}
#[doc = "IrDA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irdaenr {
    #[doc = "0: IrDA is disabled"]
    Disabled = 0,
    #[doc = "1: IrDA is enabled"]
    Enabled = 1,
}
impl From<Irdaenr> for bool {
    #[inline(always)]
    fn from(variant: Irdaenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAEN` reader - IrDA enable"]
pub type IRDAEN_R = crate::BitReader<Irdaenr>;
impl IRDAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irdaenr {
        match self.bits {
            false => Irdaenr::Disabled,
            true => Irdaenr::Enabled,
        }
    }
    #[doc = "IrDA is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irdaenr::Disabled
    }
    #[doc = "IrDA is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irdaenr::Enabled
    }
}
#[doc = "IrDA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrdaenwWO {
    #[doc = "0: IrDA disable"]
    Disable = 0,
    #[doc = "1: IrDA enable"]
    Enable = 1,
}
impl From<IrdaenwWO> for bool {
    #[inline(always)]
    fn from(variant: IrdaenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAEN` writer - IrDA enable"]
pub type IRDAEN_W<'a, REG> = crate::BitWriter<'a, REG, IrdaenwWO>;
impl<'a, REG> IRDAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IrdaenwWO::Disable)
    }
    #[doc = "IrDA enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IrdaenwWO::Enable)
    }
}
#[doc = "IrDA low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irdalpr {
    #[doc = "0: IrDA low-power mode is disabled"]
    Disabled = 0,
    #[doc = "1: IrDA low-power mode is enabled"]
    Enabled = 1,
}
impl From<Irdalpr> for bool {
    #[inline(always)]
    fn from(variant: Irdalpr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDALP` reader - IrDA low-power mode"]
pub type IRDALP_R = crate::BitReader<Irdalpr>;
impl IRDALP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irdalpr {
        match self.bits {
            false => Irdalpr::Disabled,
            true => Irdalpr::Enabled,
        }
    }
    #[doc = "IrDA low-power mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irdalpr::Disabled
    }
    #[doc = "IrDA low-power mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irdalpr::Enabled
    }
}
#[doc = "IrDA low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrdalpwWO {
    #[doc = "0: IrDA low-power mode disable"]
    Disable = 0,
    #[doc = "1: IrDA low-power mode enable"]
    Enable = 1,
}
impl From<IrdalpwWO> for bool {
    #[inline(always)]
    fn from(variant: IrdalpwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDALP` writer - IrDA low-power mode"]
pub type IRDALP_W<'a, REG> = crate::BitWriter<'a, REG, IrdalpwWO>;
impl<'a, REG> IRDALP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA low-power mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IrdalpwWO::Disable)
    }
    #[doc = "IrDA low-power mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IrdalpwWO::Enable)
    }
}
#[doc = "Single line bidirectional half-duplex enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slbenr {
    #[doc = "0: Single-wire bidirectional half-duplex is disabled"]
    Disabled = 0,
    #[doc = "1: Single-wire bidirectional half-duplex is enabled"]
    Enabled = 1,
}
impl From<Slbenr> for bool {
    #[inline(always)]
    fn from(variant: Slbenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SLBEN_R = crate::BitReader<Slbenr>;
impl SLBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slbenr {
        match self.bits {
            false => Slbenr::Disabled,
            true => Slbenr::Enabled,
        }
    }
    #[doc = "Single-wire bidirectional half-duplex is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slbenr::Disabled
    }
    #[doc = "Single-wire bidirectional half-duplex is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slbenr::Enabled
    }
}
#[doc = "Single line bidirectional half-duplex enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlbenwWO {
    #[doc = "0: Single-wire bidirectional half-duplex disable"]
    Disable = 0,
    #[doc = "1: Single-wire bidirectional half-duplex enable"]
    Enable = 1,
}
impl From<SlbenwWO> for bool {
    #[inline(always)]
    fn from(variant: SlbenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SLBEN_W<'a, REG> = crate::BitWriter<'a, REG, SlbenwWO>;
impl<'a, REG> SLBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-wire bidirectional half-duplex disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SlbenwWO::Disable)
    }
    #[doc = "Single-wire bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SlbenwWO::Enable)
    }
}
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scnackenr {
    #[doc = "0: Smartcard NACK is disabled"]
    Disabled = 0,
    #[doc = "1: Smartcard NACK is enabled"]
    Enabled = 1,
}
impl From<Scnackenr> for bool {
    #[inline(always)]
    fn from(variant: Scnackenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCNACKEN` reader - Smartcard NACK enable"]
pub type SCNACKEN_R = crate::BitReader<Scnackenr>;
impl SCNACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scnackenr {
        match self.bits {
            false => Scnackenr::Disabled,
            true => Scnackenr::Enabled,
        }
    }
    #[doc = "Smartcard NACK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scnackenr::Disabled
    }
    #[doc = "Smartcard NACK is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scnackenr::Enabled
    }
}
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScnackenwWO {
    #[doc = "0: Smartcard NACK disable"]
    Disable = 0,
    #[doc = "1: Smartcard NACK enable"]
    Enable = 1,
}
impl From<ScnackenwWO> for bool {
    #[inline(always)]
    fn from(variant: ScnackenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCNACKEN` writer - Smartcard NACK enable"]
pub type SCNACKEN_W<'a, REG> = crate::BitWriter<'a, REG, ScnackenwWO>;
impl<'a, REG> SCNACKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard NACK disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ScnackenwWO::Disable)
    }
    #[doc = "Smartcard NACK enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ScnackenwWO::Enable)
    }
}
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scmenr {
    #[doc = "0: Smartcard mode is disabled"]
    Disabled = 0,
    #[doc = "1: Smartcard mode is enabled"]
    Enabled = 1,
}
impl From<Scmenr> for bool {
    #[inline(always)]
    fn from(variant: Scmenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMEN` reader - Smartcard mode enable"]
pub type SCMEN_R = crate::BitReader<Scmenr>;
impl SCMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scmenr {
        match self.bits {
            false => Scmenr::Disabled,
            true => Scmenr::Enabled,
        }
    }
    #[doc = "Smartcard mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scmenr::Disabled
    }
    #[doc = "Smartcard mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scmenr::Enabled
    }
}
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScmenwWO {
    #[doc = "0: Smartcard mode disable"]
    Disable = 0,
    #[doc = "1: Smartcard mode enable"]
    Enable = 1,
}
impl From<ScmenwWO> for bool {
    #[inline(always)]
    fn from(variant: ScmenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMEN` writer - Smartcard mode enable"]
pub type SCMEN_W<'a, REG> = crate::BitWriter<'a, REG, ScmenwWO>;
impl<'a, REG> SCMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ScmenwWO::Disable)
    }
    #[doc = "Smartcard mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ScmenwWO::Enable)
    }
}
#[doc = "DMA receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarenr {
    #[doc = "0: DMA receiver is disabled"]
    Disabled = 0,
    #[doc = "1: DMA receiver is enabled"]
    Enabled = 1,
}
impl From<Dmarenr> for bool {
    #[inline(always)]
    fn from(variant: Dmarenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` reader - DMA receiver enable"]
pub type DMAREN_R = crate::BitReader<Dmarenr>;
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarenr {
        match self.bits {
            false => Dmarenr::Disabled,
            true => Dmarenr::Enabled,
        }
    }
    #[doc = "DMA receiver is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarenr::Disabled
    }
    #[doc = "DMA receiver is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarenr::Enabled
    }
}
#[doc = "DMA receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmarenwWO {
    #[doc = "0: DMA receiver disable"]
    Disable = 0,
    #[doc = "1: DMA receiver enable"]
    Enable = 1,
}
impl From<DmarenwWO> for bool {
    #[inline(always)]
    fn from(variant: DmarenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` writer - DMA receiver enable"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG, DmarenwWO>;
impl<'a, REG> DMAREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA receiver disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmarenwWO::Disable)
    }
    #[doc = "DMA receiver enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmarenwWO::Enable)
    }
}
#[doc = "DMA transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatenr {
    #[doc = "0: DMA transmitter is disabled"]
    Disabled = 0,
    #[doc = "1: DMA transmitter is enabled"]
    Enabled = 1,
}
impl From<Dmatenr> for bool {
    #[inline(always)]
    fn from(variant: Dmatenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` reader - DMA transmitter enable"]
pub type DMATEN_R = crate::BitReader<Dmatenr>;
impl DMATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatenr {
        match self.bits {
            false => Dmatenr::Disabled,
            true => Dmatenr::Enabled,
        }
    }
    #[doc = "DMA transmitter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatenr::Disabled
    }
    #[doc = "DMA transmitter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatenr::Enabled
    }
}
#[doc = "DMA transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmatenwWO {
    #[doc = "0: DMA transmitter disable"]
    Disable = 0,
    #[doc = "1: DMA transmitter enable"]
    Enable = 1,
}
impl From<DmatenwWO> for bool {
    #[inline(always)]
    fn from(variant: DmatenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` writer - DMA transmitter enable"]
pub type DMATEN_W<'a, REG> = crate::BitWriter<'a, REG, DmatenwWO>;
impl<'a, REG> DMATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transmitter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmatenwWO::Disable)
    }
    #[doc = "DMA transmitter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmatenwWO::Enable)
    }
}
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsenr {
    #[doc = "0: RTS is disabled"]
    Disabled = 0,
    #[doc = "1: RTS is enabled"]
    Enabled = 1,
}
impl From<Rtsenr> for bool {
    #[inline(always)]
    fn from(variant: Rtsenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RTSEN_R = crate::BitReader<Rtsenr>;
impl RTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsenr {
        match self.bits {
            false => Rtsenr::Disabled,
            true => Rtsenr::Enabled,
        }
    }
    #[doc = "RTS is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtsenr::Disabled
    }
    #[doc = "RTS is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rtsenr::Enabled
    }
}
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtsenwWO {
    #[doc = "0: RTS disable"]
    Disable = 0,
    #[doc = "1: RTS enable"]
    Enable = 1,
}
impl From<RtsenwWO> for bool {
    #[inline(always)]
    fn from(variant: RtsenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RTSEN_W<'a, REG> = crate::BitWriter<'a, REG, RtsenwWO>;
impl<'a, REG> RTSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RtsenwWO::Disable)
    }
    #[doc = "RTS enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RtsenwWO::Enable)
    }
}
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsenr {
    #[doc = "0: CTS is disabled"]
    Disabled = 0,
    #[doc = "1: CTS is enabled"]
    Enabled = 1,
}
impl From<Ctsenr> for bool {
    #[inline(always)]
    fn from(variant: Ctsenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CTSEN_R = crate::BitReader<Ctsenr>;
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsenr {
        match self.bits {
            false => Ctsenr::Disabled,
            true => Ctsenr::Enabled,
        }
    }
    #[doc = "CTS is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctsenr::Disabled
    }
    #[doc = "CTS is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctsenr::Enabled
    }
}
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtsenwWO {
    #[doc = "0: CTS disable"]
    Disable = 0,
    #[doc = "1: CTS enable"]
    Enable = 1,
}
impl From<CtsenwWO> for bool {
    #[inline(always)]
    fn from(variant: CtsenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CTSEN_W<'a, REG> = crate::BitWriter<'a, REG, CtsenwWO>;
impl<'a, REG> CTSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtsenwWO::Disable)
    }
    #[doc = "CTS enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CtsenwWO::Enable)
    }
}
#[doc = "CTSCF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctscfienr {
    #[doc = "0: CTSCF interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: CTSCF interrupt is enabled"]
    Enabled = 1,
}
impl From<Ctscfienr> for bool {
    #[inline(always)]
    fn from(variant: Ctscfienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCFIEN` reader - CTSCF interrupt enable"]
pub type CTSCFIEN_R = crate::BitReader<Ctscfienr>;
impl CTSCFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctscfienr {
        match self.bits {
            false => Ctscfienr::Disabled,
            true => Ctscfienr::Enabled,
        }
    }
    #[doc = "CTSCF interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctscfienr::Disabled
    }
    #[doc = "CTSCF interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctscfienr::Enabled
    }
}
#[doc = "CTSCF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtscfienwWO {
    #[doc = "0: CTSCF interrupt disable"]
    Disable = 0,
    #[doc = "1: CTSCF interrupt enable"]
    Enable = 1,
}
impl From<CtscfienwWO> for bool {
    #[inline(always)]
    fn from(variant: CtscfienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCFIEN` writer - CTSCF interrupt enable"]
pub type CTSCFIEN_W<'a, REG> = crate::BitWriter<'a, REG, CtscfienwWO>;
impl<'a, REG> CTSCFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTSCF interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtscfienwWO::Disable)
    }
    #[doc = "CTSCF interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CtscfienwWO::Enable)
    }
}
#[doc = "Field `SMUSEN` reader - Deepsleep mode usart enable"]
pub type SMUSEN_R = crate::BitReader;
#[doc = "Field `SMUSEN` writer - Deepsleep mode usart enable"]
pub type SMUSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWUFIE` reader - Low power wakeup flag interrupt enable"]
pub type LPWUFIE_R = crate::BitReader;
#[doc = "Field `LPWUFIE` writer - Low power wakeup flag interrupt enable"]
pub type LPWUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS485EN` reader - RS485 enable"]
pub type RS485EN_R = crate::BitReader;
#[doc = "Field `RS485EN` writer - RS485 enable"]
pub type RS485EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - DE polarity selection"]
pub type DEP_R = crate::BitReader;
#[doc = "Field `DEP` writer - DE polarity selection"]
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWUM` reader - Low power wakeup method"]
pub type LPWUM_R = crate::FieldReader;
#[doc = "Field `LPWUM` writer - Low power wakeup method"]
pub type LPWUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA enable"]
    #[inline(always)]
    pub fn irdaen(&self) -> IRDAEN_R {
        IRDAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power mode"]
    #[inline(always)]
    pub fn irdalp(&self) -> IRDALP_R {
        IRDALP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn slben(&self) -> SLBEN_R {
        SLBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn scnacken(&self) -> SCNACKEN_R {
        SCNACKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scmen(&self) -> SCMEN_R {
        SCMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA receiver enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA transmitter enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSCF interrupt enable"]
    #[inline(always)]
    pub fn ctscfien(&self) -> CTSCFIEN_R {
        CTSCFIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Deepsleep mode usart enable"]
    #[inline(always)]
    pub fn smusen(&self) -> SMUSEN_R {
        SMUSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Low power wakeup flag interrupt enable"]
    #[inline(always)]
    pub fn lpwufie(&self) -> LPWUFIE_R {
        LPWUFIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RS485 enable"]
    #[inline(always)]
    pub fn rs485en(&self) -> RS485EN_R {
        RS485EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DE polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Low power wakeup method"]
    #[inline(always)]
    pub fn lpwum(&self) -> LPWUM_R {
        LPWUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL3")
            .field("lpwum", &self.lpwum())
            .field("dep", &self.dep())
            .field("rs485en", &self.rs485en())
            .field("lpwufie", &self.lpwufie())
            .field("smusen", &self.smusen())
            .field("ctscfien", &self.ctscfien())
            .field("ctsen", &self.ctsen())
            .field("rtsen", &self.rtsen())
            .field("dmaten", &self.dmaten())
            .field("dmaren", &self.dmaren())
            .field("scmen", &self.scmen())
            .field("scnacken", &self.scnacken())
            .field("slben", &self.slben())
            .field("irdalp", &self.irdalp())
            .field("irdaen", &self.irdaen())
            .field("errien", &self.errien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&mut self) -> ERRIEN_W<'_, CTRL3_SPEC> {
        ERRIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA enable"]
    #[inline(always)]
    pub fn irdaen(&mut self) -> IRDAEN_W<'_, CTRL3_SPEC> {
        IRDAEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power mode"]
    #[inline(always)]
    pub fn irdalp(&mut self) -> IRDALP_W<'_, CTRL3_SPEC> {
        IRDALP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn slben(&mut self) -> SLBEN_W<'_, CTRL3_SPEC> {
        SLBEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn scnacken(&mut self) -> SCNACKEN_W<'_, CTRL3_SPEC> {
        SCNACKEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scmen(&mut self) -> SCMEN_W<'_, CTRL3_SPEC> {
        SCMEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA receiver enable"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W<'_, CTRL3_SPEC> {
        DMAREN_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA transmitter enable"]
    #[inline(always)]
    pub fn dmaten(&mut self) -> DMATEN_W<'_, CTRL3_SPEC> {
        DMATEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W<'_, CTRL3_SPEC> {
        RTSEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W<'_, CTRL3_SPEC> {
        CTSEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTSCF interrupt enable"]
    #[inline(always)]
    pub fn ctscfien(&mut self) -> CTSCFIEN_W<'_, CTRL3_SPEC> {
        CTSCFIEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Deepsleep mode usart enable"]
    #[inline(always)]
    pub fn smusen(&mut self) -> SMUSEN_W<'_, CTRL3_SPEC> {
        SMUSEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - Low power wakeup flag interrupt enable"]
    #[inline(always)]
    pub fn lpwufie(&mut self) -> LPWUFIE_W<'_, CTRL3_SPEC> {
        LPWUFIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - RS485 enable"]
    #[inline(always)]
    pub fn rs485en(&mut self) -> RS485EN_W<'_, CTRL3_SPEC> {
        RS485EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - DE polarity selection"]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<'_, CTRL3_SPEC> {
        DEP_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Low power wakeup method"]
    #[inline(always)]
    pub fn lpwum(&mut self) -> LPWUM_W<'_, CTRL3_SPEC> {
        LPWUM_W::new(self, 16)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL3_SPEC;
impl crate::RegisterSpec for CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for CTRL3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for CTRL3_SPEC {}
