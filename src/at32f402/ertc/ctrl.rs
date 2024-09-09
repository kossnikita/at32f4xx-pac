#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `WATCLK` reader - Wakeup timer clock selection"]
pub type WATCLK_R = crate::FieldReader;
#[doc = "Field `WATCLK` writer - Wakeup timer clock selection"]
pub type WATCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Timestamp trigger edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDG_A {
    #[doc = "0: Rising edge"]
    Rising = 0,
    #[doc = "1: Falling edge"]
    Falling = 1,
}
impl From<TSEDG_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEDG` reader - Timestamp trigger edge"]
pub type TSEDG_R = crate::BitReader<TSEDG_A>;
impl TSEDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEDG_A {
        match self.bits {
            false => TSEDG_A::Rising,
            true => TSEDG_A::Falling,
        }
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == TSEDG_A::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == TSEDG_A::Falling
    }
}
#[doc = "Field `TSEDG` writer - Timestamp trigger edge"]
pub type TSEDG_W<'a, REG> = crate::BitWriter<'a, REG, TSEDG_A>;
impl<'a, REG> TSEDG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDG_A::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDG_A::Falling)
    }
}
#[doc = "Reference clock detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcdenr {
    #[doc = "0: Reference clock detection is disabled"]
    Disabled = 0,
    #[doc = "1: Reference clock detection is enabled"]
    Enabled = 1,
}
impl From<Rcdenr> for bool {
    #[inline(always)]
    fn from(variant: Rcdenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCDEN` reader - Reference clock detection enable"]
