#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<CTRL3_SPEC>;
#[doc = "Register `CTRL3` writer"]
pub type W = crate::W<CTRL3_SPEC>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ERRIEN_R = crate::BitReader<ERRIENR_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIENR_A {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt is enabled"]
    Enabled = 1,
}
impl From<ERRIENR_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIENR_A {
        match self.bits {
            false => ERRIENR_A::Disabled,
            true => ERRIENR_A::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIENR_A::Disabled
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIENR_A::Enabled
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIENW_AW {
    #[doc = "0: Error interrupt disable"]
    Disable = 0,
    #[doc = "1: Error interrupt enable"]
    Enable = 1,
}
impl From<ERRIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIENW_AW>;
impl<'a, REG, const O: u8> ERRIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIENW_AW::Disable)
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIENW_AW::Enable)
    }
}
#[doc = "Field `IRDAEN` reader - IrDA enable"]
pub type IRDAEN_R = crate::BitReader<IRDAENR_A>;
#[doc = "IrDA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRDAENR_A {
    #[doc = "0: IrDA is disabled"]
    Disabled = 0,
    #[doc = "1: IrDA is enabled"]
    Enabled = 1,
}
impl From<IRDAENR_A> for bool {
    #[inline(always)]
    fn from(variant: IRDAENR_A) -> Self {
        variant as u8 != 0
    }
}
impl IRDAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDAENR_A {
        match self.bits {
            false => IRDAENR_A::Disabled,
            true => IRDAENR_A::Enabled,
        }
    }
    #[doc = "IrDA is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRDAENR_A::Disabled
    }
    #[doc = "IrDA is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRDAENR_A::Enabled
    }
}
#[doc = "IrDA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRDAENW_AW {
    #[doc = "0: IrDA disable"]
    Disable = 0,
    #[doc = "1: IrDA enable"]
    Enable = 1,
}
impl From<IRDAENW_AW> for bool {
    #[inline(always)]
    fn from(variant: IRDAENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAEN` writer - IrDA enable"]
pub type IRDAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRDAENW_AW>;
impl<'a, REG, const O: u8> IRDAEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IRDAENW_AW::Disable)
    }
    #[doc = "IrDA enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IRDAENW_AW::Enable)
    }
}
#[doc = "Field `IRDALP` reader - IrDA low-power mode"]
pub type IRDALP_R = crate::BitReader<IRDALPR_A>;
#[doc = "IrDA low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRDALPR_A {
    #[doc = "0: IrDA low-power mode is disabled"]
    Disabled = 0,
    #[doc = "1: IrDA low-power mode is enabled"]
    Enabled = 1,
}
impl From<IRDALPR_A> for bool {
    #[inline(always)]
    fn from(variant: IRDALPR_A) -> Self {
        variant as u8 != 0
    }
}
impl IRDALP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRDALPR_A {
        match self.bits {
            false => IRDALPR_A::Disabled,
            true => IRDALPR_A::Enabled,
        }
    }
    #[doc = "IrDA low-power mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IRDALPR_A::Disabled
    }
    #[doc = "IrDA low-power mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IRDALPR_A::Enabled
    }
}
#[doc = "IrDA low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRDALPW_AW {
    #[doc = "0: IrDA low-power mode disable"]
    Disable = 0,
    #[doc = "1: IrDA low-power mode enable"]
    Enable = 1,
}
impl From<IRDALPW_AW> for bool {
    #[inline(always)]
    fn from(variant: IRDALPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDALP` writer - IrDA low-power mode"]
pub type IRDALP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IRDALPW_AW>;
impl<'a, REG, const O: u8> IRDALP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA low-power mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IRDALPW_AW::Disable)
    }
    #[doc = "IrDA low-power mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IRDALPW_AW::Enable)
    }
}
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SLBEN_R = crate::BitReader<SLBENR_A>;
#[doc = "Single line bidirectional half-duplex enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLBENR_A {
    #[doc = "0: Single-wire bidirectional half-duplex is disabled"]
    Disabled = 0,
    #[doc = "1: Single-wire bidirectional half-duplex is enabled"]
    Enabled = 1,
}
impl From<SLBENR_A> for bool {
    #[inline(always)]
    fn from(variant: SLBENR_A) -> Self {
        variant as u8 != 0
    }
}
impl SLBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLBENR_A {
        match self.bits {
            false => SLBENR_A::Disabled,
            true => SLBENR_A::Enabled,
        }
    }
    #[doc = "Single-wire bidirectional half-duplex is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLBENR_A::Disabled
    }
    #[doc = "Single-wire bidirectional half-duplex is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLBENR_A::Enabled
    }
}
#[doc = "Single line bidirectional half-duplex enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLBENW_AW {
    #[doc = "0: Single-wire bidirectional half-duplex disable"]
    Disable = 0,
    #[doc = "1: Single-wire bidirectional half-duplex enable"]
    Enable = 1,
}
impl From<SLBENW_AW> for bool {
    #[inline(always)]
    fn from(variant: SLBENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SLBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLBENW_AW>;
impl<'a, REG, const O: u8> SLBEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-wire bidirectional half-duplex disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SLBENW_AW::Disable)
    }
    #[doc = "Single-wire bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SLBENW_AW::Enable)
    }
}
#[doc = "Field `SCNACKEN` reader - Smartcard NACK enable"]
pub type SCNACKEN_R = crate::BitReader<SCNACKENR_A>;
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCNACKENR_A {
    #[doc = "0: Smartcard NACK is disabled"]
    Disabled = 0,
    #[doc = "1: Smartcard NACK is enabled"]
    Enabled = 1,
}
impl From<SCNACKENR_A> for bool {
    #[inline(always)]
    fn from(variant: SCNACKENR_A) -> Self {
        variant as u8 != 0
    }
}
impl SCNACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCNACKENR_A {
        match self.bits {
            false => SCNACKENR_A::Disabled,
            true => SCNACKENR_A::Enabled,
        }
    }
    #[doc = "Smartcard NACK is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCNACKENR_A::Disabled
    }
    #[doc = "Smartcard NACK is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCNACKENR_A::Enabled
    }
}
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCNACKENW_AW {
    #[doc = "0: Smartcard NACK disable"]
    Disable = 0,
    #[doc = "1: Smartcard NACK enable"]
    Enable = 1,
}
impl From<SCNACKENW_AW> for bool {
    #[inline(always)]
    fn from(variant: SCNACKENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCNACKEN` writer - Smartcard NACK enable"]
pub type SCNACKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCNACKENW_AW>;
impl<'a, REG, const O: u8> SCNACKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard NACK disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SCNACKENW_AW::Disable)
    }
    #[doc = "Smartcard NACK enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SCNACKENW_AW::Enable)
    }
}
#[doc = "Field `SCMEN` reader - Smartcard mode enable"]
pub type SCMEN_R = crate::BitReader<SCMENR_A>;
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCMENR_A {
    #[doc = "0: Smartcard mode is disabled"]
    Disabled = 0,
    #[doc = "1: Smartcard mode is enabled"]
    Enabled = 1,
}
impl From<SCMENR_A> for bool {
    #[inline(always)]
    fn from(variant: SCMENR_A) -> Self {
        variant as u8 != 0
    }
}
impl SCMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCMENR_A {
        match self.bits {
            false => SCMENR_A::Disabled,
            true => SCMENR_A::Enabled,
        }
    }
    #[doc = "Smartcard mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCMENR_A::Disabled
    }
    #[doc = "Smartcard mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCMENR_A::Enabled
    }
}
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCMENW_AW {
    #[doc = "0: Smartcard mode disable"]
    Disable = 0,
    #[doc = "1: Smartcard mode enable"]
    Enable = 1,
}
impl From<SCMENW_AW> for bool {
    #[inline(always)]
    fn from(variant: SCMENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCMEN` writer - Smartcard mode enable"]
pub type SCMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCMENW_AW>;
impl<'a, REG, const O: u8> SCMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SCMENW_AW::Disable)
    }
    #[doc = "Smartcard mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SCMENW_AW::Enable)
    }
}
#[doc = "Field `DMAREN` reader - DMA receiver enable"]
pub type DMAREN_R = crate::BitReader<DMARENR_A>;
#[doc = "DMA receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARENR_A {
    #[doc = "0: DMA receiver is disabled"]
    Disabled = 0,
    #[doc = "1: DMA receiver is enabled"]
    Enabled = 1,
}
impl From<DMARENR_A> for bool {
    #[inline(always)]
    fn from(variant: DMARENR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMARENR_A {
        match self.bits {
            false => DMARENR_A::Disabled,
            true => DMARENR_A::Enabled,
        }
    }
    #[doc = "DMA receiver is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMARENR_A::Disabled
    }
    #[doc = "DMA receiver is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMARENR_A::Enabled
    }
}
#[doc = "DMA receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARENW_AW {
    #[doc = "0: DMA receiver disable"]
    Disable = 0,
    #[doc = "1: DMA receiver enable"]
    Enable = 1,
}
impl From<DMARENW_AW> for bool {
    #[inline(always)]
    fn from(variant: DMARENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` writer - DMA receiver enable"]
pub type DMAREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMARENW_AW>;
impl<'a, REG, const O: u8> DMAREN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA receiver disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DMARENW_AW::Disable)
    }
    #[doc = "DMA receiver enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DMARENW_AW::Enable)
    }
}
#[doc = "Field `DMATEN` reader - DMA transmitter enable"]
pub type DMATEN_R = crate::BitReader<DMATENR_A>;
#[doc = "DMA transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMATENR_A {
    #[doc = "0: DMA transmitter is disabled"]
    Disabled = 0,
    #[doc = "1: DMA transmitter is enabled"]
    Enabled = 1,
}
impl From<DMATENR_A> for bool {
    #[inline(always)]
    fn from(variant: DMATENR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMATENR_A {
        match self.bits {
            false => DMATENR_A::Disabled,
            true => DMATENR_A::Enabled,
        }
    }
    #[doc = "DMA transmitter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATENR_A::Disabled
    }
    #[doc = "DMA transmitter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATENR_A::Enabled
    }
}
#[doc = "DMA transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMATENW_AW {
    #[doc = "0: DMA transmitter disable"]
    Disable = 0,
    #[doc = "1: DMA transmitter enable"]
    Enable = 1,
}
impl From<DMATENW_AW> for bool {
    #[inline(always)]
    fn from(variant: DMATENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` writer - DMA transmitter enable"]
pub type DMATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMATENW_AW>;
impl<'a, REG, const O: u8> DMATEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transmitter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DMATENW_AW::Disable)
    }
    #[doc = "DMA transmitter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DMATENW_AW::Enable)
    }
}
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RTSEN_R = crate::BitReader<RTSENR_A>;
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSENR_A {
    #[doc = "0: RTS is disabled"]
    Disabled = 0,
    #[doc = "1: RTS is enabled"]
    Enabled = 1,
}
impl From<RTSENR_A> for bool {
    #[inline(always)]
    fn from(variant: RTSENR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSENR_A {
        match self.bits {
            false => RTSENR_A::Disabled,
            true => RTSENR_A::Enabled,
        }
    }
    #[doc = "RTS is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSENR_A::Disabled
    }
    #[doc = "RTS is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSENR_A::Enabled
    }
}
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSENW_AW {
    #[doc = "0: RTS disable"]
    Disable = 0,
    #[doc = "1: RTS enable"]
    Enable = 1,
}
impl From<RTSENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RTSENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTSENW_AW>;
impl<'a, REG, const O: u8> RTSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RTSENW_AW::Disable)
    }
    #[doc = "RTS enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RTSENW_AW::Enable)
    }
}
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CTSEN_R = crate::BitReader<CTSENR_A>;
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSENR_A {
    #[doc = "0: CTS is disabled"]
    Disabled = 0,
    #[doc = "1: CTS is enabled"]
    Enabled = 1,
}
impl From<CTSENR_A> for bool {
    #[inline(always)]
    fn from(variant: CTSENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSENR_A {
        match self.bits {
            false => CTSENR_A::Disabled,
            true => CTSENR_A::Enabled,
        }
    }
    #[doc = "CTS is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSENR_A::Disabled
    }
    #[doc = "CTS is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSENR_A::Enabled
    }
}
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSENW_AW {
    #[doc = "0: CTS disable"]
    Disable = 0,
    #[doc = "1: CTS enable"]
    Enable = 1,
}
impl From<CTSENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSENW_AW>;
impl<'a, REG, const O: u8> CTSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CTSENW_AW::Disable)
    }
    #[doc = "CTS enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CTSENW_AW::Enable)
    }
}
#[doc = "Field `CTSCFIEN` reader - CTSCF interrupt enable"]
pub type CTSCFIEN_R = crate::BitReader<CTSCFIENR_A>;
#[doc = "CTSCF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCFIENR_A {
    #[doc = "0: CTSCF interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: CTSCF interrupt is enabled"]
    Enabled = 1,
}
impl From<CTSCFIENR_A> for bool {
    #[inline(always)]
    fn from(variant: CTSCFIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSCFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSCFIENR_A {
        match self.bits {
            false => CTSCFIENR_A::Disabled,
            true => CTSCFIENR_A::Enabled,
        }
    }
    #[doc = "CTSCF interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSCFIENR_A::Disabled
    }
    #[doc = "CTSCF interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSCFIENR_A::Enabled
    }
}
#[doc = "CTSCF interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSCFIENW_AW {
    #[doc = "0: CTSCF interrupt disable"]
    Disable = 0,
    #[doc = "1: CTSCF interrupt enable"]
    Enable = 1,
}
impl From<CTSCFIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSCFIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCFIEN` writer - CTSCF interrupt enable"]
pub type CTSCFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSCFIENW_AW>;
impl<'a, REG, const O: u8> CTSCFIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTSCF interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CTSCFIENW_AW::Disable)
    }
    #[doc = "CTSCF interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CTSCFIENW_AW::Enable)
    }
}
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL3")
            .field("ctscfien", &format_args!("{}", self.ctscfien().bit()))
            .field("ctsen", &format_args!("{}", self.ctsen().bit()))
            .field("rtsen", &format_args!("{}", self.rtsen().bit()))
            .field("dmaten", &format_args!("{}", self.dmaten().bit()))
            .field("dmaren", &format_args!("{}", self.dmaren().bit()))
            .field("scmen", &format_args!("{}", self.scmen().bit()))
            .field("scnacken", &format_args!("{}", self.scnacken().bit()))
            .field("slben", &format_args!("{}", self.slben().bit()))
            .field("irdalp", &format_args!("{}", self.irdalp().bit()))
            .field("irdaen", &format_args!("{}", self.irdaen().bit()))
            .field("errien", &format_args!("{}", self.errien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ERRIEN_W<CTRL3_SPEC, 0> {
        ERRIEN_W::new(self)
    }
    #[doc = "Bit 1 - IrDA enable"]
    #[inline(always)]
    #[must_use]
    pub fn irdaen(&mut self) -> IRDAEN_W<CTRL3_SPEC, 1> {
        IRDAEN_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn irdalp(&mut self) -> IRDALP_W<CTRL3_SPEC, 2> {
        IRDALP_W::new(self)
    }
    #[doc = "Bit 3 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn slben(&mut self) -> SLBEN_W<CTRL3_SPEC, 3> {
        SLBEN_W::new(self)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn scnacken(&mut self) -> SCNACKEN_W<CTRL3_SPEC, 4> {
        SCNACKEN_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scmen(&mut self) -> SCMEN_W<CTRL3_SPEC, 5> {
        SCMEN_W::new(self)
    }
    #[doc = "Bit 6 - DMA receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTRL3_SPEC, 6> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 7 - DMA transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTRL3_SPEC, 7> {
        DMATEN_W::new(self)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<CTRL3_SPEC, 8> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<CTRL3_SPEC, 9> {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 10 - CTSCF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctscfien(&mut self) -> CTSCFIEN_W<CTRL3_SPEC, 10> {
        CTSCFIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL3_SPEC;
impl crate::RegisterSpec for CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl3::W`](W) writer structure"]
impl crate::Writable for CTRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for CTRL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
