#[doc = "Register `OMODE` reader"]
pub type R = crate::R<OMODE_SPEC>;
#[doc = "Register `OMODE` writer"]
pub type W = crate::W<OMODE_SPEC>;
#[doc = "Field `OM[0-15]` reader - GPIOx pin %s outpu mode configurate"]
pub type OM_R = crate::BitReader<OM0_A>;
#[doc = "GPIOx pin %s outpu mode configurate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OM0_A {
    #[doc = "0: Push-pull"]
    PushPull = 0,
    #[doc = "1: Open-drain"]
    OpenDrain = 1,
}
impl From<OM0_A> for bool {
    #[inline(always)]
    fn from(variant: OM0_A) -> Self {
        variant as u8 != 0
    }
}
impl OM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OM0_A {
        match self.bits {
            false => OM0_A::PushPull,
            true => OM0_A::OpenDrain,
        }
    }
    #[doc = "Push-pull"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OM0_A::PushPull
    }
    #[doc = "Open-drain"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OM0_A::OpenDrain
    }
}
#[doc = "Field `OM[0-15]` writer - GPIOx pin %s outpu mode configurate"]
pub type OM_W<'a, REG> = crate::BitWriter<'a, REG, OM0_A>;
impl<'a, REG> OM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push-pull"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OM0_A::PushPull)
    }
    #[doc = "Open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OM0_A::OpenDrain)
    }
}
impl R {
    #[doc = "GPIOx pin [0-15]
outpu mode configurate\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn om(&self, n: u8) -> OM_R {
        assert!(n < 16);
        OM_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - GPIOx pin 0 outpu mode configurate"]
    #[inline(always)]
    pub fn om0(&self) -> OM_R {
        OM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOx pin 1 outpu mode configurate"]
    #[inline(always)]
    pub fn om1(&self) -> OM_R {
        OM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOx pin 2 outpu mode configurate"]
    #[inline(always)]
    pub fn om2(&self) -> OM_R {
        OM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOx pin 3 outpu mode configurate"]
    #[inline(always)]
    pub fn om3(&self) -> OM_R {
        OM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOx pin 4 outpu mode configurate"]
    #[inline(always)]
    pub fn om4(&self) -> OM_R {
        OM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOx pin 5 outpu mode configurate"]
    #[inline(always)]
    pub fn om5(&self) -> OM_R {
        OM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOx pin 6 outpu mode configurate"]
    #[inline(always)]
    pub fn om6(&self) -> OM_R {
        OM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOx pin 7 outpu mode configurate"]
    #[inline(always)]
    pub fn om7(&self) -> OM_R {
        OM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOx pin 8 outpu mode configurate"]
    #[inline(always)]
    pub fn om8(&self) -> OM_R {
        OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOx pin 9 outpu mode configurate"]
    #[inline(always)]
    pub fn om9(&self) -> OM_R {
        OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOx pin 10 outpu mode configurate"]
    #[inline(always)]
    pub fn om10(&self) -> OM_R {
        OM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOx pin 11 outpu mode configurate"]
    #[inline(always)]
    pub fn om11(&self) -> OM_R {
        OM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOx pin 12 outpu mode configurate"]
    #[inline(always)]
    pub fn om12(&self) -> OM_R {
        OM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOx pin 13 outpu mode configurate"]
    #[inline(always)]
    pub fn om13(&self) -> OM_R {
        OM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOx pin 14 outpu mode configurate"]
    #[inline(always)]
    pub fn om14(&self) -> OM_R {
        OM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOx pin 15 outpu mode configurate"]
    #[inline(always)]
    pub fn om15(&self) -> OM_R {
        OM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OMODE")
            .field("om0", &format_args!("{}", self.om0().bit()))
            .field("om1", &format_args!("{}", self.om1().bit()))
            .field("om2", &format_args!("{}", self.om2().bit()))
            .field("om3", &format_args!("{}", self.om3().bit()))
            .field("om4", &format_args!("{}", self.om4().bit()))
            .field("om5", &format_args!("{}", self.om5().bit()))
            .field("om6", &format_args!("{}", self.om6().bit()))
            .field("om7", &format_args!("{}", self.om7().bit()))
            .field("om8", &format_args!("{}", self.om8().bit()))
            .field("om9", &format_args!("{}", self.om9().bit()))
            .field("om10", &format_args!("{}", self.om10().bit()))
            .field("om11", &format_args!("{}", self.om11().bit()))
            .field("om12", &format_args!("{}", self.om12().bit()))
            .field("om13", &format_args!("{}", self.om13().bit()))
            .field("om14", &format_args!("{}", self.om14().bit()))
            .field("om15", &format_args!("{}", self.om15().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<OMODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "GPIOx pin [0-15]
outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om(&mut self, n: u8) -> OM_W<OMODE_SPEC> {
        assert!(n < 16);
        OM_W::new(self, n)
    }
    #[doc = "Bit 0 - GPIOx pin 0 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om0(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOx pin 1 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om1(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOx pin 2 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om2(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOx pin 3 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om3(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOx pin 4 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om4(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOx pin 5 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om5(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOx pin 6 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om6(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOx pin 7 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om7(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOx pin 8 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om8(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOx pin 9 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om9(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOx pin 10 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om10(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 10)
    }
    #[doc = "Bit 11 - GPIOx pin 11 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om11(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIOx pin 12 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om12(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 12)
    }
    #[doc = "Bit 13 - GPIOx pin 13 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om13(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIOx pin 14 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om14(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 14)
    }
    #[doc = "Bit 15 - GPIOx pin 15 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om15(&mut self) -> OM_W<OMODE_SPEC> {
        OM_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OMODE_SPEC;
impl crate::RegisterSpec for OMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omode::R`](R) reader structure"]
impl crate::Readable for OMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`omode::W`](W) writer structure"]
impl crate::Writable for OMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OMODE to value 0"]
impl crate::Resettable for OMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