pub type RCDEN_R = crate::BitReader<Rcdenr>;
impl RCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcdenr {
        match self.bits {
            false => Rcdenr::Disabled,
            true => Rcdenr::Enabled,
        }
    }
    #[doc = "Reference clock detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rcdenr::Disabled
    }
    #[doc = "Reference clock detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rcdenr::Enabled
    }
}
#[doc = "Reference clock detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RcdenwWO {
    #[doc = "0: Reference clock detection disable"]
    Disable = 0,
    #[doc = "1: Reference clock detection enable"]
    Enable = 1,
}
impl From<RcdenwWO> for bool {
    #[inline(always)]
    fn from(variant: RcdenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCDEN` writer - Reference clock detection enable"]
pub type RCDEN_W<'a, REG> = crate::BitWriter<'a, REG, RcdenwWO>;
impl<'a, REG> RCDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference clock detection disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RcdenwWO::Disable)
    }
    #[doc = "Reference clock detection enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RcdenwWO::Enable)
    }
}
#[doc = "Date/time register direct read enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drenr {
    #[doc = "0: Date/time register direct read is disabled"]
    Disabled = 0,
    #[doc = "1: Date/time register direct read is enabled"]
    Enabled = 1,
}
impl From<Drenr> for bool {
    #[inline(always)]
    fn from(variant: Drenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREN` reader - Date/time register direct read enable"]
pub type DREN_R = crate::BitReader<Drenr>;
impl DREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drenr {
        match self.bits {
            false => Drenr::Disabled,
            true => Drenr::Enabled,
        }
    }
    #[doc = "Date/time register direct read is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Drenr::Disabled
    }
    #[doc = "Date/time register direct read is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Drenr::Enabled
    }
}
#[doc = "Date/time register direct read enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrenwWO {
    #[doc = "0: Date/time register direct read disable"]
    Disable = 0,
    #[doc = "1: Date/time register direct read enable"]
    Enable = 1,
}
impl From<DrenwWO> for bool {
    #[inline(always)]
    fn from(variant: DrenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREN` writer - Date/time register direct read enable"]
pub type DREN_W<'a, REG> = crate::BitWriter<'a, REG, DrenwWO>;
impl<'a, REG> DREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Date/time register direct read disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DrenwWO::Disable)
    }
    #[doc = "Date/time register direct read enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DrenwWO::Enable)
    }
}
#[doc = "Hour mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HM_A {
    #[doc = "0: 24-hour format"]
    Hour24 = 0,
    #[doc = "1: 12-hour format"]
    Hour12 = 1,
}
impl From<HM_A> for bool {
    #[inline(always)]
    fn from(variant: HM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HM` reader - Hour mode"]
pub type HM_R = crate::BitReader<HM_A>;
impl HM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HM_A {
        match self.bits {
            false => HM_A::Hour24,
            true => HM_A::Hour12,
        }
    }
    #[doc = "24-hour format"]
    #[inline(always)]
    pub fn is_hour24(&self) -> bool {
        *self == HM_A::Hour24
    }
    #[doc = "12-hour format"]
    #[inline(always)]
    pub fn is_hour12(&self) -> bool {
        *self == HM_A::Hour12
    }
}
#[doc = "Field `HM` writer - Hour mode"]
pub type HM_W<'a, REG> = crate::BitWriter<'a, REG, HM_A>;
impl<'a, REG> HM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24-hour format"]
    #[inline(always)]
    pub fn hour24(self) -> &'a mut crate::W<REG> {
        self.variant(HM_A::Hour24)
    }
    #[doc = "12-hour format"]
    #[inline(always)]
    pub fn hour12(self) -> &'a mut crate::W<REG> {
        self.variant(HM_A::Hour12)
    }
}
#[doc = "Field `CCALEN` reader - Coarse calibration enable"]
pub type CCALEN_R = crate::BitReader;
#[doc = "Field `CCALEN` writer - Coarse calibration enable"]
pub type CCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alaenr {
    #[doc = "0: Alarm is disabled"]
    Disabled = 0,
    #[doc = "1: Alarm is enabled"]
    Enabled = 1,
}
impl From<Alaenr> for bool {
    #[inline(always)]
    fn from(variant: Alaenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALAEN` reader - Alarm A enable"]
pub type ALAEN_R = crate::BitReader<Alaenr>;
impl ALAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alaenr {
        match self.bits {
            false => Alaenr::Disabled,
            true => Alaenr::Enabled,
        }
    }
    #[doc = "Alarm is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Alaenr::Disabled
    }
    #[doc = "Alarm is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Alaenr::Enabled
    }
}
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlaenwWO {
    #[doc = "0: Alarm disable"]
    Disable = 0,
    #[doc = "1: Alarm enable"]
    Enable = 1,
}
impl From<AlaenwWO> for bool {
    #[inline(always)]
    fn from(variant: AlaenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALAEN` writer - Alarm A enable"]
pub type ALAEN_W<'a, REG> = crate::BitWriter<'a, REG, AlaenwWO>;
impl<'a, REG> ALAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AlaenwWO::Disable)
    }
    #[doc = "Alarm enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AlaenwWO::Enable)
    }
}
#[doc = "Field `ALBEN` reader - Alarm B enable"]
pub use ALAEN_R as ALBEN_R;
#[doc = "Field `ALBEN` writer - Alarm B enable"]
pub use ALAEN_W as ALBEN_W;
#[doc = "Field `WATEN` reader - Wakeup timer enable"]
pub type WATEN_R = crate::BitReader;
#[doc = "Field `WATEN` writer - Wakeup timer enable"]
pub type WATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsenr {
    #[doc = "0: Timestamp is disabled"]
    Disabled = 0,
    #[doc = "1: Timestamp is enabled"]
    Enabled = 1,
}
impl From<Tsenr> for bool {
    #[inline(always)]
    fn from(variant: Tsenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` reader - Timestamp enable"]
pub type TSEN_R = crate::BitReader<Tsenr>;
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsenr {
        match self.bits {
            false => Tsenr::Disabled,
            true => Tsenr::Enabled,
        }
    }
    #[doc = "Timestamp is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tsenr::Disabled
    }
    #[doc = "Timestamp is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tsenr::Enabled
    }
}
#[doc = "Timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TsenwWO {
    #[doc = "0: Timestamp disable"]
    Disable = 0,
    #[doc = "1: Timestamp enable"]
    Enable = 1,
}
impl From<TsenwWO> for bool {
    #[inline(always)]
    fn from(variant: TsenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` writer - Timestamp enable"]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TsenwWO>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TsenwWO::Disable)
    }
    #[doc = "Timestamp enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TsenwWO::Enable)
    }
}
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alaienr {
    #[doc = "0: Alarm interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Alarm interrupt is enabled"]
    Enabled = 1,
}
impl From<Alaienr> for bool {
    #[inline(always)]
    fn from(variant: Alaienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALAIEN` reader - Alarm A interrupt enable"]
pub type ALAIEN_R = crate::BitReader<Alaienr>;
impl ALAIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alaienr {
        match self.bits {
            false => Alaienr::Disabled,
            true => Alaienr::Enabled,
        }
    }
    #[doc = "Alarm interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Alaienr::Disabled
    }
    #[doc = "Alarm interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Alaienr::Enabled
    }
}
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlaienwWO {
    #[doc = "0: Alarm interrupt disable"]
    Disable = 0,
    #[doc = "1: Alarm interrupt enable"]
    Enable = 1,
}
impl From<AlaienwWO> for bool {
    #[inline(always)]
    fn from(variant: AlaienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALAIEN` writer - Alarm A interrupt enable"]
pub type ALAIEN_W<'a, REG> = crate::BitWriter<'a, REG, AlaienwWO>;
impl<'a, REG> ALAIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AlaienwWO::Disable)
    }
    #[doc = "Alarm interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AlaienwWO::Enable)
    }
}
#[doc = "Field `ALBIEN` reader - Alarm B interrupt enable"]
pub use ALAIEN_R as ALBIEN_R;
#[doc = "Field `ALBIEN` writer - Alarm B interrupt enable"]
pub use ALAIEN_W as ALBIEN_W;
#[doc = "Field `WATIEN` reader - Wakeup timer interrupt enable"]
pub type WATIEN_R = crate::BitReader;
#[doc = "Field `WATIEN` writer - Wakeup timer interrupt enable"]
pub type WATIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsienr {
    #[doc = "0: Timestamp interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Timestamp interrupt is enabled"]
    Enabled = 1,
}
impl From<Tsienr> for bool {
    #[inline(always)]
    fn from(variant: Tsienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIEN` reader - Timestamp interrupt enable"]
pub type TSIEN_R = crate::BitReader<Tsienr>;
impl TSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsienr {
        match self.bits {
            false => Tsienr::Disabled,
            true => Tsienr::Enabled,
        }
    }
    #[doc = "Timestamp interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tsienr::Disabled
    }
    #[doc = "Timestamp interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tsienr::Enabled
    }
}
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TsienwWO {
    #[doc = "0: Timestamp interrupt disable"]
    Disable = 0,
    #[doc = "1: Timestamp interrupt enable"]
    Enable = 1,
}
impl From<TsienwWO> for bool {
    #[inline(always)]
    fn from(variant: TsienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIEN` writer - Timestamp interrupt enable"]
pub type TSIEN_W<'a, REG> = crate::BitWriter<'a, REG, TsienwWO>;
impl<'a, REG> TSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TsienwWO::Disable)
    }
    #[doc = "Timestamp interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TsienwWO::Enable)
    }
}
#[doc = "Add 1 hour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1HW_A {
    #[doc = "1: Add 1 hour"]
    Add = 1,
}
impl From<ADD1HW_A> for bool {
    #[inline(always)]
    fn from(variant: ADD1HW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1H` reader - Add 1 hour"]
pub type ADD1H_R = crate::BitReader<ADD1HW_A>;
impl ADD1H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADD1HW_A> {
        match self.bits {
            true => Some(ADD1HW_A::Add),
            _ => None,
        }
    }
    #[doc = "Add 1 hour"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == ADD1HW_A::Add
    }
}
#[doc = "Field `ADD1H` writer - Add 1 hour"]
pub type ADD1H_W<'a, REG> = crate::BitWriter1S<'a, REG, ADD1HW_A>;
impl<'a, REG> ADD1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Add 1 hour"]
    #[inline(always)]
    pub fn add(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1HW_A::Add)
    }
}
#[doc = "Decrease 1 hour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEC1HW_A {
    #[doc = "1: Subtract 1 hour"]
    Sub = 1,
}
impl From<DEC1HW_A> for bool {
    #[inline(always)]
    fn from(variant: DEC1HW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEC1H` reader - Decrease 1 hour"]
pub type DEC1H_R = crate::BitReader<DEC1HW_A>;
impl DEC1H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DEC1HW_A> {
        match self.bits {
            true => Some(DEC1HW_A::Sub),
            _ => None,
        }
    }
    #[doc = "Subtract 1 hour"]
    #[inline(always)]
    pub fn is_sub(&self) -> bool {
        *self == DEC1HW_A::Sub
    }
}
#[doc = "Field `DEC1H` writer - Decrease 1 hour"]
pub type DEC1H_W<'a, REG> = crate::BitWriter1S<'a, REG, DEC1HW_A>;
impl<'a, REG> DEC1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Subtract 1 hour"]
    #[inline(always)]
    pub fn sub(self) -> &'a mut crate::W<REG> {
        self.variant(DEC1HW_A::Sub)
    }
}
#[doc = "Field `BPR` reader - Battery power domain data register"]
pub type BPR_R = crate::BitReader;
#[doc = "Field `BPR` writer - Battery power domain data register"]
pub type BPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Calibration output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALOSEL_A {
    #[doc = "0: 512Hz"]
    Hz512 = 0,
    #[doc = "1: 1Hz"]
    Hz1 = 1,
}
impl From<CALOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CALOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOSEL` reader - Calibration output selection"]
pub type CALOSEL_R = crate::BitReader<CALOSEL_A>;
impl CALOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALOSEL_A {
        match self.bits {
            false => CALOSEL_A::Hz512,
            true => CALOSEL_A::Hz1,
        }
    }
    #[doc = "512Hz"]
    #[inline(always)]
    pub fn is_hz512(&self) -> bool {
        *self == CALOSEL_A::Hz512
    }
    #[doc = "1Hz"]
    #[inline(always)]
    pub fn is_hz1(&self) -> bool {
        *self == CALOSEL_A::Hz1
    }
}
#[doc = "Field `CALOSEL` writer - Calibration output selection"]
pub type CALOSEL_W<'a, REG> = crate::BitWriter<'a, REG, CALOSEL_A>;
impl<'a, REG> CALOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "512Hz"]
    #[inline(always)]
    pub fn hz512(self) -> &'a mut crate::W<REG> {
        self.variant(CALOSEL_A::Hz512)
    }
    #[doc = "1Hz"]
    #[inline(always)]
    pub fn hz1(self) -> &'a mut crate::W<REG> {
        self.variant(CALOSEL_A::Hz1)
    }
}
#[doc = "Output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTP_A {
    #[doc = "0: High"]
    High = 0,
    #[doc = "1: Low"]
    Low = 1,
}
impl From<OUTP_A> for bool {
    #[inline(always)]
    fn from(variant: OUTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTP` reader - Output polarity"]
