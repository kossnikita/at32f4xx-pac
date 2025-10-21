#[doc = "Register `POLCFG1` reader"]
pub type R = crate::R<POLCFG1_SPEC>;
#[doc = "Register `POLCFG1` writer"]
pub type W = crate::W<POLCFG1_SPEC>;
#[doc = "Rising polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rp0r {
    #[doc = "0: Rising trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising trigger is enabled"]
    Enabled = 1,
}
impl From<Rp0r> for bool {
    #[inline(always)]
    fn from(variant: Rp0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RP(0-17)` reader - Rising polarity configuration bit on line %s"]
pub type RP_R = crate::BitReader<Rp0r>;
impl RP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rp0r {
        match self.bits {
            false => Rp0r::Disabled,
            true => Rp0r::Enabled,
        }
    }
    #[doc = "Rising trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rp0r::Disabled
    }
    #[doc = "Rising trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rp0r::Enabled
    }
}
#[doc = "Rising polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rp0wWO {
    #[doc = "0: Rising trigger disable"]
    Disable = 0,
    #[doc = "1: Rising trigger enable"]
    Enable = 1,
}
impl From<Rp0wWO> for bool {
    #[inline(always)]
    fn from(variant: Rp0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RP(0-17)` writer - Rising polarity configuration bit on line %s"]
pub type RP_W<'a, REG> = crate::BitWriter<'a, REG, Rp0wWO>;
impl<'a, REG> RP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rp0wWO::Disable)
    }
    #[doc = "Rising trigger enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rp0wWO::Enable)
    }
}
#[doc = "Field `RP19` reader - Rising polarity configuration bit of line 19"]
pub use RP_R as RP19_R;
#[doc = "Field `RP21` reader - Rising polarity configuration bit of line 21"]
pub use RP_R as RP21_R;
#[doc = "Field `RP19` writer - Rising polarity configuration bit of line 19"]
pub use RP_W as RP19_W;
#[doc = "Field `RP21` writer - Rising polarity configuration bit of line 21"]
pub use RP_W as RP21_W;
impl R {
    #[doc = "Rising polarity configuration bit on line (0-17)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RP0` field.</div>"]
    #[inline(always)]
    pub fn rp(&self, n: u8) -> RP_R {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        RP_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Rising polarity configuration bit on line (0-17)"]
    #[inline(always)]
    pub fn rp_iter(&self) -> impl Iterator<Item = RP_R> + '_ {
        (0..18).map(move |n| RP_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Rising polarity configuration bit on line 0"]
    #[inline(always)]
    pub fn rp0(&self) -> RP_R {
        RP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising polarity configuration bit on line 1"]
    #[inline(always)]
    pub fn rp1(&self) -> RP_R {
        RP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising polarity configuration bit on line 2"]
    #[inline(always)]
    pub fn rp2(&self) -> RP_R {
        RP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising polarity configuration bit on line 3"]
    #[inline(always)]
    pub fn rp3(&self) -> RP_R {
        RP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising polarity configuration bit on line 4"]
    #[inline(always)]
    pub fn rp4(&self) -> RP_R {
        RP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising polarity configuration bit on line 5"]
    #[inline(always)]
    pub fn rp5(&self) -> RP_R {
        RP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising polarity configuration bit on line 6"]
    #[inline(always)]
    pub fn rp6(&self) -> RP_R {
        RP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising polarity configuration bit on line 7"]
    #[inline(always)]
    pub fn rp7(&self) -> RP_R {
        RP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising polarity configuration bit on line 8"]
    #[inline(always)]
    pub fn rp8(&self) -> RP_R {
        RP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising polarity configuration bit on line 9"]
    #[inline(always)]
    pub fn rp9(&self) -> RP_R {
        RP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising polarity configuration bit on line 10"]
    #[inline(always)]
    pub fn rp10(&self) -> RP_R {
        RP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising polarity configuration bit on line 11"]
    #[inline(always)]
    pub fn rp11(&self) -> RP_R {
        RP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising polarity configuration bit on line 12"]
    #[inline(always)]
    pub fn rp12(&self) -> RP_R {
        RP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising polarity configuration bit on line 13"]
    #[inline(always)]
    pub fn rp13(&self) -> RP_R {
        RP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising polarity configuration bit on line 14"]
    #[inline(always)]
    pub fn rp14(&self) -> RP_R {
        RP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising polarity configuration bit on line 15"]
    #[inline(always)]
    pub fn rp15(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising polarity configuration bit on line 16"]
    #[inline(always)]
    pub fn rp16(&self) -> RP_R {
        RP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising polarity configuration bit on line 17"]
    #[inline(always)]
    pub fn rp17(&self) -> RP_R {
        RP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising polarity configuration bit of line 19"]
    #[inline(always)]
    pub fn rp19(&self) -> RP19_R {
        RP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising polarity configuration bit of line 21"]
    #[inline(always)]
    pub fn rp21(&self) -> RP21_R {
        RP21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLCFG1")
            .field("rp0", &self.rp0())
            .field("rp1", &self.rp1())
            .field("rp2", &self.rp2())
            .field("rp3", &self.rp3())
            .field("rp4", &self.rp4())
            .field("rp5", &self.rp5())
            .field("rp6", &self.rp6())
            .field("rp7", &self.rp7())
            .field("rp8", &self.rp8())
            .field("rp9", &self.rp9())
            .field("rp10", &self.rp10())
            .field("rp11", &self.rp11())
            .field("rp12", &self.rp12())
            .field("rp13", &self.rp13())
            .field("rp14", &self.rp14())
            .field("rp15", &self.rp15())
            .field("rp16", &self.rp16())
            .field("rp17", &self.rp17())
            .field("rp19", &self.rp19())
            .field("rp21", &self.rp21())
            .finish()
    }
}
impl W {
    #[doc = "Rising polarity configuration bit on line (0-17)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RP0` field.</div>"]
    #[inline(always)]
    pub fn rp(&mut self, n: u8) -> RP_W<'_, POLCFG1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        RP_W::new(self, n)
    }
    #[doc = "Bit 0 - Rising polarity configuration bit on line 0"]
    #[inline(always)]
    pub fn rp0(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising polarity configuration bit on line 1"]
    #[inline(always)]
    pub fn rp1(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising polarity configuration bit on line 2"]
    #[inline(always)]
    pub fn rp2(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising polarity configuration bit on line 3"]
    #[inline(always)]
    pub fn rp3(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising polarity configuration bit on line 4"]
    #[inline(always)]
    pub fn rp4(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising polarity configuration bit on line 5"]
    #[inline(always)]
    pub fn rp5(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising polarity configuration bit on line 6"]
    #[inline(always)]
    pub fn rp6(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising polarity configuration bit on line 7"]
    #[inline(always)]
    pub fn rp7(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising polarity configuration bit on line 8"]
    #[inline(always)]
    pub fn rp8(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising polarity configuration bit on line 9"]
    #[inline(always)]
    pub fn rp9(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising polarity configuration bit on line 10"]
    #[inline(always)]
    pub fn rp10(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising polarity configuration bit on line 11"]
    #[inline(always)]
    pub fn rp11(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising polarity configuration bit on line 12"]
    #[inline(always)]
    pub fn rp12(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising polarity configuration bit on line 13"]
    #[inline(always)]
    pub fn rp13(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising polarity configuration bit on line 14"]
    #[inline(always)]
    pub fn rp14(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising polarity configuration bit on line 15"]
    #[inline(always)]
    pub fn rp15(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising polarity configuration bit on line 16"]
    #[inline(always)]
    pub fn rp16(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising polarity configuration bit on line 17"]
    #[inline(always)]
    pub fn rp17(&mut self) -> RP_W<'_, POLCFG1_SPEC> {
        RP_W::new(self, 17)
    }
    #[doc = "Bit 19 - Rising polarity configuration bit of line 19"]
    #[inline(always)]
    pub fn rp19(&mut self) -> RP19_W<'_, POLCFG1_SPEC> {
        RP19_W::new(self, 19)
    }
    #[doc = "Bit 21 - Rising polarity configuration bit of line 21"]
    #[inline(always)]
    pub fn rp21(&mut self) -> RP21_W<'_, POLCFG1_SPEC> {
        RP21_W::new(self, 21)
    }
}
#[doc = "Rising polarity configuration register(EXTINT_POLCFG1)\n\nYou can [`read`](crate::Reg::read) this register and get [`polcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLCFG1_SPEC;
impl crate::RegisterSpec for POLCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polcfg1::R`](R) reader structure"]
impl crate::Readable for POLCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`polcfg1::W`](W) writer structure"]
impl crate::Writable for POLCFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLCFG1 to value 0"]
impl crate::Resettable for POLCFG1_SPEC {}
