#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `TSEDG` reader - Timestamp trigger edge"]
pub type TSEDG_R = crate::BitReader<TSEDG_A>;
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
pub type TSEDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSEDG_A>;
impl<'a, REG, const O: u8> TSEDG_W<'a, REG, O>
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
#[doc = "Field `RCDEN` reader - Reference clock detection enable"]
pub type RCDEN_R = crate::BitReader<RCDENR_A>;
#[doc = "Reference clock detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCDENR_A {
    #[doc = "0: Reference clock detection is disabled"]
    Disabled = 0,
    #[doc = "1: Reference clock detection is enabled"]
    Enabled = 1,
}
impl From<RCDENR_A> for bool {
    #[inline(always)]
    fn from(variant: RCDENR_A) -> Self {
        variant as u8 != 0
    }
}
impl RCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RCDENR_A {
        match self.bits {
            false => RCDENR_A::Disabled,
            true => RCDENR_A::Enabled,
        }
    }
    #[doc = "Reference clock detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RCDENR_A::Disabled
    }
    #[doc = "Reference clock detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RCDENR_A::Enabled
    }
}
#[doc = "Reference clock detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCDENW_AW {
    #[doc = "0: Reference clock detection disable"]
    Disable = 0,
    #[doc = "1: Reference clock detection enable"]
    Enable = 1,
}
impl From<RCDENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RCDENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCDEN` writer - Reference clock detection enable"]
pub type RCDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RCDENW_AW>;
impl<'a, REG, const O: u8> RCDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference clock detection disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RCDENW_AW::Disable)
    }
    #[doc = "Reference clock detection enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RCDENW_AW::Enable)
    }
}
#[doc = "Field `DREN` reader - Date/time register direct read enable"]
pub type DREN_R = crate::BitReader<DRENR_A>;
#[doc = "Date/time register direct read enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRENR_A {
    #[doc = "0: Date/time register direct read is disabled"]
    Disabled = 0,
    #[doc = "1: Date/time register direct read is enabled"]
    Enabled = 1,
}
impl From<DRENR_A> for bool {
    #[inline(always)]
    fn from(variant: DRENR_A) -> Self {
        variant as u8 != 0
    }
}
impl DREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRENR_A {
        match self.bits {
            false => DRENR_A::Disabled,
            true => DRENR_A::Enabled,
        }
    }
    #[doc = "Date/time register direct read is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DRENR_A::Disabled
    }
    #[doc = "Date/time register direct read is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DRENR_A::Enabled
    }
}
#[doc = "Date/time register direct read enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRENW_AW {
    #[doc = "0: Date/time register direct read disable"]
    Disable = 0,
    #[doc = "1: Date/time register direct read enable"]
    Enable = 1,
}
impl From<DRENW_AW> for bool {
    #[inline(always)]
    fn from(variant: DRENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREN` writer - Date/time register direct read enable"]