pub type OUTP_R = crate::BitReader<OUTP_A>;
impl OUTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTP_A {
        match self.bits {
            false => OUTP_A::High,
            true => OUTP_A::Low,
        }
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTP_A::High
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTP_A::Low
    }
}
#[doc = "Field `OUTP` writer - Output polarity"]
pub type OUTP_W<'a, REG> = crate::BitWriter<'a, REG, OUTP_A>;
impl<'a, REG> OUTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OUTP_A::High)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OUTP_A::Low)
    }
}
#[doc = "Output source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTSEL_A {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: Alarm clock A"]
    AlarmA = 1,
    #[doc = "2: Alarm clock B"]
    AlarmB = 2,
    #[doc = "3: Wakeup events"]
    Wakeup = 3,
}
impl From<OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OUTSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for OUTSEL_A {}
#[doc = "Field `OUTSEL` reader - Output source selection"]
pub type OUTSEL_R = crate::FieldReader<OUTSEL_A>;
impl OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTSEL_A {
        match self.bits {
            0 => OUTSEL_A::Disabled,
            1 => OUTSEL_A::AlarmA,
            2 => OUTSEL_A::AlarmB,
            3 => OUTSEL_A::Wakeup,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUTSEL_A::Disabled
    }
    #[doc = "Alarm clock A"]
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        *self == OUTSEL_A::AlarmA
    }
    #[doc = "Alarm clock B"]
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        *self == OUTSEL_A::AlarmB
    }
    #[doc = "Wakeup events"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OUTSEL_A::Wakeup
    }
}
#[doc = "Field `OUTSEL` writer - Output source selection"]
pub type OUTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OUTSEL_A, crate::Safe>;
impl<'a, REG> OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSEL_A::Disabled)
    }
    #[doc = "Alarm clock A"]
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSEL_A::AlarmA)
    }
    #[doc = "Alarm clock B"]
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSEL_A::AlarmB)
    }
    #[doc = "Wakeup events"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSEL_A::Wakeup)
    }
}
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caloenr {
    #[doc = "0: Calibration output is disabled"]
    Disabled = 0,
    #[doc = "1: Calibration output is enabled"]
    Enabled = 1,
}
impl From<Caloenr> for bool {
    #[inline(always)]
    fn from(variant: Caloenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOEN` reader - Calibration output enable"]
pub type CALOEN_R = crate::BitReader<Caloenr>;
impl CALOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Caloenr {
        match self.bits {
            false => Caloenr::Disabled,
            true => Caloenr::Enabled,
        }
    }
    #[doc = "Calibration output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Caloenr::Disabled
    }
    #[doc = "Calibration output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Caloenr::Enabled
    }
}
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaloenwWO {
    #[doc = "0: Calibration output disable"]
    Disable = 0,
    #[doc = "1: Calibration output enable"]
    Enable = 1,
}
impl From<CaloenwWO> for bool {
    #[inline(always)]
    fn from(variant: CaloenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOEN` writer - Calibration output enable"]
pub type CALOEN_W<'a, REG> = crate::BitWriter<'a, REG, CaloenwWO>;
impl<'a, REG> CALOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CaloenwWO::Disable)
    }
    #[doc = "Calibration output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CaloenwWO::Enable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Wakeup timer clock selection"]
    #[inline(always)]
    pub fn watclk(&self) -> WATCLK_R {
        WATCLK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp trigger edge"]
    #[inline(always)]
    pub fn tsedg(&self) -> TSEDG_R {
        TSEDG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection enable"]
    #[inline(always)]
    pub fn rcden(&self) -> RCDEN_R {
        RCDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Date/time register direct read enable"]
    #[inline(always)]
    pub fn dren(&self) -> DREN_R {
        DREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour mode"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Coarse calibration enable"]
    #[inline(always)]
    pub fn ccalen(&self) -> CCALEN_R {
        CCALEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alaen(&self) -> ALAEN_R {
        ALAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alben(&self) -> ALBEN_R {
        ALBEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn waten(&self) -> WATEN_R {
        WATEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timestamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alaien(&self) -> ALAIEN_R {
        ALAIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn albien(&self) -> ALBIEN_R {
        ALBIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn watien(&self) -> WATIEN_R {
        WATIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add 1 hour"]
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Decrease 1 hour"]
    #[inline(always)]
    pub fn dec1h(&self) -> DEC1H_R {
        DEC1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Battery power domain data register"]
    #[inline(always)]
    pub fn bpr(&self) -> BPR_R {
        BPR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn calosel(&self) -> CALOSEL_R {
        CALOSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn outp(&self) -> OUTP_R {
        OUTP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output source selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn caloen(&self) -> CALOEN_R {
        CALOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("caloen", &self.caloen())
            .field("outsel", &self.outsel())
            .field("outp", &self.outp())
            .field("calosel", &self.calosel())
            .field("bpr", &self.bpr())
            .field("dec1h", &self.dec1h())
            .field("add1h", &self.add1h())
            .field("tsien", &self.tsien())
            .field("watien", &self.watien())
            .field("alaien", &self.alaien())
            .field("albien", &self.albien())
            .field("tsen", &self.tsen())
            .field("waten", &self.waten())
            .field("alaen", &self.alaen())
            .field("alben", &self.alben())
            .field("ccalen", &self.ccalen())
            .field("hm", &self.hm())
            .field("dren", &self.dren())
            .field("rcden", &self.rcden())
            .field("tsedg", &self.tsedg())
            .field("watclk", &self.watclk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup timer clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn watclk(&mut self) -> WATCLK_W<CTRL_SPEC> {
        WATCLK_W::new(self, 0)
    }
    #[doc = "Bit 3 - Timestamp trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedg(&mut self) -> TSEDG_W<CTRL_SPEC> {
        TSEDG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reference clock detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn rcden(&mut self) -> RCDEN_W<CTRL_SPEC> {
        RCDEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Date/time register direct read enable"]
    #[inline(always)]
    #[must_use]
    pub fn dren(&mut self) -> DREN_W<CTRL_SPEC> {
        DREN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Hour mode"]
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HM_W<CTRL_SPEC> {
        HM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Coarse calibration enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccalen(&mut self) -> CCALEN_W<CTRL_SPEC> {
        CCALEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaen(&mut self) -> ALAEN_W<CTRL_SPEC> {
        ALAEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    #[must_use]
    pub fn alben(&mut self) -> ALBEN_W<CTRL_SPEC> {
        ALBEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn waten(&mut self) -> WATEN_W<CTRL_SPEC> {
        WATEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CTRL_SPEC> {
        TSEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaien(&mut self) -> ALAIEN_W<CTRL_SPEC> {
        ALAIEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn albien(&mut self) -> ALBIEN_W<CTRL_SPEC> {
        ALBIEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn watien(&mut self) -> WATIEN_W<CTRL_SPEC> {
        WATIEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TSIEN_W<CTRL_SPEC> {
        TSIEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CTRL_SPEC> {
        ADD1H_W::new(self, 16)
    }
    #[doc = "Bit 17 - Decrease 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn dec1h(&mut self) -> DEC1H_W<CTRL_SPEC> {
        DEC1H_W::new(self, 17)
    }
    #[doc = "Bit 18 - Battery power domain data register"]
    #[inline(always)]
    #[must_use]
    pub fn bpr(&mut self) -> BPR_W<CTRL_SPEC> {
        BPR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn calosel(&mut self) -> CALOSEL_W<CTRL_SPEC> {
        CALOSEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn outp(&mut self) -> OUTP_W<CTRL_SPEC> {
        OUTP_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output source selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<CTRL_SPEC> {
        OUTSEL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn caloen(&mut self) -> CALOEN_W<CTRL_SPEC> {
        CALOEN_W::new(self, 23)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0003_0000;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
