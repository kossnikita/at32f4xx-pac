#[doc = "Register `SWTRG` reader"]
pub type R = crate::R<SWTRG_SPEC>;
#[doc = "Register `SWTRG` writer"]
pub type W = crate::W<SWTRG_SPEC>;
#[doc = "Field `SWT[0-22]` reader - Software trigger on line %s"]
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
    pub const fn variant(&self) -> SWT0R_A {
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
#[doc = "Field `SWT[0-22]` writer - Software trigger on line %s"]
pub type SWT_W<'a, REG> = crate::BitWriter1S<'a, REG, SWT0W_AW>;
impl<'a, REG> SWT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate trigger"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWT0W_AW::Trigger)
    }
}
impl R {
    #[doc = "Software trigger on line [0-22]\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn swt(&self, n: u8) -> SWT_R {
        assert!(n < 23);
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
    #[doc = "Bit 18 - Software trigger on line 18"]
    #[inline(always)]
    pub fn swt18(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Software trigger on line 19"]
    #[inline(always)]
    pub fn swt19(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software trigger on line 20"]
    #[inline(always)]
    pub fn swt20(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software trigger on line 21"]
    #[inline(always)]
    pub fn swt21(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software trigger on line 22"]
    #[inline(always)]
    pub fn swt22(&self) -> SWT_R {
        SWT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWTRG")
            .field("swt0", &format_args!("{}", self.swt0().bit()))
            .field("swt1", &format_args!("{}", self.swt1().bit()))
            .field("swt2", &format_args!("{}", self.swt2().bit()))
            .field("swt3", &format_args!("{}", self.swt3().bit()))
            .field("swt4", &format_args!("{}", self.swt4().bit()))
            .field("swt5", &format_args!("{}", self.swt5().bit()))
            .field("swt6", &format_args!("{}", self.swt6().bit()))
            .field("swt7", &format_args!("{}", self.swt7().bit()))
            .field("swt8", &format_args!("{}", self.swt8().bit()))
            .field("swt9", &format_args!("{}", self.swt9().bit()))
            .field("swt10", &format_args!("{}", self.swt10().bit()))
            .field("swt11", &format_args!("{}", self.swt11().bit()))
            .field("swt12", &format_args!("{}", self.swt12().bit()))
            .field("swt13", &format_args!("{}", self.swt13().bit()))
            .field("swt14", &format_args!("{}", self.swt14().bit()))
            .field("swt15", &format_args!("{}", self.swt15().bit()))
            .field("swt16", &format_args!("{}", self.swt16().bit()))
            .field("swt17", &format_args!("{}", self.swt17().bit()))
            .field("swt18", &format_args!("{}", self.swt18().bit()))
            .field("swt19", &format_args!("{}", self.swt19().bit()))
            .field("swt20", &format_args!("{}", self.swt20().bit()))
            .field("swt21", &format_args!("{}", self.swt21().bit()))
            .field("swt22", &format_args!("{}", self.swt22().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SWTRG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Software trigger on line [0-22]"]
    #[inline(always)]
    #[must_use]
    pub fn swt(&mut self, n: u8) -> SWT_W<SWTRG_SPEC> {
        assert!(n < 23);
        SWT_W::new(self, n)
    }
    #[doc = "Bit 0 - Software trigger on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swt0(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software trigger on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swt1(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software trigger on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swt2(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software trigger on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swt3(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software trigger on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swt4(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software trigger on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swt5(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software trigger on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swt6(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software trigger on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swt7(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software trigger on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swt8(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software trigger on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swt9(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software trigger on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swt10(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software trigger on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swt11(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software trigger on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swt12(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software trigger on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swt13(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software trigger on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swt14(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software trigger on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swt15(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 15)
    }
    #[doc = "Bit 16 - Software trigger on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swt16(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Software trigger on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swt17(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Software trigger on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn swt18(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Software trigger on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn swt19(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Software trigger on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn swt20(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 20)
    }
    #[doc = "Bit 21 - Software trigger on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn swt21(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 21)
    }
    #[doc = "Bit 22 - Software trigger on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn swt22(&mut self) -> SWT_W<SWTRG_SPEC> {
        SWT_W::new(self, 22)
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
#[doc = "Software triggle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRG_SPEC;
impl crate::RegisterSpec for SWTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrg::R`](R) reader structure"]
impl crate::Readable for SWTRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swtrg::W`](W) writer structure"]
impl crate::Writable for SWTRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets SWTRG to value 0"]
impl crate::Resettable for SWTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
