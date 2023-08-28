#[doc = "Register `FBWCFG` reader"]
pub type R = crate::R<FBWCFG_SPEC>;
#[doc = "Register `FBWCFG` writer"]
pub type W = crate::W<FBWCFG_SPEC>;
#[doc = "Field `FBWSEL0` reader - Filter bit width select"]
pub type FBWSEL0_R = crate::BitReader;
#[doc = "Field `FBWSEL0` writer - Filter bit width select"]
pub type FBWSEL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL1` reader - Filter bit width select"]
pub type FBWSEL1_R = crate::BitReader;
#[doc = "Field `FBWSEL1` writer - Filter bit width select"]
pub type FBWSEL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL2` reader - Filter bit width select"]
pub type FBWSEL2_R = crate::BitReader;
#[doc = "Field `FBWSEL2` writer - Filter bit width select"]
pub type FBWSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL3` reader - Filter bit width select"]
pub type FBWSEL3_R = crate::BitReader;
#[doc = "Field `FBWSEL3` writer - Filter bit width select"]
pub type FBWSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL4` reader - Filter bit width select"]
pub type FBWSEL4_R = crate::BitReader;
#[doc = "Field `FBWSEL4` writer - Filter bit width select"]
pub type FBWSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL5` reader - Filter bit width select"]
pub type FBWSEL5_R = crate::BitReader;
#[doc = "Field `FBWSEL5` writer - Filter bit width select"]
pub type FBWSEL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL6` reader - Filter bit width select"]
pub type FBWSEL6_R = crate::BitReader;
#[doc = "Field `FBWSEL6` writer - Filter bit width select"]
pub type FBWSEL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL7` reader - Filter bit width select"]
pub type FBWSEL7_R = crate::BitReader;
#[doc = "Field `FBWSEL7` writer - Filter bit width select"]
pub type FBWSEL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL8` reader - Filter bit width select"]
pub type FBWSEL8_R = crate::BitReader;
#[doc = "Field `FBWSEL8` writer - Filter bit width select"]
pub type FBWSEL8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL9` reader - Filter bit width select"]
pub type FBWSEL9_R = crate::BitReader;
#[doc = "Field `FBWSEL9` writer - Filter bit width select"]
pub type FBWSEL9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL10` reader - Filter bit width select"]
pub type FBWSEL10_R = crate::BitReader;
#[doc = "Field `FBWSEL10` writer - Filter bit width select"]
pub type FBWSEL10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL11` reader - Filter bit width select"]
pub type FBWSEL11_R = crate::BitReader;
#[doc = "Field `FBWSEL11` writer - Filter bit width select"]
pub type FBWSEL11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL12` reader - Filter bit width select"]
pub type FBWSEL12_R = crate::BitReader;
#[doc = "Field `FBWSEL12` writer - Filter bit width select"]
pub type FBWSEL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBWSEL13` reader - Filter bit width select"]
pub type FBWSEL13_R = crate::BitReader;
#[doc = "Field `FBWSEL13` writer - Filter bit width select"]
pub type FBWSEL13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel0(&self) -> FBWSEL0_R {
        FBWSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel1(&self) -> FBWSEL1_R {
        FBWSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel2(&self) -> FBWSEL2_R {
        FBWSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel3(&self) -> FBWSEL3_R {
        FBWSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel4(&self) -> FBWSEL4_R {
        FBWSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel5(&self) -> FBWSEL5_R {
        FBWSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel6(&self) -> FBWSEL6_R {
        FBWSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel7(&self) -> FBWSEL7_R {
        FBWSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel8(&self) -> FBWSEL8_R {
        FBWSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel9(&self) -> FBWSEL9_R {
        FBWSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel10(&self) -> FBWSEL10_R {
        FBWSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel11(&self) -> FBWSEL11_R {
        FBWSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel12(&self) -> FBWSEL12_R {
        FBWSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel13(&self) -> FBWSEL13_R {
        FBWSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel0(&mut self) -> FBWSEL0_W<FBWCFG_SPEC, 0> {
        FBWSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel1(&mut self) -> FBWSEL1_W<FBWCFG_SPEC, 1> {
        FBWSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel2(&mut self) -> FBWSEL2_W<FBWCFG_SPEC, 2> {
        FBWSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel3(&mut self) -> FBWSEL3_W<FBWCFG_SPEC, 3> {
        FBWSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel4(&mut self) -> FBWSEL4_W<FBWCFG_SPEC, 4> {
        FBWSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel5(&mut self) -> FBWSEL5_W<FBWCFG_SPEC, 5> {
        FBWSEL5_W::new(self)
    }
    #[doc = "Bit 6 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel6(&mut self) -> FBWSEL6_W<FBWCFG_SPEC, 6> {
        FBWSEL6_W::new(self)
    }
    #[doc = "Bit 7 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel7(&mut self) -> FBWSEL7_W<FBWCFG_SPEC, 7> {
        FBWSEL7_W::new(self)
    }
    #[doc = "Bit 8 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel8(&mut self) -> FBWSEL8_W<FBWCFG_SPEC, 8> {
        FBWSEL8_W::new(self)
    }
    #[doc = "Bit 9 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel9(&mut self) -> FBWSEL9_W<FBWCFG_SPEC, 9> {
        FBWSEL9_W::new(self)
    }
    #[doc = "Bit 10 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel10(&mut self) -> FBWSEL10_W<FBWCFG_SPEC, 10> {
        FBWSEL10_W::new(self)
    }
    #[doc = "Bit 11 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel11(&mut self) -> FBWSEL11_W<FBWCFG_SPEC, 11> {
        FBWSEL11_W::new(self)
    }
    #[doc = "Bit 12 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel12(&mut self) -> FBWSEL12_W<FBWCFG_SPEC, 12> {
        FBWSEL12_W::new(self)
    }
    #[doc = "Bit 13 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn fbwsel13(&mut self) -> FBWSEL13_W<FBWCFG_SPEC, 13> {
        FBWSEL13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Filter bit width config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbwcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbwcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FBWCFG_SPEC;
impl crate::RegisterSpec for FBWCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbwcfg::R`](R) reader structure"]
impl crate::Readable for FBWCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fbwcfg::W`](W) writer structure"]
impl crate::Writable for FBWCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBWCFG to value 0"]
impl crate::Resettable for FBWCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
