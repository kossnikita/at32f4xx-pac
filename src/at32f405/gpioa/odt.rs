#[doc = "Register `ODT` reader"]
pub type R = crate::R<ODT_SPEC>;
#[doc = "Register `ODT` writer"]
pub type W = crate::W<ODT_SPEC>;
#[doc = "Field `ODT0` reader - Port output data"]
pub type ODT0_R = crate::BitReader;
#[doc = "Field `ODT0` writer - Port output data"]
pub type ODT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT1` reader - Port output data"]
pub type ODT1_R = crate::BitReader;
#[doc = "Field `ODT1` writer - Port output data"]
pub type ODT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT2` reader - Port output data"]
pub type ODT2_R = crate::BitReader;
#[doc = "Field `ODT2` writer - Port output data"]
pub type ODT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT3` reader - Port output data"]
pub type ODT3_R = crate::BitReader;
#[doc = "Field `ODT3` writer - Port output data"]
pub type ODT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT4` reader - Port output data"]
pub type ODT4_R = crate::BitReader;
#[doc = "Field `ODT4` writer - Port output data"]
pub type ODT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT5` reader - Port output data"]
pub type ODT5_R = crate::BitReader;
#[doc = "Field `ODT5` writer - Port output data"]
pub type ODT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT6` reader - Port output data"]
pub type ODT6_R = crate::BitReader;
#[doc = "Field `ODT6` writer - Port output data"]
pub type ODT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT7` reader - Port output data"]
pub type ODT7_R = crate::BitReader;
#[doc = "Field `ODT7` writer - Port output data"]
pub type ODT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT8` reader - Port output data"]
pub type ODT8_R = crate::BitReader;
#[doc = "Field `ODT8` writer - Port output data"]
pub type ODT8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT9` reader - Port output data"]
pub type ODT9_R = crate::BitReader;
#[doc = "Field `ODT9` writer - Port output data"]
pub type ODT9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT10` reader - Port output data"]
pub type ODT10_R = crate::BitReader;
#[doc = "Field `ODT10` writer - Port output data"]
pub type ODT10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT11` reader - Port output data"]
pub type ODT11_R = crate::BitReader;
#[doc = "Field `ODT11` writer - Port output data"]
pub type ODT11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT12` reader - Port output data"]
pub type ODT12_R = crate::BitReader;
#[doc = "Field `ODT12` writer - Port output data"]
pub type ODT12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT13` reader - Port output data"]
pub type ODT13_R = crate::BitReader;
#[doc = "Field `ODT13` writer - Port output data"]
pub type ODT13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT14` reader - Port output data"]
pub type ODT14_R = crate::BitReader;
#[doc = "Field `ODT14` writer - Port output data"]
pub type ODT14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODT15` reader - Port output data"]
pub type ODT15_R = crate::BitReader;
#[doc = "Field `ODT15` writer - Port output data"]
pub type ODT15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    pub fn odt0(&self) -> ODT0_R {
        ODT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    pub fn odt1(&self) -> ODT1_R {
        ODT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    pub fn odt2(&self) -> ODT2_R {
        ODT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    pub fn odt3(&self) -> ODT3_R {
        ODT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    pub fn odt4(&self) -> ODT4_R {
        ODT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    pub fn odt5(&self) -> ODT5_R {
        ODT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    pub fn odt6(&self) -> ODT6_R {
        ODT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    pub fn odt7(&self) -> ODT7_R {
        ODT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    pub fn odt8(&self) -> ODT8_R {
        ODT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    pub fn odt9(&self) -> ODT9_R {
        ODT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    pub fn odt10(&self) -> ODT10_R {
        ODT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    pub fn odt11(&self) -> ODT11_R {
        ODT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    pub fn odt12(&self) -> ODT12_R {
        ODT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    pub fn odt13(&self) -> ODT13_R {
        ODT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    pub fn odt14(&self) -> ODT14_R {
        ODT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    pub fn odt15(&self) -> ODT15_R {
        ODT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt0(&mut self) -> ODT0_W<ODT_SPEC, 0> {
        ODT0_W::new(self)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt1(&mut self) -> ODT1_W<ODT_SPEC, 1> {
        ODT1_W::new(self)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt2(&mut self) -> ODT2_W<ODT_SPEC, 2> {
        ODT2_W::new(self)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt3(&mut self) -> ODT3_W<ODT_SPEC, 3> {
        ODT3_W::new(self)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt4(&mut self) -> ODT4_W<ODT_SPEC, 4> {
        ODT4_W::new(self)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt5(&mut self) -> ODT5_W<ODT_SPEC, 5> {
        ODT5_W::new(self)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt6(&mut self) -> ODT6_W<ODT_SPEC, 6> {
        ODT6_W::new(self)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt7(&mut self) -> ODT7_W<ODT_SPEC, 7> {
        ODT7_W::new(self)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt8(&mut self) -> ODT8_W<ODT_SPEC, 8> {
        ODT8_W::new(self)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt9(&mut self) -> ODT9_W<ODT_SPEC, 9> {
        ODT9_W::new(self)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt10(&mut self) -> ODT10_W<ODT_SPEC, 10> {
        ODT10_W::new(self)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt11(&mut self) -> ODT11_W<ODT_SPEC, 11> {
        ODT11_W::new(self)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt12(&mut self) -> ODT12_W<ODT_SPEC, 12> {
        ODT12_W::new(self)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt13(&mut self) -> ODT13_W<ODT_SPEC, 13> {
        ODT13_W::new(self)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt14(&mut self) -> ODT14_W<ODT_SPEC, 14> {
        ODT14_W::new(self)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt15(&mut self) -> ODT15_W<ODT_SPEC, 15> {
        ODT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODT_SPEC;
impl crate::RegisterSpec for ODT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for ODT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odt::W`](W) writer structure"]
impl crate::Writable for ODT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for ODT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
