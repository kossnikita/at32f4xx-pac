#[doc = "Register `ODRVR` reader"]
pub type R = crate::R<ODRVR_SPEC>;
#[doc = "Register `ODRVR` writer"]
pub type W = crate::W<ODRVR_SPEC>;
#[doc = "Field `ODRV0` reader - GPIOx pin 0 output drive capability"]
pub type ODRV0_R = crate::FieldReader;
#[doc = "Field `ODRV0` writer - GPIOx pin 0 output drive capability"]
pub type ODRV0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV1` reader - GPIOx pin 1 output drive capability"]
pub type ODRV1_R = crate::FieldReader;
#[doc = "Field `ODRV1` writer - GPIOx pin 1 output drive capability"]
pub type ODRV1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV2` reader - GPIOx pin 2 output drive capability"]
pub type ODRV2_R = crate::FieldReader;
#[doc = "Field `ODRV2` writer - GPIOx pin 2 output drive capability"]
pub type ODRV2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV3` reader - GPIOx pin 3 output drive capability"]
pub type ODRV3_R = crate::FieldReader;
#[doc = "Field `ODRV3` writer - GPIOx pin 3 output drive capability"]
pub type ODRV3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV4` reader - GPIOx pin 4 output drive capability"]
pub type ODRV4_R = crate::FieldReader;
#[doc = "Field `ODRV4` writer - GPIOx pin 4 output drive capability"]
pub type ODRV4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV5` reader - GPIOx pin 5 output drive capability"]
pub type ODRV5_R = crate::FieldReader;
#[doc = "Field `ODRV5` writer - GPIOx pin 5 output drive capability"]
pub type ODRV5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV6` reader - GPIOx pin 6 output drive capability"]
pub type ODRV6_R = crate::FieldReader;
#[doc = "Field `ODRV6` writer - GPIOx pin 6 output drive capability"]
pub type ODRV6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV7` reader - GPIOx pin 7 output drive capability"]
pub type ODRV7_R = crate::FieldReader;
#[doc = "Field `ODRV7` writer - GPIOx pin 7 output drive capability"]
pub type ODRV7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV8` reader - GPIOx pin 8 output drive capability"]
pub type ODRV8_R = crate::FieldReader;
#[doc = "Field `ODRV8` writer - GPIOx pin 8 output drive capability"]
pub type ODRV8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV9` reader - GPIOx pin 9 output drive capability"]
pub type ODRV9_R = crate::FieldReader;
#[doc = "Field `ODRV9` writer - GPIOx pin 9 output drive capability"]
pub type ODRV9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV10` reader - GPIOx pin 10 output drive capability"]
pub type ODRV10_R = crate::FieldReader;
#[doc = "Field `ODRV10` writer - GPIOx pin 10 output drive capability"]
pub type ODRV10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV11` reader - GPIOx pin 11 output drive capability"]
pub type ODRV11_R = crate::FieldReader;
#[doc = "Field `ODRV11` writer - GPIOx pin 11 output drive capability"]
pub type ODRV11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV12` reader - GPIOx pin 12 output drive capability"]
pub type ODRV12_R = crate::FieldReader;
#[doc = "Field `ODRV12` writer - GPIOx pin 12 output drive capability"]
pub type ODRV12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV13` reader - GPIOx pin 13 output drive capability"]
pub type ODRV13_R = crate::FieldReader;
#[doc = "Field `ODRV13` writer - GPIOx pin 13 output drive capability"]
pub type ODRV13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV14` reader - GPIOx pin 14 output drive capability"]
pub type ODRV14_R = crate::FieldReader;
#[doc = "Field `ODRV14` writer - GPIOx pin 14 output drive capability"]
pub type ODRV14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ODRV15` reader - GPIOx pin 15 output drive capability"]
pub type ODRV15_R = crate::FieldReader;
#[doc = "Field `ODRV15` writer - GPIOx pin 15 output drive capability"]
pub type ODRV15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - GPIOx pin 0 output drive capability"]
    #[inline(always)]
    pub fn odrv0(&self) -> ODRV0_R {
        ODRV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 output drive capability"]
    #[inline(always)]
    pub fn odrv1(&self) -> ODRV1_R {
        ODRV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 output drive capability"]
    #[inline(always)]
    pub fn odrv2(&self) -> ODRV2_R {
        ODRV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 output drive capability"]
    #[inline(always)]
    pub fn odrv3(&self) -> ODRV3_R {
        ODRV3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 output drive capability"]
    #[inline(always)]
    pub fn odrv4(&self) -> ODRV4_R {
        ODRV4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 output drive capability"]
    #[inline(always)]
    pub fn odrv5(&self) -> ODRV5_R {
        ODRV5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 output drive capability"]
    #[inline(always)]
    pub fn odrv6(&self) -> ODRV6_R {
        ODRV6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 output drive capability"]
    #[inline(always)]
    pub fn odrv7(&self) -> ODRV7_R {
        ODRV7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 output drive capability"]
    #[inline(always)]
    pub fn odrv8(&self) -> ODRV8_R {
        ODRV8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 output drive capability"]
    #[inline(always)]
    pub fn odrv9(&self) -> ODRV9_R {
        ODRV9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 output drive capability"]
    #[inline(always)]
    pub fn odrv10(&self) -> ODRV10_R {
        ODRV10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 output drive capability"]
    #[inline(always)]
    pub fn odrv11(&self) -> ODRV11_R {
        ODRV11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 output drive capability"]
    #[inline(always)]
    pub fn odrv12(&self) -> ODRV12_R {
        ODRV12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 output drive capability"]
    #[inline(always)]
    pub fn odrv13(&self) -> ODRV13_R {
        ODRV13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 output drive capability"]
    #[inline(always)]
    pub fn odrv14(&self) -> ODRV14_R {
        ODRV14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 output drive capability"]
    #[inline(always)]
    pub fn odrv15(&self) -> ODRV15_R {
        ODRV15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOx pin 0 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv0(&mut self) -> ODRV0_W<ODRVR_SPEC, 0> {
        ODRV0_W::new(self)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv1(&mut self) -> ODRV1_W<ODRVR_SPEC, 2> {
        ODRV1_W::new(self)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv2(&mut self) -> ODRV2_W<ODRVR_SPEC, 4> {
        ODRV2_W::new(self)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv3(&mut self) -> ODRV3_W<ODRVR_SPEC, 6> {
        ODRV3_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv4(&mut self) -> ODRV4_W<ODRVR_SPEC, 8> {
        ODRV4_W::new(self)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv5(&mut self) -> ODRV5_W<ODRVR_SPEC, 10> {
        ODRV5_W::new(self)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv6(&mut self) -> ODRV6_W<ODRVR_SPEC, 12> {
        ODRV6_W::new(self)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv7(&mut self) -> ODRV7_W<ODRVR_SPEC, 14> {
        ODRV7_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv8(&mut self) -> ODRV8_W<ODRVR_SPEC, 16> {
        ODRV8_W::new(self)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv9(&mut self) -> ODRV9_W<ODRVR_SPEC, 18> {
        ODRV9_W::new(self)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv10(&mut self) -> ODRV10_W<ODRVR_SPEC, 20> {
        ODRV10_W::new(self)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv11(&mut self) -> ODRV11_W<ODRVR_SPEC, 22> {
        ODRV11_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv12(&mut self) -> ODRV12_W<ODRVR_SPEC, 24> {
        ODRV12_W::new(self)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv13(&mut self) -> ODRV13_W<ODRVR_SPEC, 26> {
        ODRV13_W::new(self)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv14(&mut self) -> ODRV14_W<ODRVR_SPEC, 28> {
        ODRV14_W::new(self)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv15(&mut self) -> ODRV15_W<ODRVR_SPEC, 30> {
        ODRV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO drive capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odrvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odrvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODRVR_SPEC;
impl crate::RegisterSpec for ODRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odrvr::R`](R) reader structure"]
impl crate::Readable for ODRVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odrvr::W`](W) writer structure"]
impl crate::Writable for ODRVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODRVR to value 0"]
impl crate::Resettable for ODRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
