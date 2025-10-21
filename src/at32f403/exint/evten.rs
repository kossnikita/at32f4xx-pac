#[doc = "Register `EVTEN` reader"]
pub type R = crate::R<EVTEN_SPEC>;
#[doc = "Register `EVTEN` writer"]
pub type W = crate::W<EVTEN_SPEC>;
#[doc = "Field `EVTEN0` reader - Event enable or disable on line 0"]
pub type EVTEN0_R = crate::BitReader;
#[doc = "Field `EVTEN0` writer - Event enable or disable on line 0"]
pub type EVTEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN1` reader - Event enable or disable on line 1"]
pub type EVTEN1_R = crate::BitReader;
#[doc = "Field `EVTEN1` writer - Event enable or disable on line 1"]
pub type EVTEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN2` reader - Event enable or disable on line 2"]
pub type EVTEN2_R = crate::BitReader;
#[doc = "Field `EVTEN2` writer - Event enable or disable on line 2"]
pub type EVTEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN3` reader - Event enable or disable on line 3"]
pub type EVTEN3_R = crate::BitReader;
#[doc = "Field `EVTEN3` writer - Event enable or disable on line 3"]
pub type EVTEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN4` reader - Event enable or disable on line 4"]
pub type EVTEN4_R = crate::BitReader;
#[doc = "Field `EVTEN4` writer - Event enable or disable on line 4"]
pub type EVTEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN5` reader - Event enable or disable on line 5"]
pub type EVTEN5_R = crate::BitReader;
#[doc = "Field `EVTEN5` writer - Event enable or disable on line 5"]
pub type EVTEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN6` reader - Event enable or disable on line 6"]
pub type EVTEN6_R = crate::BitReader;
#[doc = "Field `EVTEN6` writer - Event enable or disable on line 6"]
pub type EVTEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN7` reader - Event enable or disable on line 7"]
pub type EVTEN7_R = crate::BitReader;
#[doc = "Field `EVTEN7` writer - Event enable or disable on line 7"]
pub type EVTEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN8` reader - Event enable or disable on line 8"]
pub type EVTEN8_R = crate::BitReader;
#[doc = "Field `EVTEN8` writer - Event enable or disable on line 8"]
pub type EVTEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN9` reader - Event enable or disable on line 9"]
pub type EVTEN9_R = crate::BitReader;
#[doc = "Field `EVTEN9` writer - Event enable or disable on line 9"]
pub type EVTEN9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN10` reader - Event enable or disable on line 10"]
pub type EVTEN10_R = crate::BitReader;
#[doc = "Field `EVTEN10` writer - Event enable or disable on line 10"]
pub type EVTEN10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN11` reader - Event enable or disable on line 11"]
pub type EVTEN11_R = crate::BitReader;
#[doc = "Field `EVTEN11` writer - Event enable or disable on line 11"]
pub type EVTEN11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN12` reader - Event enable or disable on line 12"]
pub type EVTEN12_R = crate::BitReader;
#[doc = "Field `EVTEN12` writer - Event enable or disable on line 12"]
pub type EVTEN12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN13` reader - Event enable or disable on line 13"]
pub type EVTEN13_R = crate::BitReader;
#[doc = "Field `EVTEN13` writer - Event enable or disable on line 13"]
pub type EVTEN13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN14` reader - Event enable or disable on line 14"]
pub type EVTEN14_R = crate::BitReader;
#[doc = "Field `EVTEN14` writer - Event enable or disable on line 14"]
pub type EVTEN14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN15` reader - Event enable or disable on line 15"]
pub type EVTEN15_R = crate::BitReader;
#[doc = "Field `EVTEN15` writer - Event enable or disable on line 15"]
pub type EVTEN15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN16` reader - Event enable or disable on line 16"]
pub type EVTEN16_R = crate::BitReader;
#[doc = "Field `EVTEN16` writer - Event enable or disable on line 16"]
pub type EVTEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN17` reader - Event enable or disable on line 17"]
pub type EVTEN17_R = crate::BitReader;
#[doc = "Field `EVTEN17` writer - Event enable or disable on line 17"]
pub type EVTEN17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTEN18` reader - Event enable or disable on line 18"]
pub type EVTEN18_R = crate::BitReader;
#[doc = "Field `EVTEN18` writer - Event enable or disable on line 18"]
pub type EVTEN18_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event enable or disable on line 0"]
    #[inline(always)]
    pub fn evten0(&self) -> EVTEN0_R {
        EVTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event enable or disable on line 1"]
    #[inline(always)]
    pub fn evten1(&self) -> EVTEN1_R {
        EVTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event enable or disable on line 2"]
    #[inline(always)]
    pub fn evten2(&self) -> EVTEN2_R {
        EVTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event enable or disable on line 3"]
    #[inline(always)]
    pub fn evten3(&self) -> EVTEN3_R {
        EVTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event enable or disable on line 4"]
    #[inline(always)]
    pub fn evten4(&self) -> EVTEN4_R {
        EVTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event enable or disable on line 5"]
    #[inline(always)]
    pub fn evten5(&self) -> EVTEN5_R {
        EVTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event enable or disable on line 6"]
    #[inline(always)]
    pub fn evten6(&self) -> EVTEN6_R {
        EVTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event enable or disable on line 7"]
    #[inline(always)]
    pub fn evten7(&self) -> EVTEN7_R {
        EVTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event enable or disable on line 8"]
    #[inline(always)]
    pub fn evten8(&self) -> EVTEN8_R {
        EVTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event enable or disable on line 9"]
    #[inline(always)]
    pub fn evten9(&self) -> EVTEN9_R {
        EVTEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event enable or disable on line 10"]
    #[inline(always)]
    pub fn evten10(&self) -> EVTEN10_R {
        EVTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event enable or disable on line 11"]
    #[inline(always)]
    pub fn evten11(&self) -> EVTEN11_R {
        EVTEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event enable or disable on line 12"]
    #[inline(always)]
    pub fn evten12(&self) -> EVTEN12_R {
        EVTEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event enable or disable on line 13"]
    #[inline(always)]
    pub fn evten13(&self) -> EVTEN13_R {
        EVTEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event enable or disable on line 14"]
    #[inline(always)]
    pub fn evten14(&self) -> EVTEN14_R {
        EVTEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event enable or disable on line 15"]
    #[inline(always)]
    pub fn evten15(&self) -> EVTEN15_R {
        EVTEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event enable or disable on line 16"]
    #[inline(always)]
    pub fn evten16(&self) -> EVTEN16_R {
        EVTEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event enable or disable on line 17"]
    #[inline(always)]
    pub fn evten17(&self) -> EVTEN17_R {
        EVTEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event enable or disable on line 18"]
    #[inline(always)]
    pub fn evten18(&self) -> EVTEN18_R {
        EVTEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTEN")
            .field("evten0", &self.evten0())
            .field("evten1", &self.evten1())
            .field("evten2", &self.evten2())
            .field("evten3", &self.evten3())
            .field("evten4", &self.evten4())
            .field("evten5", &self.evten5())
            .field("evten6", &self.evten6())
            .field("evten7", &self.evten7())
            .field("evten8", &self.evten8())
            .field("evten9", &self.evten9())
            .field("evten10", &self.evten10())
            .field("evten11", &self.evten11())
            .field("evten12", &self.evten12())
            .field("evten13", &self.evten13())
            .field("evten14", &self.evten14())
            .field("evten15", &self.evten15())
            .field("evten16", &self.evten16())
            .field("evten17", &self.evten17())
            .field("evten18", &self.evten18())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Event enable or disable on line 0"]
    #[inline(always)]
    pub fn evten0(&mut self) -> EVTEN0_W<'_, EVTEN_SPEC> {
        EVTEN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event enable or disable on line 1"]
    #[inline(always)]
    pub fn evten1(&mut self) -> EVTEN1_W<'_, EVTEN_SPEC> {
        EVTEN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event enable or disable on line 2"]
    #[inline(always)]
    pub fn evten2(&mut self) -> EVTEN2_W<'_, EVTEN_SPEC> {
        EVTEN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event enable or disable on line 3"]
    #[inline(always)]
    pub fn evten3(&mut self) -> EVTEN3_W<'_, EVTEN_SPEC> {
        EVTEN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event enable or disable on line 4"]
    #[inline(always)]
    pub fn evten4(&mut self) -> EVTEN4_W<'_, EVTEN_SPEC> {
        EVTEN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event enable or disable on line 5"]
    #[inline(always)]
    pub fn evten5(&mut self) -> EVTEN5_W<'_, EVTEN_SPEC> {
        EVTEN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Event enable or disable on line 6"]
    #[inline(always)]
    pub fn evten6(&mut self) -> EVTEN6_W<'_, EVTEN_SPEC> {
        EVTEN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Event enable or disable on line 7"]
    #[inline(always)]
    pub fn evten7(&mut self) -> EVTEN7_W<'_, EVTEN_SPEC> {
        EVTEN7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Event enable or disable on line 8"]
    #[inline(always)]
    pub fn evten8(&mut self) -> EVTEN8_W<'_, EVTEN_SPEC> {
        EVTEN8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event enable or disable on line 9"]
    #[inline(always)]
    pub fn evten9(&mut self) -> EVTEN9_W<'_, EVTEN_SPEC> {
        EVTEN9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event enable or disable on line 10"]
    #[inline(always)]
    pub fn evten10(&mut self) -> EVTEN10_W<'_, EVTEN_SPEC> {
        EVTEN10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Event enable or disable on line 11"]
    #[inline(always)]
    pub fn evten11(&mut self) -> EVTEN11_W<'_, EVTEN_SPEC> {
        EVTEN11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Event enable or disable on line 12"]
    #[inline(always)]
    pub fn evten12(&mut self) -> EVTEN12_W<'_, EVTEN_SPEC> {
        EVTEN12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Event enable or disable on line 13"]
    #[inline(always)]
    pub fn evten13(&mut self) -> EVTEN13_W<'_, EVTEN_SPEC> {
        EVTEN13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Event enable or disable on line 14"]
    #[inline(always)]
    pub fn evten14(&mut self) -> EVTEN14_W<'_, EVTEN_SPEC> {
        EVTEN14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Event enable or disable on line 15"]
    #[inline(always)]
    pub fn evten15(&mut self) -> EVTEN15_W<'_, EVTEN_SPEC> {
        EVTEN15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Event enable or disable on line 16"]
    #[inline(always)]
    pub fn evten16(&mut self) -> EVTEN16_W<'_, EVTEN_SPEC> {
        EVTEN16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Event enable or disable on line 17"]
    #[inline(always)]
    pub fn evten17(&mut self) -> EVTEN17_W<'_, EVTEN_SPEC> {
        EVTEN17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Event enable or disable on line 18"]
    #[inline(always)]
    pub fn evten18(&mut self) -> EVTEN18_W<'_, EVTEN_SPEC> {
        EVTEN18_W::new(self, 18)
    }
}
#[doc = "Event enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`evten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVTEN_SPEC;
impl crate::RegisterSpec for EVTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evten::R`](R) reader structure"]
impl crate::Readable for EVTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evten::W`](W) writer structure"]
impl crate::Writable for EVTEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EVTEN_SPEC {}
