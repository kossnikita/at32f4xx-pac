#[doc = "Register `POLCFG1` reader"]
pub type R = crate::R<POLCFG1_SPEC>;
#[doc = "Register `POLCFG1` writer"]
pub type W = crate::W<POLCFG1_SPEC>;
#[doc = "Field `RP0` reader - Rising polarity configuration bit of line 0"]
pub type RP0_R = crate::BitReader;
#[doc = "Field `RP0` writer - Rising polarity configuration bit of line 0"]
pub type RP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP1` reader - Rising polarity configuration bit of line 1"]
pub type RP1_R = crate::BitReader;
#[doc = "Field `RP1` writer - Rising polarity configuration bit of line 1"]
pub type RP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP2` reader - Rising polarity configuration bit of line 2"]
pub type RP2_R = crate::BitReader;
#[doc = "Field `RP2` writer - Rising polarity configuration bit of line 2"]
pub type RP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP3` reader - Rising polarity configuration bit of line 3"]
pub type RP3_R = crate::BitReader;
#[doc = "Field `RP3` writer - Rising polarity configuration bit of line 3"]
pub type RP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP4` reader - Rising polarity configuration bit of line 4"]
pub type RP4_R = crate::BitReader;
#[doc = "Field `RP4` writer - Rising polarity configuration bit of line 4"]
pub type RP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP5` reader - Rising polarity configuration bit of line 5"]
pub type RP5_R = crate::BitReader;
#[doc = "Field `RP5` writer - Rising polarity configuration bit of line 5"]
pub type RP5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP6` reader - Rising polarity configuration bit of linee 6"]
pub type RP6_R = crate::BitReader;
#[doc = "Field `RP6` writer - Rising polarity configuration bit of linee 6"]
pub type RP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP7` reader - Rising polarity configuration bit of line 7"]
pub type RP7_R = crate::BitReader;
#[doc = "Field `RP7` writer - Rising polarity configuration bit of line 7"]
pub type RP7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP8` reader - Rising polarity configuration bit of line 8"]
pub type RP8_R = crate::BitReader;
#[doc = "Field `RP8` writer - Rising polarity configuration bit of line 8"]
pub type RP8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP9` reader - Rising polarity configuration bit of line 9"]
pub type RP9_R = crate::BitReader;
#[doc = "Field `RP9` writer - Rising polarity configuration bit of line 9"]
pub type RP9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP10` reader - Rising polarity configuration bit of line 10"]
pub type RP10_R = crate::BitReader;
#[doc = "Field `RP10` writer - Rising polarity configuration bit of line 10"]
pub type RP10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP11` reader - Rising polarity configuration bit of line 11"]
pub type RP11_R = crate::BitReader;
#[doc = "Field `RP11` writer - Rising polarity configuration bit of line 11"]
pub type RP11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP12` reader - Rising polarity configuration bit of line 12"]
pub type RP12_R = crate::BitReader;
#[doc = "Field `RP12` writer - Rising polarity configuration bit of line 12"]
pub type RP12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP13` reader - Rising polarity configuration bit of line 13"]
pub type RP13_R = crate::BitReader;
#[doc = "Field `RP13` writer - Rising polarity configuration bit of line 13"]
pub type RP13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP14` reader - Rising polarity configuration bit of line 14"]
pub type RP14_R = crate::BitReader;
#[doc = "Field `RP14` writer - Rising polarity configuration bit of line 14"]
pub type RP14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP15` reader - Rising polarity configuration bit of line 15"]
pub type RP15_R = crate::BitReader;
#[doc = "Field `RP15` writer - Rising polarity configuration bit of line 15"]
pub type RP15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP16` reader - Rising polarity configuration bit of line 16"]
pub type RP16_R = crate::BitReader;
#[doc = "Field `RP16` writer - Rising polarity configuration bit of line 16"]
pub type RP16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP17` reader - Rising polarity configuration bit of line 17"]
pub type RP17_R = crate::BitReader;
#[doc = "Field `RP17` writer - Rising polarity configuration bit of line 17"]
pub type RP17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RP18` reader - Rising polarity configuration bit of line 18"]
pub type RP18_R = crate::BitReader;
#[doc = "Field `RP18` writer - Rising polarity configuration bit of line 18"]
pub type RP18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Rising polarity configuration bit of line 0"]
    #[inline(always)]
    pub fn rp0(&self) -> RP0_R {
        RP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising polarity configuration bit of line 1"]
    #[inline(always)]
    pub fn rp1(&self) -> RP1_R {
        RP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising polarity configuration bit of line 2"]
    #[inline(always)]
    pub fn rp2(&self) -> RP2_R {
        RP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising polarity configuration bit of line 3"]
    #[inline(always)]
    pub fn rp3(&self) -> RP3_R {
        RP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising polarity configuration bit of line 4"]
    #[inline(always)]
    pub fn rp4(&self) -> RP4_R {
        RP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising polarity configuration bit of line 5"]
    #[inline(always)]
    pub fn rp5(&self) -> RP5_R {
        RP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising polarity configuration bit of linee 6"]
    #[inline(always)]
    pub fn rp6(&self) -> RP6_R {
        RP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising polarity configuration bit of line 7"]
    #[inline(always)]
    pub fn rp7(&self) -> RP7_R {
        RP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising polarity configuration bit of line 8"]
    #[inline(always)]
    pub fn rp8(&self) -> RP8_R {
        RP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising polarity configuration bit of line 9"]
    #[inline(always)]
    pub fn rp9(&self) -> RP9_R {
        RP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising polarity configuration bit of line 10"]
    #[inline(always)]
    pub fn rp10(&self) -> RP10_R {
        RP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising polarity configuration bit of line 11"]
    #[inline(always)]
    pub fn rp11(&self) -> RP11_R {
        RP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising polarity configuration bit of line 12"]
    #[inline(always)]
    pub fn rp12(&self) -> RP12_R {
        RP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising polarity configuration bit of line 13"]
    #[inline(always)]
    pub fn rp13(&self) -> RP13_R {
        RP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising polarity configuration bit of line 14"]
    #[inline(always)]
    pub fn rp14(&self) -> RP14_R {
        RP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising polarity configuration bit of line 15"]
    #[inline(always)]
    pub fn rp15(&self) -> RP15_R {
        RP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising polarity configuration bit of line 16"]
    #[inline(always)]
    pub fn rp16(&self) -> RP16_R {
        RP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising polarity configuration bit of line 17"]
    #[inline(always)]
    pub fn rp17(&self) -> RP17_R {
        RP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising polarity configuration bit of line 18"]
    #[inline(always)]
    pub fn rp18(&self) -> RP18_R {
        RP18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising polarity configuration bit of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn rp0(&mut self) -> RP0_W<POLCFG1_SPEC, 0> {
        RP0_W::new(self)
    }
    #[doc = "Bit 1 - Rising polarity configuration bit of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn rp1(&mut self) -> RP1_W<POLCFG1_SPEC, 1> {
        RP1_W::new(self)
    }
    #[doc = "Bit 2 - Rising polarity configuration bit of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn rp2(&mut self) -> RP2_W<POLCFG1_SPEC, 2> {
        RP2_W::new(self)
    }
    #[doc = "Bit 3 - Rising polarity configuration bit of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn rp3(&mut self) -> RP3_W<POLCFG1_SPEC, 3> {
        RP3_W::new(self)
    }
    #[doc = "Bit 4 - Rising polarity configuration bit of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn rp4(&mut self) -> RP4_W<POLCFG1_SPEC, 4> {
        RP4_W::new(self)
    }
    #[doc = "Bit 5 - Rising polarity configuration bit of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn rp5(&mut self) -> RP5_W<POLCFG1_SPEC, 5> {
        RP5_W::new(self)
    }
    #[doc = "Bit 6 - Rising polarity configuration bit of linee 6"]
    #[inline(always)]
    #[must_use]
    pub fn rp6(&mut self) -> RP6_W<POLCFG1_SPEC, 6> {
        RP6_W::new(self)
    }
    #[doc = "Bit 7 - Rising polarity configuration bit of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn rp7(&mut self) -> RP7_W<POLCFG1_SPEC, 7> {
        RP7_W::new(self)
    }
    #[doc = "Bit 8 - Rising polarity configuration bit of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn rp8(&mut self) -> RP8_W<POLCFG1_SPEC, 8> {
        RP8_W::new(self)
    }
    #[doc = "Bit 9 - Rising polarity configuration bit of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn rp9(&mut self) -> RP9_W<POLCFG1_SPEC, 9> {
        RP9_W::new(self)
    }
    #[doc = "Bit 10 - Rising polarity configuration bit of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn rp10(&mut self) -> RP10_W<POLCFG1_SPEC, 10> {
        RP10_W::new(self)
    }
    #[doc = "Bit 11 - Rising polarity configuration bit of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn rp11(&mut self) -> RP11_W<POLCFG1_SPEC, 11> {
        RP11_W::new(self)
    }
    #[doc = "Bit 12 - Rising polarity configuration bit of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn rp12(&mut self) -> RP12_W<POLCFG1_SPEC, 12> {
        RP12_W::new(self)
    }
    #[doc = "Bit 13 - Rising polarity configuration bit of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn rp13(&mut self) -> RP13_W<POLCFG1_SPEC, 13> {
        RP13_W::new(self)
    }
    #[doc = "Bit 14 - Rising polarity configuration bit of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn rp14(&mut self) -> RP14_W<POLCFG1_SPEC, 14> {
        RP14_W::new(self)
    }
    #[doc = "Bit 15 - Rising polarity configuration bit of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn rp15(&mut self) -> RP15_W<POLCFG1_SPEC, 15> {
        RP15_W::new(self)
    }
    #[doc = "Bit 16 - Rising polarity configuration bit of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn rp16(&mut self) -> RP16_W<POLCFG1_SPEC, 16> {
        RP16_W::new(self)
    }
    #[doc = "Bit 17 - Rising polarity configuration bit of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn rp17(&mut self) -> RP17_W<POLCFG1_SPEC, 17> {
        RP17_W::new(self)
    }
    #[doc = "Bit 18 - Rising polarity configuration bit of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn rp18(&mut self) -> RP18_W<POLCFG1_SPEC, 18> {
        RP18_W::new(self)
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