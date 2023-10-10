#[doc = "Register `SWTRG` reader"]
pub type R = crate::R<SWTRG_SPEC>;
#[doc = "Register `SWTRG` writer"]
pub type W = crate::W<SWTRG_SPEC>;
#[doc = "Field `SWT0` reader - Software triggle on line 0"]
pub type SWT0_R = crate::BitReader;
#[doc = "Field `SWT0` writer - Software triggle on line 0"]
pub type SWT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT1` reader - Software triggle on line 1"]
pub type SWT1_R = crate::BitReader;
#[doc = "Field `SWT1` writer - Software triggle on line 1"]
pub type SWT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT2` reader - Software triggle on line 2"]
pub type SWT2_R = crate::BitReader;
#[doc = "Field `SWT2` writer - Software triggle on line 2"]
pub type SWT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT3` reader - Software triggle on line 3"]
pub type SWT3_R = crate::BitReader;
#[doc = "Field `SWT3` writer - Software triggle on line 3"]
pub type SWT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT4` reader - Software triggle on line 4"]
pub type SWT4_R = crate::BitReader;
#[doc = "Field `SWT4` writer - Software triggle on line 4"]
pub type SWT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT5` reader - Software triggle on line 5"]
pub type SWT5_R = crate::BitReader;
#[doc = "Field `SWT5` writer - Software triggle on line 5"]
pub type SWT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT6` reader - Software triggle on line 6"]
pub type SWT6_R = crate::BitReader;
#[doc = "Field `SWT6` writer - Software triggle on line 6"]
pub type SWT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT7` reader - Software triggle on line 7"]
pub type SWT7_R = crate::BitReader;
#[doc = "Field `SWT7` writer - Software triggle on line 7"]
pub type SWT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT8` reader - Software triggle on line 8"]
pub type SWT8_R = crate::BitReader;
#[doc = "Field `SWT8` writer - Software triggle on line 8"]
pub type SWT8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT9` reader - Software triggle on line 9"]
pub type SWT9_R = crate::BitReader;
#[doc = "Field `SWT9` writer - Software triggle on line 9"]
pub type SWT9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT10` reader - Software triggle on line 10"]
pub type SWT10_R = crate::BitReader;
#[doc = "Field `SWT10` writer - Software triggle on line 10"]
pub type SWT10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT11` reader - Software triggle on line 11"]
pub type SWT11_R = crate::BitReader;
#[doc = "Field `SWT11` writer - Software triggle on line 11"]
pub type SWT11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT12` reader - Software triggle on line 12"]
pub type SWT12_R = crate::BitReader;
#[doc = "Field `SWT12` writer - Software triggle on line 12"]
pub type SWT12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT13` reader - Software triggle on line 13"]
pub type SWT13_R = crate::BitReader;
#[doc = "Field `SWT13` writer - Software triggle on line 13"]
pub type SWT13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT14` reader - Software triggle on line 14"]
pub type SWT14_R = crate::BitReader;
#[doc = "Field `SWT14` writer - Software triggle on line 14"]
pub type SWT14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT15` reader - Software triggle on line 15"]
pub type SWT15_R = crate::BitReader;
#[doc = "Field `SWT15` writer - Software triggle on line 15"]
pub type SWT15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT16` reader - Software triggle on line 16"]
pub type SWT16_R = crate::BitReader;
#[doc = "Field `SWT16` writer - Software triggle on line 16"]
pub type SWT16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT17` reader - Software triggle on line 17"]
pub type SWT17_R = crate::BitReader;
#[doc = "Field `SWT17` writer - Software triggle on line 17"]
pub type SWT17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWT18` reader - Software triggle on line 18"]
pub type SWT18_R = crate::BitReader;
#[doc = "Field `SWT18` writer - Software triggle on line 18"]
pub type SWT18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software triggle on line 0"]
    #[inline(always)]
    pub fn swt0(&self) -> SWT0_R {
        SWT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software triggle on line 1"]
    #[inline(always)]
    pub fn swt1(&self) -> SWT1_R {
        SWT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software triggle on line 2"]
    #[inline(always)]
    pub fn swt2(&self) -> SWT2_R {
        SWT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software triggle on line 3"]
    #[inline(always)]
    pub fn swt3(&self) -> SWT3_R {
        SWT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software triggle on line 4"]
    #[inline(always)]
    pub fn swt4(&self) -> SWT4_R {
        SWT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software triggle on line 5"]
    #[inline(always)]
    pub fn swt5(&self) -> SWT5_R {
        SWT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software triggle on line 6"]
    #[inline(always)]
    pub fn swt6(&self) -> SWT6_R {
        SWT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software triggle on line 7"]
    #[inline(always)]
    pub fn swt7(&self) -> SWT7_R {
        SWT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software triggle on line 8"]
    #[inline(always)]
    pub fn swt8(&self) -> SWT8_R {
        SWT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software triggle on line 9"]
    #[inline(always)]
    pub fn swt9(&self) -> SWT9_R {
        SWT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software triggle on line 10"]
    #[inline(always)]
    pub fn swt10(&self) -> SWT10_R {
        SWT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software triggle on line 11"]
    #[inline(always)]
    pub fn swt11(&self) -> SWT11_R {
        SWT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software triggle on line 12"]
    #[inline(always)]
    pub fn swt12(&self) -> SWT12_R {
        SWT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software triggle on line 13"]
    #[inline(always)]
    pub fn swt13(&self) -> SWT13_R {
        SWT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software triggle on line 14"]
    #[inline(always)]
    pub fn swt14(&self) -> SWT14_R {
        SWT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software triggle on line 15"]
    #[inline(always)]
    pub fn swt15(&self) -> SWT15_R {
        SWT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software triggle on line 16"]
    #[inline(always)]
    pub fn swt16(&self) -> SWT16_R {
        SWT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software triggle on line 17"]
    #[inline(always)]
    pub fn swt17(&self) -> SWT17_R {
        SWT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Software triggle on line 18"]
    #[inline(always)]
    pub fn swt18(&self) -> SWT18_R {
        SWT18_R::new(((self.bits >> 18) & 1) != 0)
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
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SWTRG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Software triggle on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swt0(&mut self) -> SWT0_W<SWTRG_SPEC, 0> {
        SWT0_W::new(self)
    }
    #[doc = "Bit 1 - Software triggle on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swt1(&mut self) -> SWT1_W<SWTRG_SPEC, 1> {
        SWT1_W::new(self)
    }
    #[doc = "Bit 2 - Software triggle on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swt2(&mut self) -> SWT2_W<SWTRG_SPEC, 2> {
        SWT2_W::new(self)
    }
    #[doc = "Bit 3 - Software triggle on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swt3(&mut self) -> SWT3_W<SWTRG_SPEC, 3> {
        SWT3_W::new(self)
    }
    #[doc = "Bit 4 - Software triggle on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swt4(&mut self) -> SWT4_W<SWTRG_SPEC, 4> {
        SWT4_W::new(self)
    }
    #[doc = "Bit 5 - Software triggle on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swt5(&mut self) -> SWT5_W<SWTRG_SPEC, 5> {
        SWT5_W::new(self)
    }
    #[doc = "Bit 6 - Software triggle on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swt6(&mut self) -> SWT6_W<SWTRG_SPEC, 6> {
        SWT6_W::new(self)
    }
    #[doc = "Bit 7 - Software triggle on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swt7(&mut self) -> SWT7_W<SWTRG_SPEC, 7> {
        SWT7_W::new(self)
    }
    #[doc = "Bit 8 - Software triggle on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swt8(&mut self) -> SWT8_W<SWTRG_SPEC, 8> {
        SWT8_W::new(self)
    }
    #[doc = "Bit 9 - Software triggle on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swt9(&mut self) -> SWT9_W<SWTRG_SPEC, 9> {
        SWT9_W::new(self)
    }
    #[doc = "Bit 10 - Software triggle on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swt10(&mut self) -> SWT10_W<SWTRG_SPEC, 10> {
        SWT10_W::new(self)
    }
    #[doc = "Bit 11 - Software triggle on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swt11(&mut self) -> SWT11_W<SWTRG_SPEC, 11> {
        SWT11_W::new(self)
    }
    #[doc = "Bit 12 - Software triggle on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swt12(&mut self) -> SWT12_W<SWTRG_SPEC, 12> {
        SWT12_W::new(self)
    }
    #[doc = "Bit 13 - Software triggle on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swt13(&mut self) -> SWT13_W<SWTRG_SPEC, 13> {
        SWT13_W::new(self)
    }
    #[doc = "Bit 14 - Software triggle on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swt14(&mut self) -> SWT14_W<SWTRG_SPEC, 14> {
        SWT14_W::new(self)
    }
    #[doc = "Bit 15 - Software triggle on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swt15(&mut self) -> SWT15_W<SWTRG_SPEC, 15> {
        SWT15_W::new(self)
    }
    #[doc = "Bit 16 - Software triggle on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swt16(&mut self) -> SWT16_W<SWTRG_SPEC, 16> {
        SWT16_W::new(self)
    }
    #[doc = "Bit 17 - Software triggle on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swt17(&mut self) -> SWT17_W<SWTRG_SPEC, 17> {
        SWT17_W::new(self)
    }
    #[doc = "Bit 18 - Software triggle on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn swt18(&mut self) -> SWT18_W<SWTRG_SPEC, 18> {
        SWT18_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRG to value 0"]
impl crate::Resettable for SWTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
