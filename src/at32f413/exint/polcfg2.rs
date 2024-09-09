#[doc = "Register `POLCFG2` reader"]
pub type R = crate::R<POLCFG2_SPEC>;
#[doc = "Register `POLCFG2` writer"]
pub type W = crate::W<POLCFG2_SPEC>;
#[doc = "Falling polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fp0r {
    #[doc = "0: Falling polarity is disabled"]
    Disabled = 0,
    #[doc = "1: Falling polarity is enabled"]
    Enabled = 1,
}
impl From<Fp0r> for bool {
    #[inline(always)]
    fn from(variant: Fp0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FP(0-18)` reader - Falling polarity configuration bit on line %s"]
pub type FP_R = crate::BitReader<Fp0r>;
impl FP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fp0r {
        match self.bits {
            false => Fp0r::Disabled,
            true => Fp0r::Enabled,
        }
    }
    #[doc = "Falling polarity is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fp0r::Disabled
    }
    #[doc = "Falling polarity is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fp0r::Enabled
    }
}
#[doc = "Falling polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fp0wWO {
    #[doc = "0: Falling polarity disable"]
    Disable = 0,
    #[doc = "1: Falling polarity enable"]
    Enable = 1,
}
impl From<Fp0wWO> for bool {
    #[inline(always)]
    fn from(variant: Fp0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FP(0-18)` writer - Falling polarity configuration bit on line %s"]
pub type FP_W<'a, REG> = crate::BitWriter<'a, REG, Fp0wWO>;
impl<'a, REG> FP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling polarity disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fp0wWO::Disable)
    }
    #[doc = "Falling polarity enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fp0wWO::Enable)
    }
}
impl R {
    #[doc = "Falling polarity configuration bit on line (0-18)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FP0` field.</div>"]
    #[inline(always)]
    pub fn fp(&self, n: u8) -> FP_R {
        #[allow(clippy::no_effect)]
        [(); 19][n as usize];
        FP_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Falling polarity configuration bit on line (0-18)"]
    #[inline(always)]
    pub fn fp_iter(&self) -> impl Iterator<Item = FP_R> + '_ {
        (0..19).map(move |n| FP_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Falling polarity configuration bit on line 0"]
    #[inline(always)]
    pub fn fp0(&self) -> FP_R {
        FP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling polarity configuration bit on line 1"]
    #[inline(always)]
    pub fn fp1(&self) -> FP_R {
        FP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling polarity configuration bit on line 2"]
    #[inline(always)]
    pub fn fp2(&self) -> FP_R {
        FP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling polarity configuration bit on line 3"]
    #[inline(always)]
    pub fn fp3(&self) -> FP_R {
        FP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling polarity configuration bit on line 4"]
    #[inline(always)]
    pub fn fp4(&self) -> FP_R {
        FP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling polarity configuration bit on line 5"]
    #[inline(always)]
    pub fn fp5(&self) -> FP_R {
        FP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling polarity configuration bit on line 6"]
    #[inline(always)]
    pub fn fp6(&self) -> FP_R {
        FP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling polarity configuration bit on line 7"]
    #[inline(always)]
    pub fn fp7(&self) -> FP_R {
        FP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling polarity configuration bit on line 8"]
    #[inline(always)]
    pub fn fp8(&self) -> FP_R {
        FP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling polarity configuration bit on line 9"]
    #[inline(always)]
    pub fn fp9(&self) -> FP_R {
        FP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling polarity configuration bit on line 10"]
    #[inline(always)]
    pub fn fp10(&self) -> FP_R {
        FP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling polarity configuration bit on line 11"]
    #[inline(always)]
    pub fn fp11(&self) -> FP_R {
        FP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling polarity configuration bit on line 12"]
    #[inline(always)]
    pub fn fp12(&self) -> FP_R {
        FP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling polarity configuration bit on line 13"]
    #[inline(always)]
    pub fn fp13(&self) -> FP_R {
        FP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling polarity configuration bit on line 14"]
    #[inline(always)]
    pub fn fp14(&self) -> FP_R {
        FP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling polarity configuration bit on line 15"]
    #[inline(always)]
    pub fn fp15(&self) -> FP_R {
        FP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling polarity configuration bit on line 16"]
    #[inline(always)]
    pub fn fp16(&self) -> FP_R {
        FP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling polarity configuration bit on line 17"]
    #[inline(always)]
    pub fn fp17(&self) -> FP_R {
        FP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling polarity configuration bit on line 18"]
    #[inline(always)]
    pub fn fp18(&self) -> FP_R {
        FP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLCFG2")
            .field("fp0", &self.fp0())
            .field("fp1", &self.fp1())
            .field("fp2", &self.fp2())
            .field("fp3", &self.fp3())
            .field("fp4", &self.fp4())
            .field("fp5", &self.fp5())
            .field("fp6", &self.fp6())
            .field("fp7", &self.fp7())
            .field("fp8", &self.fp8())
            .field("fp9", &self.fp9())
            .field("fp10", &self.fp10())
            .field("fp11", &self.fp11())
            .field("fp12", &self.fp12())
            .field("fp13", &self.fp13())
            .field("fp14", &self.fp14())
            .field("fp15", &self.fp15())
            .field("fp16", &self.fp16())
            .field("fp17", &self.fp17())
            .field("fp18", &self.fp18())
            .finish()
    }
}
impl W {
    #[doc = "Falling polarity configuration bit on line (0-18)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `FP0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self, n: u8) -> FP_W<POLCFG2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 19][n as usize];
        FP_W::new(self, n)
    }
    #[doc = "Bit 0 - Falling polarity configuration bit on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn fp0(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling polarity configuration bit on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn fp1(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling polarity configuration bit on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn fp2(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling polarity configuration bit on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn fp3(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling polarity configuration bit on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn fp4(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling polarity configuration bit on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn fp5(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling polarity configuration bit on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn fp6(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling polarity configuration bit on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn fp7(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling polarity configuration bit on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn fp8(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling polarity configuration bit on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn fp9(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling polarity configuration bit on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn fp10(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling polarity configuration bit on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn fp11(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling polarity configuration bit on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn fp12(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling polarity configuration bit on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn fp13(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling polarity configuration bit on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn fp14(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling polarity configuration bit on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn fp15(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling polarity configuration bit on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn fp16(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling polarity configuration bit on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn fp17(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Falling polarity configuration bit on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn fp18(&mut self) -> FP_W<POLCFG2_SPEC> {
        FP_W::new(self, 18)
    }
}
#[doc = "Falling polarity configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`polcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLCFG2_SPEC;
impl crate::RegisterSpec for POLCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polcfg2::R`](R) reader structure"]
impl crate::Readable for POLCFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`polcfg2::W`](W) writer structure"]
impl crate::Writable for POLCFG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLCFG2 to value 0"]
impl crate::Resettable for POLCFG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
