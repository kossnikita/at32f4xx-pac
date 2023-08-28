#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `IOMC0` reader - GPIOx pin 0 mode configurate"]
pub type IOMC0_R = crate::FieldReader;
#[doc = "Field `IOMC0` writer - GPIOx pin 0 mode configurate"]
pub type IOMC0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC1` reader - GPIOx pin 1 mode configurate"]
pub type IOMC1_R = crate::FieldReader;
#[doc = "Field `IOMC1` writer - GPIOx pin 1 mode configurate"]
pub type IOMC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC2` reader - GPIOx pin 2 mode configurate"]
pub type IOMC2_R = crate::FieldReader;
#[doc = "Field `IOMC2` writer - GPIOx pin 2 mode configurate"]
pub type IOMC2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC3` reader - GPIOx pin 3 mode configurate"]
pub type IOMC3_R = crate::FieldReader;
#[doc = "Field `IOMC3` writer - GPIOx pin 3 mode configurate"]
pub type IOMC3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC4` reader - GPIOx pin 4 mode configurate"]
pub type IOMC4_R = crate::FieldReader;
#[doc = "Field `IOMC4` writer - GPIOx pin 4 mode configurate"]
pub type IOMC4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC5` reader - GPIOx pin 5 mode configurate"]
pub type IOMC5_R = crate::FieldReader;
#[doc = "Field `IOMC5` writer - GPIOx pin 5 mode configurate"]
pub type IOMC5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC6` reader - GPIOx pin 6 mode configurate"]
pub type IOMC6_R = crate::FieldReader;
#[doc = "Field `IOMC6` writer - GPIOx pin 6 mode configurate"]
pub type IOMC6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC7` reader - GPIOx pin 7 mode configurate"]
pub type IOMC7_R = crate::FieldReader;
#[doc = "Field `IOMC7` writer - GPIOx pin 7 mode configurate"]
pub type IOMC7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC8` reader - GPIOx pin 8 mode configurate"]
pub type IOMC8_R = crate::FieldReader;
#[doc = "Field `IOMC8` writer - GPIOx pin 8 mode configurate"]
pub type IOMC8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC9` reader - GPIOx pin 9 mode configurate"]
pub type IOMC9_R = crate::FieldReader;
#[doc = "Field `IOMC9` writer - GPIOx pin 9 mode configurate"]
pub type IOMC9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC10` reader - GPIOx pin 10 mode configurate"]
pub type IOMC10_R = crate::FieldReader;
#[doc = "Field `IOMC10` writer - GPIOx pin 10 mode configurate"]
pub type IOMC10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC11` reader - GPIOx pin 11 mode configurate"]
pub type IOMC11_R = crate::FieldReader;
#[doc = "Field `IOMC11` writer - GPIOx pin 11 mode configurate"]
pub type IOMC11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC12` reader - GPIOx pin 12 mode configurate"]
pub type IOMC12_R = crate::FieldReader;
#[doc = "Field `IOMC12` writer - GPIOx pin 12 mode configurate"]
pub type IOMC12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC13` reader - GPIOx pin 13 mode configurate"]
pub type IOMC13_R = crate::FieldReader;
#[doc = "Field `IOMC13` writer - GPIOx pin 13 mode configurate"]
pub type IOMC13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC14` reader - GPIOx pin 14 mode configurate"]
pub type IOMC14_R = crate::FieldReader;
#[doc = "Field `IOMC14` writer - GPIOx pin 14 mode configurate"]
pub type IOMC14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC15` reader - GPIOx pin 15 mode configurate"]
pub type IOMC15_R = crate::FieldReader;
#[doc = "Field `IOMC15` writer - GPIOx pin 15 mode configurate"]
pub type IOMC15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - GPIOx pin 0 mode configurate"]
    #[inline(always)]
    pub fn iomc0(&self) -> IOMC0_R {
        IOMC0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 mode configurate"]
    #[inline(always)]
    pub fn iomc1(&self) -> IOMC1_R {
        IOMC1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 mode configurate"]
    #[inline(always)]
    pub fn iomc2(&self) -> IOMC2_R {
        IOMC2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 mode configurate"]
    #[inline(always)]
    pub fn iomc3(&self) -> IOMC3_R {
        IOMC3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 mode configurate"]
    #[inline(always)]
    pub fn iomc4(&self) -> IOMC4_R {
        IOMC4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 mode configurate"]
    #[inline(always)]
    pub fn iomc5(&self) -> IOMC5_R {
        IOMC5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 mode configurate"]
    #[inline(always)]
    pub fn iomc6(&self) -> IOMC6_R {
        IOMC6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 mode configurate"]
    #[inline(always)]
    pub fn iomc7(&self) -> IOMC7_R {
        IOMC7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 mode configurate"]
    #[inline(always)]
    pub fn iomc8(&self) -> IOMC8_R {
        IOMC8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 mode configurate"]
    #[inline(always)]
    pub fn iomc9(&self) -> IOMC9_R {
        IOMC9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 mode configurate"]
    #[inline(always)]
    pub fn iomc10(&self) -> IOMC10_R {
        IOMC10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 mode configurate"]
    #[inline(always)]
    pub fn iomc11(&self) -> IOMC11_R {
        IOMC11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 mode configurate"]
    #[inline(always)]
    pub fn iomc12(&self) -> IOMC12_R {
        IOMC12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 mode configurate"]
    #[inline(always)]
    pub fn iomc13(&self) -> IOMC13_R {
        IOMC13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 mode configurate"]
    #[inline(always)]
    pub fn iomc14(&self) -> IOMC14_R {
        IOMC14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 mode configurate"]
    #[inline(always)]
    pub fn iomc15(&self) -> IOMC15_R {
        IOMC15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOx pin 0 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc0(&mut self) -> IOMC0_W<CFGR_SPEC, 0> {
        IOMC0_W::new(self)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc1(&mut self) -> IOMC1_W<CFGR_SPEC, 2> {
        IOMC1_W::new(self)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc2(&mut self) -> IOMC2_W<CFGR_SPEC, 4> {
        IOMC2_W::new(self)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc3(&mut self) -> IOMC3_W<CFGR_SPEC, 6> {
        IOMC3_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc4(&mut self) -> IOMC4_W<CFGR_SPEC, 8> {
        IOMC4_W::new(self)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc5(&mut self) -> IOMC5_W<CFGR_SPEC, 10> {
        IOMC5_W::new(self)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc6(&mut self) -> IOMC6_W<CFGR_SPEC, 12> {
        IOMC6_W::new(self)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc7(&mut self) -> IOMC7_W<CFGR_SPEC, 14> {
        IOMC7_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc8(&mut self) -> IOMC8_W<CFGR_SPEC, 16> {
        IOMC8_W::new(self)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc9(&mut self) -> IOMC9_W<CFGR_SPEC, 18> {
        IOMC9_W::new(self)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc10(&mut self) -> IOMC10_W<CFGR_SPEC, 20> {
        IOMC10_W::new(self)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc11(&mut self) -> IOMC11_W<CFGR_SPEC, 22> {
        IOMC11_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc12(&mut self) -> IOMC12_W<CFGR_SPEC, 24> {
        IOMC12_W::new(self)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc13(&mut self) -> IOMC13_W<CFGR_SPEC, 26> {
        IOMC13_W::new(self)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc14(&mut self) -> IOMC14_W<CFGR_SPEC, 28> {
        IOMC14_W::new(self)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc15(&mut self) -> IOMC15_W<CFGR_SPEC, 30> {
        IOMC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
