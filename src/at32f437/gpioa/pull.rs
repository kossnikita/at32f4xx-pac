#[doc = "Register `PULL` reader"]
pub type R = crate::R<PULL_SPEC>;
#[doc = "Register `PULL` writer"]
pub type W = crate::W<PULL_SPEC>;
#[doc = "GPIOx pin %s pull configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PULL0_A {
    #[doc = "0: No pull-up/pull-down"]
    NoPull = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<PULL0_A> for u8 {
    #[inline(always)]
    fn from(variant: PULL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PULL0_A {
    type Ux = u8;
}
impl crate::IsEnum for PULL0_A {}
#[doc = "Field `PULL(0-15)` reader - GPIOx pin %s pull configuration"]
pub type PULL_R = crate::FieldReader<PULL0_A>;
impl PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PULL0_A> {
        match self.bits {
            0 => Some(PULL0_A::NoPull),
            1 => Some(PULL0_A::PullUp),
            2 => Some(PULL0_A::PullDown),
            _ => None,
        }
    }
    #[doc = "No pull-up/pull-down"]
    #[inline(always)]
    pub fn is_no_pull(&self) -> bool {
        *self == PULL0_A::NoPull
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PULL0_A::PullUp
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PULL0_A::PullDown
    }
}
#[doc = "Field `PULL(0-15)` writer - GPIOx pin %s pull configuration"]
pub type PULL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PULL0_A>;
impl<'a, REG> PULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up/pull-down"]
    #[inline(always)]
    pub fn no_pull(self) -> &'a mut crate::W<REG> {
        self.variant(PULL0_A::NoPull)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PULL0_A::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PULL0_A::PullDown)
    }
}
impl R {
    #[doc = "GPIOx pin (0-15) pull configuration"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PULL0` field.</div>"]
    #[inline(always)]
    pub fn pull(&self, n: u8) -> PULL_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "GPIOx pin (0-15) pull configuration"]
    #[inline(always)]
    pub fn pull_iter(&self) -> impl Iterator<Item = PULL_R> + '_ {
        (0..16).map(move |n| PULL_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - GPIOx pin 0 pull configuration"]
    #[inline(always)]
    pub fn pull0(&self) -> PULL_R {
        PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 pull configuration"]
    #[inline(always)]
    pub fn pull1(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 pull configuration"]
    #[inline(always)]
    pub fn pull2(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 pull configuration"]
    #[inline(always)]
    pub fn pull3(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 pull configuration"]
    #[inline(always)]
    pub fn pull4(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 pull configuration"]
    #[inline(always)]
    pub fn pull5(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 pull configuration"]
    #[inline(always)]
    pub fn pull6(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 pull configuration"]
    #[inline(always)]
    pub fn pull7(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 pull configuration"]
    #[inline(always)]
    pub fn pull8(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 pull configuration"]
    #[inline(always)]
    pub fn pull9(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 pull configuration"]
    #[inline(always)]
    pub fn pull10(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 pull configuration"]
    #[inline(always)]
    pub fn pull11(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 pull configuration"]
    #[inline(always)]
    pub fn pull12(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 pull configuration"]
    #[inline(always)]
    pub fn pull13(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 pull configuration"]
    #[inline(always)]
    pub fn pull14(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 pull configuration"]
    #[inline(always)]
    pub fn pull15(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PULL")
            .field("pull0", &self.pull0())
            .field("pull1", &self.pull1())
            .field("pull2", &self.pull2())
            .field("pull3", &self.pull3())
            .field("pull4", &self.pull4())
            .field("pull5", &self.pull5())
            .field("pull6", &self.pull6())
            .field("pull7", &self.pull7())
            .field("pull8", &self.pull8())
            .field("pull9", &self.pull9())
            .field("pull10", &self.pull10())
            .field("pull11", &self.pull11())
            .field("pull12", &self.pull12())
            .field("pull13", &self.pull13())
            .field("pull14", &self.pull14())
            .field("pull15", &self.pull15())
            .finish()
    }
}
impl W {
    #[doc = "GPIOx pin (0-15) pull configuration"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PULL0` field.</div>"]
    #[inline(always)]
    pub fn pull(&mut self, n: u8) -> PULL_W<'_, PULL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PULL_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - GPIOx pin 0 pull configuration"]
    #[inline(always)]
    pub fn pull0(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 pull configuration"]
    #[inline(always)]
    pub fn pull1(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 pull configuration"]
    #[inline(always)]
    pub fn pull2(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 pull configuration"]
    #[inline(always)]
    pub fn pull3(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 pull configuration"]
    #[inline(always)]
    pub fn pull4(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 pull configuration"]
    #[inline(always)]
    pub fn pull5(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 pull configuration"]
    #[inline(always)]
    pub fn pull6(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 pull configuration"]
    #[inline(always)]
    pub fn pull7(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 pull configuration"]
    #[inline(always)]
    pub fn pull8(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 pull configuration"]
    #[inline(always)]
    pub fn pull9(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 pull configuration"]
    #[inline(always)]
    pub fn pull10(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 pull configuration"]
    #[inline(always)]
    pub fn pull11(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 pull configuration"]
    #[inline(always)]
    pub fn pull12(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 pull configuration"]
    #[inline(always)]
    pub fn pull13(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 pull configuration"]
    #[inline(always)]
    pub fn pull14(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 pull configuration"]
    #[inline(always)]
    pub fn pull15(&mut self) -> PULL_W<'_, PULL_SPEC> {
        PULL_W::new(self, 30)
    }
}
#[doc = "GPIO pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pull::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pull::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULL_SPEC;
impl crate::RegisterSpec for PULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pull::R`](R) reader structure"]
impl crate::Readable for PULL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pull::W`](W) writer structure"]
impl crate::Writable for PULL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PULL to value 0"]
impl crate::Resettable for PULL_SPEC {}
