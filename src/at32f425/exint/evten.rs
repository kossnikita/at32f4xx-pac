#[doc = "Register `EVTEN` reader"]
pub type R = crate::R<EVTEN_SPEC>;
#[doc = "Register `EVTEN` writer"]
pub type W = crate::W<EVTEN_SPEC>;
#[doc = "Event enable or disable on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evten0r {
    #[doc = "0: Event request is disabled"]
    Disabled = 0,
    #[doc = "1: Event request is enabled"]
    Enabled = 1,
}
impl From<Evten0r> for bool {
    #[inline(always)]
    fn from(variant: Evten0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTEN(0-20)` reader - Event enable or disable on line %s"]
pub type EVTEN_R = crate::BitReader<Evten0r>;
impl EVTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evten0r {
        match self.bits {
            false => Evten0r::Disabled,
            true => Evten0r::Enabled,
        }
    }
    #[doc = "Event request is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Evten0r::Disabled
    }
    #[doc = "Event request is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Evten0r::Enabled
    }
}
#[doc = "Event enable or disable on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evten0wWO {
    #[doc = "0: Event request disable"]
    Disable = 0,
    #[doc = "1: Event request enable"]
    Enable = 1,
}
impl From<Evten0wWO> for bool {
    #[inline(always)]
    fn from(variant: Evten0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTEN(0-20)` writer - Event enable or disable on line %s"]
pub type EVTEN_W<'a, REG> = crate::BitWriter<'a, REG, Evten0wWO>;
impl<'a, REG> EVTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event request disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Evten0wWO::Disable)
    }
    #[doc = "Event request enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Evten0wWO::Enable)
    }
}
impl R {
    #[doc = "Event enable or disable on line (0-20)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EVTEN0` field.</div>"]
    #[inline(always)]
    pub fn evten(&self, n: u8) -> EVTEN_R {
        #[allow(clippy::no_effect)]
        [(); 21][n as usize];
        EVTEN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Event enable or disable on line (0-20)"]
    #[inline(always)]
    pub fn evten_iter(&self) -> impl Iterator<Item = EVTEN_R> + '_ {
        (0..21).map(move |n| EVTEN_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Event enable or disable on line 0"]
    #[inline(always)]
    pub fn evten0(&self) -> EVTEN_R {
        EVTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event enable or disable on line 1"]
    #[inline(always)]
    pub fn evten1(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event enable or disable on line 2"]
    #[inline(always)]
    pub fn evten2(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event enable or disable on line 3"]
    #[inline(always)]
    pub fn evten3(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event enable or disable on line 4"]
    #[inline(always)]
    pub fn evten4(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event enable or disable on line 5"]
    #[inline(always)]
    pub fn evten5(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event enable or disable on line 6"]
    #[inline(always)]
    pub fn evten6(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event enable or disable on line 7"]
    #[inline(always)]
    pub fn evten7(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event enable or disable on line 8"]
    #[inline(always)]
    pub fn evten8(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event enable or disable on line 9"]
    #[inline(always)]
    pub fn evten9(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event enable or disable on line 10"]
    #[inline(always)]
    pub fn evten10(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event enable or disable on line 11"]
    #[inline(always)]
    pub fn evten11(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event enable or disable on line 12"]
    #[inline(always)]
    pub fn evten12(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event enable or disable on line 13"]
    #[inline(always)]
    pub fn evten13(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event enable or disable on line 14"]
    #[inline(always)]
    pub fn evten14(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event enable or disable on line 15"]
    #[inline(always)]
    pub fn evten15(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event enable or disable on line 16"]
    #[inline(always)]
    pub fn evten16(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event enable or disable on line 17"]
    #[inline(always)]
    pub fn evten17(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Event enable or disable on line 18"]
    #[inline(always)]
    pub fn evten18(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Event enable or disable on line 19"]
    #[inline(always)]
    pub fn evten19(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Event enable or disable on line 20"]
    #[inline(always)]
    pub fn evten20(&self) -> EVTEN_R {
        EVTEN_R::new(((self.bits >> 20) & 1) != 0)
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
            .field("evten19", &self.evten19())
            .field("evten20", &self.evten20())
            .finish()
    }
}
impl W {
    #[doc = "Event enable or disable on line (0-20)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EVTEN0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn evten(&mut self, n: u8) -> EVTEN_W<EVTEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 21][n as usize];
        EVTEN_W::new(self, n)
    }
    #[doc = "Bit 0 - Event enable or disable on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn evten0(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event enable or disable on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn evten1(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event enable or disable on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn evten2(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event enable or disable on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn evten3(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event enable or disable on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn evten4(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event enable or disable on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn evten5(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Event enable or disable on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn evten6(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Event enable or disable on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn evten7(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Event enable or disable on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn evten8(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event enable or disable on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn evten9(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Event enable or disable on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn evten10(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Event enable or disable on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn evten11(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Event enable or disable on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn evten12(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Event enable or disable on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn evten13(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Event enable or disable on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn evten14(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Event enable or disable on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn evten15(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Event enable or disable on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn evten16(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Event enable or disable on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn evten17(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Event enable or disable on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn evten18(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Event enable or disable on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn evten19(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Event enable or disable on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn evten20(&mut self) -> EVTEN_W<EVTEN_SPEC> {
        EVTEN_W::new(self, 20)
    }
}
#[doc = "Event enable register (EXTINT_EVTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`evten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVTEN_SPEC;
impl crate::RegisterSpec for EVTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evten::R`](R) reader structure"]
impl crate::Readable for EVTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evten::W`](W) writer structure"]
impl crate::Writable for EVTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EVTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
