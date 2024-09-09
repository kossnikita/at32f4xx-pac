#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MCTRL_SPEC>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MCTRL_SPEC>;
#[doc = "Freeze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fzenr {
    #[doc = "0: Freeze mode is disabled"]
    Disabled = 0,
    #[doc = "1: Freeze mode is enabled"]
    Enabled = 1,
}
impl From<Fzenr> for bool {
    #[inline(always)]
    fn from(variant: Fzenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FZEN` reader - Freeze mode enable"]
pub type FZEN_R = crate::BitReader<Fzenr>;
impl FZEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fzenr {
        match self.bits {
            false => Fzenr::Disabled,
            true => Fzenr::Enabled,
        }
    }
    #[doc = "Freeze mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fzenr::Disabled
    }
    #[doc = "Freeze mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fzenr::Enabled
    }
}
#[doc = "Freeze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FzenwWO {
    #[doc = "0: Freeze mode disable"]
    Disable = 0,
    #[doc = "1: Freeze mode enable"]
    Enable = 1,
}
impl From<FzenwWO> for bool {
    #[inline(always)]
    fn from(variant: FzenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FZEN` writer - Freeze mode enable"]
pub type FZEN_W<'a, REG> = crate::BitWriter<'a, REG, FzenwWO>;
impl<'a, REG> FZEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Freeze mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FzenwWO::Disable)
    }
    #[doc = "Freeze mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FzenwWO::Enable)
    }
}
#[doc = "Doze mode enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dzenr {
    #[doc = "0: Doze mode is disabled"]
    Disabled = 0,
    #[doc = "1: Doze mode is enabled"]
    Enabled = 1,
}
impl From<Dzenr> for bool {
    #[inline(always)]
    fn from(variant: Dzenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DZEN` reader - Doze mode enable"]
pub type DZEN_R = crate::BitReader<Dzenr>;
impl DZEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dzenr {
        match self.bits {
            false => Dzenr::Disabled,
            true => Dzenr::Enabled,
        }
    }
    #[doc = "Doze mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dzenr::Disabled
    }
    #[doc = "Doze mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dzenr::Enabled
    }
}
#[doc = "Doze mode enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DzenwWO {
    #[doc = "0: Doze mode disable"]
    Disable = 0,
    #[doc = "1: Doze mode enable"]
    Enable = 1,
}
impl From<DzenwWO> for bool {
    #[inline(always)]
    fn from(variant: DzenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DZEN` writer - Doze mode enable"]
pub type DZEN_W<'a, REG> = crate::BitWriter<'a, REG, DzenwWO>;
impl<'a, REG> DZEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Doze mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DzenwWO::Disable)
    }
    #[doc = "Doze mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DzenwWO::Enable)
    }
}
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
#[doc = "Field `MMSSR` reader - Multiple message sending sequence rule"]
pub type MMSSR_R = crate::BitReader<MMSSR_A>;
impl MMSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMSSR_A {
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
pub type MMSSR_W<'a, REG> = crate::BitWriter<'a, REG, MMSSR_A>;
impl<'a, REG> MMSSR_W<'a, REG>
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
#[doc = "Field `MDRSEL` reader - Message discarding rule select when overflow"]
pub type MDRSEL_R = crate::BitReader<MDRSEL_A>;
impl MDRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDRSEL_A {
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
pub type MDRSEL_W<'a, REG> = crate::BitWriter<'a, REG, MDRSEL_A>;
impl<'a, REG> MDRSEL_W<'a, REG>
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
#[doc = "Prohibit retransmission when sending fails enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prsfenr {
    #[doc = "0: Prohibit retransmission is disabled when sending fails enable"]
    Disabled = 0,
    #[doc = "1: Prohibit retransmission is enabled when sending fails enable"]
    Enabled = 1,
}
impl From<Prsfenr> for bool {
    #[inline(always)]
    fn from(variant: Prsfenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRSFEN` reader - Prohibit retransmission when sending fails enable"]
pub type PRSFEN_R = crate::BitReader<Prsfenr>;
impl PRSFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prsfenr {
        match self.bits {
            false => Prsfenr::Disabled,
            true => Prsfenr::Enabled,
        }
    }
    #[doc = "Prohibit retransmission is disabled when sending fails enable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Prsfenr::Disabled
    }
    #[doc = "Prohibit retransmission is enabled when sending fails enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Prsfenr::Enabled
    }
}
#[doc = "Prohibit retransmission when sending fails enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrsfenwWO {
    #[doc = "0: Prohibit retransmission disable"]
    Disable = 0,
    #[doc = "1: Prohibit retransmission enable"]
    Enable = 1,
}
impl From<PrsfenwWO> for bool {
    #[inline(always)]
    fn from(variant: PrsfenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRSFEN` writer - Prohibit retransmission when sending fails enable"]
pub type PRSFEN_W<'a, REG> = crate::BitWriter<'a, REG, PrsfenwWO>;
impl<'a, REG> PRSFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prohibit retransmission disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PrsfenwWO::Disable)
    }
    #[doc = "Prohibit retransmission enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PrsfenwWO::Enable)
    }
}
#[doc = "Automatic exit doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aedenr {
    #[doc = "0: Automatic exit doze mode is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic exit doze mode is enabled"]
    Enabled = 1,
}
impl From<Aedenr> for bool {
    #[inline(always)]
    fn from(variant: Aedenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEDEN` reader - Automatic exit doze mode enable"]
pub type AEDEN_R = crate::BitReader<Aedenr>;
impl AEDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aedenr {
        match self.bits {
            false => Aedenr::Disabled,
            true => Aedenr::Enabled,
        }
    }
    #[doc = "Automatic exit doze mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Aedenr::Disabled
    }
    #[doc = "Automatic exit doze mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Aedenr::Enabled
    }
}
#[doc = "Automatic exit doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AedenwWO {
    #[doc = "0: Automatic exit doze mode disable"]
    Disable = 0,
    #[doc = "1: Automatic exit doze mode enable"]
    Enable = 1,
}
impl From<AedenwWO> for bool {
    #[inline(always)]
    fn from(variant: AedenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEDEN` writer - Automatic exit doze mode enable"]
pub type AEDEN_W<'a, REG> = crate::BitWriter<'a, REG, AedenwWO>;
impl<'a, REG> AEDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic exit doze mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AedenwWO::Disable)
    }
    #[doc = "Automatic exit doze mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AedenwWO::Enable)
    }
}
#[doc = "Automatic exit bus-off enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aeboenr {
    #[doc = "0: Automatic exit bus-off is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic exit bus-off is enabled"]
    Enabled = 1,
}
impl From<Aeboenr> for bool {
    #[inline(always)]
    fn from(variant: Aeboenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEBOEN` reader - Automatic exit bus-off enable"]
pub type AEBOEN_R = crate::BitReader<Aeboenr>;
impl AEBOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aeboenr {
        match self.bits {
            false => Aeboenr::Disabled,
            true => Aeboenr::Enabled,
        }
    }
    #[doc = "Automatic exit bus-off is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Aeboenr::Disabled
    }
    #[doc = "Automatic exit bus-off is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Aeboenr::Enabled
    }
}
#[doc = "Automatic exit bus-off enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AeboenwWO {
    #[doc = "0: Automatic exit bus-off disable"]
    Disable = 0,
    #[doc = "1: Automatic exit bus-off enable"]
    Enable = 1,
}
impl From<AeboenwWO> for bool {
    #[inline(always)]
    fn from(variant: AeboenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEBOEN` writer - Automatic exit bus-off enable"]
pub type AEBOEN_W<'a, REG> = crate::BitWriter<'a, REG, AeboenwWO>;
impl<'a, REG> AEBOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic exit bus-off disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AeboenwWO::Disable)
    }
    #[doc = "Automatic exit bus-off enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AeboenwWO::Enable)
    }
}
#[doc = "Time triggered communication mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttcenr {
    #[doc = "0: Time triggered communication mode is disabled"]
    Disabled = 0,
    #[doc = "1: Time triggered communication mode is enabled"]
    Enabled = 1,
}
impl From<Ttcenr> for bool {
    #[inline(always)]
    fn from(variant: Ttcenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTCEN` reader - Time triggered communication mode enable"]
pub type TTCEN_R = crate::BitReader<Ttcenr>;
impl TTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttcenr {
        match self.bits {
            false => Ttcenr::Disabled,
            true => Ttcenr::Enabled,
        }
    }
    #[doc = "Time triggered communication mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ttcenr::Disabled
    }
    #[doc = "Time triggered communication mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ttcenr::Enabled
    }
}
#[doc = "Time triggered communication mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TtcenwWO {
    #[doc = "0: Time triggered communication mode disable"]
    Disable = 0,
    #[doc = "1: Time triggered communication mode enable"]
    Enable = 1,
}
impl From<TtcenwWO> for bool {
    #[inline(always)]
    fn from(variant: TtcenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTCEN` writer - Time triggered communication mode enable"]
pub type TTCEN_W<'a, REG> = crate::BitWriter<'a, REG, TtcenwWO>;
impl<'a, REG> TTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time triggered communication mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TtcenwWO::Disable)
    }
    #[doc = "Time triggered communication mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TtcenwWO::Enable)
    }
}
#[doc = "Software partial reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprstr {
    #[doc = "0: Normal"]
    Normal = 0,
    #[doc = "1: Software partial reset"]
    Reset = 1,
}
impl From<Sprstr> for bool {
    #[inline(always)]
    fn from(variant: Sprstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRST` reader - Software partial reset"]
pub type SPRST_R = crate::BitReader<Sprstr>;
impl SPRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sprstr {
        match self.bits {
            false => Sprstr::Normal,
            true => Sprstr::Reset,
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sprstr::Normal
    }
    #[doc = "Software partial reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Sprstr::Reset
    }
}
#[doc = "Software partial reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SprstwWO {
    #[doc = "1: Software partial reset"]
    Reset = 1,
}
impl From<SprstwWO> for bool {
    #[inline(always)]
    fn from(variant: SprstwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRST` writer - Software partial reset"]
pub type SPRST_W<'a, REG> = crate::BitWriter1S<'a, REG, SprstwWO>;
impl<'a, REG> SPRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software partial reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SprstwWO::Reset)
    }
}
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
#[doc = "Field `PTD` reader - Prohibit transmission when debug"]
pub type PTD_R = crate::BitReader<PTD_A>;
impl PTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTD_A {
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
pub type PTD_W<'a, REG> = crate::BitWriter<'a, REG, PTD_A>;
impl<'a, REG> PTD_W<'a, REG>
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
            .field("ptd", &self.ptd())
            .field("sprst", &self.sprst())
            .field("ttcen", &self.ttcen())
            .field("aeboen", &self.aeboen())
            .field("aeden", &self.aeden())
            .field("prsfen", &self.prsfen())
            .field("mdrsel", &self.mdrsel())
            .field("mmssr", &self.mmssr())
            .field("dzen", &self.dzen())
            .field("fzen", &self.fzen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Freeze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn fzen(&mut self) -> FZEN_W<MCTRL_SPEC> {
        FZEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dzen(&mut self) -> DZEN_W<MCTRL_SPEC> {
        DZEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Multiple message sending sequence rule"]
    #[inline(always)]
    #[must_use]
    pub fn mmssr(&mut self) -> MMSSR_W<MCTRL_SPEC> {
        MMSSR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Message discarding rule select when overflow"]
    #[inline(always)]
    #[must_use]
    pub fn mdrsel(&mut self) -> MDRSEL_W<MCTRL_SPEC> {
        MDRSEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Prohibit retransmission when sending fails enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsfen(&mut self) -> PRSFEN_W<MCTRL_SPEC> {
        PRSFEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic exit doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeden(&mut self) -> AEDEN_W<MCTRL_SPEC> {
        AEDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic exit bus-off enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeboen(&mut self) -> AEBOEN_W<MCTRL_SPEC> {
        AEBOEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Time triggered communication mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ttcen(&mut self) -> TTCEN_W<MCTRL_SPEC> {
        TTCEN_W::new(self, 7)
    }
    #[doc = "Bit 15 - Software partial reset"]
    #[inline(always)]
    #[must_use]
    pub fn sprst(&mut self) -> SPRST_W<MCTRL_SPEC> {
        SPRST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Prohibit transmission when debug"]
    #[inline(always)]
    #[must_use]
    pub fn ptd(&mut self) -> PTD_W<MCTRL_SPEC> {
        PTD_W::new(self, 16)
    }
}
#[doc = "Main control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000;
}
#[doc = "`reset()` method sets MCTRL to value 0x0001_0002"]
impl crate::Resettable for MCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_0002;
}
