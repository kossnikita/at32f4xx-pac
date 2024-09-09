#[doc = "Register `ODT` reader"]
pub type R = crate::R<ODT_SPEC>;
#[doc = "Register `ODT` writer"]
pub type W = crate::W<ODT_SPEC>;
#[doc = "Port output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Odt0r {
    #[doc = "0: Pull-down"]
    PullDown = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
}
impl From<Odt0r> for bool {
    #[inline(always)]
    fn from(variant: Odt0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODT(0-15)` reader - Port output data"]
pub type ODT_R = crate::BitReader<Odt0r>;
impl ODT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Odt0r {
        match self.bits {
            false => Odt0r::PullDown,
            true => Odt0r::PullUp,
        }
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Odt0r::PullDown
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Odt0r::PullUp
    }
}
#[doc = "Port output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Odt0wWO {
    #[doc = "0: Set output to logic low"]
    Low = 0,
    #[doc = "1: Set output to logic high"]
    High = 1,
}
impl From<Odt0wWO> for bool {
    #[inline(always)]
    fn from(variant: Odt0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODT(0-15)` writer - Port output data"]
pub type ODT_W<'a, REG> = crate::BitWriter<'a, REG, Odt0wWO>;
impl<'a, REG> ODT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Odt0wWO::Low)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Odt0wWO::High)
    }
}
impl R {
    #[doc = "Port output data"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ODT0` field.</div>"]
    #[inline(always)]
    pub fn odt(&self, n: u8) -> ODT_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ODT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port output data"]
    #[inline(always)]
    pub fn odt_iter(&self) -> impl Iterator<Item = ODT_R> + '_ {
        (0..16).map(move |n| ODT_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    pub fn odt0(&self) -> ODT_R {
        ODT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    pub fn odt1(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    pub fn odt2(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    pub fn odt3(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    pub fn odt4(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    pub fn odt5(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    pub fn odt6(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    pub fn odt7(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    pub fn odt8(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    pub fn odt9(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    pub fn odt10(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    pub fn odt11(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    pub fn odt12(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    pub fn odt13(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    pub fn odt14(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    pub fn odt15(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODT")
            .field("odt0", &self.odt0())
            .field("odt1", &self.odt1())
            .field("odt2", &self.odt2())
            .field("odt3", &self.odt3())
            .field("odt4", &self.odt4())
            .field("odt5", &self.odt5())
            .field("odt6", &self.odt6())
            .field("odt7", &self.odt7())
            .field("odt8", &self.odt8())
            .field("odt9", &self.odt9())
            .field("odt10", &self.odt10())
            .field("odt11", &self.odt11())
            .field("odt12", &self.odt12())
            .field("odt13", &self.odt13())
            .field("odt14", &self.odt14())
            .field("odt15", &self.odt15())
            .finish()
    }
}
impl W {
    #[doc = "Port output data"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ODT0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn odt(&mut self, n: u8) -> ODT_W<ODT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ODT_W::new(self, n)
    }
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt0(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt1(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt2(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt3(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt4(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt5(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt6(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt7(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt8(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt9(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt10(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt11(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt12(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt13(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt14(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt15(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 15)
    }
}
#[doc = "Port output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODT_SPEC;
impl crate::RegisterSpec for ODT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for ODT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odt::W`](W) writer structure"]
impl crate::Writable for ODT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for ODT_SPEC {
    const RESET_VALUE: u32 = 0;
}
