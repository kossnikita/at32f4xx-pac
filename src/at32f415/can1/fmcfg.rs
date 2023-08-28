#[doc = "Register `FMCFG` reader"]
pub type R = crate::R<FMCFG_SPEC>;
#[doc = "Register `FMCFG` writer"]
pub type W = crate::W<FMCFG_SPEC>;
#[doc = "Field `FMSEL0` reader - Filter mode select"]
pub type FMSEL0_R = crate::BitReader;
#[doc = "Field `FMSEL0` writer - Filter mode select"]
pub type FMSEL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL1` reader - Filter mode select"]
pub type FMSEL1_R = crate::BitReader;
#[doc = "Field `FMSEL1` writer - Filter mode select"]
pub type FMSEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL2` reader - Filter mode select"]
pub type FMSEL2_R = crate::BitReader;
#[doc = "Field `FMSEL2` writer - Filter mode select"]
pub type FMSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL3` reader - Filter mode select"]
pub type FMSEL3_R = crate::BitReader;
#[doc = "Field `FMSEL3` writer - Filter mode select"]
pub type FMSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL4` reader - Filter mode select"]
pub type FMSEL4_R = crate::BitReader;
#[doc = "Field `FMSEL4` writer - Filter mode select"]
pub type FMSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL5` reader - Filter mode select"]
pub type FMSEL5_R = crate::BitReader;
#[doc = "Field `FMSEL5` writer - Filter mode select"]
pub type FMSEL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL6` reader - Filter mode select"]
pub type FMSEL6_R = crate::BitReader;
#[doc = "Field `FMSEL6` writer - Filter mode select"]
pub type FMSEL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL7` reader - Filter mode select"]
pub type FMSEL7_R = crate::BitReader;
#[doc = "Field `FMSEL7` writer - Filter mode select"]
pub type FMSEL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL8` reader - Filter mode select"]
pub type FMSEL8_R = crate::BitReader;
#[doc = "Field `FMSEL8` writer - Filter mode select"]
pub type FMSEL8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL9` reader - Filter mode select"]
pub type FMSEL9_R = crate::BitReader;
#[doc = "Field `FMSEL9` writer - Filter mode select"]
pub type FMSEL9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL10` reader - Filter mode select"]
pub type FMSEL10_R = crate::BitReader;
#[doc = "Field `FMSEL10` writer - Filter mode select"]
pub type FMSEL10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL11` reader - Filter mode select"]
pub type FMSEL11_R = crate::BitReader;
#[doc = "Field `FMSEL11` writer - Filter mode select"]
pub type FMSEL11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL12` reader - Filter mode select"]
pub type FMSEL12_R = crate::BitReader;
#[doc = "Field `FMSEL12` writer - Filter mode select"]
pub type FMSEL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMSEL13` reader - Filter mode select"]
pub type FMSEL13_R = crate::BitReader;
#[doc = "Field `FMSEL13` writer - Filter mode select"]
pub type FMSEL13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel0(&self) -> FMSEL0_R {
        FMSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel1(&self) -> FMSEL1_R {
        FMSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel2(&self) -> FMSEL2_R {
        FMSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel3(&self) -> FMSEL3_R {
        FMSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel4(&self) -> FMSEL4_R {
        FMSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel5(&self) -> FMSEL5_R {
        FMSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel6(&self) -> FMSEL6_R {
        FMSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel7(&self) -> FMSEL7_R {
        FMSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel8(&self) -> FMSEL8_R {
        FMSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel9(&self) -> FMSEL9_R {
        FMSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel10(&self) -> FMSEL10_R {
        FMSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel11(&self) -> FMSEL11_R {
        FMSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel12(&self) -> FMSEL12_R {
        FMSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel13(&self) -> FMSEL13_R {
        FMSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel0(&mut self) -> FMSEL0_W<FMCFG_SPEC, 0> {
        FMSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel1(&mut self) -> FMSEL1_W<FMCFG_SPEC, 1> {
        FMSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel2(&mut self) -> FMSEL2_W<FMCFG_SPEC, 2> {
        FMSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel3(&mut self) -> FMSEL3_W<FMCFG_SPEC, 3> {
        FMSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel4(&mut self) -> FMSEL4_W<FMCFG_SPEC, 4> {
        FMSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel5(&mut self) -> FMSEL5_W<FMCFG_SPEC, 5> {
        FMSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel6(&mut self) -> FMSEL6_W<FMCFG_SPEC, 6> {
        FMSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel7(&mut self) -> FMSEL7_W<FMCFG_SPEC, 7> {
        FMSEL7_W::new(self)
    }
    #[doc = "Bit 8 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel8(&mut self) -> FMSEL8_W<FMCFG_SPEC, 8> {
        FMSEL8_W::new(self)
    }
    #[doc = "Bit 9 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel9(&mut self) -> FMSEL9_W<FMCFG_SPEC, 9> {
        FMSEL9_W::new(self)
    }
    #[doc = "Bit 10 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel10(&mut self) -> FMSEL10_W<FMCFG_SPEC, 10> {
        FMSEL10_W::new(self)
    }
    #[doc = "Bit 11 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel11(&mut self) -> FMSEL11_W<FMCFG_SPEC, 11> {
        FMSEL11_W::new(self)
    }
    #[doc = "Bit 12 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel12(&mut self) -> FMSEL12_W<FMCFG_SPEC, 12> {
        FMSEL12_W::new(self)
    }
    #[doc = "Bit 13 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel13(&mut self) -> FMSEL13_W<FMCFG_SPEC, 13> {
        FMSEL13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Filter mode config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMCFG_SPEC;
impl crate::RegisterSpec for FMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcfg::R`](R) reader structure"]
impl crate::Readable for FMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmcfg::W`](W) writer structure"]
impl crate::Writable for FMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMCFG to value 0"]
impl crate::Resettable for FMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
