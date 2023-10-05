#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `INTEN[0-22]` reader - Interrupt enable or disable on line %s"]
pub type INTEN_R = crate::BitReader<INTEN0R_A>;
#[doc = "Interrupt enable or disable on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN0R_A {
    #[doc = "0: Interrupt request is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt request is enabled"]
    Enabled = 1,
}
impl From<INTEN0R_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN0R_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN0R_A {
        match self.bits {
            false => INTEN0R_A::Disabled,
            true => INTEN0R_A::Enabled,
        }
    }
    #[doc = "Interrupt request is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN0R_A::Disabled
    }
    #[doc = "Interrupt request is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN0R_A::Enabled
    }
}
#[doc = "Interrupt enable or disable on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN0W_AW {
    #[doc = "0: Interrupt request disable"]
    Disable = 0,
    #[doc = "1: Interrupt request enable"]
    Enable = 1,
}
impl From<INTEN0W_AW> for bool {
    #[inline(always)]
    fn from(variant: INTEN0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN[0-22]` writer - Interrupt enable or disable on line %s"]
pub type INTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INTEN0W_AW>;
impl<'a, REG, const O: u8> INTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(INTEN0W_AW::Disable)
    }
    #[doc = "Interrupt request enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(INTEN0W_AW::Enable)
    }
}
impl R {
    #[doc = "Interrupt enable or disable on line [0-22]"]
    #[inline(always)]
    pub unsafe fn inten(&self, n: u8) -> INTEN_R {
        INTEN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Interrupt enable or disable on line 0"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable or disable on line 1"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable or disable on line 2"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable or disable on line 3"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable or disable on line 4"]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable or disable on line 5"]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable or disable on line 6"]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable or disable on line 7"]
    #[inline(always)]
    pub fn inten7(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable or disable on line 8"]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable or disable on line 9"]
    #[inline(always)]
    pub fn inten9(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable or disable on line 10"]
    #[inline(always)]
    pub fn inten10(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt enable or disable on line 11"]
    #[inline(always)]
    pub fn inten11(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt enable or disable on line 12"]
    #[inline(always)]
    pub fn inten12(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt enable or disable on line 13"]
    #[inline(always)]
    pub fn inten13(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt enable or disable on line 14"]
    #[inline(always)]
    pub fn inten14(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt enable or disable on line 15"]
    #[inline(always)]
    pub fn inten15(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable or disable on line 16"]
    #[inline(always)]
    pub fn inten16(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable or disable on line 17"]
    #[inline(always)]
    pub fn inten17(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable or disable on line 18"]
    #[inline(always)]
    pub fn inten18(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt enable or disable on line 19"]
    #[inline(always)]
    pub fn inten19(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt enable or disable on line 20"]
    #[inline(always)]
    pub fn inten20(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt enable or disable on line 21"]
    #[inline(always)]
    pub fn inten21(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt enable or disable on line 22"]
    #[inline(always)]
    pub fn inten22(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Interrupt enable or disable on line [0-22]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn inten<const O: u8>(&mut self) -> INTEN_W<INTEN_SPEC, O> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 0 - Interrupt enable or disable on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> INTEN_W<INTEN_SPEC, 0> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable or disable on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> INTEN_W<INTEN_SPEC, 1> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable or disable on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> INTEN_W<INTEN_SPEC, 2> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable or disable on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn inten3(&mut self) -> INTEN_W<INTEN_SPEC, 3> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable or disable on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn inten4(&mut self) -> INTEN_W<INTEN_SPEC, 4> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable or disable on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn inten5(&mut self) -> INTEN_W<INTEN_SPEC, 5> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable or disable on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn inten6(&mut self) -> INTEN_W<INTEN_SPEC, 6> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable or disable on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn inten7(&mut self) -> INTEN_W<INTEN_SPEC, 7> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable or disable on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn inten8(&mut self) -> INTEN_W<INTEN_SPEC, 8> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt enable or disable on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn inten9(&mut self) -> INTEN_W<INTEN_SPEC, 9> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt enable or disable on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn inten10(&mut self) -> INTEN_W<INTEN_SPEC, 10> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt enable or disable on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn inten11(&mut self) -> INTEN_W<INTEN_SPEC, 11> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt enable or disable on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn inten12(&mut self) -> INTEN_W<INTEN_SPEC, 12> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt enable or disable on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn inten13(&mut self) -> INTEN_W<INTEN_SPEC, 13> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt enable or disable on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn inten14(&mut self) -> INTEN_W<INTEN_SPEC, 14> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt enable or disable on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn inten15(&mut self) -> INTEN_W<INTEN_SPEC, 15> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt enable or disable on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn inten16(&mut self) -> INTEN_W<INTEN_SPEC, 16> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt enable or disable on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn inten17(&mut self) -> INTEN_W<INTEN_SPEC, 17> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt enable or disable on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn inten18(&mut self) -> INTEN_W<INTEN_SPEC, 18> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt enable or disable on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn inten19(&mut self) -> INTEN_W<INTEN_SPEC, 19> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt enable or disable on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn inten20(&mut self) -> INTEN_W<INTEN_SPEC, 20> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt enable or disable on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn inten21(&mut self) -> INTEN_W<INTEN_SPEC, 21> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt enable or disable on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn inten22(&mut self) -> INTEN_W<INTEN_SPEC, 22> {
        INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
