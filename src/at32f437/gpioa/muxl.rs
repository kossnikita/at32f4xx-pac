#[doc = "Register `MUXL` reader"]
pub type R = crate::R<MUXL_SPEC>;
#[doc = "Register `MUXL` writer"]
pub type W = crate::W<MUXL_SPEC>;
#[doc = "Field `MUXL0` reader - GPIOx pin 0 muxing"]
pub type MUXL0_R = crate::FieldReader;
#[doc = "Field `MUXL0` writer - GPIOx pin 0 muxing"]
pub type MUXL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXL1` reader - GPIOx pin 1 muxing"]
pub type MUXL1_R = crate::FieldReader;
#[doc = "Field `MUXL1` writer - GPIOx pin 1 muxing"]
pub type MUXL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXL2` reader - GPIOx pin 2 muxing"]
pub type MUXL2_R = crate::FieldReader;
#[doc = "Field `MUXL2` writer - GPIOx pin 2 muxing"]
pub type MUXL2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXL3` reader - GPIOx pin 3 muxing"]
pub type MUXL3_R = crate::FieldReader;
#[doc = "Field `MUXL3` writer - GPIOx pin 3 muxing"]
pub type MUXL3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXL4` reader - GPIOx pin 4 muxing"]
pub type MUXL4_R = crate::FieldReader;
#[doc = "Field `MUXL4` writer - GPIOx pin 4 muxing"]
pub type MUXL4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXL5` reader - GPIOx pin 5 muxing"]
pub type MUXL5_R = crate::FieldReader;
#[doc = "Field `MUXL5` writer - GPIOx pin 5 muxing"]
pub type MUXL5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXL6` reader - GPIOx pin 6 muxing"]
pub type MUXL6_R = crate::FieldReader;
#[doc = "Field `MUXL6` writer - GPIOx pin 6 muxing"]
pub type MUXL6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MUXL7` reader - GPIOx pin 7 muxing"]
pub type MUXL7_R = crate::FieldReader;
#[doc = "Field `MUXL7` writer - GPIOx pin 7 muxing"]
pub type MUXL7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - GPIOx pin 0 muxing"]
    #[inline(always)]
    pub fn muxl0(&self) -> MUXL0_R {
        MUXL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOx pin 1 muxing"]
    #[inline(always)]
    pub fn muxl1(&self) -> MUXL1_R {
        MUXL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOx pin 2 muxing"]
    #[inline(always)]
    pub fn muxl2(&self) -> MUXL2_R {
        MUXL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 3 muxing"]
    #[inline(always)]
    pub fn muxl3(&self) -> MUXL3_R {
        MUXL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOx pin 4 muxing"]
    #[inline(always)]
    pub fn muxl4(&self) -> MUXL4_R {
        MUXL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOx pin 5 muxing"]
    #[inline(always)]
    pub fn muxl5(&self) -> MUXL5_R {
        MUXL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPIOx pin 6 muxing"]
    #[inline(always)]
    pub fn muxl6(&self) -> MUXL6_R {
        MUXL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GPIOx pin 7 muxing"]
    #[inline(always)]
    pub fn muxl7(&self) -> MUXL7_R {
        MUXL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - GPIOx pin 0 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl0(&mut self) -> MUXL0_W<MUXL_SPEC, 0> {
        MUXL0_W::new(self)
    }
    #[doc = "Bits 4:7 - GPIOx pin 1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl1(&mut self) -> MUXL1_W<MUXL_SPEC, 4> {
        MUXL1_W::new(self)
    }
    #[doc = "Bits 8:11 - GPIOx pin 2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl2(&mut self) -> MUXL2_W<MUXL_SPEC, 8> {
        MUXL2_W::new(self)
    }
    #[doc = "Bits 12:15 - GPIOx pin 3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl3(&mut self) -> MUXL3_W<MUXL_SPEC, 12> {
        MUXL3_W::new(self)
    }
    #[doc = "Bits 16:19 - GPIOx pin 4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl4(&mut self) -> MUXL4_W<MUXL_SPEC, 16> {
        MUXL4_W::new(self)
    }
    #[doc = "Bits 20:23 - GPIOx pin 5 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl5(&mut self) -> MUXL5_W<MUXL_SPEC, 20> {
        MUXL5_W::new(self)
    }
    #[doc = "Bits 24:27 - GPIOx pin 6 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl6(&mut self) -> MUXL6_W<MUXL_SPEC, 24> {
        MUXL6_W::new(self)
    }
    #[doc = "Bits 28:31 - GPIOx pin 7 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl7(&mut self) -> MUXL7_W<MUXL_SPEC, 28> {
        MUXL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO muxing function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXL_SPEC;
impl crate::RegisterSpec for MUXL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxl::R`](R) reader structure"]
impl crate::Readable for MUXL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxl::W`](W) writer structure"]
impl crate::Writable for MUXL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXL to value 0"]
impl crate::Resettable for MUXL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
