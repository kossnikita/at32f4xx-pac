#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcienr {
    #[doc = "0: Transmit mailbox empty interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Transmit mailbox empty interrupt is enabled"]
    Enabled = 1,
}
impl From<Tcienr> for bool {
    #[inline(always)]
    fn from(variant: Tcienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIEN` reader - Transmission complete interrupt enable"]
pub type TCIEN_R = crate::BitReader<Tcienr>;
impl TCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcienr {
        match self.bits {
            false => Tcienr::Disabled,
            true => Tcienr::Enabled,
        }
    }
    #[doc = "Transmit mailbox empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tcienr::Disabled
    }
    #[doc = "Transmit mailbox empty interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tcienr::Enabled
    }
}
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcienwWO {
    #[doc = "0: Transmit mailbox empty interrupt disable"]
    Disable = 0,
    #[doc = "1: Transmit mailbox empty interrupt enable"]
    Enable = 1,
}
impl From<TcienwWO> for bool {
    #[inline(always)]
    fn from(variant: TcienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIEN` writer - Transmission complete interrupt enable"]
pub type TCIEN_W<'a, REG> = crate::BitWriter<'a, REG, TcienwWO>;
impl<'a, REG> TCIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit mailbox empty interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TcienwWO::Disable)
    }
    #[doc = "Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TcienwWO::Enable)
    }
}
#[doc = "FIFO %s receive message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0mienr {
    #[doc = "0: FIFO receive message interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: FIFO receive message interrupt is enabled"]
    Enabled = 1,
}
impl From<Rf0mienr> for bool {
    #[inline(always)]
    fn from(variant: Rf0mienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFMIEN(0-1)` reader - FIFO %s receive message interrupt enable"]
pub type RFMIEN_R = crate::BitReader<Rf0mienr>;
impl RFMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0mienr {
        match self.bits {
            false => Rf0mienr::Disabled,
            true => Rf0mienr::Enabled,
        }
    }
    #[doc = "FIFO receive message interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rf0mienr::Disabled
    }
    #[doc = "FIFO receive message interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rf0mienr::Enabled
    }
}
#[doc = "FIFO %s receive message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0mienwWO {
    #[doc = "0: FIFO receive message interrupt disable"]
    Disable = 0,
    #[doc = "1: FIFO receive message interrupt enable"]
    Enable = 1,
}
impl From<Rf0mienwWO> for bool {
    #[inline(always)]
    fn from(variant: Rf0mienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFMIEN(0-1)` writer - FIFO %s receive message interrupt enable"]
pub type RFMIEN_W<'a, REG> = crate::BitWriter<'a, REG, Rf0mienwWO>;
impl<'a, REG> RFMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO receive message interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0mienwWO::Disable)
    }
    #[doc = "FIFO receive message interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0mienwWO::Enable)
    }
}
#[doc = "Receive FIFO %s full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0fienr {
    #[doc = "0: Receive FIFO full interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive FIFO full interrupt is enabled"]
    Enabled = 1,
}
impl From<Rf0fienr> for bool {
    #[inline(always)]
    fn from(variant: Rf0fienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIEN(0-1)` reader - Receive FIFO %s full interrupt enable"]
pub type RFFIEN_R = crate::BitReader<Rf0fienr>;
impl RFFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0fienr {
        match self.bits {
            false => Rf0fienr::Disabled,
            true => Rf0fienr::Enabled,
        }
    }
    #[doc = "Receive FIFO full interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rf0fienr::Disabled
    }
    #[doc = "Receive FIFO full interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rf0fienr::Enabled
    }
}
#[doc = "Receive FIFO %s full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0fienwWO {
    #[doc = "0: Receive FIFO full interrupt disable"]
    Disable = 0,
    #[doc = "1: Receive FIFO full interrupt enable"]
    Enable = 1,
}
impl From<Rf0fienwWO> for bool {
    #[inline(always)]
    fn from(variant: Rf0fienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIEN(0-1)` writer - Receive FIFO %s full interrupt enable"]
pub type RFFIEN_W<'a, REG> = crate::BitWriter<'a, REG, Rf0fienwWO>;
impl<'a, REG> RFFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO full interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0fienwWO::Disable)
    }
    #[doc = "Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0fienwWO::Enable)
    }
}
#[doc = "Receive FIFO %s overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0oienr {
    #[doc = "0: Receive FIFO overflow interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive FIFO overflow interrupt is enabled"]
    Enabled = 1,
}
impl From<Rf0oienr> for bool {
    #[inline(always)]
    fn from(variant: Rf0oienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOIEN(0-1)` reader - Receive FIFO %s overflow interrupt enable"]
pub type RFOIEN_R = crate::BitReader<Rf0oienr>;
impl RFOIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf0oienr {
        match self.bits {
            false => Rf0oienr::Disabled,
            true => Rf0oienr::Enabled,
        }
    }
    #[doc = "Receive FIFO overflow interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rf0oienr::Disabled
    }
    #[doc = "Receive FIFO overflow interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rf0oienr::Enabled
    }
}
#[doc = "Receive FIFO %s overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rf0oienwWO {
    #[doc = "0: Receive FIFO overflow interrupt disable"]
    Disable = 0,
    #[doc = "1: Receive FIFO overflow interrupt enable"]
    Enable = 1,
}
impl From<Rf0oienwWO> for bool {
    #[inline(always)]
    fn from(variant: Rf0oienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOIEN(0-1)` writer - Receive FIFO %s overflow interrupt enable"]
pub type RFOIEN_W<'a, REG> = crate::BitWriter<'a, REG, Rf0oienwWO>;
impl<'a, REG> RFOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO overflow interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0oienwWO::Disable)
    }
    #[doc = "Receive FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rf0oienwWO::Enable)
    }
}
#[doc = "Error active interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eaienr {
    #[doc = "0: Error active interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error active interrupt is enabled"]
    Enabled = 1,
}
impl From<Eaienr> for bool {
    #[inline(always)]
    fn from(variant: Eaienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EAIEN` reader - Error active interrupt enable"]
pub type EAIEN_R = crate::BitReader<Eaienr>;
impl EAIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eaienr {
        match self.bits {
            false => Eaienr::Disabled,
            true => Eaienr::Enabled,
        }
    }
    #[doc = "Error active interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eaienr::Disabled
    }
    #[doc = "Error active interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eaienr::Enabled
    }
}
#[doc = "Error active interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EaienwWO {
    #[doc = "0: Error active interrupt disable"]
    Disable = 0,
    #[doc = "1: Error active interrupt enable"]
    Enable = 1,
}
impl From<EaienwWO> for bool {
    #[inline(always)]
    fn from(variant: EaienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EAIEN` writer - Error active interrupt enable"]
pub type EAIEN_W<'a, REG> = crate::BitWriter<'a, REG, EaienwWO>;
impl<'a, REG> EAIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error active interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EaienwWO::Disable)
    }
    #[doc = "Error active interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EaienwWO::Enable)
    }
}
#[doc = "Error passive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epienr {
    #[doc = "0: Error passive interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error passive interrupt is enabled"]
    Enabled = 1,
}
impl From<Epienr> for bool {
    #[inline(always)]
    fn from(variant: Epienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIEN` reader - Error passive interrupt enable"]
pub type EPIEN_R = crate::BitReader<Epienr>;
impl EPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epienr {
        match self.bits {
            false => Epienr::Disabled,
            true => Epienr::Enabled,
        }
    }
    #[doc = "Error passive interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Epienr::Disabled
    }
    #[doc = "Error passive interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Epienr::Enabled
    }
}
#[doc = "Error passive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpienwWO {
    #[doc = "0: Error passive interrupt disable"]
    Disable = 0,
    #[doc = "1: Error passive interrupt enable"]
    Enable = 1,
}
impl From<EpienwWO> for bool {
    #[inline(always)]
    fn from(variant: EpienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIEN` writer - Error passive interrupt enable"]
pub type EPIEN_W<'a, REG> = crate::BitWriter<'a, REG, EpienwWO>;
impl<'a, REG> EPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error passive interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EpienwWO::Disable)
    }
    #[doc = "Error passive interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EpienwWO::Enable)
    }
}
#[doc = "Bus-off interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boienr {
    #[doc = "0: Bus-off interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Bus-off interrupt is enabled"]
    Enabled = 1,
}
impl From<Boienr> for bool {
    #[inline(always)]
    fn from(variant: Boienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOIEN` reader - Bus-off interrupt enable"]
pub type BOIEN_R = crate::BitReader<Boienr>;
impl BOIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boienr {
        match self.bits {
            false => Boienr::Disabled,
            true => Boienr::Enabled,
        }
    }
    #[doc = "Bus-off interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Boienr::Disabled
    }
    #[doc = "Bus-off interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Boienr::Enabled
    }
}
#[doc = "Bus-off interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoienwWO {
    #[doc = "0: Bus-off interrupt disable"]
    Disable = 0,
    #[doc = "1: Bus-off interrupt enable"]
    Enable = 1,
}
impl From<BoienwWO> for bool {
    #[inline(always)]
    fn from(variant: BoienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOIEN` writer - Bus-off interrupt enable"]
pub type BOIEN_W<'a, REG> = crate::BitWriter<'a, REG, BoienwWO>;
impl<'a, REG> BOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus-off interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BoienwWO::Disable)
    }
    #[doc = "Bus-off interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BoienwWO::Enable)
    }
}
#[doc = "Error type record interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etrienr {
    #[doc = "0: Error type record interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error type record interrupt is enabled"]
    Enabled = 1,
}
impl From<Etrienr> for bool {
    #[inline(always)]
    fn from(variant: Etrienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETRIEN` reader - Error type record interrupt enable"]
pub type ETRIEN_R = crate::BitReader<Etrienr>;
impl ETRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etrienr {
        match self.bits {
            false => Etrienr::Disabled,
            true => Etrienr::Enabled,
        }
    }
    #[doc = "Error type record interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Etrienr::Disabled
    }
    #[doc = "Error type record interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Etrienr::Enabled
    }
}
#[doc = "Error type record interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EtrienwWO {
    #[doc = "0: Error type record interrupt disable"]
    Disable = 0,
    #[doc = "1: Error type record interrupt enable"]
    Enable = 1,
}
impl From<EtrienwWO> for bool {
    #[inline(always)]
    fn from(variant: EtrienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETRIEN` writer - Error type record interrupt enable"]
pub type ETRIEN_W<'a, REG> = crate::BitWriter<'a, REG, EtrienwWO>;
impl<'a, REG> ETRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error type record interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EtrienwWO::Disable)
    }
    #[doc = "Error type record interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EtrienwWO::Enable)
    }
}
#[doc = "Error occur interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoienr {
    #[doc = "0: Error occur interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error occur interrupt is enabled"]
    Enabled = 1,
}
impl From<Eoienr> for bool {
    #[inline(always)]
    fn from(variant: Eoienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIEN` reader - Error occur interrupt enable"]
pub type EOIEN_R = crate::BitReader<Eoienr>;
impl EOIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoienr {
        match self.bits {
            false => Eoienr::Disabled,
            true => Eoienr::Enabled,
        }
    }
    #[doc = "Error occur interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eoienr::Disabled
    }
    #[doc = "Error occur interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eoienr::Enabled
    }
}
#[doc = "Error occur interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EoienwWO {
    #[doc = "0: Error occur interrupt disable"]
    Disable = 0,
    #[doc = "1: Error occur interrupt enable"]
    Enable = 1,
}
impl From<EoienwWO> for bool {
    #[inline(always)]
    fn from(variant: EoienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIEN` writer - Error occur interrupt enable"]
pub type EOIEN_W<'a, REG> = crate::BitWriter<'a, REG, EoienwWO>;
impl<'a, REG> EOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error occur interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EoienwWO::Disable)
    }
    #[doc = "Error occur interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EoienwWO::Enable)
    }
}
#[doc = "Quit doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdzienr {
    #[doc = "0: Quit doze mode interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Quit doze mode interrupt is enabled"]
    Enabled = 1,
}
impl From<Qdzienr> for bool {
    #[inline(always)]
    fn from(variant: Qdzienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDZIEN` reader - Quit doze mode interrupt enable"]
pub type QDZIEN_R = crate::BitReader<Qdzienr>;
impl QDZIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdzienr {
        match self.bits {
            false => Qdzienr::Disabled,
            true => Qdzienr::Enabled,
        }
    }
    #[doc = "Quit doze mode interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Qdzienr::Disabled
    }
    #[doc = "Quit doze mode interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Qdzienr::Enabled
    }
}
#[doc = "Quit doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QdzienwWO {
    #[doc = "0: Quit doze mode interrupt disable"]
    Disable = 0,
    #[doc = "1: Quit doze mode interrupt enable"]
    Enable = 1,
}
impl From<QdzienwWO> for bool {
    #[inline(always)]
    fn from(variant: QdzienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDZIEN` writer - Quit doze mode interrupt enable"]
pub type QDZIEN_W<'a, REG> = crate::BitWriter<'a, REG, QdzienwWO>;
impl<'a, REG> QDZIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quit doze mode interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(QdzienwWO::Disable)
    }
    #[doc = "Quit doze mode interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(QdzienwWO::Enable)
    }
}
#[doc = "Enter doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edzienr {
    #[doc = "0: Enter doze mode interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Enter doze mode interrupt is enabled"]
    Enabled = 1,
}
impl From<Edzienr> for bool {
    #[inline(always)]
    fn from(variant: Edzienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDZIEN` reader - Enter doze mode interrupt enable"]
pub type EDZIEN_R = crate::BitReader<Edzienr>;
impl EDZIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edzienr {
        match self.bits {
            false => Edzienr::Disabled,
            true => Edzienr::Enabled,
        }
    }
    #[doc = "Enter doze mode interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Edzienr::Disabled
    }
    #[doc = "Enter doze mode interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Edzienr::Enabled
    }
}
#[doc = "Enter doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdzienwWO {
    #[doc = "0: Enter doze mode interrupt disable"]
    Disable = 0,
    #[doc = "1: Enter doze mode interrupt enable"]
    Enable = 1,
}
impl From<EdzienwWO> for bool {
    #[inline(always)]
    fn from(variant: EdzienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDZIEN` writer - Enter doze mode interrupt enable"]
pub type EDZIEN_W<'a, REG> = crate::BitWriter<'a, REG, EdzienwWO>;
impl<'a, REG> EDZIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enter doze mode interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EdzienwWO::Disable)
    }
    #[doc = "Enter doze mode interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EdzienwWO::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "FIFO (0-1) receive message interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RF0MIEN` field.</div>"]
    #[inline(always)]
    pub fn rfmien(&self, n: u8) -> RFMIEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RFMIEN_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "FIFO (0-1) receive message interrupt enable"]
    #[inline(always)]
    pub fn rfmien_iter(&self) -> impl Iterator<Item = RFMIEN_R> + '_ {
        (0..2).map(move |n| RFMIEN_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - FIFO 0 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf0mien(&self) -> RFMIEN_R {
        RFMIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO 1 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf1mien(&self) -> RFMIEN_R {
        RFMIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Receive FIFO (0-1) full interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RF0FIEN` field.</div>"]
    #[inline(always)]
    pub fn rffien(&self, n: u8) -> RFFIEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RFFIEN_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Receive FIFO (0-1) full interrupt enable"]
    #[inline(always)]
    pub fn rffien_iter(&self) -> impl Iterator<Item = RFFIEN_R> + '_ {
        (0..2).map(move |n| RFFIEN_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Receive FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fien(&self) -> RFFIEN_R {
        RFFIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fien(&self) -> RFFIEN_R {
        RFFIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Receive FIFO (0-1) overflow interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RF0OIEN` field.</div>"]
    #[inline(always)]
    pub fn rfoien(&self, n: u8) -> RFOIEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RFOIEN_R::new(((self.bits >> (n * 3 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Receive FIFO (0-1) overflow interrupt enable"]
    #[inline(always)]
    pub fn rfoien_iter(&self) -> impl Iterator<Item = RFOIEN_R> + '_ {
        (0..2).map(move |n| RFOIEN_R::new(((self.bits >> (n * 3 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Receive FIFO 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf0oien(&self) -> RFOIEN_R {
        RFOIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf1oien(&self) -> RFOIEN_R {
        RFOIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Error active interrupt enable"]
    #[inline(always)]
    pub fn eaien(&self) -> EAIEN_R {
        EAIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epien(&self) -> EPIEN_R {
        EPIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boien(&self) -> BOIEN_R {
        BOIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error type record interrupt enable"]
    #[inline(always)]
    pub fn etrien(&self) -> ETRIEN_R {
        ETRIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Error occur interrupt enable"]
    #[inline(always)]
    pub fn eoien(&self) -> EOIEN_R {
        EOIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Quit doze mode interrupt enable"]
    #[inline(always)]
    pub fn qdzien(&self) -> QDZIEN_R {
        QDZIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enter doze mode interrupt enable"]
    #[inline(always)]
    pub fn edzien(&self) -> EDZIEN_R {
        EDZIEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("edzien", &self.edzien())
            .field("qdzien", &self.qdzien())
            .field("eoien", &self.eoien())
            .field("etrien", &self.etrien())
            .field("boien", &self.boien())
            .field("epien", &self.epien())
            .field("eaien", &self.eaien())
            .field("rf0oien", &self.rf0oien())
            .field("rf1oien", &self.rf1oien())
            .field("rf0fien", &self.rf0fien())
            .field("rf1fien", &self.rf1fien())
            .field("rf0mien", &self.rf0mien())
            .field("rf1mien", &self.rf1mien())
            .field("tcien", &self.tcien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TCIEN_W<INTEN_SPEC> {
        TCIEN_W::new(self, 0)
    }
    #[doc = "FIFO (0-1) receive message interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RF0MIEN` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn rfmien(&mut self, n: u8) -> RFMIEN_W<INTEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RFMIEN_W::new(self, n * 3 + 1)
    }
    #[doc = "Bit 1 - FIFO 0 receive message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0mien(&mut self) -> RFMIEN_W<INTEN_SPEC> {
        RFMIEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - FIFO 1 receive message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1mien(&mut self) -> RFMIEN_W<INTEN_SPEC> {
        RFMIEN_W::new(self, 4)
    }
    #[doc = "Receive FIFO (0-1) full interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RF0FIEN` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn rffien(&mut self, n: u8) -> RFFIEN_W<INTEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RFFIEN_W::new(self, n * 3 + 2)
    }
    #[doc = "Bit 2 - Receive FIFO 0 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fien(&mut self) -> RFFIEN_W<INTEN_SPEC> {
        RFFIEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Receive FIFO 1 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fien(&mut self) -> RFFIEN_W<INTEN_SPEC> {
        RFFIEN_W::new(self, 5)
    }
    #[doc = "Receive FIFO (0-1) overflow interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RF0OIEN` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn rfoien(&mut self, n: u8) -> RFOIEN_W<INTEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RFOIEN_W::new(self, n * 3 + 3)
    }
    #[doc = "Bit 3 - Receive FIFO 0 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0oien(&mut self) -> RFOIEN_W<INTEN_SPEC> {
        RFOIEN_W::new(self, 3)
    }
    #[doc = "Bit 6 - Receive FIFO 1 overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1oien(&mut self) -> RFOIEN_W<INTEN_SPEC> {
        RFOIEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Error active interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eaien(&mut self) -> EAIEN_W<INTEN_SPEC> {
        EAIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epien(&mut self) -> EPIEN_W<INTEN_SPEC> {
        EPIEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn boien(&mut self) -> BOIEN_W<INTEN_SPEC> {
        BOIEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Error type record interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn etrien(&mut self) -> ETRIEN_W<INTEN_SPEC> {
        ETRIEN_W::new(self, 11)
    }
    #[doc = "Bit 15 - Error occur interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoien(&mut self) -> EOIEN_W<INTEN_SPEC> {
        EOIEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Quit doze mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn qdzien(&mut self) -> QDZIEN_W<INTEN_SPEC> {
        QDZIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enter doze mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn edzien(&mut self) -> EDZIEN_W<INTEN_SPEC> {
        EDZIEN_W::new(self, 17)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
