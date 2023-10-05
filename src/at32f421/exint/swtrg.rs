#[doc = "Register `SWTRG` reader"]
pub type R = crate::R<SWTRG_SPEC>;
#[doc = "Register `SWTRG` writer"]
pub type W = crate::W<SWTRG_SPEC>;
#[doc = "Field `SWT[0-17]` reader - Software trigger on line %s"]
pub type SWT_R = crate::BitReader<SWT0R_A>;
#[doc = "Software trigger on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT0R_A {
    #[doc = "0: Default value"]
    NoTrigger = 0,
    #[doc = "1: Software trigger generated"]
    Triggered = 1,
}
impl From<SWT0R_A> for bool {
    #[inline(always)]
    fn from(variant: SWT0R_A) -> Self {
        variant as u8 != 0
    }
}
impl SWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWT0R_A {
        match self.bits {
            false => SWT0R_A::NoTrigger,
            true => SWT0R_A::Triggered,
        }
    }
    #[doc = "Default value"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == SWT0R_A::NoTrigger
    }
    #[doc = "Software trigger generated"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == SWT0R_A::Triggered
    }
}
#[doc = "Software trigger on line %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWT0W_AW {
    #[doc = "1: Generate trigger"]
    Trigger = 1,
}
impl From<SWT0W_AW> for bool {
    #[inline(always)]
    fn from(variant: SWT0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWT[0-17]` writer - Software trigger on line %s"]
pub type SWT_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, SWT0W_AW>;
impl<'a, REG, const O: u8> SWT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate trigger"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWT0W_AW::Trigger)
    }
}
#[doc = "Field `SWT19` reader - Software triggle on line 19"]
pub use SWT0_R as SWT19_R;
#[doc = "Field `SWT21` reader - Software triggle on line 21"]
pub use SWT0_R as SWT21_R;
#[doc = "Field `SWT19` writer - Software triggle on line 19"]
pub use SWT0_W as SWT19_W;
#[doc = "Field `SWT21` writer - Software triggle on line 21"]
pub use SWT0_W as SWT21_W;
impl R {
    #[doc = "Software trigger on line [0-17]"]
    #[inline(always)]
    pub unsafe fn swt(&self, n: u8) -> SWT_R {
        SWT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Software trigger on line 0"]
    #[inline(always)]
    pub fn swt0(&self) -> SWT_R {
        SWT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software trigger on line 1"]
    #[inline(always)]
    pub fn swt1(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software trigger on line 2"]
    #[inline(always)]
    pub fn swt2(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software trigger on line 3"]
    #[inline(always)]
    pub fn swt3(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software trigger on line 4"]
    #[inline(always)]
    pub fn swt4(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software trigger on line 5"]
    #[inline(always)]
    pub fn swt5(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software trigger on line 6"]
    #[inline(always)]
    pub fn swt6(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software trigger on line 7"]
    #[inline(always)]
    pub fn swt7(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software trigger on line 8"]
    #[inline(always)]
    pub fn swt8(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software trigger on line 9"]
    #[inline(always)]
    pub fn swt9(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software trigger on line 10"]
    #[inline(always)]
    pub fn swt10(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software trigger on line 11"]
    #[inline(always)]
    pub fn swt11(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software trigger on line 12"]
    #[inline(always)]
    pub fn swt12(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software trigger on line 13"]
    #[inline(always)]
    pub fn swt13(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software trigger on line 14"]
    #[inline(always)]
    pub fn swt14(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software trigger on line 15"]
    #[inline(always)]
    pub fn swt15(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software trigger on line 16"]
    #[inline(always)]
    pub fn swt16(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software trigger on line 17"]
    #[inline(always)]
    pub fn swt17(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software triggle on line 19"]
    #[inline(always)]
    pub fn swt19(&self) -> SWT19_R {
        SWT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Software triggle on line 21"]
    #[inline(always)]
    pub fn swt21(&self) -> SWT21_R {
        SWT21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Software trigger on line [0-17]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn swt<const O: u8>(&mut self) -> SWT_W<SWTRG_SPEC, O> {
        SWT_W::new(self)
    }
    #[doc = "Bit 0 - Software trigger on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swt0(&mut self) -> SWT_W<SWTRG_SPEC, 0> {
        SWT_W::new(self)
    }
    #[doc = "Bit 1 - Software trigger on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swt1(&mut self) -> SWT_W<SWTRG_SPEC, 1> {
        SWT_W::new(self)
    }
    #[doc = "Bit 2 - Software trigger on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swt2(&mut self) -> SWT_W<SWTRG_SPEC, 2> {
        SWT_W::new(self)
    }
    #[doc = "Bit 3 - Software trigger on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swt3(&mut self) -> SWT_W<SWTRG_SPEC, 3> {
        SWT_W::new(self)
    }
    #[doc = "Bit 4 - Software trigger on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swt4(&mut self) -> SWT_W<SWTRG_SPEC, 4> {
        SWT_W::new(self)
    }
    #[doc = "Bit 5 - Software trigger on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swt5(&mut self) -> SWT_W<SWTRG_SPEC, 5> {
        SWT_W::new(self)
    }
    #[doc = "Bit 6 - Software trigger on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swt6(&mut self) -> SWT_W<SWTRG_SPEC, 6> {
        SWT_W::new(self)
    }
    #[doc = "Bit 7 - Software trigger on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swt7(&mut self) -> SWT_W<SWTRG_SPEC, 7> {
        SWT_W::new(self)
    }
    #[doc = "Bit 8 - Software trigger on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swt8(&mut self) -> SWT_W<SWTRG_SPEC, 8> {
        SWT_W::new(self)
    }
    #[doc = "Bit 9 - Software trigger on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swt9(&mut self) -> SWT_W<SWTRG_SPEC, 9> {
        SWT_W::new(self)
    }
    #[doc = "Bit 10 - Software trigger on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swt10(&mut self) -> SWT_W<SWTRG_SPEC, 10> {
        SWT_W::new(self)
    }
    #[doc = "Bit 11 - Software trigger on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swt11(&mut self) -> SWT_W<SWTRG_SPEC, 11> {
        SWT_W::new(self)
    }
    #[doc = "Bit 12 - Software trigger on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swt12(&mut self) -> SWT_W<SWTRG_SPEC, 12> {
        SWT_W::new(self)
    }
    #[doc = "Bit 13 - Software trigger on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swt13(&mut self) -> SWT_W<SWTRG_SPEC, 13> {
        SWT_W::new(self)
    }
    #[doc = "Bit 14 - Software trigger on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swt14(&mut self) -> SWT_W<SWTRG_SPEC, 14> {
        SWT_W::new(self)
    }
    #[doc = "Bit 15 - Software trigger on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swt15(&mut self) -> SWT_W<SWTRG_SPEC, 15> {
        SWT_W::new(self)
    }
    #[doc = "Bit 16 - Software trigger on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swt16(&mut self) -> SWT_W<SWTRG_SPEC, 16> {
        SWT_W::new(self)
    }
    #[doc = "Bit 17 - Software trigger on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swt17(&mut self) -> SWT_W<SWTRG_SPEC, 17> {
        SWT_W::new(self)
    }
    #[doc = "Bit 19 - Software triggle on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn swt19(&mut self) -> SWT19_W<SWTRG_SPEC, 19> {
        SWT19_W::new(self)
    }
    #[doc = "Bit 21 - Software triggle on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn swt21(&mut self) -> SWT21_W<SWTRG_SPEC, 21> {
        SWT21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software triggle register (EXTINT_SWIE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRG_SPEC;
impl crate::RegisterSpec for SWTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrg::R`](R) reader structure"]
impl crate::Readable for SWTRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swtrg::W`](W) writer structure"]
impl crate::Writable for SWTRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0028_0001;
}
#[doc = "`reset()` method sets SWTRG to value 0"]
impl crate::Resettable for SWTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
