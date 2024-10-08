#[doc = "Register `ALA` reader"]
pub type R = crate::R<ALA_SPEC>;
#[doc = "Register `ALA` writer"]
pub type W = crate::W<ALA_SPEC>;
#[doc = "Field `SU` reader - Second units"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `ST` reader - Second tens"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Seconds mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK1_A {
    #[doc = "0: No second mask"]
    NoMask = 0,
    #[doc = "1: Alarm clock doesn't care about seconds"]
    Mask = 1,
}
impl From<MASK1_A> for bool {
    #[inline(always)]
    fn from(variant: MASK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK1` reader - Seconds mask"]
pub type MASK1_R = crate::BitReader<MASK1_A>;
impl MASK1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASK1_A {
        match self.bits {
            false => MASK1_A::NoMask,
            true => MASK1_A::Mask,
        }
    }
    #[doc = "No second mask"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == MASK1_A::NoMask
    }
    #[doc = "Alarm clock doesn't care about seconds"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MASK1_A::Mask
    }
}
#[doc = "Field `MASK1` writer - Seconds mask"]
pub type MASK1_W<'a, REG> = crate::BitWriter<'a, REG, MASK1_A>;
impl<'a, REG> MASK1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No second mask"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK1_A::NoMask)
    }
    #[doc = "Alarm clock doesn't care about seconds"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK1_A::Mask)
    }
}
#[doc = "Field `MU` reader - Minute units"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MU` writer - Minute units"]
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `MT` reader - Minute tens"]
pub type MT_R = crate::FieldReader;
#[doc = "Field `MT` writer - Minute tens"]
pub type MT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Minutes mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK2_A {
    #[doc = "0: No minute mask"]
    NoMask = 0,
    #[doc = "1: Alarm clock doesn't care about minutes"]
    Mask = 1,
}
impl From<MASK2_A> for bool {
    #[inline(always)]
    fn from(variant: MASK2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK2` reader - Minutes mask"]
pub type MASK2_R = crate::BitReader<MASK2_A>;
impl MASK2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASK2_A {
        match self.bits {
            false => MASK2_A::NoMask,
            true => MASK2_A::Mask,
        }
    }
    #[doc = "No minute mask"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == MASK2_A::NoMask
    }
    #[doc = "Alarm clock doesn't care about minutes"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MASK2_A::Mask
    }
}
#[doc = "Field `MASK2` writer - Minutes mask"]
pub type MASK2_W<'a, REG> = crate::BitWriter<'a, REG, MASK2_A>;
impl<'a, REG> MASK2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No minute mask"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK2_A::NoMask)
    }
    #[doc = "Alarm clock doesn't care about minutes"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK2_A::Mask)
    }
}
#[doc = "Field `HU` reader - Hour units"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units"]
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `HT` reader - Hour tens"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "AM/PM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMPM_A {
    #[doc = "0: AM"]
    Am = 0,
    #[doc = "1: PM"]
    Pm = 1,
}
impl From<AMPM_A> for bool {
    #[inline(always)]
    fn from(variant: AMPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPM` reader - AM/PM"]
pub type AMPM_R = crate::BitReader<AMPM_A>;
impl AMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AMPM_A {
        match self.bits {
            false => AMPM_A::Am,
            true => AMPM_A::Pm,
        }
    }
    #[doc = "AM"]
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == AMPM_A::Am
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == AMPM_A::Pm
    }
}
#[doc = "Field `AMPM` writer - AM/PM"]
pub type AMPM_W<'a, REG> = crate::BitWriter<'a, REG, AMPM_A>;
impl<'a, REG> AMPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AM"]
    #[inline(always)]
    pub fn am(self) -> &'a mut crate::W<REG> {
        self.variant(AMPM_A::Am)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn pm(self) -> &'a mut crate::W<REG> {
        self.variant(AMPM_A::Pm)
    }
}
#[doc = "Hours mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK3_A {
    #[doc = "0: No hour mask"]
    NoMask = 0,
    #[doc = "1: Alarm clock doesn't care about hours"]
    Mask = 1,
}
impl From<MASK3_A> for bool {
    #[inline(always)]
    fn from(variant: MASK3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK3` reader - Hours mask"]
pub type MASK3_R = crate::BitReader<MASK3_A>;
impl MASK3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASK3_A {
        match self.bits {
            false => MASK3_A::NoMask,
            true => MASK3_A::Mask,
        }
    }
    #[doc = "No hour mask"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == MASK3_A::NoMask
    }
    #[doc = "Alarm clock doesn't care about hours"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MASK3_A::Mask
    }
}
#[doc = "Field `MASK3` writer - Hours mask"]
pub type MASK3_W<'a, REG> = crate::BitWriter<'a, REG, MASK3_A>;
impl<'a, REG> MASK3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No hour mask"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK3_A::NoMask)
    }
    #[doc = "Alarm clock doesn't care about hours"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK3_A::Mask)
    }
}
#[doc = "Field `DU` reader - Date units"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units"]
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DT` reader - Date tens"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Date/week mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKSEL_A {
    #[doc = "0: Date"]
    Date = 0,
    #[doc = "1: Week day"]
    WeekDay = 1,
}
impl From<WKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKSEL` reader - Date/week mode select"]
pub type WKSEL_R = crate::BitReader<WKSEL_A>;
impl WKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKSEL_A {
        match self.bits {
            false => WKSEL_A::Date,
            true => WKSEL_A::WeekDay,
        }
    }
    #[doc = "Date"]
    #[inline(always)]
    pub fn is_date(&self) -> bool {
        *self == WKSEL_A::Date
    }
    #[doc = "Week day"]
    #[inline(always)]
    pub fn is_week_day(&self) -> bool {
        *self == WKSEL_A::WeekDay
    }
}
#[doc = "Field `WKSEL` writer - Date/week mode select"]
pub type WKSEL_W<'a, REG> = crate::BitWriter<'a, REG, WKSEL_A>;
impl<'a, REG> WKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Date"]
    #[inline(always)]
    pub fn date(self) -> &'a mut crate::W<REG> {
        self.variant(WKSEL_A::Date)
    }
    #[doc = "Week day"]
    #[inline(always)]
    pub fn week_day(self) -> &'a mut crate::W<REG> {
        self.variant(WKSEL_A::WeekDay)
    }
}
#[doc = "Date/week mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK4_A {
    #[doc = "0: Date/week day is not masked"]
    NoMask = 0,
    #[doc = "1: Alarm clock doesn't care about date/week day"]
    Mask = 1,
}
impl From<MASK4_A> for bool {
    #[inline(always)]
    fn from(variant: MASK4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK4` reader - Date/week mask"]
pub type MASK4_R = crate::BitReader<MASK4_A>;
impl MASK4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASK4_A {
        match self.bits {
            false => MASK4_A::NoMask,
            true => MASK4_A::Mask,
        }
    }
    #[doc = "Date/week day is not masked"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == MASK4_A::NoMask
    }
    #[doc = "Alarm clock doesn't care about date/week day"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == MASK4_A::Mask
    }
}
#[doc = "Field `MASK4` writer - Date/week mask"]
pub type MASK4_W<'a, REG> = crate::BitWriter<'a, REG, MASK4_A>;
impl<'a, REG> MASK4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Date/week day is not masked"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK4_A::NoMask)
    }
    #[doc = "Alarm clock doesn't care about date/week day"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(MASK4_A::Mask)
    }
}
impl R {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Seconds mask"]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Minutes mask"]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hours mask"]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Date/week mode select"]
    #[inline(always)]
    pub fn wksel(&self) -> WKSEL_R {
        WKSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Date/week mask"]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALA")
            .field("mask4", &self.mask4())
            .field("wksel", &self.wksel())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .field("mask3", &self.mask3())
            .field("ampm", &self.ampm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mask2", &self.mask2())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("mask1", &self.mask1())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<ALA_SPEC> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<ALA_SPEC> {
        ST_W::new(self, 4)
    }
    #[doc = "Bit 7 - Seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask1(&mut self) -> MASK1_W<ALA_SPEC> {
        MASK1_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<ALA_SPEC> {
        MU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<ALA_SPEC> {
        MT_W::new(self, 12)
    }
    #[doc = "Bit 15 - Minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask2(&mut self) -> MASK2_W<ALA_SPEC> {
        MASK2_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<ALA_SPEC> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<ALA_SPEC> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AMPM_W<ALA_SPEC> {
        AMPM_W::new(self, 22)
    }
    #[doc = "Bit 23 - Hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask3(&mut self) -> MASK3_W<ALA_SPEC> {
        MASK3_W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<ALA_SPEC> {
        DU_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<ALA_SPEC> {
        DT_W::new(self, 28)
    }
    #[doc = "Bit 30 - Date/week mode select"]
    #[inline(always)]
    #[must_use]
    pub fn wksel(&mut self) -> WKSEL_W<ALA_SPEC> {
        WKSEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Date/week mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask4(&mut self) -> MASK4_W<ALA_SPEC> {
        MASK4_W::new(self, 31)
    }
}
#[doc = "Alarm A register\n\nYou can [`read`](crate::Reg::read) this register and get [`ala::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ala::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALA_SPEC;
impl crate::RegisterSpec for ALA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ala::R`](R) reader structure"]
impl crate::Readable for ALA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ala::W`](W) writer structure"]
impl crate::Writable for ALA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALA to value 0"]
impl crate::Resettable for ALA_SPEC {
    const RESET_VALUE: u32 = 0;
}
