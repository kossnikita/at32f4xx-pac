#[doc = "Register `HDRV` reader"]
pub type R = crate::R<HDRV_SPEC>;
#[doc = "Register `HDRV` writer"]
pub type W = crate::W<HDRV_SPEC>;
#[doc = "Port hdrv bit %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDRV0_A {
    #[doc = "0: GPIO is configured as large or normal sourcing/sinking strength, depending on IOMC"]
    NoEffect = 0,
    #[doc = "1: GPIO is configured as maximum sourcing/sinking strength, ignoring IOMC"]
    MaximumSpeed = 1,
}
impl From<HDRV0_A> for bool {
    #[inline(always)]
    fn from(variant: HDRV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRV(0-15)` reader - Port hdrv bit %s"]
pub type HDRV_R = crate::BitReader<HDRV0_A>;
impl HDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDRV0_A {
        match self.bits {
            false => HDRV0_A::NoEffect,
            true => HDRV0_A::MaximumSpeed,
        }
    }
    #[doc = "GPIO is configured as large or normal sourcing/sinking strength, depending on IOMC"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == HDRV0_A::NoEffect
    }
    #[doc = "GPIO is configured as maximum sourcing/sinking strength, ignoring IOMC"]
    #[inline(always)]
    pub fn is_maximum_speed(&self) -> bool {
        *self == HDRV0_A::MaximumSpeed
    }
}
#[doc = "Field `HDRV(0-15)` writer - Port hdrv bit %s"]
pub type HDRV_W<'a, REG> = crate::BitWriter<'a, REG, HDRV0_A>;
impl<'a, REG> HDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPIO is configured as large or normal sourcing/sinking strength, depending on IOMC"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HDRV0_A::NoEffect)
    }
    #[doc = "GPIO is configured as maximum sourcing/sinking strength, ignoring IOMC"]
    #[inline(always)]
    pub fn maximum_speed(self) -> &'a mut crate::W<REG> {
        self.variant(HDRV0_A::MaximumSpeed)
    }
}
impl R {
    #[doc = "Port hdrv bit (0-15)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDRV0` field.</div>"]
    #[inline(always)]
    pub fn hdrv(&self, n: u8) -> HDRV_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        HDRV_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port hdrv bit (0-15)"]
    #[inline(always)]
    pub fn hdrv_iter(&self) -> impl Iterator<Item = HDRV_R> + '_ {
        (0..16).map(move |n| HDRV_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port hdrv bit 0"]
    #[inline(always)]
    pub fn hdrv0(&self) -> HDRV_R {
        HDRV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port hdrv bit 1"]
    #[inline(always)]
    pub fn hdrv1(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port hdrv bit 2"]
    #[inline(always)]
    pub fn hdrv2(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port hdrv bit 3"]
    #[inline(always)]
    pub fn hdrv3(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port hdrv bit 4"]
    #[inline(always)]
    pub fn hdrv4(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port hdrv bit 5"]
    #[inline(always)]
    pub fn hdrv5(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port hdrv bit 6"]
    #[inline(always)]
    pub fn hdrv6(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port hdrv bit 7"]
    #[inline(always)]
    pub fn hdrv7(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port hdrv bit 8"]
    #[inline(always)]
    pub fn hdrv8(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port hdrv bit 9"]
    #[inline(always)]
    pub fn hdrv9(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port hdrv bit 10"]
    #[inline(always)]
    pub fn hdrv10(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port hdrv bit 11"]
    #[inline(always)]
    pub fn hdrv11(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port hdrv bit 12"]
    #[inline(always)]
    pub fn hdrv12(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port hdrv bit 13"]
    #[inline(always)]
    pub fn hdrv13(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port hdrv bit 14"]
    #[inline(always)]
    pub fn hdrv14(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port hdrv bit 15"]
    #[inline(always)]
    pub fn hdrv15(&self) -> HDRV_R {
        HDRV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDRV")
            .field("hdrv0", &self.hdrv0())
            .field("hdrv1", &self.hdrv1())
            .field("hdrv2", &self.hdrv2())
            .field("hdrv3", &self.hdrv3())
            .field("hdrv4", &self.hdrv4())
            .field("hdrv5", &self.hdrv5())
            .field("hdrv6", &self.hdrv6())
            .field("hdrv7", &self.hdrv7())
            .field("hdrv8", &self.hdrv8())
            .field("hdrv9", &self.hdrv9())
            .field("hdrv10", &self.hdrv10())
            .field("hdrv11", &self.hdrv11())
            .field("hdrv12", &self.hdrv12())
            .field("hdrv13", &self.hdrv13())
            .field("hdrv14", &self.hdrv14())
            .field("hdrv15", &self.hdrv15())
            .finish()
    }
}
impl W {
    #[doc = "Port hdrv bit (0-15)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `HDRV0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv(&mut self, n: u8) -> HDRV_W<HDRV_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        HDRV_W::new(self, n)
    }
    #[doc = "Bit 0 - Port hdrv bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv0(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port hdrv bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv1(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port hdrv bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv2(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port hdrv bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv3(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port hdrv bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv4(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port hdrv bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv5(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port hdrv bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv6(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port hdrv bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv7(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port hdrv bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv8(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port hdrv bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv9(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port hdrv bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv10(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port hdrv bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv11(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port hdrv bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv12(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port hdrv bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv13(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port hdrv bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv14(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port hdrv bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn hdrv15(&mut self) -> HDRV_W<HDRV_SPEC> {
        HDRV_W::new(self, 15)
    }
}
#[doc = "Port configuration driver register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDRV_SPEC;
impl crate::RegisterSpec for HDRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdrv::R`](R) reader structure"]
impl crate::Readable for HDRV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hdrv::W`](W) writer structure"]
impl crate::Writable for HDRV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDRV to value 0"]
impl crate::Resettable for HDRV_SPEC {
    const RESET_VALUE: u32 = 0;
}
