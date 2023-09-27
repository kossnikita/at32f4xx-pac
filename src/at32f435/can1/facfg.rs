#[doc = "Register `FACFG` reader"]
pub type R = crate::R<FACFG_SPEC>;
#[doc = "Register `FACFG` writer"]
pub type W = crate::W<FACFG_SPEC>;
#[doc = "Field `EN[0-27]` reader - Filter activate enable"]
pub type EN_R = crate::BitReader<EN0R_A>;
#[doc = "Filter activate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN0R_A {
    #[doc = "0: Filter is disabled"]
    Disabled = 0,
    #[doc = "1: Filter is enabled"]
    Enabled = 1,
}
impl From<EN0R_A> for bool {
    #[inline(always)]
    fn from(variant: EN0R_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN0R_A {
        match self.bits {
            false => EN0R_A::Disabled,
            true => EN0R_A::Enabled,
        }
    }
    #[doc = "Filter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN0R_A::Disabled
    }
    #[doc = "Filter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN0R_A::Enabled
    }
}
#[doc = "Filter activate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN0W_AW {
    #[doc = "0: Filter disable"]
    Disable = 0,
    #[doc = "1: Filter enable"]
    Enable = 1,
}
impl From<EN0W_AW> for bool {
    #[inline(always)]
    fn from(variant: EN0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN[0-27]` writer - Filter activate enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EN0W_AW>;
impl<'a, REG, const O: u8> EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EN0W_AW::Disable)
    }
    #[doc = "Filter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EN0W_AW::Enable)
    }
}
impl R {
    #[doc = "Filter activate enable"]
    #[inline(always)]
    pub unsafe fn en(&self, n: u8) -> EN_R {
        EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    pub fn en0(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN_R {
        EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    pub fn en3(&self) -> EN_R {
        EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    pub fn en4(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    pub fn en5(&self) -> EN_R {
        EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    pub fn en6(&self) -> EN_R {
        EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    pub fn en7(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    pub fn en8(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    pub fn en9(&self) -> EN_R {
        EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    pub fn en10(&self) -> EN_R {
        EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    pub fn en11(&self) -> EN_R {
        EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    pub fn en12(&self) -> EN_R {
        EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    pub fn en13(&self) -> EN_R {
        EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter activate enable"]
    #[inline(always)]
    pub fn en14(&self) -> EN_R {
        EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter activate enable"]
    #[inline(always)]
    pub fn en15(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter activate enable"]
    #[inline(always)]
    pub fn en16(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter activate enable"]
    #[inline(always)]
    pub fn en17(&self) -> EN_R {
        EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter activate enable"]
    #[inline(always)]
    pub fn en18(&self) -> EN_R {
        EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter activate enable"]
    #[inline(always)]
    pub fn en19(&self) -> EN_R {
        EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter activate enable"]
    #[inline(always)]
    pub fn en20(&self) -> EN_R {
        EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter activate enable"]
    #[inline(always)]
    pub fn en21(&self) -> EN_R {
        EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter activate enable"]
    #[inline(always)]
    pub fn en22(&self) -> EN_R {
        EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter activate enable"]
    #[inline(always)]
    pub fn en23(&self) -> EN_R {
        EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter activate enable"]
    #[inline(always)]
    pub fn en24(&self) -> EN_R {
        EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter activate enable"]
    #[inline(always)]
    pub fn en25(&self) -> EN_R {
        EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter activate enable"]
    #[inline(always)]
    pub fn en26(&self) -> EN_R {
        EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter activate enable"]
    #[inline(always)]
    pub fn en27(&self) -> EN_R {
        EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn en<const O: u8>(&mut self) -> EN_W<FACFG_SPEC, O> {
        EN_W::new(self)
    }
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> EN_W<FACFG_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN_W<FACFG_SPEC, 1> {
        EN_W::new(self)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN_W<FACFG_SPEC, 2> {
        EN_W::new(self)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> EN_W<FACFG_SPEC, 3> {
        EN_W::new(self)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> EN_W<FACFG_SPEC, 4> {
        EN_W::new(self)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en5(&mut self) -> EN_W<FACFG_SPEC, 5> {
        EN_W::new(self)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en6(&mut self) -> EN_W<FACFG_SPEC, 6> {
        EN_W::new(self)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en7(&mut self) -> EN_W<FACFG_SPEC, 7> {
        EN_W::new(self)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en8(&mut self) -> EN_W<FACFG_SPEC, 8> {
        EN_W::new(self)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en9(&mut self) -> EN_W<FACFG_SPEC, 9> {
        EN_W::new(self)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en10(&mut self) -> EN_W<FACFG_SPEC, 10> {
        EN_W::new(self)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en11(&mut self) -> EN_W<FACFG_SPEC, 11> {
        EN_W::new(self)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en12(&mut self) -> EN_W<FACFG_SPEC, 12> {
        EN_W::new(self)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en13(&mut self) -> EN_W<FACFG_SPEC, 13> {
        EN_W::new(self)
    }
    #[doc = "Bit 14 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en14(&mut self) -> EN_W<FACFG_SPEC, 14> {
        EN_W::new(self)
    }
    #[doc = "Bit 15 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en15(&mut self) -> EN_W<FACFG_SPEC, 15> {
        EN_W::new(self)
    }
    #[doc = "Bit 16 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en16(&mut self) -> EN_W<FACFG_SPEC, 16> {
        EN_W::new(self)
    }
    #[doc = "Bit 17 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en17(&mut self) -> EN_W<FACFG_SPEC, 17> {
        EN_W::new(self)
    }
    #[doc = "Bit 18 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en18(&mut self) -> EN_W<FACFG_SPEC, 18> {
        EN_W::new(self)
    }
    #[doc = "Bit 19 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en19(&mut self) -> EN_W<FACFG_SPEC, 19> {
        EN_W::new(self)
    }
    #[doc = "Bit 20 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en20(&mut self) -> EN_W<FACFG_SPEC, 20> {
        EN_W::new(self)
    }
    #[doc = "Bit 21 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en21(&mut self) -> EN_W<FACFG_SPEC, 21> {
        EN_W::new(self)
    }
    #[doc = "Bit 22 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en22(&mut self) -> EN_W<FACFG_SPEC, 22> {
        EN_W::new(self)
    }
    #[doc = "Bit 23 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en23(&mut self) -> EN_W<FACFG_SPEC, 23> {
        EN_W::new(self)
    }
    #[doc = "Bit 24 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en24(&mut self) -> EN_W<FACFG_SPEC, 24> {
        EN_W::new(self)
    }
    #[doc = "Bit 25 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en25(&mut self) -> EN_W<FACFG_SPEC, 25> {
        EN_W::new(self)
    }
    #[doc = "Bit 26 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en26(&mut self) -> EN_W<FACFG_SPEC, 26> {
        EN_W::new(self)
    }
    #[doc = "Bit 27 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en27(&mut self) -> EN_W<FACFG_SPEC, 27> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Filter activate configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`facfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`facfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FACFG_SPEC;
impl crate::RegisterSpec for FACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`facfg::R`](R) reader structure"]
impl crate::Readable for FACFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`facfg::W`](W) writer structure"]
impl crate::Writable for FACFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FACFG to value 0"]
impl crate::Resettable for FACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
