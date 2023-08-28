#[doc = "Register `MUXH` reader"]
pub type R = crate::R<MUXH_SPEC>;
#[doc = "Register `MUXH` writer"]
pub type W = crate::W<MUXH_SPEC>;
#[doc = "Field `MUXH8` reader - GPIOx pin 8 muxing"]
pub type MUXH8_R = crate::FieldReader;
#[doc = "Field `MUXH8` writer - GPIOx pin 8 muxing"]
pub type MUXH8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXH9` reader - GPIOx pin 9 muxing"]
pub type MUXH9_R = crate::FieldReader;
#[doc = "Field `MUXH9` writer - GPIOx pin 9 muxing"]
pub type MUXH9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXH10` reader - GPIOx pin 10 muxing"]
pub type MUXH10_R = crate::FieldReader;
#[doc = "Field `MUXH10` writer - GPIOx pin 10 muxing"]
pub type MUXH10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXH11` reader - GPIOx pin 11 muxing"]
pub type MUXH11_R = crate::FieldReader;
#[doc = "Field `MUXH11` writer - GPIOx pin 11 muxing"]
pub type MUXH11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXH12` reader - GPIOx pin 12 muxing"]
pub type MUXH12_R = crate::FieldReader;
#[doc = "Field `MUXH12` writer - GPIOx pin 12 muxing"]
pub type MUXH12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXH13` reader - GPIOx pin 13 muxing"]
pub type MUXH13_R = crate::FieldReader;
#[doc = "Field `MUXH13` writer - GPIOx pin 13 muxing"]
pub type MUXH13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXH14` reader - GPIOx pin 14 muxing"]
pub type MUXH14_R = crate::FieldReader;
#[doc = "Field `MUXH14` writer - GPIOx pin 14 muxing"]
pub type MUXH14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXH15` reader - GPIOx pin 15 muxing"]
pub type MUXH15_R = crate::FieldReader;
#[doc = "Field `MUXH15` writer - GPIOx pin 15 muxing"]
pub type MUXH15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - GPIOx pin 8 muxing"]
    #[inline(always)]
    pub fn muxh8(&self) -> MUXH8_R {
        MUXH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOx pin 9 muxing"]
    #[inline(always)]
    pub fn muxh9(&self) -> MUXH9_R {
        MUXH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOx pin 10 muxing"]
    #[inline(always)]
    pub fn muxh10(&self) -> MUXH10_R {
        MUXH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 11 muxing"]
    #[inline(always)]
    pub fn muxh11(&self) -> MUXH11_R {
        MUXH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOx pin 12 muxing"]
    #[inline(always)]
    pub fn muxh12(&self) -> MUXH12_R {
        MUXH12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOx pin 13 muxing"]
    #[inline(always)]
    pub fn muxh13(&self) -> MUXH13_R {
        MUXH13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPIOx pin 14 muxing"]
    #[inline(always)]
    pub fn muxh14(&self) -> MUXH14_R {
        MUXH14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GPIOx pin 15 muxing"]
    #[inline(always)]
    pub fn muxh15(&self) -> MUXH15_R {
        MUXH15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - GPIOx pin 8 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh8(&mut self) -> MUXH8_W<MUXH_SPEC, 0> {
        MUXH8_W::new(self)
    }
    #[doc = "Bits 4:7 - GPIOx pin 9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh9(&mut self) -> MUXH9_W<MUXH_SPEC, 4> {
        MUXH9_W::new(self)
    }
    #[doc = "Bits 8:11 - GPIOx pin 10 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh10(&mut self) -> MUXH10_W<MUXH_SPEC, 8> {
        MUXH10_W::new(self)
    }
    #[doc = "Bits 12:15 - GPIOx pin 11 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh11(&mut self) -> MUXH11_W<MUXH_SPEC, 12> {
        MUXH11_W::new(self)
    }
    #[doc = "Bits 16:19 - GPIOx pin 12 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh12(&mut self) -> MUXH12_W<MUXH_SPEC, 16> {
        MUXH12_W::new(self)
    }
    #[doc = "Bits 20:23 - GPIOx pin 13 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh13(&mut self) -> MUXH13_W<MUXH_SPEC, 20> {
        MUXH13_W::new(self)
    }
    #[doc = "Bits 24:27 - GPIOx pin 14 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh14(&mut self) -> MUXH14_W<MUXH_SPEC, 24> {
        MUXH14_W::new(self)
    }
    #[doc = "Bits 28:31 - GPIOx pin 15 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh15(&mut self) -> MUXH15_W<MUXH_SPEC, 28> {
        MUXH15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO muxing function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXH_SPEC;
impl crate::RegisterSpec for MUXH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxh::R`](R) reader structure"]
impl crate::Readable for MUXH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxh::W`](W) writer structure"]
impl crate::Writable for MUXH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXH to value 0"]
impl crate::Resettable for MUXH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
