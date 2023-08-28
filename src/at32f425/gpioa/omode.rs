#[doc = "Register `OMODE` reader"]
pub type R = crate::R<OMODE_SPEC>;
#[doc = "Register `OMODE` writer"]
pub type W = crate::W<OMODE_SPEC>;
#[doc = "Field `OM0` reader - GPIOx pin 0 outpu mode configurate"]
pub type OM0_R = crate::BitReader;
#[doc = "Field `OM0` writer - GPIOx pin 0 outpu mode configurate"]
pub type OM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM1` reader - GPIOx pin 1 outpu mode configurate"]
pub type OM1_R = crate::BitReader;
#[doc = "Field `OM1` writer - GPIOx pin 1 outpu mode configurate"]
pub type OM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM2` reader - GPIOx pin 2 outpu mode configurate"]
pub type OM2_R = crate::BitReader;
#[doc = "Field `OM2` writer - GPIOx pin 2 outpu mode configurate"]
pub type OM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM3` reader - GPIOx pin 3 outpu mode configurate"]
pub type OM3_R = crate::BitReader;
#[doc = "Field `OM3` writer - GPIOx pin 3 outpu mode configurate"]
pub type OM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM4` reader - GPIOx pin 4 outpu mode configurate"]
pub type OM4_R = crate::BitReader;
#[doc = "Field `OM4` writer - GPIOx pin 4 outpu mode configurate"]
pub type OM4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM5` reader - GPIOx pin 5 outpu mode configurate"]
pub type OM5_R = crate::BitReader;
#[doc = "Field `OM5` writer - GPIOx pin 5 outpu mode configurate"]
pub type OM5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM6` reader - GPIOx pin 6 outpu mode configurate"]
pub type OM6_R = crate::BitReader;
#[doc = "Field `OM6` writer - GPIOx pin 6 outpu mode configurate"]
pub type OM6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM7` reader - GPIOx pin 7 outpu mode configurate"]
pub type OM7_R = crate::BitReader;
#[doc = "Field `OM7` writer - GPIOx pin 7 outpu mode configurate"]
pub type OM7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM8` reader - GPIOx pin 8 outpu mode configurate"]
pub type OM8_R = crate::BitReader;
#[doc = "Field `OM8` writer - GPIOx pin 8 outpu mode configurate"]
pub type OM8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM9` reader - GPIOx pin 9 outpu mode configurate"]
pub type OM9_R = crate::BitReader;
#[doc = "Field `OM9` writer - GPIOx pin 9 outpu mode configurate"]
pub type OM9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM10` reader - GPIOx pin 10 outpu mode configurate"]
pub type OM10_R = crate::BitReader;
#[doc = "Field `OM10` writer - GPIOx pin 10 outpu mode configurate"]
pub type OM10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM11` reader - GPIOx pin 11 outpu mode configurate"]
pub type OM11_R = crate::BitReader;
#[doc = "Field `OM11` writer - GPIOx pin 11 outpu mode configurate"]
pub type OM11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM12` reader - GPIOx pin 12 outpu mode configurate"]
pub type OM12_R = crate::BitReader;
#[doc = "Field `OM12` writer - GPIOx pin 12 outpu mode configurate"]
pub type OM12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM13` reader - GPIOx pin 13 outpu mode configurate"]
pub type OM13_R = crate::BitReader;
#[doc = "Field `OM13` writer - GPIOx pin 13 outpu mode configurate"]
pub type OM13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM14` reader - GPIOx pin 14 outpu mode configurate"]
pub type OM14_R = crate::BitReader;
#[doc = "Field `OM14` writer - GPIOx pin 14 outpu mode configurate"]
pub type OM14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OM15` reader - GPIOx pin 15 outpu mode configurate"]
pub type OM15_R = crate::BitReader;
#[doc = "Field `OM15` writer - GPIOx pin 15 outpu mode configurate"]
pub type OM15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIOx pin 0 outpu mode configurate"]
    #[inline(always)]
    pub fn om0(&self) -> OM0_R {
        OM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOx pin 1 outpu mode configurate"]
    #[inline(always)]
    pub fn om1(&self) -> OM1_R {
        OM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOx pin 2 outpu mode configurate"]
    #[inline(always)]
    pub fn om2(&self) -> OM2_R {
        OM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOx pin 3 outpu mode configurate"]
    #[inline(always)]
    pub fn om3(&self) -> OM3_R {
        OM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOx pin 4 outpu mode configurate"]
    #[inline(always)]
    pub fn om4(&self) -> OM4_R {
        OM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOx pin 5 outpu mode configurate"]
    #[inline(always)]
    pub fn om5(&self) -> OM5_R {
        OM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOx pin 6 outpu mode configurate"]
    #[inline(always)]
    pub fn om6(&self) -> OM6_R {
        OM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOx pin 7 outpu mode configurate"]
    #[inline(always)]
    pub fn om7(&self) -> OM7_R {
        OM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOx pin 8 outpu mode configurate"]
    #[inline(always)]
    pub fn om8(&self) -> OM8_R {
        OM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOx pin 9 outpu mode configurate"]
    #[inline(always)]
    pub fn om9(&self) -> OM9_R {
        OM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOx pin 10 outpu mode configurate"]
    #[inline(always)]
    pub fn om10(&self) -> OM10_R {
        OM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIOx pin 11 outpu mode configurate"]
    #[inline(always)]
    pub fn om11(&self) -> OM11_R {
        OM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIOx pin 12 outpu mode configurate"]
    #[inline(always)]
    pub fn om12(&self) -> OM12_R {
        OM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIOx pin 13 outpu mode configurate"]
    #[inline(always)]
    pub fn om13(&self) -> OM13_R {
        OM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIOx pin 14 outpu mode configurate"]
    #[inline(always)]
    pub fn om14(&self) -> OM14_R {
        OM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIOx pin 15 outpu mode configurate"]
    #[inline(always)]
    pub fn om15(&self) -> OM15_R {
        OM15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOx pin 0 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om0(&mut self) -> OM0_W<OMODE_SPEC, 0> {
        OM0_W::new(self)
    }
    #[doc = "Bit 1 - GPIOx pin 1 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om1(&mut self) -> OM1_W<OMODE_SPEC, 1> {
        OM1_W::new(self)
    }
    #[doc = "Bit 2 - GPIOx pin 2 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om2(&mut self) -> OM2_W<OMODE_SPEC, 2> {
        OM2_W::new(self)
    }
    #[doc = "Bit 3 - GPIOx pin 3 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om3(&mut self) -> OM3_W<OMODE_SPEC, 3> {
        OM3_W::new(self)
    }
    #[doc = "Bit 4 - GPIOx pin 4 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om4(&mut self) -> OM4_W<OMODE_SPEC, 4> {
        OM4_W::new(self)
    }
    #[doc = "Bit 5 - GPIOx pin 5 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om5(&mut self) -> OM5_W<OMODE_SPEC, 5> {
        OM5_W::new(self)
    }
    #[doc = "Bit 6 - GPIOx pin 6 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om6(&mut self) -> OM6_W<OMODE_SPEC, 6> {
        OM6_W::new(self)
    }
    #[doc = "Bit 7 - GPIOx pin 7 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om7(&mut self) -> OM7_W<OMODE_SPEC, 7> {
        OM7_W::new(self)
    }
    #[doc = "Bit 8 - GPIOx pin 8 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om8(&mut self) -> OM8_W<OMODE_SPEC, 8> {
        OM8_W::new(self)
    }
    #[doc = "Bit 9 - GPIOx pin 9 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om9(&mut self) -> OM9_W<OMODE_SPEC, 9> {
        OM9_W::new(self)
    }
    #[doc = "Bit 10 - GPIOx pin 10 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om10(&mut self) -> OM10_W<OMODE_SPEC, 10> {
        OM10_W::new(self)
    }
    #[doc = "Bit 11 - GPIOx pin 11 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om11(&mut self) -> OM11_W<OMODE_SPEC, 11> {
        OM11_W::new(self)
    }
    #[doc = "Bit 12 - GPIOx pin 12 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om12(&mut self) -> OM12_W<OMODE_SPEC, 12> {
        OM12_W::new(self)
    }
    #[doc = "Bit 13 - GPIOx pin 13 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om13(&mut self) -> OM13_W<OMODE_SPEC, 13> {
        OM13_W::new(self)
    }
    #[doc = "Bit 14 - GPIOx pin 14 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om14(&mut self) -> OM14_W<OMODE_SPEC, 14> {
        OM14_W::new(self)
    }
    #[doc = "Bit 15 - GPIOx pin 15 outpu mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn om15(&mut self) -> OM15_W<OMODE_SPEC, 15> {
        OM15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OMODE_SPEC;
impl crate::RegisterSpec for OMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omode::R`](R) reader structure"]
impl crate::Readable for OMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`omode::W`](W) writer structure"]
impl crate::Writable for OMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OMODE to value 0"]
impl crate::Resettable for OMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