pub type DREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DRENW_AW>;
impl<'a, REG, const O: u8> DREN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Date/time register direct read disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DRENW_AW::Disable)
    }
    #[doc = "Date/time register direct read enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DRENW_AW::Enable)
    }
}
#[doc = "Field `HM` reader - Hour mode"]
pub type HM_R = crate::BitReader<HM_A>;
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
pub type HM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HM_A>;
impl<'a, REG, const O: u8> HM_W<'a, REG, O>
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
#[doc = "Field `ALAEN` reader - Alarm A enable"]
pub type ALAEN_R = crate::BitReader<ALAENR_A>;
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALAENR_A {
    #[doc = "0: Alarm is disabled"]
    Disabled = 0,
    #[doc = "1: Alarm is enabled"]
    Enabled = 1,
}
impl From<ALAENR_A> for bool {
    #[inline(always)]
    fn from(variant: ALAENR_A) -> Self {
        variant as u8 != 0
    }
}
impl ALAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALAENR_A {
        match self.bits {
            false => ALAENR_A::Disabled,
            true => ALAENR_A::Enabled,
        }
    }
    #[doc = "Alarm is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALAENR_A::Disabled
    }
    #[doc = "Alarm is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALAENR_A::Enabled
    }
}
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALAENW_AW {
    #[doc = "0: Alarm disable"]
    Disable = 0,
    #[doc = "1: Alarm enable"]
    Enable = 1,
}
impl From<ALAENW_AW> for bool {
    #[inline(always)]
    fn from(variant: ALAENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALAEN` writer - Alarm A enable"]
pub type ALAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALAENW_AW>;
impl<'a, REG, const O: u8> ALAEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ALAENW_AW::Disable)
    }
    #[doc = "Alarm enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ALAENW_AW::Enable)
    }
}
#[doc = "Field `TSEN` reader - Timestamp enable"]
pub type TSEN_R = crate::BitReader<TSENR_A>;
#[doc = "Timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSENR_A {
    #[doc = "0: Timestamp is disabled"]
    Disabled = 0,
    #[doc = "1: Timestamp is enabled"]
    Enabled = 1,
}
impl From<TSENR_A> for bool {
    #[inline(always)]
    fn from(variant: TSENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSENR_A {
        match self.bits {
            false => TSENR_A::Disabled,
            true => TSENR_A::Enabled,
        }
    }
    #[doc = "Timestamp is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSENR_A::Disabled
    }
    #[doc = "Timestamp is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSENR_A::Enabled
    }
}
#[doc = "Timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSENW_AW {
    #[doc = "0: Timestamp disable"]
    Disable = 0,
    #[doc = "1: Timestamp enable"]
    Enable = 1,
}
impl From<TSENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TSENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEN` writer - Timestamp enable"]
pub type TSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSENW_AW>;
impl<'a, REG, const O: u8> TSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSENW_AW::Disable)
    }
    #[doc = "Timestamp enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TSENW_AW::Enable)
    }
}
#[doc = "Field `ALAIEN` reader - Alarm A interrupt enable"]
pub type ALAIEN_R = crate::BitReader<ALAIENR_A>;
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALAIENR_A {
    #[doc = "0: Alarm interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Alarm interrupt is enabled"]
    Enabled = 1,
}
impl From<ALAIENR_A> for bool {
    #[inline(always)]
    fn from(variant: ALAIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl ALAIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALAIENR_A {
        match self.bits {
            false => ALAIENR_A::Disabled,
            true => ALAIENR_A::Enabled,
        }
    }
    #[doc = "Alarm interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALAIENR_A::Disabled
    }
    #[doc = "Alarm interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALAIENR_A::Enabled
    }
}
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALAIENW_AW {
    #[doc = "0: Alarm interrupt disable"]
    Disable = 0,
    #[doc = "1: Alarm interrupt enable"]
    Enable = 1,
}
impl From<ALAIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: ALAIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALAIEN` writer - Alarm A interrupt enable"]
pub type ALAIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ALAIENW_AW>;
impl<'a, REG, const O: u8> ALAIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ALAIENW_AW::Disable)
    }
    #[doc = "Alarm interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ALAIENW_AW::Enable)
    }
}
#[doc = "Field `TSIEN` reader - Timestamp interrupt enable"]
pub type TSIEN_R = crate::BitReader<TSIENR_A>;
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIENR_A {
    #[doc = "0: Timestamp interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Timestamp interrupt is enabled"]
    Enabled = 1,
}
impl From<TSIENR_A> for bool {
    #[inline(always)]
    fn from(variant: TSIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSIENR_A {
        match self.bits {
            false => TSIENR_A::Disabled,
            true => TSIENR_A::Enabled,
        }
    }
    #[doc = "Timestamp interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSIENR_A::Disabled
    }
    #[doc = "Timestamp interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSIENR_A::Enabled
    }
}
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIENW_AW {
    #[doc = "0: Timestamp interrupt disable"]
    Disable = 0,
    #[doc = "1: Timestamp interrupt enable"]
    Enable = 1,
}
impl From<TSIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TSIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIEN` writer - Timestamp interrupt enable"]
pub type TSIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSIENW_AW>;
impl<'a, REG, const O: u8> TSIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSIENW_AW::Disable)
    }
    #[doc = "Timestamp interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TSIENW_AW::Enable)
    }
}
#[doc = "Field `ADD1H` reader - Add 1 hour"]
pub type ADD1H_R = crate::BitReader<ADD1HW_A>;
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
pub type ADD1H_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, ADD1HW_A>;
impl<'a, REG, const O: u8> ADD1H_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Add 1 hour"]
    #[inline(always)]
    pub fn add(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1HW_A::Add)
    }
}
#[doc = "Field `DEC1H` reader - Decrease 1 hour"]
pub type DEC1H_R = crate::BitReader<DEC1HW_A>;
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
pub type DEC1H_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, DEC1HW_A>;
impl<'a, REG, const O: u8> DEC1H_W<'a, REG, O>
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
pub type BPR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALOSEL` reader - Calibration output selection"]
pub type CALOSEL_R = crate::BitReader<CALOSEL_A>;
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
pub type CALOSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CALOSEL_A>;
impl<'a, REG, const O: u8> CALOSEL_W<'a, REG, O>
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
#[doc = "Field `OUTP` reader - Output polarity"]
pub type OUTP_R = crate::BitReader<OUTP_A>;
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
pub type OUTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OUTP_A>;
impl<'a, REG, const O: u8> OUTP_W<'a, REG, O>
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
#[doc = "Field `OUTSEL` reader - Output source selection"]
pub type OUTSEL_R = crate::FieldReader<OUTSEL_A>;
#[doc = "Output source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTSEL_A {
    #[doc = "0: `0`"]
    Disabled = 0,
    #[doc = "1: Alarm clock A"]
    AlarmA = 1,
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
impl OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OUTSEL_A> {
        match self.bits {
            0 => Some(OUTSEL_A::Disabled),
            1 => Some(OUTSEL_A::AlarmA),
            3 => Some(OUTSEL_A::Wakeup),
            _ => None,
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
    #[doc = "Wakeup events"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OUTSEL_A::Wakeup
    }
}
#[doc = "Field `OUTSEL` writer - Output source selection"]
pub type OUTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, OUTSEL_A>;
impl<'a, REG, const O: u8> OUTSEL_W<'a, REG, O>
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
    #[doc = "Wakeup events"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSEL_A::Wakeup)
    }
}
#[doc = "Field `CALOEN` reader - Calibration output enable"]
pub type CALOEN_R = crate::BitReader<CALOENR_A>;
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALOENR_A {
    #[doc = "0: Calibration output is disabled"]
    Disabled = 0,
    #[doc = "1: Calibration output is enabled"]
    Enabled = 1,
}
impl From<CALOENR_A> for bool {
    #[inline(always)]
    fn from(variant: CALOENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CALOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALOENR_A {
        match self.bits {
            false => CALOENR_A::Disabled,
            true => CALOENR_A::Enabled,
        }
    }
    #[doc = "Calibration output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALOENR_A::Disabled
    }
    #[doc = "Calibration output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALOENR_A::Enabled
    }
}
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALOENW_AW {
    #[doc = "0: Calibration output disable"]
    Disable = 0,
    #[doc = "1: Calibration output enable"]
    Enable = 1,
}
impl From<CALOENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CALOENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOEN` writer - Calibration output enable"]
pub type CALOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CALOENW_AW>;
impl<'a, REG, const O: u8> CALOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CALOENW_AW::Disable)
    }
    #[doc = "Calibration output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CALOENW_AW::Enable)
    }
}
impl R {
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
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alaen(&self) -> ALAEN_R {
        ALAEN_R::new(((self.bits >> 8) & 1) != 0)
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
            .field("caloen", &format_args!("{}", self.caloen().bit()))
            .field("outsel", &format_args!("{}", self.outsel().bits()))
            .field("outp", &format_args!("{}", self.outp().bit()))
            .field("calosel", &format_args!("{}", self.calosel().bit()))
            .field("bpr", &format_args!("{}", self.bpr().bit()))
            .field("dec1h", &format_args!("{}", self.dec1h().bit()))
            .field("add1h", &format_args!("{}", self.add1h().bit()))
            .field("tsien", &format_args!("{}", self.tsien().bit()))
            .field("alaien", &format_args!("{}", self.alaien().bit()))
            .field("tsen", &format_args!("{}", self.tsen().bit()))
            .field("alaen", &format_args!("{}", self.alaen().bit()))
            .field("hm", &format_args!("{}", self.hm().bit()))
            .field("dren", &format_args!("{}", self.dren().bit()))
            .field("rcden", &format_args!("{}", self.rcden().bit()))
            .field("tsedg", &format_args!("{}", self.tsedg().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Timestamp trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedg(&mut self) -> TSEDG_W<CTRL_SPEC, 3> {
        TSEDG_W::new(self)
    }
    #[doc = "Bit 4 - Reference clock detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn rcden(&mut self) -> RCDEN_W<CTRL_SPEC, 4> {
        RCDEN_W::new(self)
    }
    #[doc = "Bit 5 - Date/time register direct read enable"]
    #[inline(always)]
    #[must_use]
    pub fn dren(&mut self) -> DREN_W<CTRL_SPEC, 5> {
        DREN_W::new(self)
    }
    #[doc = "Bit 6 - Hour mode"]
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HM_W<CTRL_SPEC, 6> {
        HM_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaen(&mut self) -> ALAEN_W<CTRL_SPEC, 8> {
        ALAEN_W::new(self)
    }
    #[doc = "Bit 11 - Timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CTRL_SPEC, 11> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaien(&mut self) -> ALAIEN_W<CTRL_SPEC, 12> {
        ALAIEN_W::new(self)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TSIEN_W<CTRL_SPEC, 15> {
        TSIEN_W::new(self)
    }
    #[doc = "Bit 16 - Add 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CTRL_SPEC, 16> {
        ADD1H_W::new(self)
    }
    #[doc = "Bit 17 - Decrease 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn dec1h(&mut self) -> DEC1H_W<CTRL_SPEC, 17> {
        DEC1H_W::new(self)
    }
    #[doc = "Bit 18 - Battery power domain data register"]
    #[inline(always)]
    #[must_use]
    pub fn bpr(&mut self) -> BPR_W<CTRL_SPEC, 18> {
        BPR_W::new(self)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn calosel(&mut self) -> CALOSEL_W<CTRL_SPEC, 19> {
        CALOSEL_W::new(self)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn outp(&mut self) -> OUTP_W<CTRL_SPEC, 20> {
        OUTP_W::new(self)
    }
    #[doc = "Bits 21:22 - Output source selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<CTRL_SPEC, 21> {
        OUTSEL_W::new(self)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn caloen(&mut self) -> CALOEN_W<CTRL_SPEC, 23> {
        CALOEN_W::new(self)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_0000;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
