#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `ADCEN` reader - A/D converter enable"]
pub type ADCEN_R = crate::BitReader<ADCENR_A>;
#[doc = "A/D converter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCENR_A {
    #[doc = "0: ADC is disabled"]
    Disabled = 0,
    #[doc = "1: ADC is enabled"]
    Enabled = 1,
}
impl From<ADCENR_A> for bool {
    #[inline(always)]
    fn from(variant: ADCENR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCENR_A {
        match self.bits {
            false => ADCENR_A::Disabled,
            true => ADCENR_A::Enabled,
        }
    }
    #[doc = "ADC is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCENR_A::Disabled
    }
    #[doc = "ADC is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCENR_A::Enabled
    }
}
#[doc = "A/D converter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCENW_AW {
    #[doc = "0: ADC disable"]
    Disable = 0,
    #[doc = "1: ADC enable"]
    Enable = 1,
}
impl From<ADCENW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADCENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` writer - A/D converter enable"]
pub type ADCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADCENW_AW>;
impl<'a, REG, const O: u8> ADCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADCENW_AW::Disable)
    }
    #[doc = "ADC enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADCENW_AW::Enable)
    }
}
#[doc = "Field `RPEN` reader - Repeat mode enable"]
pub type RPEN_R = crate::BitReader<RPENR_A>;
#[doc = "Repeat mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPENR_A {
    #[doc = "0: Repetition mode is disabled"]
    Disabled = 0,
    #[doc = "1: Repetition mode is enabled"]
    Enabled = 1,
}
impl From<RPENR_A> for bool {
    #[inline(always)]
    fn from(variant: RPENR_A) -> Self {
        variant as u8 != 0
    }
}
impl RPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPENR_A {
        match self.bits {
            false => RPENR_A::Disabled,
            true => RPENR_A::Enabled,
        }
    }
    #[doc = "Repetition mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RPENR_A::Disabled
    }
    #[doc = "Repetition mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RPENR_A::Enabled
    }
}
#[doc = "Repeat mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPENW_AW {
    #[doc = "0: Repetition mode disable"]
    Disable = 0,
    #[doc = "1: Repetition mode enable"]
    Enable = 1,
}
impl From<RPENW_AW> for bool {
    #[inline(always)]
    fn from(variant: RPENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPEN` writer - Repeat mode enable"]
