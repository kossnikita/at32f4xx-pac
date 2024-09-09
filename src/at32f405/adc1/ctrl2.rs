#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "A/D converter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcenr {
    #[doc = "0: ADC is disabled"]
    Disabled = 0,
    #[doc = "1: ADC is enabled"]
    Enabled = 1,
}
impl From<Adcenr> for bool {
    #[inline(always)]
    fn from(variant: Adcenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - A/D converter enable"]
pub type ADCEN_R = crate::BitReader<Adcenr>;
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcenr {
        match self.bits {
            false => Adcenr::Disabled,
            true => Adcenr::Enabled,
        }
    }
    #[doc = "ADC is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adcenr::Disabled
    }
    #[doc = "ADC is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adcenr::Enabled
    }
}
#[doc = "A/D converter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcenwWO {
    #[doc = "0: ADC disable"]
    Disable = 0,
    #[doc = "1: ADC enable"]
    Enable = 1,
}
impl From<AdcenwWO> for bool {
    #[inline(always)]
    fn from(variant: AdcenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` writer - A/D converter enable"]
pub type ADCEN_W<'a, REG> = crate::BitWriter<'a, REG, AdcenwWO>;
impl<'a, REG> ADCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AdcenwWO::Disable)
    }
    #[doc = "ADC enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AdcenwWO::Enable)
    }
}
#[doc = "Repeat mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpenr {
    #[doc = "0: Repetition mode is disabled"]
    Disabled = 0,
    #[doc = "1: Repetition mode is enabled"]
    Enabled = 1,
}
impl From<Rpenr> for bool {
    #[inline(always)]
    fn from(variant: Rpenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPEN` reader - Repeat mode enable"]
pub type RPEN_R = crate::BitReader<Rpenr>;
impl RPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpenr {
        match self.bits {
            false => Rpenr::Disabled,
            true => Rpenr::Enabled,
        }
    }
    #[doc = "Repetition mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rpenr::Disabled
    }
    #[doc = "Repetition mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rpenr::Enabled
    }
}
#[doc = "Repeat mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RpenwWO {
    #[doc = "0: Repetition mode disable"]
    Disable = 0,
    #[doc = "1: Repetition mode enable"]
    Enable = 1,
}
impl From<RpenwWO> for bool {
    #[inline(always)]
    fn from(variant: RpenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPEN` writer - Repeat mode enable"]
pub type RPEN_W<'a, REG> = crate::BitWriter<'a, REG, RpenwWO>;
impl<'a, REG> RPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repetition mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RpenwWO::Disable)
    }
    #[doc = "Repetition mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RpenwWO::Enable)
    }
}
#[doc = "A/D Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcalr {
    #[doc = "0: Calibration completed"]
    Completed = 0,
    #[doc = "1: Calibration is in process"]
    InProgress = 1,
}
impl From<Adcalr> for bool {
    #[inline(always)]
    fn from(variant: Adcalr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - A/D Calibration"]
pub type ADCAL_R = crate::BitReader<Adcalr>;
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcalr {
        match self.bits {
            false => Adcalr::Completed,
            true => Adcalr::InProgress,
        }
    }
    #[doc = "Calibration completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == Adcalr::Completed
    }
    #[doc = "Calibration is in process"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Adcalr::InProgress
    }
}
#[doc = "A/D Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcalwWO {
    #[doc = "1: Enable calibration"]
    Calibrate = 1,
}
impl From<AdcalwWO> for bool {
    #[inline(always)]
    fn from(variant: AdcalwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` writer - A/D Calibration"]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, AdcalwWO>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable calibration"]
    #[inline(always)]
    pub fn calibrate(self) -> &'a mut crate::W<REG> {
        self.variant(AdcalwWO::Calibrate)
    }
}
#[doc = "initialize A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcalinitr {
    #[doc = "0: Initialization completed"]
    Completed = 0,
    #[doc = "1: Initialization is in process"]
    InProgress = 1,
}
impl From<Adcalinitr> for bool {
    #[inline(always)]
    fn from(variant: Adcalinitr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALINIT` reader - initialize A/D calibration"]
pub type ADCALINIT_R = crate::BitReader<Adcalinitr>;
impl ADCALINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcalinitr {
        match self.bits {
            false => Adcalinitr::Completed,
            true => Adcalinitr::InProgress,
        }
    }
    #[doc = "Initialization completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == Adcalinitr::Completed
    }
    #[doc = "Initialization is in process"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == Adcalinitr::InProgress
    }
}
#[doc = "initialize A/D calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcalinitwWO {
    #[doc = "1: Enable initialization"]
    Init = 1,
}
impl From<AdcalinitwWO> for bool {
    #[inline(always)]
    fn from(variant: AdcalinitwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALINIT` writer - initialize A/D calibration"]
pub type ADCALINIT_W<'a, REG> = crate::BitWriter<'a, REG, AdcalinitwWO>;
impl<'a, REG> ADCALINIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable initialization"]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(AdcalinitwWO::Init)
    }
}
#[doc = "Field `OCDMAEN` reader - DMA transfer enable of ordinary channels"]
pub type OCDMAEN_R = crate::BitReader;
#[doc = "Field `OCDMAEN` writer - DMA transfer enable of ordinary channels"]
pub type OCDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `DTALIGN` reader - Data alignment"]
pub type DTALIGN_R = crate::BitReader<DTALIGN_A>;
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
#[doc = "Field `PCSWTRG` reader - Conversion trigger by software of preempted channels"]
pub type PCSWTRG_R = crate::BitReader<PCSWTRG_A>;
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
            .field("itsrven", &self.itsrven())
            .field("pcswtrg", &self.pcswtrg())
            .field("ocswtrg", &self.ocswtrg())
            .field("octen", &self.octen())
            .field("octesel", &self.octesel())
            .field("pcten", &self.pcten())
            .field("pctesel", &self.pctesel())
            .field("dtalign", &self.dtalign())
            .field("ocdmaen", &self.ocdmaen())
            .field("adcalinit", &self.adcalinit())
            .field("adcal", &self.adcal())
            .field("rpen", &self.rpen())
            .field("adcen", &self.adcen())
            .finish()
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
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
