#[doc = "Register `FACFG` reader"]
pub type R = crate::R<FACFG_SPEC>;
#[doc = "Register `FACFG` writer"]
pub type W = crate::W<FACFG_SPEC>;
#[doc = "Field `FAEN0` reader - Filter activate enable"]
pub type FAEN0_R = crate::BitReader;
#[doc = "Field `FAEN0` writer - Filter activate enable"]
pub type FAEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN1` reader - Filter activate enable"]
pub type FAEN1_R = crate::BitReader;
#[doc = "Field `FAEN1` writer - Filter activate enable"]
pub type FAEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN2` reader - Filter activate enable"]
pub type FAEN2_R = crate::BitReader;
#[doc = "Field `FAEN2` writer - Filter activate enable"]
pub type FAEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN3` reader - Filter activate enable"]
pub type FAEN3_R = crate::BitReader;
#[doc = "Field `FAEN3` writer - Filter activate enable"]
pub type FAEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN4` reader - Filter activate enable"]
pub type FAEN4_R = crate::BitReader;
#[doc = "Field `FAEN4` writer - Filter activate enable"]
pub type FAEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN5` reader - Filter activate enable"]
pub type FAEN5_R = crate::BitReader;
#[doc = "Field `FAEN5` writer - Filter activate enable"]
pub type FAEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN6` reader - Filter activate enable"]
pub type FAEN6_R = crate::BitReader;
#[doc = "Field `FAEN6` writer - Filter activate enable"]
pub type FAEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN7` reader - Filter activate enable"]
pub type FAEN7_R = crate::BitReader;
#[doc = "Field `FAEN7` writer - Filter activate enable"]
pub type FAEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN8` reader - Filter activate enable"]
pub type FAEN8_R = crate::BitReader;
#[doc = "Field `FAEN8` writer - Filter activate enable"]
pub type FAEN8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN9` reader - Filter activate enable"]
pub type FAEN9_R = crate::BitReader;
#[doc = "Field `FAEN9` writer - Filter activate enable"]
pub type FAEN9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN10` reader - Filter activate enable"]
pub type FAEN10_R = crate::BitReader;
#[doc = "Field `FAEN10` writer - Filter activate enable"]
pub type FAEN10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN11` reader - Filter activate enable"]
pub type FAEN11_R = crate::BitReader;
#[doc = "Field `FAEN11` writer - Filter activate enable"]
pub type FAEN11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN12` reader - Filter activate enable"]
pub type FAEN12_R = crate::BitReader;
#[doc = "Field `FAEN12` writer - Filter activate enable"]
pub type FAEN12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FAEN13` reader - Filter activate enable"]
pub type FAEN13_R = crate::BitReader;
#[doc = "Field `FAEN13` writer - Filter activate enable"]
pub type FAEN13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    pub fn faen0(&self) -> FAEN0_R {
        FAEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    pub fn faen1(&self) -> FAEN1_R {
        FAEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    pub fn faen2(&self) -> FAEN2_R {
        FAEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    pub fn faen3(&self) -> FAEN3_R {
        FAEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    pub fn faen4(&self) -> FAEN4_R {
        FAEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    pub fn faen5(&self) -> FAEN5_R {
        FAEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    pub fn faen6(&self) -> FAEN6_R {
        FAEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    pub fn faen7(&self) -> FAEN7_R {
        FAEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    pub fn faen8(&self) -> FAEN8_R {
        FAEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    pub fn faen9(&self) -> FAEN9_R {
        FAEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    pub fn faen10(&self) -> FAEN10_R {
        FAEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    pub fn faen11(&self) -> FAEN11_R {
        FAEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    pub fn faen12(&self) -> FAEN12_R {
        FAEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    pub fn faen13(&self) -> FAEN13_R {
        FAEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen0(&mut self) -> FAEN0_W<FACFG_SPEC, 0> {
        FAEN0_W::new(self)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen1(&mut self) -> FAEN1_W<FACFG_SPEC, 1> {
        FAEN1_W::new(self)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen2(&mut self) -> FAEN2_W<FACFG_SPEC, 2> {
        FAEN2_W::new(self)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen3(&mut self) -> FAEN3_W<FACFG_SPEC, 3> {
        FAEN3_W::new(self)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen4(&mut self) -> FAEN4_W<FACFG_SPEC, 4> {
        FAEN4_W::new(self)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen5(&mut self) -> FAEN5_W<FACFG_SPEC, 5> {
        FAEN5_W::new(self)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen6(&mut self) -> FAEN6_W<FACFG_SPEC, 6> {
        FAEN6_W::new(self)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen7(&mut self) -> FAEN7_W<FACFG_SPEC, 7> {
        FAEN7_W::new(self)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen8(&mut self) -> FAEN8_W<FACFG_SPEC, 8> {
        FAEN8_W::new(self)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen9(&mut self) -> FAEN9_W<FACFG_SPEC, 9> {
        FAEN9_W::new(self)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen10(&mut self) -> FAEN10_W<FACFG_SPEC, 10> {
        FAEN10_W::new(self)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen11(&mut self) -> FAEN11_W<FACFG_SPEC, 11> {
        FAEN11_W::new(self)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen12(&mut self) -> FAEN12_W<FACFG_SPEC, 12> {
        FAEN12_W::new(self)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen13(&mut self) -> FAEN13_W<FACFG_SPEC, 13> {
        FAEN13_W::new(self)
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
