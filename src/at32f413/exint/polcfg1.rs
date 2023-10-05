#[doc = "Register `POLCFG1` reader"]
pub type R = crate::R<POLCFG1_SPEC>;
#[doc = "Register `POLCFG1` writer"]
pub type W = crate::W<POLCFG1_SPEC>;
#[doc = "Field `RP[0-18]` reader - Rising polarity configuration bit on line %s"]
pub type RP_R = crate::BitReader<RP0R_A>;
#[doc = "Rising polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP0R_A {
    #[doc = "0: Rising trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising trigger is enabled"]
    Enabled = 1,
}
impl From<RP0R_A> for bool {
    #[inline(always)]
    fn from(variant: RP0R_A) -> Self {
        variant as u8 != 0
    }
}
impl RP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP0R_A {
        match self.bits {
            false => RP0R_A::Disabled,
            true => RP0R_A::Enabled,
        }
    }
    #[doc = "Rising trigger is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RP0R_A::Disabled
    }
    #[doc = "Rising trigger is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RP0R_A::Enabled
    }
}
#[doc = "Rising polarity configuration bit on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP0W_AW {
    #[doc = "0: Rising trigger disable"]
    Disable = 0,
    #[doc = "1: Rising trigger enable"]
    Enable = 1,
}
impl From<RP0W_AW> for bool {
    #[inline(always)]
    fn from(variant: RP0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RP[0-18]` writer - Rising polarity configuration bit on line %s"]
pub type RP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RP0W_AW>;
impl<'a, REG, const O: u8> RP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising trigger disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RP0W_AW::Disable)
    }
    #[doc = "Rising trigger enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RP0W_AW::Enable)
    }
}
impl R {
    #[doc = "Rising polarity configuration bit on line [0-18]"]
    #[inline(always)]
    pub unsafe fn rp(&self, n: u8) -> RP_R {
        RP_R::new(((self.bits >> n) & 1) != 0)
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
    #[doc = "Bit 18 - Rising polarity configuration bit on line 18"]
    #[inline(always)]
    pub fn rp18(&self) -> RP_R {
        RP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Rising polarity configuration bit on line [0-18]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn rp<const O: u8>(&mut self) -> RP_W<POLCFG1_SPEC, O> {
        RP_W::new(self)
    }
    #[doc = "Bit 0 - Rising polarity configuration bit on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn rp0(&mut self) -> RP_W<POLCFG1_SPEC, 0> {
        RP_W::new(self)
    }
    #[doc = "Bit 1 - Rising polarity configuration bit on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn rp1(&mut self) -> RP_W<POLCFG1_SPEC, 1> {
        RP_W::new(self)
    }
    #[doc = "Bit 2 - Rising polarity configuration bit on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn rp2(&mut self) -> RP_W<POLCFG1_SPEC, 2> {
        RP_W::new(self)
    }
    #[doc = "Bit 3 - Rising polarity configuration bit on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn rp3(&mut self) -> RP_W<POLCFG1_SPEC, 3> {
        RP_W::new(self)
    }
    #[doc = "Bit 4 - Rising polarity configuration bit on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn rp4(&mut self) -> RP_W<POLCFG1_SPEC, 4> {
        RP_W::new(self)
    }
    #[doc = "Bit 5 - Rising polarity configuration bit on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn rp5(&mut self) -> RP_W<POLCFG1_SPEC, 5> {
        RP_W::new(self)
    }
    #[doc = "Bit 6 - Rising polarity configuration bit on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn rp6(&mut self) -> RP_W<POLCFG1_SPEC, 6> {
        RP_W::new(self)
    }
    #[doc = "Bit 7 - Rising polarity configuration bit on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn rp7(&mut self) -> RP_W<POLCFG1_SPEC, 7> {
        RP_W::new(self)
    }
    #[doc = "Bit 8 - Rising polarity configuration bit on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rp8(&mut self) -> RP_W<POLCFG1_SPEC, 8> {
        RP_W::new(self)
    }
    #[doc = "Bit 9 - Rising polarity configuration bit on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rp9(&mut self) -> RP_W<POLCFG1_SPEC, 9> {
        RP_W::new(self)
    }
    #[doc = "Bit 10 - Rising polarity configuration bit on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rp10(&mut self) -> RP_W<POLCFG1_SPEC, 10> {
        RP_W::new(self)
    }
    #[doc = "Bit 11 - Rising polarity configuration bit on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rp11(&mut self) -> RP_W<POLCFG1_SPEC, 11> {
        RP_W::new(self)
    }
    #[doc = "Bit 12 - Rising polarity configuration bit on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn rp12(&mut self) -> RP_W<POLCFG1_SPEC, 12> {
        RP_W::new(self)
    }
    #[doc = "Bit 13 - Rising polarity configuration bit on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn rp13(&mut self) -> RP_W<POLCFG1_SPEC, 13> {
        RP_W::new(self)
    }
    #[doc = "Bit 14 - Rising polarity configuration bit on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn rp14(&mut self) -> RP_W<POLCFG1_SPEC, 14> {
        RP_W::new(self)
    }
    #[doc = "Bit 15 - Rising polarity configuration bit on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn rp15(&mut self) -> RP_W<POLCFG1_SPEC, 15> {
        RP_W::new(self)
    }
    #[doc = "Bit 16 - Rising polarity configuration bit on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn rp16(&mut self) -> RP_W<POLCFG1_SPEC, 16> {
        RP_W::new(self)
    }
    #[doc = "Bit 17 - Rising polarity configuration bit on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn rp17(&mut self) -> RP_W<POLCFG1_SPEC, 17> {
        RP_W::new(self)
    }
    #[doc = "Bit 18 - Rising polarity configuration bit on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn rp18(&mut self) -> RP_W<POLCFG1_SPEC, 18> {
        RP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Rising polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polcfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polcfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLCFG1_SPEC;
impl crate::RegisterSpec for POLCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polcfg1::R`](R) reader structure"]
impl crate::Readable for POLCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`polcfg1::W`](W) writer structure"]
impl crate::Writable for POLCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POLCFG1 to value 0"]
impl crate::Resettable for POLCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
