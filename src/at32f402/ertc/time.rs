#[doc = "Register `TIME` reader"]
pub type R = crate::R<TIME_SPEC>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TIME_SPEC>;
#[doc = "Field `SU` reader - Second units"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `ST` reader - Second tens"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `MU` reader - Minute units"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MU` writer - Minute units"]
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `MT` reader - Minute tens"]
pub type MT_R = crate::FieldReader;
#[doc = "Field `MT` writer - Minute tens"]
pub type MT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `HU` reader - Hour units"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units"]
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `HT` reader - Hour tens"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "AM/PM notation\n\nValue on reset: 0"]
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
#[doc = "Field `AMPM` reader - AM/PM notation"]
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
#[doc = "Field `AMPM` writer - AM/PM notation"]
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
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME")
            .field("ampm", &self.ampm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mt", &self.mt())
            .field("mu", &self.mu())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<TIME_SPEC> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<TIME_SPEC> {
        ST_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<TIME_SPEC> {
        MU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<TIME_SPEC> {
        MT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<TIME_SPEC> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<TIME_SPEC> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AMPM_W<TIME_SPEC> {
        AMPM_W::new(self, 22)
    }
}
#[doc = "time register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
