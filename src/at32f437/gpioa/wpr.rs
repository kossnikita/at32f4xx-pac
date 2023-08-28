#[doc = "Register `WPR` reader"]
pub type R = crate::R<WPR_SPEC>;
#[doc = "Register `WPR` writer"]
pub type W = crate::W<WPR_SPEC>;
#[doc = "Field `WPEN0` reader - Write protect enable 0"]
pub type WPEN0_R = crate::BitReader;
#[doc = "Field `WPEN0` writer - Write protect enable 0"]
pub type WPEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN1` reader - Write protect enable 1"]
pub type WPEN1_R = crate::BitReader;
#[doc = "Field `WPEN1` writer - Write protect enable 1"]
pub type WPEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN2` reader - Write protect enable 2"]
pub type WPEN2_R = crate::BitReader;
#[doc = "Field `WPEN2` writer - Write protect enable 2"]
pub type WPEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN3` reader - Write protect enable 3"]
pub type WPEN3_R = crate::BitReader;
#[doc = "Field `WPEN3` writer - Write protect enable 3"]
pub type WPEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN4` reader - Write protect enable 4"]
pub type WPEN4_R = crate::BitReader;
#[doc = "Field `WPEN4` writer - Write protect enable 4"]
pub type WPEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN5` reader - Write protect enable 5"]
pub type WPEN5_R = crate::BitReader;
#[doc = "Field `WPEN5` writer - Write protect enable 5"]
pub type WPEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN6` reader - Write protect enable 6"]
pub type WPEN6_R = crate::BitReader;
#[doc = "Field `WPEN6` writer - Write protect enable 6"]
pub type WPEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN7` reader - Write protect enable 7"]
pub type WPEN7_R = crate::BitReader;
#[doc = "Field `WPEN7` writer - Write protect enable 7"]
pub type WPEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN8` reader - Write protect enable 8"]
pub type WPEN8_R = crate::BitReader;
#[doc = "Field `WPEN8` writer - Write protect enable 8"]
pub type WPEN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN9` reader - Write protect enable 9"]
pub type WPEN9_R = crate::BitReader;
#[doc = "Field `WPEN9` writer - Write protect enable 9"]
pub type WPEN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN10` reader - Write protect enable 10"]
pub type WPEN10_R = crate::BitReader;
#[doc = "Field `WPEN10` writer - Write protect enable 10"]
pub type WPEN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN11` reader - Write protect enable 11"]
pub type WPEN11_R = crate::BitReader;
#[doc = "Field `WPEN11` writer - Write protect enable 11"]
pub type WPEN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN12` reader - Write protect enable 12"]
pub type WPEN12_R = crate::BitReader;
#[doc = "Field `WPEN12` writer - Write protect enable 12"]
pub type WPEN12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN13` reader - Write protect enable 13"]
pub type WPEN13_R = crate::BitReader;
#[doc = "Field `WPEN13` writer - Write protect enable 13"]
pub type WPEN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN14` reader - Write protect enable 14"]
pub type WPEN14_R = crate::BitReader;
#[doc = "Field `WPEN14` writer - Write protect enable 14"]
pub type WPEN14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPEN15` reader - Write protect enable 15"]
pub type WPEN15_R = crate::BitReader;
#[doc = "Field `WPEN15` writer - Write protect enable 15"]
pub type WPEN15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WPSEQ` reader - Write protect sequence"]
pub type WPSEQ_R = crate::BitReader;
#[doc = "Field `WPSEQ` writer - Write protect sequence"]
pub type WPSEQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Write protect enable 0"]
    #[inline(always)]
    pub fn wpen0(&self) -> WPEN0_R {
        WPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write protect enable 1"]
    #[inline(always)]
    pub fn wpen1(&self) -> WPEN1_R {
        WPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write protect enable 2"]
    #[inline(always)]
    pub fn wpen2(&self) -> WPEN2_R {
        WPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write protect enable 3"]
    #[inline(always)]
    pub fn wpen3(&self) -> WPEN3_R {
        WPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protect enable 4"]
    #[inline(always)]
    pub fn wpen4(&self) -> WPEN4_R {
        WPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write protect enable 5"]
    #[inline(always)]
    pub fn wpen5(&self) -> WPEN5_R {
        WPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write protect enable 6"]
    #[inline(always)]
    pub fn wpen6(&self) -> WPEN6_R {
        WPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write protect enable 7"]
    #[inline(always)]
    pub fn wpen7(&self) -> WPEN7_R {
        WPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write protect enable 8"]
    #[inline(always)]
    pub fn wpen8(&self) -> WPEN8_R {
        WPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write protect enable 9"]
    #[inline(always)]
    pub fn wpen9(&self) -> WPEN9_R {
        WPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write protect enable 10"]
    #[inline(always)]
    pub fn wpen10(&self) -> WPEN10_R {
        WPEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write protect enable 11"]
    #[inline(always)]
    pub fn wpen11(&self) -> WPEN11_R {
        WPEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write protect enable 12"]
    #[inline(always)]
    pub fn wpen12(&self) -> WPEN12_R {
        WPEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write protect enable 13"]
    #[inline(always)]
    pub fn wpen13(&self) -> WPEN13_R {
        WPEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write protect enable 14"]
    #[inline(always)]
    pub fn wpen14(&self) -> WPEN14_R {
        WPEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write protect enable 15"]
    #[inline(always)]
    pub fn wpen15(&self) -> WPEN15_R {
        WPEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write protect sequence"]
    #[inline(always)]
    pub fn wpseq(&self) -> WPSEQ_R {
        WPSEQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write protect enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn wpen0(&mut self) -> WPEN0_W<WPR_SPEC, 0> {
        WPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Write protect enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn wpen1(&mut self) -> WPEN1_W<WPR_SPEC, 1> {
        WPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Write protect enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn wpen2(&mut self) -> WPEN2_W<WPR_SPEC, 2> {
        WPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Write protect enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn wpen3(&mut self) -> WPEN3_W<WPR_SPEC, 3> {
        WPEN3_W::new(self)
    }
    #[doc = "Bit 4 - Write protect enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn wpen4(&mut self) -> WPEN4_W<WPR_SPEC, 4> {
        WPEN4_W::new(self)
    }
    #[doc = "Bit 5 - Write protect enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn wpen5(&mut self) -> WPEN5_W<WPR_SPEC, 5> {
        WPEN5_W::new(self)
    }
    #[doc = "Bit 6 - Write protect enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn wpen6(&mut self) -> WPEN6_W<WPR_SPEC, 6> {
        WPEN6_W::new(self)
    }
    #[doc = "Bit 7 - Write protect enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn wpen7(&mut self) -> WPEN7_W<WPR_SPEC, 7> {
        WPEN7_W::new(self)
    }
    #[doc = "Bit 8 - Write protect enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn wpen8(&mut self) -> WPEN8_W<WPR_SPEC, 8> {
        WPEN8_W::new(self)
    }
    #[doc = "Bit 9 - Write protect enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn wpen9(&mut self) -> WPEN9_W<WPR_SPEC, 9> {
        WPEN9_W::new(self)
    }
    #[doc = "Bit 10 - Write protect enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn wpen10(&mut self) -> WPEN10_W<WPR_SPEC, 10> {
        WPEN10_W::new(self)
    }
    #[doc = "Bit 11 - Write protect enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn wpen11(&mut self) -> WPEN11_W<WPR_SPEC, 11> {
        WPEN11_W::new(self)
    }
    #[doc = "Bit 12 - Write protect enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn wpen12(&mut self) -> WPEN12_W<WPR_SPEC, 12> {
        WPEN12_W::new(self)
    }
    #[doc = "Bit 13 - Write protect enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn wpen13(&mut self) -> WPEN13_W<WPR_SPEC, 13> {
        WPEN13_W::new(self)
    }
    #[doc = "Bit 14 - Write protect enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn wpen14(&mut self) -> WPEN14_W<WPR_SPEC, 14> {
        WPEN14_W::new(self)
    }
    #[doc = "Bit 15 - Write protect enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn wpen15(&mut self) -> WPEN15_W<WPR_SPEC, 15> {
        WPEN15_W::new(self)
    }
    #[doc = "Bit 16 - Write protect sequence"]
    #[inline(always)]
    #[must_use]
    pub fn wpseq(&mut self) -> WPSEQ_W<WPR_SPEC, 16> {
        WPSEQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpr::R`](R) reader structure"]
impl crate::Readable for WPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
