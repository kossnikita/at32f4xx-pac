#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Interrupt enable or disable on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten0r {
    #[doc = "0: Interrupt request is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt request is enabled"]
    Enabled = 1,
}
impl From<Inten0r> for bool {
    #[inline(always)]
    fn from(variant: Inten0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN(0-19)` reader - Interrupt enable or disable on line %s"]
pub type INTEN_R = crate::BitReader<Inten0r>;
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten0r {
        match self.bits {
            false => Inten0r::Disabled,
            true => Inten0r::Enabled,
        }
    }
    #[doc = "Interrupt request is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inten0r::Disabled
    }
    #[doc = "Interrupt request is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inten0r::Enabled
    }
}
#[doc = "Interrupt enable or disable on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten0wWO {
    #[doc = "0: Interrupt request disable"]
    Disable = 0,
    #[doc = "1: Interrupt request enable"]
    Enable = 1,
}
impl From<Inten0wWO> for bool {
    #[inline(always)]
    fn from(variant: Inten0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN(0-19)` writer - Interrupt enable or disable on line %s"]
pub type INTEN_W<'a, REG> = crate::BitWriter<'a, REG, Inten0wWO>;
impl<'a, REG> INTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten0wWO::Disable)
    }
    #[doc = "Interrupt request enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten0wWO::Enable)
    }
}
impl R {
    #[doc = "Interrupt enable or disable on line (0-19)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INTEN0` field.</div>"]
    #[inline(always)]
    pub fn inten(&self, n: u8) -> INTEN_R {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        INTEN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Interrupt enable or disable on line (0-19)"]
    #[inline(always)]
    pub fn inten_iter(&self) -> impl Iterator<Item = INTEN_R> + '_ {
        (0..20).map(move |n| INTEN_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Interrupt enable or disable on line 0"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable or disable on line 1"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable or disable on line 2"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable or disable on line 3"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable or disable on line 4"]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable or disable on line 5"]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable or disable on line 6"]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable or disable on line 7"]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable or disable on line 8"]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable or disable on line 9"]
    #[inline(always)]
    pub fn inten9(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable or disable on line 10"]
    #[inline(always)]
    pub fn inten10(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt enable or disable on line 11"]
    #[inline(always)]
    pub fn inten11(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt enable or disable on line 12"]
    #[inline(always)]
    pub fn inten12(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt enable or disable on line 13"]
    #[inline(always)]
    pub fn inten13(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt enable or disable on line 14"]
    #[inline(always)]
    pub fn inten14(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt enable or disable on line 15"]
    #[inline(always)]
    pub fn inten15(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable or disable on line 16"]
    #[inline(always)]
    pub fn inten16(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable or disable on line 17"]
    #[inline(always)]
    pub fn inten17(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable or disable on line 18"]
    #[inline(always)]
    pub fn inten18(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt enable or disable on line 19"]
    #[inline(always)]
    pub fn inten19(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("inten0", &self.inten0())
            .field("inten1", &self.inten1())
            .field("inten2", &self.inten2())
            .field("inten3", &self.inten3())
            .field("inten4", &self.inten4())
            .field("inten5", &self.inten5())
            .field("inten6", &self.inten6())
            .field("inten7", &self.inten7())
            .field("inten8", &self.inten8())
            .field("inten9", &self.inten9())
            .field("inten10", &self.inten10())
            .field("inten11", &self.inten11())
            .field("inten12", &self.inten12())
            .field("inten13", &self.inten13())
            .field("inten14", &self.inten14())
            .field("inten15", &self.inten15())
            .field("inten16", &self.inten16())
            .field("inten17", &self.inten17())
            .field("inten18", &self.inten18())
            .field("inten19", &self.inten19())
            .finish()
    }
}
impl W {
    #[doc = "Interrupt enable or disable on line (0-19)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INTEN0` field.</div>"]
    #[inline(always)]
    pub fn inten(&mut self, n: u8) -> INTEN_W<'_, INTEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 20][n as usize];
        INTEN_W::new(self, n)
    }
    #[doc = "Bit 0 - Interrupt enable or disable on line 0"]
    #[inline(always)]
    pub fn inten0(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable or disable on line 1"]
    #[inline(always)]
    pub fn inten1(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable or disable on line 2"]
    #[inline(always)]
    pub fn inten2(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable or disable on line 3"]
    #[inline(always)]
    pub fn inten3(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable or disable on line 4"]
    #[inline(always)]
    pub fn inten4(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable or disable on line 5"]
    #[inline(always)]
    pub fn inten5(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt enable or disable on line 6"]
    #[inline(always)]
    pub fn inten6(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable or disable on line 7"]
    #[inline(always)]
    pub fn inten7(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt enable or disable on line 8"]
    #[inline(always)]
    pub fn inten8(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable or disable on line 9"]
    #[inline(always)]
    pub fn inten9(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt enable or disable on line 10"]
    #[inline(always)]
    pub fn inten10(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt enable or disable on line 11"]
    #[inline(always)]
    pub fn inten11(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt enable or disable on line 12"]
    #[inline(always)]
    pub fn inten12(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt enable or disable on line 13"]
    #[inline(always)]
    pub fn inten13(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt enable or disable on line 14"]
    #[inline(always)]
    pub fn inten14(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt enable or disable on line 15"]
    #[inline(always)]
    pub fn inten15(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt enable or disable on line 16"]
    #[inline(always)]
    pub fn inten16(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt enable or disable on line 17"]
    #[inline(always)]
    pub fn inten17(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt enable or disable on line 18"]
    #[inline(always)]
    pub fn inten18(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt enable or disable on line 19"]
    #[inline(always)]
    pub fn inten19(&mut self) -> INTEN_W<'_, INTEN_SPEC> {
        INTEN_W::new(self, 19)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {}
