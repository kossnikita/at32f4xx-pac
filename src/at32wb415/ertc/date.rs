#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `DU` reader - Date units"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units"]
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DT` reader - Date tens"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `MU` reader - Month units"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MU` writer - Month units"]
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `MT` reader - Month tens"]
pub type MT_R = crate::BitReader;
#[doc = "Field `MT` writer - Month tens"]
pub type MT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Week\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WK_A {
    #[doc = "1: Monday"]
    Monday = 1,
    #[doc = "2: Tuesday"]
    Tuesday = 2,
    #[doc = "3: Wednesday"]
    Wednesday = 3,
    #[doc = "4: Thursday"]
    Thursday = 4,
    #[doc = "5: Friday"]
    Friday = 5,
    #[doc = "6: Saturday"]
    Saturday = 6,
    #[doc = "7: Sunday"]
    Sunday = 7,
}
impl From<WK_A> for u8 {
    #[inline(always)]
    fn from(variant: WK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WK_A {
    type Ux = u8;
}
impl crate::IsEnum for WK_A {}
#[doc = "Field `WK` reader - Week"]
pub type WK_R = crate::FieldReader<WK_A>;
impl WK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WK_A> {
        match self.bits {
            1 => Some(WK_A::Monday),
            2 => Some(WK_A::Tuesday),
            3 => Some(WK_A::Wednesday),
            4 => Some(WK_A::Thursday),
            5 => Some(WK_A::Friday),
            6 => Some(WK_A::Saturday),
            7 => Some(WK_A::Sunday),
            _ => None,
        }
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn is_monday(&self) -> bool {
        *self == WK_A::Monday
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn is_tuesday(&self) -> bool {
        *self == WK_A::Tuesday
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn is_wednesday(&self) -> bool {
        *self == WK_A::Wednesday
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn is_thursday(&self) -> bool {
        *self == WK_A::Thursday
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn is_friday(&self) -> bool {
        *self == WK_A::Friday
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn is_saturday(&self) -> bool {
        *self == WK_A::Saturday
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn is_sunday(&self) -> bool {
        *self == WK_A::Sunday
    }
}
#[doc = "Field `WK` writer - Week"]
pub type WK_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WK_A>;
impl<'a, REG> WK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Monday"]
    #[inline(always)]
    pub fn monday(self) -> &'a mut crate::W<REG> {
        self.variant(WK_A::Monday)
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn tuesday(self) -> &'a mut crate::W<REG> {
        self.variant(WK_A::Tuesday)
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn wednesday(self) -> &'a mut crate::W<REG> {
        self.variant(WK_A::Wednesday)
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn thursday(self) -> &'a mut crate::W<REG> {
        self.variant(WK_A::Thursday)
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn friday(self) -> &'a mut crate::W<REG> {
        self.variant(WK_A::Friday)
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn saturday(self) -> &'a mut crate::W<REG> {
        self.variant(WK_A::Saturday)
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn sunday(self) -> &'a mut crate::W<REG> {
        self.variant(WK_A::Sunday)
    }
}
#[doc = "Field `YU` reader - Year units"]
pub type YU_R = crate::FieldReader;
#[doc = "Field `YU` writer - Year units"]
pub type YU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `YT` reader - Year tens"]
pub type YT_R = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens"]
pub type YT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:3 - Date units"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week"]
    #[inline(always)]
    pub fn wk(&self) -> WK_R {
        WK_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units"]
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens"]
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("yt", &self.yt())
            .field("yu", &self.yu())
            .field("wk", &self.wk())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<DATE_SPEC> {
        DU_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<DATE_SPEC> {
        DT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<DATE_SPEC> {
        MU_W::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<DATE_SPEC> {
        MT_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week"]
    #[inline(always)]
    #[must_use]
    pub fn wk(&mut self) -> WK_W<DATE_SPEC> {
        WK_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units"]
    #[inline(always)]
    #[must_use]
    pub fn yu(&mut self) -> YU_W<DATE_SPEC> {
        YU_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens"]
    #[inline(always)]
    #[must_use]
    pub fn yt(&mut self) -> YT_W<DATE_SPEC> {
        YT_W::new(self, 20)
    }
}
#[doc = "date register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0x2101"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x2101;
}
