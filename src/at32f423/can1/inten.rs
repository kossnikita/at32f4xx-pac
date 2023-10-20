#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `TCIEN` reader - Transmission complete interrupt enable"]
pub type TCIEN_R = crate::BitReader<TCIENR_A>;
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIENR_A {
    #[doc = "0: Transmit mailbox empty interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Transmit mailbox empty interrupt is enabled"]
    Enabled = 1,
}
impl From<TCIENR_A> for bool {
    #[inline(always)]
    fn from(variant: TCIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIENR_A {
        match self.bits {
            false => TCIENR_A::Disabled,
            true => TCIENR_A::Enabled,
        }
    }
    #[doc = "Transmit mailbox empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIENR_A::Disabled
    }
    #[doc = "Transmit mailbox empty interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIENR_A::Enabled
    }
}
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIENW_AW {
    #[doc = "0: Transmit mailbox empty interrupt disable"]
    Disable = 0,
    #[doc = "1: Transmit mailbox empty interrupt enable"]
    Enable = 1,
}
impl From<TCIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TCIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIEN` writer - Transmission complete interrupt enable"]
pub type TCIEN_W<'a, REG> = crate::BitWriter<'a, REG, TCIENW_AW>;
impl<'a, REG> TCIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit mailbox empty interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TCIENW_AW::Disable)
    }
    #[doc = "Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TCIENW_AW::Enable)
    }
}
#[doc = "Field `RFMIEN[0-1]` reader - FIFO %s receive message interrupt enable"]
pub type RFMIEN_R = crate::BitReader<RF0MIENR_A>;
#[doc = "FIFO %s receive message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0MIENR_A {
    #[doc = "0: FIFO receive message interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: FIFO receive message interrupt is enabled"]
    Enabled = 1,
}
impl From<RF0MIENR_A> for bool {
    #[inline(always)]
    fn from(variant: RF0MIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl RFMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0MIENR_A {
        match self.bits {
            false => RF0MIENR_A::Disabled,
            true => RF0MIENR_A::Enabled,
        }
    }
    #[doc = "FIFO receive message interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RF0MIENR_A::Disabled
    }
    #[doc = "FIFO receive message interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RF0MIENR_A::Enabled
    }
}
#[doc = "FIFO %s receive message interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0MIENW_AW {
    #[doc = "0: FIFO receive message interrupt disable"]
    Disable = 0,
    #[doc = "1: FIFO receive message interrupt enable"]
    Enable = 1,
}
impl From<RF0MIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RF0MIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFMIEN[0-1]` writer - FIFO %s receive message interrupt enable"]
pub type RFMIEN_W<'a, REG> = crate::BitWriter<'a, REG, RF0MIENW_AW>;
impl<'a, REG> RFMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO receive message interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RF0MIENW_AW::Disable)
    }
    #[doc = "FIFO receive message interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RF0MIENW_AW::Enable)
    }
}
#[doc = "Field `RFFIEN[0-1]` reader - Receive FIFO %s full interrupt enable"]
pub type RFFIEN_R = crate::BitReader<RF0FIENR_A>;
#[doc = "Receive FIFO %s full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0FIENR_A {
    #[doc = "0: Receive FIFO full interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive FIFO full interrupt is enabled"]
    Enabled = 1,
}
impl From<RF0FIENR_A> for bool {
    #[inline(always)]
    fn from(variant: RF0FIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl RFFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0FIENR_A {
        match self.bits {
            false => RF0FIENR_A::Disabled,
            true => RF0FIENR_A::Enabled,
        }
    }
    #[doc = "Receive FIFO full interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RF0FIENR_A::Disabled
    }
    #[doc = "Receive FIFO full interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RF0FIENR_A::Enabled
    }
}
#[doc = "Receive FIFO %s full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0FIENW_AW {
    #[doc = "0: Receive FIFO full interrupt disable"]
    Disable = 0,
    #[doc = "1: Receive FIFO full interrupt enable"]
    Enable = 1,
}
impl From<RF0FIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RF0FIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFFIEN[0-1]` writer - Receive FIFO %s full interrupt enable"]
pub type RFFIEN_W<'a, REG> = crate::BitWriter<'a, REG, RF0FIENW_AW>;
impl<'a, REG> RFFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO full interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RF0FIENW_AW::Disable)
    }
    #[doc = "Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RF0FIENW_AW::Enable)
    }
}
#[doc = "Field `RFOIEN[0-1]` reader - Receive FIFO %s overflow interrupt enable"]
pub type RFOIEN_R = crate::BitReader<RF0OIENR_A>;
#[doc = "Receive FIFO %s overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0OIENR_A {
    #[doc = "0: Receive FIFO overflow interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive FIFO overflow interrupt is enabled"]
    Enabled = 1,
}
impl From<RF0OIENR_A> for bool {
    #[inline(always)]
    fn from(variant: RF0OIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RF0OIENR_A {
        match self.bits {
            false => RF0OIENR_A::Disabled,
            true => RF0OIENR_A::Enabled,
        }
    }
    #[doc = "Receive FIFO overflow interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RF0OIENR_A::Disabled
    }
    #[doc = "Receive FIFO overflow interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RF0OIENR_A::Enabled
    }
}
#[doc = "Receive FIFO %s overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0OIENW_AW {
    #[doc = "0: Receive FIFO overflow interrupt disable"]
    Disable = 0,
    #[doc = "1: Receive FIFO overflow interrupt enable"]
    Enable = 1,
}
impl From<RF0OIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RF0OIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOIEN[0-1]` writer - Receive FIFO %s overflow interrupt enable"]
pub type RFOIEN_W<'a, REG> = crate::BitWriter<'a, REG, RF0OIENW_AW>;
impl<'a, REG> RFOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive FIFO overflow interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RF0OIENW_AW::Disable)
    }
    #[doc = "Receive FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RF0OIENW_AW::Enable)
    }
}
#[doc = "Field `EAIEN` reader - Error active interrupt enable"]
pub type EAIEN_R = crate::BitReader<EAIENR_A>;
#[doc = "Error active interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EAIENR_A {
    #[doc = "0: Error active interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error active interrupt is enabled"]
    Enabled = 1,
}
impl From<EAIENR_A> for bool {
    #[inline(always)]
    fn from(variant: EAIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl EAIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EAIENR_A {
        match self.bits {
            false => EAIENR_A::Disabled,
            true => EAIENR_A::Enabled,
        }
    }
    #[doc = "Error active interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EAIENR_A::Disabled
    }
    #[doc = "Error active interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EAIENR_A::Enabled
    }
}
#[doc = "Error active interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EAIENW_AW {
    #[doc = "0: Error active interrupt disable"]
    Disable = 0,
    #[doc = "1: Error active interrupt enable"]
    Enable = 1,
}
impl From<EAIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: EAIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EAIEN` writer - Error active interrupt enable"]
pub type EAIEN_W<'a, REG> = crate::BitWriter<'a, REG, EAIENW_AW>;
impl<'a, REG> EAIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error active interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EAIENW_AW::Disable)
    }
    #[doc = "Error active interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EAIENW_AW::Enable)
    }
}
#[doc = "Field `EPIEN` reader - Error passive interrupt enable"]
pub type EPIEN_R = crate::BitReader<EPIENR_A>;
#[doc = "Error passive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIENR_A {
    #[doc = "0: Error passive interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error passive interrupt is enabled"]
    Enabled = 1,
}
impl From<EPIENR_A> for bool {
    #[inline(always)]
    fn from(variant: EPIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPIENR_A {
        match self.bits {
            false => EPIENR_A::Disabled,
            true => EPIENR_A::Enabled,
        }
    }
    #[doc = "Error passive interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPIENR_A::Disabled
    }
    #[doc = "Error passive interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EPIENR_A::Enabled
    }
}
#[doc = "Error passive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIENW_AW {
    #[doc = "0: Error passive interrupt disable"]
    Disable = 0,
    #[doc = "1: Error passive interrupt enable"]
    Enable = 1,
}
impl From<EPIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: EPIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIEN` writer - Error passive interrupt enable"]
pub type EPIEN_W<'a, REG> = crate::BitWriter<'a, REG, EPIENW_AW>;
impl<'a, REG> EPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error passive interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EPIENW_AW::Disable)
    }
    #[doc = "Error passive interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EPIENW_AW::Enable)
    }
}
#[doc = "Field `BOIEN` reader - Bus-off interrupt enable"]
pub type BOIEN_R = crate::BitReader<BOIENR_A>;
#[doc = "Bus-off interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOIENR_A {
    #[doc = "0: Bus-off interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Bus-off interrupt is enabled"]
    Enabled = 1,
}
impl From<BOIENR_A> for bool {
    #[inline(always)]
    fn from(variant: BOIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl BOIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOIENR_A {
        match self.bits {
            false => BOIENR_A::Disabled,
            true => BOIENR_A::Enabled,
        }
    }
    #[doc = "Bus-off interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOIENR_A::Disabled
    }
    #[doc = "Bus-off interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOIENR_A::Enabled
    }
}
#[doc = "Bus-off interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOIENW_AW {
    #[doc = "0: Bus-off interrupt disable"]
    Disable = 0,
    #[doc = "1: Bus-off interrupt enable"]
    Enable = 1,
}
impl From<BOIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: BOIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOIEN` writer - Bus-off interrupt enable"]
pub type BOIEN_W<'a, REG> = crate::BitWriter<'a, REG, BOIENW_AW>;
impl<'a, REG> BOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus-off interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BOIENW_AW::Disable)
    }
    #[doc = "Bus-off interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BOIENW_AW::Enable)
    }
}
#[doc = "Field `ETRIEN` reader - Error type record interrupt enable"]
pub type ETRIEN_R = crate::BitReader<ETRIENR_A>;
#[doc = "Error type record interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETRIENR_A {
    #[doc = "0: Error type record interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error type record interrupt is enabled"]
    Enabled = 1,
}
impl From<ETRIENR_A> for bool {
    #[inline(always)]
    fn from(variant: ETRIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl ETRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETRIENR_A {
        match self.bits {
            false => ETRIENR_A::Disabled,
            true => ETRIENR_A::Enabled,
        }
    }
    #[doc = "Error type record interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ETRIENR_A::Disabled
    }
    #[doc = "Error type record interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ETRIENR_A::Enabled
    }
}
#[doc = "Error type record interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETRIENW_AW {
    #[doc = "0: Error type record interrupt disable"]
    Disable = 0,
    #[doc = "1: Error type record interrupt enable"]
    Enable = 1,
}
impl From<ETRIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: ETRIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETRIEN` writer - Error type record interrupt enable"]
pub type ETRIEN_W<'a, REG> = crate::BitWriter<'a, REG, ETRIENW_AW>;
impl<'a, REG> ETRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error type record interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ETRIENW_AW::Disable)
    }
    #[doc = "Error type record interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ETRIENW_AW::Enable)
    }
}
#[doc = "Field `EOIEN` reader - Error occur interrupt enable"]
pub type EOIEN_R = crate::BitReader<EOIENR_A>;
#[doc = "Error occur interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOIENR_A {
    #[doc = "0: Error occur interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error occur interrupt is enabled"]
    Enabled = 1,
}
impl From<EOIENR_A> for bool {
    #[inline(always)]
    fn from(variant: EOIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOIENR_A {
        match self.bits {
            false => EOIENR_A::Disabled,
            true => EOIENR_A::Enabled,
        }
    }
    #[doc = "Error occur interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOIENR_A::Disabled
    }
    #[doc = "Error occur interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOIENR_A::Enabled
    }
}
#[doc = "Error occur interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOIENW_AW {
    #[doc = "0: Error occur interrupt disable"]
    Disable = 0,
    #[doc = "1: Error occur interrupt enable"]
    Enable = 1,
}
impl From<EOIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOIEN` writer - Error occur interrupt enable"]
pub type EOIEN_W<'a, REG> = crate::BitWriter<'a, REG, EOIENW_AW>;
impl<'a, REG> EOIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error occur interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EOIENW_AW::Disable)
    }
    #[doc = "Error occur interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EOIENW_AW::Enable)
    }
}
#[doc = "Field `QDZIEN` reader - Quit doze mode interrupt enable"]
pub type QDZIEN_R = crate::BitReader<QDZIENR_A>;
#[doc = "Quit doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QDZIENR_A {
    #[doc = "0: Quit doze mode interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Quit doze mode interrupt is enabled"]
    Enabled = 1,
}
impl From<QDZIENR_A> for bool {
    #[inline(always)]
    fn from(variant: QDZIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl QDZIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QDZIENR_A {
        match self.bits {
            false => QDZIENR_A::Disabled,
            true => QDZIENR_A::Enabled,
        }
    }
    #[doc = "Quit doze mode interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == QDZIENR_A::Disabled
    }
    #[doc = "Quit doze mode interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == QDZIENR_A::Enabled
    }
}
#[doc = "Quit doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QDZIENW_AW {
    #[doc = "0: Quit doze mode interrupt disable"]
    Disable = 0,
    #[doc = "1: Quit doze mode interrupt enable"]
    Enable = 1,
}
impl From<QDZIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: QDZIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDZIEN` writer - Quit doze mode interrupt enable"]
pub type QDZIEN_W<'a, REG> = crate::BitWriter<'a, REG, QDZIENW_AW>;
impl<'a, REG> QDZIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quit doze mode interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(QDZIENW_AW::Disable)
    }
    #[doc = "Quit doze mode interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(QDZIENW_AW::Enable)
    }
}
#[doc = "Field `EDZIEN` reader - Enter doze mode interrupt enable"]
pub type EDZIEN_R = crate::BitReader<EDZIENR_A>;
#[doc = "Enter doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDZIENR_A {
    #[doc = "0: Enter doze mode interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Enter doze mode interrupt is enabled"]
    Enabled = 1,
}
impl From<EDZIENR_A> for bool {
    #[inline(always)]
    fn from(variant: EDZIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl EDZIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDZIENR_A {
        match self.bits {
            false => EDZIENR_A::Disabled,
            true => EDZIENR_A::Enabled,
        }
    }
    #[doc = "Enter doze mode interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDZIENR_A::Disabled
    }
    #[doc = "Enter doze mode interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDZIENR_A::Enabled
    }
}
#[doc = "Enter doze mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDZIENW_AW {
    #[doc = "0: Enter doze mode interrupt disable"]
    Disable = 0,
    #[doc = "1: Enter doze mode interrupt enable"]
    Enable = 1,
}
impl From<EDZIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: EDZIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDZIEN` writer - Enter doze mode interrupt enable"]
pub type EDZIEN_W<'a, REG> = crate::BitWriter<'a, REG, EDZIENW_AW>;
impl<'a, REG> EDZIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enter doze mode interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EDZIENW_AW::Disable)
    }
    #[doc = "Enter doze mode interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EDZIENW_AW::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "FIFO [0-1]
receive message interrupt enable\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn rfmien(&self, n: u8) -> RFMIEN_R {
        assert!(n < 2);
        RFMIEN_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
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
    #[doc = "Receive FIFO [0-1]
full interrupt enable\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn rffien(&self, n: u8) -> RFFIEN_R {
        assert!(n < 2);
        RFFIEN_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
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
    #[doc = "Receive FIFO [0-1]
overflow interrupt enable\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn rfoien(&self, n: u8) -> RFOIEN_R {
        assert!(n < 2);
        RFOIEN_R::new(((self.bits >> (n * 3 + 3)) & 1) != 0)
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
            .field("edzien", &format_args!("{}", self.edzien().bit()))
            .field("qdzien", &format_args!("{}", self.qdzien().bit()))
            .field("eoien", &format_args!("{}", self.eoien().bit()))
            .field("etrien", &format_args!("{}", self.etrien().bit()))
            .field("boien", &format_args!("{}", self.boien().bit()))
            .field("epien", &format_args!("{}", self.epien().bit()))
            .field("eaien", &format_args!("{}", self.eaien().bit()))
            .field("rf0oien", &format_args!("{}", self.rf0oien().bit()))
            .field("rf1oien", &format_args!("{}", self.rf1oien().bit()))
            .field("rf0fien", &format_args!("{}", self.rf0fien().bit()))
            .field("rf1fien", &format_args!("{}", self.rf1fien().bit()))
            .field("rf0mien", &format_args!("{}", self.rf0mien().bit()))
            .field("rf1mien", &format_args!("{}", self.rf1mien().bit()))
            .field("tcien", &format_args!("{}", self.tcien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TCIEN_W<INTEN_SPEC> {
        TCIEN_W::new(self, 0)
    }
    #[doc = "FIFO [0-1]
receive message interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfmien(&mut self, n: u8) -> RFMIEN_W<INTEN_SPEC> {
        assert!(n < 2);
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
    #[doc = "Receive FIFO [0-1]
full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffien(&mut self, n: u8) -> RFFIEN_W<INTEN_SPEC> {
        assert!(n < 2);
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
    #[doc = "Receive FIFO [0-1]
overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfoien(&mut self, n: u8) -> RFOIEN_W<INTEN_SPEC> {
        assert!(n < 2);
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
