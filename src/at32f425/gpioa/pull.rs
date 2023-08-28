#[doc = "Register `PULL` reader"]
pub type R = crate::R<PULL_SPEC>;
#[doc = "Register `PULL` writer"]
pub type W = crate::W<PULL_SPEC>;
#[doc = "Field `PULL0` reader - GPIOx pin 0 pull configuration"]
pub type PULL0_R = crate::FieldReader;
#[doc = "Field `PULL0` writer - GPIOx pin 0 pull configuration"]
pub type PULL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL1` reader - GPIOx pin 1 pull configuration"]
pub type PULL1_R = crate::FieldReader;
#[doc = "Field `PULL1` writer - GPIOx pin 1 pull configuration"]
pub type PULL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL2` reader - GPIOx pin 2 pull configuration"]
pub type PULL2_R = crate::FieldReader;
#[doc = "Field `PULL2` writer - GPIOx pin 2 pull configuration"]
pub type PULL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL3` reader - GPIOx pin 3 pull configuration"]
pub type PULL3_R = crate::FieldReader;
#[doc = "Field `PULL3` writer - GPIOx pin 3 pull configuration"]
pub type PULL3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL4` reader - GPIOx pin 4 pull configuration"]
pub type PULL4_R = crate::FieldReader;
#[doc = "Field `PULL4` writer - GPIOx pin 4 pull configuration"]
pub type PULL4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL5` reader - GPIOx pin 5 pull configuration"]
pub type PULL5_R = crate::FieldReader;
#[doc = "Field `PULL5` writer - GPIOx pin 5 pull configuration"]
pub type PULL5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL6` reader - GPIOx pin 6 pull configuration"]
pub type PULL6_R = crate::FieldReader;
#[doc = "Field `PULL6` writer - GPIOx pin 6 pull configuration"]
pub type PULL6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL7` reader - GPIOx pin 7 pull configuration"]
pub type PULL7_R = crate::FieldReader;
#[doc = "Field `PULL7` writer - GPIOx pin 7 pull configuration"]
pub type PULL7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL8` reader - GPIOx pin 8 pull configuration"]
pub type PULL8_R = crate::FieldReader;
#[doc = "Field `PULL8` writer - GPIOx pin 8 pull configuration"]
pub type PULL8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL9` reader - GPIOx pin 9 pull configuration"]
pub type PULL9_R = crate::FieldReader;
#[doc = "Field `PULL9` writer - GPIOx pin 9 pull configuration"]
pub type PULL9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL10` reader - GPIOx pin 10 pull configuration"]
pub type PULL10_R = crate::FieldReader;
#[doc = "Field `PULL10` writer - GPIOx pin 10 pull configuration"]
pub type PULL10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL11` reader - GPIOx pin 11 pull configuration"]
pub type PULL11_R = crate::FieldReader;
#[doc = "Field `PULL11` writer - GPIOx pin 11 pull configuration"]
pub type PULL11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL12` reader - GPIOx pin 12 pull configuration"]
pub type PULL12_R = crate::FieldReader;
#[doc = "Field `PULL12` writer - GPIOx pin 12 pull configuration"]
pub type PULL12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL13` reader - GPIOx pin 13 pull configuration"]
pub type PULL13_R = crate::FieldReader;
#[doc = "Field `PULL13` writer - GPIOx pin 13 pull configuration"]
pub type PULL13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL14` reader - GPIOx pin 14 pull configuration"]
pub type PULL14_R = crate::FieldReader;
#[doc = "Field `PULL14` writer - GPIOx pin 14 pull configuration"]
pub type PULL14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PULL15` reader - GPIOx pin 15 pull configuration"]
pub type PULL15_R = crate::FieldReader;
#[doc = "Field `PULL15` writer - GPIOx pin 15 pull configuration"]
pub type PULL15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - GPIOx pin 0 pull configuration"]
    #[inline(always)]
    pub fn pull0(&self) -> PULL0_R {
        PULL0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 pull configuration"]
    #[inline(always)]
    pub fn pull1(&self) -> PULL1_R {
        PULL1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 pull configuration"]
    #[inline(always)]
    pub fn pull2(&self) -> PULL2_R {
        PULL2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 pull configuration"]
    #[inline(always)]
    pub fn pull3(&self) -> PULL3_R {
        PULL3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 pull configuration"]
    #[inline(always)]
    pub fn pull4(&self) -> PULL4_R {
        PULL4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 pull configuration"]
    #[inline(always)]
    pub fn pull5(&self) -> PULL5_R {
        PULL5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 pull configuration"]
    #[inline(always)]
    pub fn pull6(&self) -> PULL6_R {
        PULL6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 pull configuration"]
    #[inline(always)]
    pub fn pull7(&self) -> PULL7_R {
        PULL7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 pull configuration"]
    #[inline(always)]
    pub fn pull8(&self) -> PULL8_R {
        PULL8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 pull configuration"]
    #[inline(always)]
    pub fn pull9(&self) -> PULL9_R {
        PULL9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 pull configuration"]
    #[inline(always)]
    pub fn pull10(&self) -> PULL10_R {
        PULL10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 pull configuration"]
    #[inline(always)]
    pub fn pull11(&self) -> PULL11_R {
        PULL11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 pull configuration"]
    #[inline(always)]
    pub fn pull12(&self) -> PULL12_R {
        PULL12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 pull configuration"]
    #[inline(always)]
    pub fn pull13(&self) -> PULL13_R {
        PULL13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 pull configuration"]
    #[inline(always)]
    pub fn pull14(&self) -> PULL14_R {
        PULL14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 pull configuration"]
    #[inline(always)]
    pub fn pull15(&self) -> PULL15_R {
        PULL15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOx pin 0 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull0(&mut self) -> PULL0_W<PULL_SPEC, 0> {
        PULL0_W::new(self)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull1(&mut self) -> PULL1_W<PULL_SPEC, 2> {
        PULL1_W::new(self)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull2(&mut self) -> PULL2_W<PULL_SPEC, 4> {
        PULL2_W::new(self)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull3(&mut self) -> PULL3_W<PULL_SPEC, 6> {
        PULL3_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull4(&mut self) -> PULL4_W<PULL_SPEC, 8> {
        PULL4_W::new(self)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull5(&mut self) -> PULL5_W<PULL_SPEC, 10> {
        PULL5_W::new(self)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull6(&mut self) -> PULL6_W<PULL_SPEC, 12> {
        PULL6_W::new(self)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull7(&mut self) -> PULL7_W<PULL_SPEC, 14> {
        PULL7_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull8(&mut self) -> PULL8_W<PULL_SPEC, 16> {
        PULL8_W::new(self)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull9(&mut self) -> PULL9_W<PULL_SPEC, 18> {
        PULL9_W::new(self)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull10(&mut self) -> PULL10_W<PULL_SPEC, 20> {
        PULL10_W::new(self)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull11(&mut self) -> PULL11_W<PULL_SPEC, 22> {
        PULL11_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull12(&mut self) -> PULL12_W<PULL_SPEC, 24> {
        PULL12_W::new(self)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull13(&mut self) -> PULL13_W<PULL_SPEC, 26> {
        PULL13_W::new(self)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull14(&mut self) -> PULL14_W<PULL_SPEC, 28> {
        PULL14_W::new(self)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull15(&mut self) -> PULL15_W<PULL_SPEC, 30> {
        PULL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pull::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pull::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULL_SPEC;
impl crate::RegisterSpec for PULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pull::R`](R) reader structure"]
impl crate::Readable for PULL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pull::W`](W) writer structure"]
impl crate::Writable for PULL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULL to value 0"]
impl crate::Resettable for PULL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