pub type RPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RPENW_AW>;
impl<'a, REG, const O: u8> RPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repetition mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RPENW_AW::Disable)
    }
    #[doc = "Repetition mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RPENW_AW::Enable)
    }
}
#[doc = "Field `ADCAL` reader - A/D Calibration"]
pub type ADCAL_R = crate::BitReader<ADCALR_A>;
#[doc = "A/D Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALR_A {
    #[doc = "0: Calibration completed"]
    Completed = 0,
    #[doc = "1: Calibration is in process"]
    InProgress = 1,
}
impl From<ADCALR_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALR_A {
        match self.bits {
            false => ADCALR_A::Completed,
            true => ADCALR_A::InProgress,
        }
    }
    #[doc = "Calibration completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == ADCALR_A::Completed
    }
    #[doc = "Calibration is in process"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == ADCALR_A::InProgress
    }
}
#[doc = "A/D Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALW_AW {
    #[doc = "1: Enable calibration"]
    Calibrate = 1,
}
impl From<ADCALW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADCALW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` writer - A/D Calibration"]
pub type ADCAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADCALW_AW>;
impl<'a, REG, const O: u8> ADCAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn calibrate(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALW_AW::Calibrate)
    }
}
#[doc = "Field `ADCALINIT` reader - Initialize A/D calibration"]
pub type ADCALINIT_R = crate::BitReader<ADCALINITR_A>;
#[doc = "Initialize A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALINITR_A {
    #[doc = "0: Initialization completed"]
    Completed = 0,
    #[doc = "1: Initialization is in process"]
    InProgress = 1,
}
impl From<ADCALINITR_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALINITR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCALINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALINITR_A {
        match self.bits {
            false => ADCALINITR_A::Completed,
            true => ADCALINITR_A::InProgress,
        }
    }
    #[doc = "Initialization completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == ADCALINITR_A::Completed
    }
    #[doc = "Initialization is in process"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == ADCALINITR_A::InProgress
    }
}
#[doc = "Initialize A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALINITW_AW {
    #[doc = "1: Enable initialization"]
    Init = 1,
}
impl From<ADCALINITW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADCALINITW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALINIT` writer - Initialize A/D calibration"]
pub type ADCALINIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADCALINITW_AW>;
impl<'a, REG, const O: u8> ADCALINIT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable initialization"]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALINITW_AW::Init)
    }
}
#[doc = "Field `ADABRT` reader - ADC conversion abort"]
pub type ADABRT_R = crate::BitReader;
#[doc = "Field `ADABRT` writer - ADC conversion abort"]
pub type ADABRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCDMAEN` reader - Ordinary channel DMA transfer enable for independent mode"]
pub type OCDMAEN_R = crate::BitReader;
#[doc = "Field `OCDMAEN` writer - Ordinary channel DMA transfer enable for independent mode"]
pub type OCDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCDRCEN` reader - Ordinary channel DMA request continuation enable for independent mode"]
pub type OCDRCEN_R = crate::BitReader;
#[doc = "Field `OCDRCEN` writer - Ordinary channel DMA request continuation enable for independent mode"]
pub type OCDRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOCSFEN` reader - Each ordinary channel conversion set OCCE flag enable"]
pub type EOCSFEN_R = crate::BitReader;
#[doc = "Field `EOCSFEN` writer - Each ordinary channel conversion set OCCE flag enable"]
pub type EOCSFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTALIGN` reader - Data alignment"]
pub type DTALIGN_R = crate::BitReader<DTALIGN_A>;
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTALIGN_A {
    #[doc = "0: Right alignment"]
    Right = 0,
    #[doc = "1: Left alignment"]
    Left = 1,
}
impl From<DTALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: DTALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTALIGN_A {
        match self.bits {
            false => DTALIGN_A::Right,
            true => DTALIGN_A::Left,
        }
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == DTALIGN_A::Right
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == DTALIGN_A::Left
    }
}
#[doc = "Field `DTALIGN` writer - Data alignment"]
pub type DTALIGN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTALIGN_A>;
impl<'a, REG, const O: u8> DTALIGN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(DTALIGN_A::Right)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(DTALIGN_A::Left)
    }
}
#[doc = "Field `PCTESEL` reader - trigger event select for preempted channels conversion"]
pub type PCTESEL_R = crate::FieldReader;
#[doc = "Field `PCTESEL` writer - trigger event select for preempted channels conversion"]
pub type PCTESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PCETE` reader - Preempted channel external trigger edge select"]
pub type PCETE_R = crate::FieldReader;
#[doc = "Field `PCETE` writer - Preempted channel external trigger edge select"]
pub type PCETE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCSWTRG` reader - Preempted channel software conversion trigger"]
pub type PCSWTRG_R = crate::BitReader<PCSWTRG_A>;
#[doc = "Preempted channel software conversion trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSWTRG_A {
    #[doc = "0: Conversion of ordinary channels not triggered"]
    NotTriggered = 0,
    #[doc = "1: Conversion of ordinary channels triggered"]
    Triggered = 1,
}
impl From<PCSWTRG_A> for bool {
    #[inline(always)]
    fn from(variant: PCSWTRG_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSWTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSWTRG_A {
        match self.bits {
            false => PCSWTRG_A::NotTriggered,
            true => PCSWTRG_A::Triggered,
        }
    }
    #[doc = "Conversion of ordinary channels not triggered"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == PCSWTRG_A::NotTriggered
    }
    #[doc = "Conversion of ordinary channels triggered"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == PCSWTRG_A::Triggered
    }
}
#[doc = "Field `PCSWTRG` writer - Preempted channel software conversion trigger"]
pub type PCSWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCSWTRG_A>;
impl<'a, REG, const O: u8> PCSWTRG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion of ordinary channels not triggered"]
    #[inline(always)]
    pub fn not_triggered(self) -> &'a mut crate::W<REG> {
        self.variant(PCSWTRG_A::NotTriggered)
    }
    #[doc = "Conversion of ordinary channels triggered"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut crate::W<REG> {
        self.variant(PCSWTRG_A::Triggered)
    }
}
#[doc = "Field `OCTESEL` reader - trigger event select for ordinary channels conversion"]
pub type OCTESEL_R = crate::FieldReader;
#[doc = "Field `OCTESEL` writer - trigger event select for ordinary channels conversion"]
pub type OCTESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `OCETE` reader - Ordinary channel external trigger edge select"]
pub type OCETE_R = crate::FieldReader;
#[doc = "Field `OCETE` writer - Ordinary channel external trigger edge select"]
pub type OCETE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `OCSWTRG` reader - Ordinary channel software conversion trigger"]
pub use PCSWTRG_R as OCSWTRG_R;
#[doc = "Field `OCSWTRG` writer - Ordinary channel software conversion trigger"]
pub use PCSWTRG_W as OCSWTRG_W;
impl R {
    #[doc = "Bit 0 - A/D converter enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat mode enable"]
    #[inline(always)]
    pub fn rpen(&self) -> RPEN_R {
        RPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D Calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Initialize A/D calibration"]
    #[inline(always)]
    pub fn adcalinit(&self) -> ADCALINIT_R {
        ADCALINIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC conversion abort"]
    #[inline(always)]
    pub fn adabrt(&self) -> ADABRT_R {
        ADABRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Ordinary channel DMA transfer enable for independent mode"]
    #[inline(always)]
    pub fn ocdmaen(&self) -> OCDMAEN_R {
        OCDMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ordinary channel DMA request continuation enable for independent mode"]
    #[inline(always)]
    pub fn ocdrcen(&self) -> OCDRCEN_R {
        OCDRCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Each ordinary channel conversion set OCCE flag enable"]
    #[inline(always)]
    pub fn eocsfen(&self) -> EOCSFEN_R {
        EOCSFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dtalign(&self) -> DTALIGN_R {
        DTALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel(&self) -> PCTESEL_R {
        PCTESEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Preempted channel external trigger edge select"]
    #[inline(always)]
    pub fn pcete(&self) -> PCETE_R {
        PCETE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Preempted channel software conversion trigger"]
    #[inline(always)]
    pub fn pcswtrg(&self) -> PCSWTRG_R {
        PCSWTRG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel(&self) -> OCTESEL_R {
        OCTESEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Ordinary channel external trigger edge select"]
    #[inline(always)]
    pub fn ocete(&self) -> OCETE_R {
        OCETE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Ordinary channel software conversion trigger"]
    #[inline(always)]
    pub fn ocswtrg(&self) -> OCSWTRG_R {
        OCSWTRG_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("pcswtrg", &format_args!("{}", self.pcswtrg().bit()))
            .field("ocswtrg", &format_args!("{}", self.ocswtrg().bit()))
            .field("ocete", &format_args!("{}", self.ocete().bits()))
            .field("octesel", &format_args!("{}", self.octesel().bits()))
            .field("pcete", &format_args!("{}", self.pcete().bits()))
            .field("pctesel", &format_args!("{}", self.pctesel().bits()))
            .field("dtalign", &format_args!("{}", self.dtalign().bit()))
            .field("eocsfen", &format_args!("{}", self.eocsfen().bit()))
            .field("ocdrcen", &format_args!("{}", self.ocdrcen().bit()))
            .field("ocdmaen", &format_args!("{}", self.ocdmaen().bit()))
            .field("adabrt", &format_args!("{}", self.adabrt().bit()))
            .field("adcalinit", &format_args!("{}", self.adcalinit().bit()))
            .field("adcal", &format_args!("{}", self.adcal().bit()))
            .field("rpen", &format_args!("{}", self.rpen().bit()))
            .field("adcen", &format_args!("{}", self.adcen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - A/D converter enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<CTRL2_SPEC, 0> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 1 - Repeat mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpen(&mut self) -> RPEN_W<CTRL2_SPEC, 1> {
        RPEN_W::new(self)
    }
    #[doc = "Bit 2 - A/D Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<CTRL2_SPEC, 2> {
        ADCAL_W::new(self)
    }
    #[doc = "Bit 3 - Initialize A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcalinit(&mut self) -> ADCALINIT_W<CTRL2_SPEC, 3> {
        ADCALINIT_W::new(self)
    }
    #[doc = "Bit 4 - ADC conversion abort"]
    #[inline(always)]
    #[must_use]
    pub fn adabrt(&mut self) -> ADABRT_W<CTRL2_SPEC, 4> {
        ADABRT_W::new(self)
    }
    #[doc = "Bit 8 - Ordinary channel DMA transfer enable for independent mode"]
    #[inline(always)]
    #[must_use]
    pub fn ocdmaen(&mut self) -> OCDMAEN_W<CTRL2_SPEC, 8> {
        OCDMAEN_W::new(self)
    }
    #[doc = "Bit 9 - Ordinary channel DMA request continuation enable for independent mode"]
    #[inline(always)]
    #[must_use]
    pub fn ocdrcen(&mut self) -> OCDRCEN_W<CTRL2_SPEC, 9> {
        OCDRCEN_W::new(self)
    }
    #[doc = "Bit 10 - Each ordinary channel conversion set OCCE flag enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocsfen(&mut self) -> EOCSFEN_W<CTRL2_SPEC, 10> {
        EOCSFEN_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dtalign(&mut self) -> DTALIGN_W<CTRL2_SPEC, 11> {
        DTALIGN_W::new(self)
    }
    #[doc = "Bits 16:19 - trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel(&mut self) -> PCTESEL_W<CTRL2_SPEC, 16> {
        PCTESEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Preempted channel external trigger edge select"]
    #[inline(always)]
    #[must_use]
    pub fn pcete(&mut self) -> PCETE_W<CTRL2_SPEC, 20> {
        PCETE_W::new(self)
    }
    #[doc = "Bit 22 - Preempted channel software conversion trigger"]
    #[inline(always)]
    #[must_use]
    pub fn pcswtrg(&mut self) -> PCSWTRG_W<CTRL2_SPEC, 22> {
        PCSWTRG_W::new(self)
    }
    #[doc = "Bits 24:27 - trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel(&mut self) -> OCTESEL_W<CTRL2_SPEC, 24> {
        OCTESEL_W::new(self)
    }
    #[doc = "Bits 28:29 - Ordinary channel external trigger edge select"]
    #[inline(always)]
    #[must_use]
    pub fn ocete(&mut self) -> OCETE_W<CTRL2_SPEC, 28> {
        OCETE_W::new(self)
    }
    #[doc = "Bit 30 - Ordinary channel software conversion trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ocswtrg(&mut self) -> OCSWTRG_W<CTRL2_SPEC, 30> {
        OCSWTRG_W::new(self)
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
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
