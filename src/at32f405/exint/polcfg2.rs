#[doc = "Register `POLCFG2` reader"]
pub type R = crate::R<POLCFG2_SPEC>;
#[doc = "Register `POLCFG2` writer"]
pub type W = crate::W<POLCFG2_SPEC>;
#[doc = "Field `FP[0-22]` reader - Falling polarity configuration bit on line %s"]
pub type FP_R = crate::BitReader<FP0R_A>;
#[doc = "Falling polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FP0R_A {
    #[doc = "0: Falling polarity is disabled"]
    Disabled = 0,
    #[doc = "1: Falling polarity is enabled"]
    Enabled = 1,
}
impl From<FP0R_A> for bool {
    #[inline(always)]
    fn from(variant: FP0R_A) -> Self {
        variant as u8 != 0
    }
}
impl FP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FP0R_A {
        match self.bits {
            false => FP0R_A::Disabled,
            true => FP0R_A::Enabled,
        }
    }
    #[doc = "Falling polarity is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FP0R_A::Disabled
    }
    #[doc = "Falling polarity is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FP0R_A::Enabled
    }
}
#[doc = "Falling polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FP0W_AW {
    #[doc = "0: Falling polarity disable"]
    Disable = 0,
    #[doc = "1: Falling polarity enable"]
    Enable = 1,
}
impl From<FP0W_AW> for bool {
    #[inline(always)]
    fn from(variant: FP0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FP[0-22]` writer - Falling polarity configuration bit on line %s"]
pub type FP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FP0W_AW>;
impl<'a, REG, const O: u8> FP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling polarity disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FP0W_AW::Disable)
    }
    #[doc = "Falling polarity enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FP0W_AW::Enable)
    }
}
impl R {
    #[doc = "Falling polarity configuration bit on line [0-22]"]
    #[inline(always)]
    pub unsafe fn fp(&self, n: u8) -> FP_R {
        FP_R::new(((self.bits >> n) & 1) != 0)
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
    #[doc = "Bit 19 - Falling polarity configuration bit on line 19"]
    #[inline(always)]
    pub fn fp19(&self) -> FP_R {
        FP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling polarity configuration bit on line 20"]
    #[inline(always)]
    pub fn fp20(&self) -> FP_R {
        FP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling polarity configuration bit on line 21"]
    #[inline(always)]
    pub fn fp21(&self) -> FP_R {
        FP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Falling polarity configuration bit on line 22"]
    #[inline(always)]
    pub fn fp22(&self) -> FP_R {
        FP_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLCFG2")
            .field("fp0", &format_args!("{}", self.fp0().bit()))
            .field("fp1", &format_args!("{}", self.fp1().bit()))
            .field("fp2", &format_args!("{}", self.fp2().bit()))
            .field("fp3", &format_args!("{}", self.fp3().bit()))
            .field("fp4", &format_args!("{}", self.fp4().bit()))
            .field("fp5", &format_args!("{}", self.fp5().bit()))
            .field("fp6", &format_args!("{}", self.fp6().bit()))
            .field("fp7", &format_args!("{}", self.fp7().bit()))
            .field("fp8", &format_args!("{}", self.fp8().bit()))
            .field("fp9", &format_args!("{}", self.fp9().bit()))
            .field("fp10", &format_args!("{}", self.fp10().bit()))
            .field("fp11", &format_args!("{}", self.fp11().bit()))
            .field("fp12", &format_args!("{}", self.fp12().bit()))
            .field("fp13", &format_args!("{}", self.fp13().bit()))
            .field("fp14", &format_args!("{}", self.fp14().bit()))
            .field("fp15", &format_args!("{}", self.fp15().bit()))
            .field("fp16", &format_args!("{}", self.fp16().bit()))
            .field("fp17", &format_args!("{}", self.fp17().bit()))
            .field("fp18", &format_args!("{}", self.fp18().bit()))
            .field("fp19", &format_args!("{}", self.fp19().bit()))
            .field("fp20", &format_args!("{}", self.fp20().bit()))
            .field("fp21", &format_args!("{}", self.fp21().bit()))
            .field("fp22", &format_args!("{}", self.fp22().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<POLCFG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Falling polarity configuration bit on line [0-22]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn fp<const O: u8>(&mut self) -> FP_W<POLCFG2_SPEC, O> {
        FP_W::new(self)
    }
    #[doc = "Bit 0 - Falling polarity configuration bit on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn fp0(&mut self) -> FP_W<POLCFG2_SPEC, 0> {
        FP_W::new(self)
    }
    #[doc = "Bit 1 - Falling polarity configuration bit on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn fp1(&mut self) -> FP_W<POLCFG2_SPEC, 1> {
        FP_W::new(self)
    }
    #[doc = "Bit 2 - Falling polarity configuration bit on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn fp2(&mut self) -> FP_W<POLCFG2_SPEC, 2> {
        FP_W::new(self)
    }
    #[doc = "Bit 3 - Falling polarity configuration bit on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn fp3(&mut self) -> FP_W<POLCFG2_SPEC, 3> {
        FP_W::new(self)
    }
    #[doc = "Bit 4 - Falling polarity configuration bit on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn fp4(&mut self) -> FP_W<POLCFG2_SPEC, 4> {
        FP_W::new(self)
    }
    #[doc = "Bit 5 - Falling polarity configuration bit on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn fp5(&mut self) -> FP_W<POLCFG2_SPEC, 5> {
        FP_W::new(self)
    }
    #[doc = "Bit 6 - Falling polarity configuration bit on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn fp6(&mut self) -> FP_W<POLCFG2_SPEC, 6> {
        FP_W::new(self)
    }
    #[doc = "Bit 7 - Falling polarity configuration bit on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn fp7(&mut self) -> FP_W<POLCFG2_SPEC, 7> {
        FP_W::new(self)
    }
    #[doc = "Bit 8 - Falling polarity configuration bit on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn fp8(&mut self) -> FP_W<POLCFG2_SPEC, 8> {
        FP_W::new(self)
    }
    #[doc = "Bit 9 - Falling polarity configuration bit on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn fp9(&mut self) -> FP_W<POLCFG2_SPEC, 9> {
        FP_W::new(self)
    }
    #[doc = "Bit 10 - Falling polarity configuration bit on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn fp10(&mut self) -> FP_W<POLCFG2_SPEC, 10> {
        FP_W::new(self)
    }
    #[doc = "Bit 11 - Falling polarity configuration bit on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn fp11(&mut self) -> FP_W<POLCFG2_SPEC, 11> {
        FP_W::new(self)
    }
    #[doc = "Bit 12 - Falling polarity configuration bit on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn fp12(&mut self) -> FP_W<POLCFG2_SPEC, 12> {
        FP_W::new(self)
    }
    #[doc = "Bit 13 - Falling polarity configuration bit on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn fp13(&mut self) -> FP_W<POLCFG2_SPEC, 13> {
        FP_W::new(self)
    }
    #[doc = "Bit 14 - Falling polarity configuration bit on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn fp14(&mut self) -> FP_W<POLCFG2_SPEC, 14> {
        FP_W::new(self)
    }
    #[doc = "Bit 15 - Falling polarity configuration bit on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn fp15(&mut self) -> FP_W<POLCFG2_SPEC, 15> {
        FP_W::new(self)
    }
    #[doc = "Bit 16 - Falling polarity configuration bit on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn fp16(&mut self) -> FP_W<POLCFG2_SPEC, 16> {
        FP_W::new(self)
    }
    #[doc = "Bit 17 - Falling polarity configuration bit on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn fp17(&mut self) -> FP_W<POLCFG2_SPEC, 17> {
        FP_W::new(self)
    }
    #[doc = "Bit 18 - Falling polarity configuration bit on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn fp18(&mut self) -> FP_W<POLCFG2_SPEC, 18> {
        FP_W::new(self)
    }
    #[doc = "Bit 19 - Falling polarity configuration bit on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn fp19(&mut self) -> FP_W<POLCFG2_SPEC, 19> {
        FP_W::new(self)
    }
    #[doc = "Bit 20 - Falling polarity configuration bit on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn fp20(&mut self) -> FP_W<POLCFG2_SPEC, 20> {
        FP_W::new(self)
    }
    #[doc = "Bit 21 - Falling polarity configuration bit on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn fp21(&mut self) -> FP_W<POLCFG2_SPEC, 21> {
        FP_W::new(self)
    }
    #[doc = "Bit 22 - Falling polarity configuration bit on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn fp22(&mut self) -> FP_W<POLCFG2_SPEC, 22> {
        FP_W::new(self)
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
#[doc = "Falling polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polcfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polcfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLCFG2_SPEC;
impl crate::RegisterSpec for POLCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polcfg2::R`](R) reader structure"]
impl crate::Readable for POLCFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`polcfg2::W`](W) writer structure"]
impl crate::Writable for POLCFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POLCFG2 to value 0"]
impl crate::Resettable for POLCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
