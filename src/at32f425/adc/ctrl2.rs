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
    pub const fn variant(&self) -> ADCENR_A {
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
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, ADCENW_AW>;
impl<'a, REG> ADCEN_W<'a, REG>
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
    pub const fn variant(&self) -> RPENR_A {
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
pub type RPEN_W<'a, REG> = crate::BitWriter<'a, REG, RPENW_AW>;
impl<'a, REG> RPEN_W<'a, REG>
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
    pub const fn variant(&self) -> ADCALR_A {
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
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, ADCALW_AW>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn calibrate(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALW_AW::Calibrate)
    }
}
#[doc = "Field `ADCALINIT` reader - initialize A/D calibration"]
pub type ADCALINIT_R = crate::BitReader<ADCALINITR_A>;
#[doc = "initialize A/D calibration\n\nValue on reset: 0"]
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
    pub const fn variant(&self) -> ADCALINITR_A {
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
#[doc = "initialize A/D calibration\n\nValue on reset: 0"]
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
#[doc = "Field `ADCALINIT` writer - initialize A/D calibration"]
pub type ADCALINIT_W<'a, REG> = crate::BitWriter<'a, REG, ADCALINITW_AW>;
impl<'a, REG> ADCALINIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable initialization"]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALINITW_AW::Init)
    }
}
#[doc = "Field `OCDMAEN` reader - DMA transfer enable of ordinary channels"]
pub type OCDMAEN_R = crate::BitReader;
#[doc = "Field `OCDMAEN` writer - DMA transfer enable of ordinary channels"]
pub type OCDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub const fn variant(&self) -> DTALIGN_A {
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
pub type DTALIGN_W<'a, REG> = crate::BitWriter<'a, REG, DTALIGN_A>;
impl<'a, REG> DTALIGN_W<'a, REG>
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
#[doc = "Field `PCTESEL` reader - Low bit of trigger event select for preempted channels conversion"]
pub type PCTESEL_R = crate::FieldReader;
#[doc = "Field `PCTESEL` writer - Low bit of trigger event select for preempted channels conversion"]
pub type PCTESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCTEN` reader - Trigger mode enable for preempted channels conversion"]
pub type PCTEN_R = crate::BitReader;
#[doc = "Field `PCTEN` writer - Trigger mode enable for preempted channels conversion"]
pub type PCTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTESEL` reader - Low bit of trigger event select for ordinary channels conversion"]
pub type OCTESEL_R = crate::FieldReader;
#[doc = "Field `OCTESEL` writer - Low bit of trigger event select for ordinary channels conversion"]
pub type OCTESEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCTEN` reader - Trigger mode enable for ordinary channels conversion"]
pub type OCTEN_R = crate::BitReader;
#[doc = "Field `OCTEN` writer - Trigger mode enable for ordinary channels conversion"]
pub type OCTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSWTRG` reader - Conversion trigger by software of preempted channels"]
pub type PCSWTRG_R = crate::BitReader<PCSWTRG_A>;
#[doc = "Conversion trigger by software of preempted channels\n\nValue on reset: 0"]
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
    pub const fn variant(&self) -> PCSWTRG_A {
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
#[doc = "Field `PCSWTRG` writer - Conversion trigger by software of preempted channels"]
pub type PCSWTRG_W<'a, REG> = crate::BitWriter<'a, REG, PCSWTRG_A>;
impl<'a, REG> PCSWTRG_W<'a, REG>
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
#[doc = "Field `OCSWTRG` reader - Conversion trigger by software of ordinary channels"]
pub use PCSWTRG_R as OCSWTRG_R;
#[doc = "Field `OCSWTRG` writer - Conversion trigger by software of ordinary channels"]
pub use PCSWTRG_W as OCSWTRG_W;
#[doc = "Field `ITSRVEN` reader - Internal temperature sensor and VINTRV enable"]
pub type ITSRVEN_R = crate::BitReader;
#[doc = "Field `ITSRVEN` writer - Internal temperature sensor and VINTRV enable"]
pub type ITSRVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - initialize A/D calibration"]
    #[inline(always)]
    pub fn adcalinit(&self) -> ADCALINIT_R {
        ADCALINIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA transfer enable of ordinary channels"]
    #[inline(always)]
    pub fn ocdmaen(&self) -> OCDMAEN_R {
        OCDMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dtalign(&self) -> DTALIGN_R {
        DTALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel(&self) -> PCTESEL_R {
        PCTESEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Trigger mode enable for preempted channels conversion"]
    #[inline(always)]
    pub fn pcten(&self) -> PCTEN_R {
        PCTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel(&self) -> OCTESEL_R {
        OCTESEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Trigger mode enable for ordinary channels conversion"]
    #[inline(always)]
    pub fn octen(&self) -> OCTEN_R {
        OCTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Conversion trigger by software of preempted channels"]
    #[inline(always)]
    pub fn pcswtrg(&self) -> PCSWTRG_R {
        PCSWTRG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Conversion trigger by software of ordinary channels"]
    #[inline(always)]
    pub fn ocswtrg(&self) -> OCSWTRG_R {
        OCSWTRG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    pub fn itsrven(&self) -> ITSRVEN_R {
        ITSRVEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("itsrven", &format_args!("{}", self.itsrven().bit()))
            .field("pcswtrg", &format_args!("{}", self.pcswtrg().bit()))
            .field("ocswtrg", &format_args!("{}", self.ocswtrg().bit()))
            .field("octen", &format_args!("{}", self.octen().bit()))
            .field("octesel", &format_args!("{}", self.octesel().bits()))
            .field("pcten", &format_args!("{}", self.pcten().bit()))
            .field("pctesel", &format_args!("{}", self.pctesel().bits()))
            .field("dtalign", &format_args!("{}", self.dtalign().bit()))
            .field("ocdmaen", &format_args!("{}", self.ocdmaen().bit()))
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
    pub fn adcen(&mut self) -> ADCEN_W<CTRL2_SPEC> {
        ADCEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Repeat mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpen(&mut self) -> RPEN_W<CTRL2_SPEC> {
        RPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - A/D Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<CTRL2_SPEC> {
        ADCAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - initialize A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcalinit(&mut self) -> ADCALINIT_W<CTRL2_SPEC> {
        ADCALINIT_W::new(self, 3)
    }
    #[doc = "Bit 8 - DMA transfer enable of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocdmaen(&mut self) -> OCDMAEN_W<CTRL2_SPEC> {
        OCDMAEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dtalign(&mut self) -> DTALIGN_W<CTRL2_SPEC> {
        DTALIGN_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel(&mut self) -> PCTESEL_W<CTRL2_SPEC> {
        PCTESEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - Trigger mode enable for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pcten(&mut self) -> PCTEN_W<CTRL2_SPEC> {
        PCTEN_W::new(self, 15)
    }
    #[doc = "Bits 17:19 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel(&mut self) -> OCTESEL_W<CTRL2_SPEC> {
        OCTESEL_W::new(self, 17)
    }
    #[doc = "Bit 20 - Trigger mode enable for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octen(&mut self) -> OCTEN_W<CTRL2_SPEC> {
        OCTEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Conversion trigger by software of preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcswtrg(&mut self) -> PCSWTRG_W<CTRL2_SPEC> {
        PCSWTRG_W::new(self, 21)
    }
    #[doc = "Bit 22 - Conversion trigger by software of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocswtrg(&mut self) -> OCSWTRG_W<CTRL2_SPEC> {
        OCSWTRG_W::new(self, 22)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    #[must_use]
    pub fn itsrven(&mut self) -> ITSRVEN_W<CTRL2_SPEC> {
        ITSRVEN_W::new(self, 23)
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
