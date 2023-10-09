#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MCTRL_SPEC>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MCTRL_SPEC>;
#[doc = "Field `FZEN` reader - Freeze mode enable"]
pub type FZEN_R = crate::BitReader<FZENR_A>;
#[doc = "Freeze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FZENR_A {
    #[doc = "0: Freeze mode is disabled"]
    Disabled = 0,
    #[doc = "1: Freeze mode is enabled"]
    Enabled = 1,
}
impl From<FZENR_A> for bool {
    #[inline(always)]
    fn from(variant: FZENR_A) -> Self {
        variant as u8 != 0
    }
}
impl FZEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FZENR_A {
        match self.bits {
            false => FZENR_A::Disabled,
            true => FZENR_A::Enabled,
        }
    }
    #[doc = "Freeze mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FZENR_A::Disabled
    }
    #[doc = "Freeze mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FZENR_A::Enabled
    }
}
#[doc = "Freeze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FZENW_AW {
    #[doc = "0: Freeze mode disable"]
    Disable = 0,
    #[doc = "1: Freeze mode enable"]
    Enable = 1,
}
impl From<FZENW_AW> for bool {
    #[inline(always)]
    fn from(variant: FZENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FZEN` writer - Freeze mode enable"]
pub type FZEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FZENW_AW>;
impl<'a, REG, const O: u8> FZEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Freeze mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FZENW_AW::Disable)
    }
    #[doc = "Freeze mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FZENW_AW::Enable)
    }
}
#[doc = "Field `DZEN` reader - Doze mode enable"]
pub type DZEN_R = crate::BitReader<DZENR_A>;
#[doc = "Doze mode enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DZENR_A {
    #[doc = "0: Doze mode is disabled"]
    Disabled = 0,
    #[doc = "1: Doze mode is enabled"]
    Enabled = 1,
}
impl From<DZENR_A> for bool {
    #[inline(always)]
    fn from(variant: DZENR_A) -> Self {
        variant as u8 != 0
    }
}
impl DZEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DZENR_A {
        match self.bits {
            false => DZENR_A::Disabled,
            true => DZENR_A::Enabled,
        }
    }
    #[doc = "Doze mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DZENR_A::Disabled
    }
    #[doc = "Doze mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DZENR_A::Enabled
    }
}
#[doc = "Doze mode enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DZENW_AW {
    #[doc = "0: Doze mode disable"]
    Disable = 0,
    #[doc = "1: Doze mode enable"]
    Enable = 1,
}
impl From<DZENW_AW> for bool {
    #[inline(always)]
    fn from(variant: DZENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DZEN` writer - Doze mode enable"]
pub type DZEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DZENW_AW>;
impl<'a, REG, const O: u8> DZEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Doze mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DZENW_AW::Disable)
    }
    #[doc = "Doze mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DZENW_AW::Enable)
    }
}
#[doc = "Field `MMSSR` reader - Multiple message sending sequence rule"]
pub type MMSSR_R = crate::BitReader<MMSSR_A>;
#[doc = "Multiple message sending sequence rule\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMSSR_A {
    #[doc = "0: The message with the smallest identifier is first transmitted"]
    SmallestIdentifer = 0,
    #[doc = "1: The message with the first request order is first transmitted"]
    FirstRequestOrder = 1,
}
impl From<MMSSR_A> for bool {
    #[inline(always)]
    fn from(variant: MMSSR_A) -> Self {
        variant as u8 != 0
    }
}
impl MMSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMSSR_A {
        match self.bits {
            false => MMSSR_A::SmallestIdentifer,
            true => MMSSR_A::FirstRequestOrder,
        }
    }
    #[doc = "The message with the smallest identifier is first transmitted"]
    #[inline(always)]
    pub fn is_smallest_identifer(&self) -> bool {
        *self == MMSSR_A::SmallestIdentifer
    }
    #[doc = "The message with the first request order is first transmitted"]
    #[inline(always)]
    pub fn is_first_request_order(&self) -> bool {
        *self == MMSSR_A::FirstRequestOrder
    }
}
#[doc = "Field `MMSSR` writer - Multiple message sending sequence rule"]
pub type MMSSR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MMSSR_A>;
impl<'a, REG, const O: u8> MMSSR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The message with the smallest identifier is first transmitted"]
    #[inline(always)]
    pub fn smallest_identifer(self) -> &'a mut crate::W<REG> {
        self.variant(MMSSR_A::SmallestIdentifer)
    }
    #[doc = "The message with the first request order is first transmitted"]
    #[inline(always)]
    pub fn first_request_order(self) -> &'a mut crate::W<REG> {
        self.variant(MMSSR_A::FirstRequestOrder)
    }
}
#[doc = "Field `MDRSEL` reader - Message discarding rule select when overflow"]
pub type MDRSEL_R = crate::BitReader<MDRSEL_A>;
#[doc = "Message discarding rule select when overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDRSEL_A {
    #[doc = "0: The previous message is discarded"]
    Previous = 0,
    #[doc = "1: The new incoming message is discarded"]
    Incoming = 1,
}
impl From<MDRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MDRSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MDRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDRSEL_A {
        match self.bits {
            false => MDRSEL_A::Previous,
            true => MDRSEL_A::Incoming,
        }
    }
    #[doc = "The previous message is discarded"]
    #[inline(always)]
    pub fn is_previous(&self) -> bool {
        *self == MDRSEL_A::Previous
    }
    #[doc = "The new incoming message is discarded"]
    #[inline(always)]
    pub fn is_incoming(&self) -> bool {
        *self == MDRSEL_A::Incoming
    }
}
#[doc = "Field `MDRSEL` writer - Message discarding rule select when overflow"]
pub type MDRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MDRSEL_A>;
impl<'a, REG, const O: u8> MDRSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The previous message is discarded"]
    #[inline(always)]
    pub fn previous(self) -> &'a mut crate::W<REG> {
        self.variant(MDRSEL_A::Previous)
    }
    #[doc = "The new incoming message is discarded"]
    #[inline(always)]
    pub fn incoming(self) -> &'a mut crate::W<REG> {
        self.variant(MDRSEL_A::Incoming)
    }
}
#[doc = "Field `PRSFEN` reader - Prohibit retransmission when sending fails enable"]
pub type PRSFEN_R = crate::BitReader<PRSFENR_A>;
#[doc = "Prohibit retransmission when sending fails enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRSFENR_A {
    #[doc = "0: Prohibit retransmission is disabled when sending fails enable"]
    Disabled = 0,
    #[doc = "1: Prohibit retransmission is enabled when sending fails enable"]
    Enabled = 1,
}
impl From<PRSFENR_A> for bool {
    #[inline(always)]
    fn from(variant: PRSFENR_A) -> Self {
        variant as u8 != 0
    }
}
impl PRSFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSFENR_A {
        match self.bits {
            false => PRSFENR_A::Disabled,
            true => PRSFENR_A::Enabled,
        }
    }
    #[doc = "Prohibit retransmission is disabled when sending fails enable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRSFENR_A::Disabled
    }
    #[doc = "Prohibit retransmission is enabled when sending fails enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRSFENR_A::Enabled
    }
}
#[doc = "Prohibit retransmission when sending fails enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRSFENW_AW {
    #[doc = "0: Prohibit retransmission disable"]
    Disable = 0,
    #[doc = "1: Prohibit retransmission enable"]
    Enable = 1,
}
impl From<PRSFENW_AW> for bool {
    #[inline(always)]
    fn from(variant: PRSFENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRSFEN` writer - Prohibit retransmission when sending fails enable"]
pub type PRSFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PRSFENW_AW>;
impl<'a, REG, const O: u8> PRSFEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prohibit retransmission disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PRSFENW_AW::Disable)
    }
    #[doc = "Prohibit retransmission enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PRSFENW_AW::Enable)
    }
}
#[doc = "Field `AEDEN` reader - Automatic exit doze mode enable"]
pub type AEDEN_R = crate::BitReader<AEDENR_A>;
#[doc = "Automatic exit doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEDENR_A {
    #[doc = "0: Automatic exit doze mode is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic exit doze mode is enabled"]
    Enabled = 1,
}
impl From<AEDENR_A> for bool {
    #[inline(always)]
    fn from(variant: AEDENR_A) -> Self {
        variant as u8 != 0
    }
}
impl AEDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEDENR_A {
        match self.bits {
            false => AEDENR_A::Disabled,
            true => AEDENR_A::Enabled,
        }
    }
    #[doc = "Automatic exit doze mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AEDENR_A::Disabled
    }
    #[doc = "Automatic exit doze mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AEDENR_A::Enabled
    }
}
#[doc = "Automatic exit doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEDENW_AW {
    #[doc = "0: Automatic exit doze mode disable"]
    Disable = 0,
    #[doc = "1: Automatic exit doze mode enable"]
    Enable = 1,
}
impl From<AEDENW_AW> for bool {
    #[inline(always)]
    fn from(variant: AEDENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEDEN` writer - Automatic exit doze mode enable"]
pub type AEDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AEDENW_AW>;
impl<'a, REG, const O: u8> AEDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic exit doze mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AEDENW_AW::Disable)
    }
    #[doc = "Automatic exit doze mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AEDENW_AW::Enable)
    }
}
#[doc = "Field `AEBOEN` reader - Automatic exit bus-off enable"]
pub type AEBOEN_R = crate::BitReader<AEBOENR_A>;
#[doc = "Automatic exit bus-off enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEBOENR_A {
    #[doc = "0: Automatic exit bus-off is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic exit bus-off is enabled"]
    Enabled = 1,
}
impl From<AEBOENR_A> for bool {
    #[inline(always)]
    fn from(variant: AEBOENR_A) -> Self {
        variant as u8 != 0
    }
}
impl AEBOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEBOENR_A {
        match self.bits {
            false => AEBOENR_A::Disabled,
            true => AEBOENR_A::Enabled,
        }
    }
    #[doc = "Automatic exit bus-off is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AEBOENR_A::Disabled
    }
    #[doc = "Automatic exit bus-off is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AEBOENR_A::Enabled
    }
}
#[doc = "Automatic exit bus-off enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEBOENW_AW {
    #[doc = "0: Automatic exit bus-off disable"]
    Disable = 0,
    #[doc = "1: Automatic exit bus-off enable"]
    Enable = 1,
}
impl From<AEBOENW_AW> for bool {
    #[inline(always)]
    fn from(variant: AEBOENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEBOEN` writer - Automatic exit bus-off enable"]
pub type AEBOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AEBOENW_AW>;
impl<'a, REG, const O: u8> AEBOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic exit bus-off disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AEBOENW_AW::Disable)
    }
    #[doc = "Automatic exit bus-off enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AEBOENW_AW::Enable)
    }
}
#[doc = "Field `TTCEN` reader - Time triggered communication mode enable"]
pub type TTCEN_R = crate::BitReader<TTCENR_A>;
#[doc = "Time triggered communication mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTCENR_A {
    #[doc = "0: Time triggered communication mode is disabled"]
    Disabled = 0,
    #[doc = "1: Time triggered communication mode is enabled"]
    Enabled = 1,
}
impl From<TTCENR_A> for bool {
    #[inline(always)]
    fn from(variant: TTCENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTCENR_A {
        match self.bits {
            false => TTCENR_A::Disabled,
            true => TTCENR_A::Enabled,
        }
    }
    #[doc = "Time triggered communication mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TTCENR_A::Disabled
    }
    #[doc = "Time triggered communication mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TTCENR_A::Enabled
    }
}
#[doc = "Time triggered communication mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTCENW_AW {
    #[doc = "0: Time triggered communication mode disable"]
    Disable = 0,
    #[doc = "1: Time triggered communication mode enable"]
    Enable = 1,
}
impl From<TTCENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TTCENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTCEN` writer - Time triggered communication mode enable"]
pub type TTCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TTCENW_AW>;
impl<'a, REG, const O: u8> TTCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time triggered communication mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TTCENW_AW::Disable)
    }
    #[doc = "Time triggered communication mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TTCENW_AW::Enable)
    }
}
#[doc = "Field `SPRST` reader - Software partial reset"]
pub type SPRST_R = crate::BitReader<SPRSTR_A>;
#[doc = "Software partial reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRSTR_A {
    #[doc = "0: Normal"]
    Normal = 0,
    #[doc = "1: Software partial reset"]
    Reset = 1,
}
impl From<SPRSTR_A> for bool {
    #[inline(always)]
    fn from(variant: SPRSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRSTR_A {
        match self.bits {
            false => SPRSTR_A::Normal,
            true => SPRSTR_A::Reset,
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SPRSTR_A::Normal
    }
    #[doc = "Software partial reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SPRSTR_A::Reset
    }
}
#[doc = "Software partial reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPRSTW_AW {
    #[doc = "1: Software partial reset"]
    Reset = 1,
}
impl From<SPRSTW_AW> for bool {
    #[inline(always)]
    fn from(variant: SPRSTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRST` writer - Software partial reset"]
pub type SPRST_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, SPRSTW_AW>;
impl<'a, REG, const O: u8> SPRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software partial reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SPRSTW_AW::Reset)
    }
}
#[doc = "Field `PTD` reader - Prohibit transmission when debug"]
pub type PTD_R = crate::BitReader<PTD_A>;
#[doc = "Prohibit transmission when debug\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTD_A {
    #[doc = "0: Transmission works during debug"]
    Work = 0,
    #[doc = "1: Transmission is prohibited during debug. Receive FIFO can be still accessible normally"]
    Prohibited = 1,
}
impl From<PTD_A> for bool {
    #[inline(always)]
    fn from(variant: PTD_A) -> Self {
        variant as u8 != 0
    }
}
impl PTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTD_A {
        match self.bits {
            false => PTD_A::Work,
            true => PTD_A::Prohibited,
        }
    }
    #[doc = "Transmission works during debug"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == PTD_A::Work
    }
    #[doc = "Transmission is prohibited during debug. Receive FIFO can be still accessible normally"]
    #[inline(always)]
    pub fn is_prohibited(&self) -> bool {
        *self == PTD_A::Prohibited
    }
}
#[doc = "Field `PTD` writer - Prohibit transmission when debug"]
pub type PTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PTD_A>;
impl<'a, REG, const O: u8> PTD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission works during debug"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(PTD_A::Work)
    }
    #[doc = "Transmission is prohibited during debug. Receive FIFO can be still accessible normally"]
    #[inline(always)]
    pub fn prohibited(self) -> &'a mut crate::W<REG> {
        self.variant(PTD_A::Prohibited)
    }
}
impl R {
    #[doc = "Bit 0 - Freeze mode enable"]
    #[inline(always)]
    pub fn fzen(&self) -> FZEN_R {
        FZEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Doze mode enable"]
    #[inline(always)]
    pub fn dzen(&self) -> DZEN_R {
        DZEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiple message sending sequence rule"]
    #[inline(always)]
    pub fn mmssr(&self) -> MMSSR_R {
        MMSSR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message discarding rule select when overflow"]
    #[inline(always)]
    pub fn mdrsel(&self) -> MDRSEL_R {
        MDRSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Prohibit retransmission when sending fails enable"]
    #[inline(always)]
    pub fn prsfen(&self) -> PRSFEN_R {
        PRSFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic exit doze mode enable"]
    #[inline(always)]
    pub fn aeden(&self) -> AEDEN_R {
        AEDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic exit bus-off enable"]
    #[inline(always)]
    pub fn aeboen(&self) -> AEBOEN_R {
        AEBOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time triggered communication mode enable"]
    #[inline(always)]
    pub fn ttcen(&self) -> TTCEN_R {
        TTCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Software partial reset"]
    #[inline(always)]
    pub fn sprst(&self) -> SPRST_R {
        SPRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Prohibit transmission when debug"]
    #[inline(always)]
    pub fn ptd(&self) -> PTD_R {
        PTD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTRL")
            .field("ptd", &format_args!("{}", self.ptd().bit()))
            .field("sprst", &format_args!("{}", self.sprst().bit()))
            .field("ttcen", &format_args!("{}", self.ttcen().bit()))
            .field("aeboen", &format_args!("{}", self.aeboen().bit()))
            .field("aeden", &format_args!("{}", self.aeden().bit()))
            .field("prsfen", &format_args!("{}", self.prsfen().bit()))
            .field("mdrsel", &format_args!("{}", self.mdrsel().bit()))
            .field("mmssr", &format_args!("{}", self.mmssr().bit()))
            .field("dzen", &format_args!("{}", self.dzen().bit()))
            .field("fzen", &format_args!("{}", self.fzen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn fzen(&mut self) -> FZEN_W<MCTRL_SPEC, 0> {
        FZEN_W::new(self)
    }
    #[doc = "Bit 1 - Doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dzen(&mut self) -> DZEN_W<MCTRL_SPEC, 1> {
        DZEN_W::new(self)
    }
    #[doc = "Bit 2 - Multiple message sending sequence rule"]
    #[inline(always)]
    #[must_use]
    pub fn mmssr(&mut self) -> MMSSR_W<MCTRL_SPEC, 2> {
        MMSSR_W::new(self)
    }
    #[doc = "Bit 3 - Message discarding rule select when overflow"]
    #[inline(always)]
    #[must_use]
    pub fn mdrsel(&mut self) -> MDRSEL_W<MCTRL_SPEC, 3> {
        MDRSEL_W::new(self)
    }
    #[doc = "Bit 4 - Prohibit retransmission when sending fails enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsfen(&mut self) -> PRSFEN_W<MCTRL_SPEC, 4> {
        PRSFEN_W::new(self)
    }
    #[doc = "Bit 5 - Automatic exit doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeden(&mut self) -> AEDEN_W<MCTRL_SPEC, 5> {
        AEDEN_W::new(self)
    }
    #[doc = "Bit 6 - Automatic exit bus-off enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeboen(&mut self) -> AEBOEN_W<MCTRL_SPEC, 6> {
        AEBOEN_W::new(self)
    }
    #[doc = "Bit 7 - Time triggered communication mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ttcen(&mut self) -> TTCEN_W<MCTRL_SPEC, 7> {
        TTCEN_W::new(self)
    }
    #[doc = "Bit 15 - Software partial reset"]
    #[inline(always)]
    #[must_use]
    pub fn sprst(&mut self) -> SPRST_W<MCTRL_SPEC, 15> {
        SPRST_W::new(self)
    }
    #[doc = "Bit 16 - Prohibit transmission when debug"]
    #[inline(always)]
    #[must_use]
    pub fn ptd(&mut self) -> PTD_W<MCTRL_SPEC, 16> {
        PTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Main control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8000;
}
#[doc = "`reset()` method sets MCTRL to value 0x0001_0002"]
impl crate::Resettable for MCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0002;
}
