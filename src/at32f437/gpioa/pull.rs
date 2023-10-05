#[doc = "Register `PULL` reader"]
pub type R = crate::R<PULL_SPEC>;
#[doc = "Register `PULL` writer"]
pub type W = crate::W<PULL_SPEC>;
#[doc = "Field `PULL[0-15]` reader - GPIOx pin %s pull configuration"]
pub type PULL_R = crate::FieldReader<PULL0_A>;
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
impl PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PULL0_A> {
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
#[doc = "Field `PULL[0-15]` writer - GPIOx pin %s pull configuration"]
pub type PULL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PULL0_A>;
impl<'a, REG, const O: u8> PULL_W<'a, REG, O>
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
    #[doc = "GPIOx pin [0-15]
pull configuration"]
    #[inline(always)]
    pub unsafe fn pull(&self, n: u8) -> PULL_R {
        PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
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
impl W {
    #[doc = "GPIOx pin [0-15]
pull configuration"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pull<const O: u8>(&mut self) -> PULL_W<PULL_SPEC, O> {
        PULL_W::new(self)
    }
    #[doc = "Bits 0:1 - GPIOx pin 0 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull0(&mut self) -> PULL_W<PULL_SPEC, 0> {
        PULL_W::new(self)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull1(&mut self) -> PULL_W<PULL_SPEC, 2> {
        PULL_W::new(self)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull2(&mut self) -> PULL_W<PULL_SPEC, 4> {
        PULL_W::new(self)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull3(&mut self) -> PULL_W<PULL_SPEC, 6> {
        PULL_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull4(&mut self) -> PULL_W<PULL_SPEC, 8> {
        PULL_W::new(self)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull5(&mut self) -> PULL_W<PULL_SPEC, 10> {
        PULL_W::new(self)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull6(&mut self) -> PULL_W<PULL_SPEC, 12> {
        PULL_W::new(self)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull7(&mut self) -> PULL_W<PULL_SPEC, 14> {
        PULL_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull8(&mut self) -> PULL_W<PULL_SPEC, 16> {
        PULL_W::new(self)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull9(&mut self) -> PULL_W<PULL_SPEC, 18> {
        PULL_W::new(self)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull10(&mut self) -> PULL_W<PULL_SPEC, 20> {
        PULL_W::new(self)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull11(&mut self) -> PULL_W<PULL_SPEC, 22> {
        PULL_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull12(&mut self) -> PULL_W<PULL_SPEC, 24> {
        PULL_W::new(self)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull13(&mut self) -> PULL_W<PULL_SPEC, 26> {
        PULL_W::new(self)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull14(&mut self) -> PULL_W<PULL_SPEC, 28> {
        PULL_W::new(self)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull15(&mut self) -> PULL_W<PULL_SPEC, 30> {
        PULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pull::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pull::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULL_SPEC;
impl crate::RegisterSpec for PULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pull::R`](R) reader structure"]
impl crate::Readable for PULL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pull::W`](W) writer structure"]
impl crate::Writable for PULL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULL to value 0"]
impl crate::Resettable for PULL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
