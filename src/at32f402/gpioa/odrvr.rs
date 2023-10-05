#[doc = "Register `ODRVR` reader"]
pub type R = crate::R<ODRVR_SPEC>;
#[doc = "Register `ODRVR` writer"]
pub type W = crate::W<ODRVR_SPEC>;
#[doc = "Field `ODRV[0-15]` reader - GPIOx pin %s output drive capability"]
pub type ODRV_R = crate::FieldReader<ODRV0_A>;
#[doc = "GPIOx pin %s output drive capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ODRV0_A {
    #[doc = "0: Normal sourcing/sinking strength"]
    Normal = 0,
    #[doc = "1: Large sourcing/sinking strength"]
    Large = 1,
}
impl From<ODRV0_A> for u8 {
    #[inline(always)]
    fn from(variant: ODRV0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ODRV0_A {
    type Ux = u8;
}
impl ODRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ODRV0_A> {
        match self.bits {
            0 => Some(ODRV0_A::Normal),
            1 => Some(ODRV0_A::Large),
            _ => None,
        }
    }
    #[doc = "Normal sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ODRV0_A::Normal
    }
    #[doc = "Large sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_large(&self) -> bool {
        *self == ODRV0_A::Large
    }
}
#[doc = "Field `ODRV[0-15]` writer - GPIOx pin %s output drive capability"]
pub type ODRV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ODRV0_A>;
impl<'a, REG, const O: u8> ODRV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal sourcing/sinking strength"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ODRV0_A::Normal)
    }
    #[doc = "Large sourcing/sinking strength"]
    #[inline(always)]
    pub fn large(self) -> &'a mut crate::W<REG> {
        self.variant(ODRV0_A::Large)
    }
}
impl R {
    #[doc = "GPIOx pin [0-15]
output drive capability"]
    #[inline(always)]
    pub unsafe fn odrv(&self, n: u8) -> ODRV_R {
        ODRV_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - GPIOx pin 0 output drive capability"]
    #[inline(always)]
    pub fn odrv0(&self) -> ODRV_R {
        ODRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 output drive capability"]
    #[inline(always)]
    pub fn odrv1(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 output drive capability"]
    #[inline(always)]
    pub fn odrv2(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 output drive capability"]
    #[inline(always)]
    pub fn odrv3(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 output drive capability"]
    #[inline(always)]
    pub fn odrv4(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 output drive capability"]
    #[inline(always)]
    pub fn odrv5(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 output drive capability"]
    #[inline(always)]
    pub fn odrv6(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 output drive capability"]
    #[inline(always)]
    pub fn odrv7(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 output drive capability"]
    #[inline(always)]
    pub fn odrv8(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 output drive capability"]
    #[inline(always)]
    pub fn odrv9(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 output drive capability"]
    #[inline(always)]
    pub fn odrv10(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 output drive capability"]
    #[inline(always)]
    pub fn odrv11(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 output drive capability"]
    #[inline(always)]
    pub fn odrv12(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 output drive capability"]
    #[inline(always)]
    pub fn odrv13(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 output drive capability"]
    #[inline(always)]
    pub fn odrv14(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 output drive capability"]
    #[inline(always)]
    pub fn odrv15(&self) -> ODRV_R {
        ODRV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "GPIOx pin [0-15]
output drive capability"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn odrv<const O: u8>(&mut self) -> ODRV_W<ODRVR_SPEC, O> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 0:1 - GPIOx pin 0 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv0(&mut self) -> ODRV_W<ODRVR_SPEC, 0> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv1(&mut self) -> ODRV_W<ODRVR_SPEC, 2> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv2(&mut self) -> ODRV_W<ODRVR_SPEC, 4> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv3(&mut self) -> ODRV_W<ODRVR_SPEC, 6> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv4(&mut self) -> ODRV_W<ODRVR_SPEC, 8> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv5(&mut self) -> ODRV_W<ODRVR_SPEC, 10> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv6(&mut self) -> ODRV_W<ODRVR_SPEC, 12> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv7(&mut self) -> ODRV_W<ODRVR_SPEC, 14> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv8(&mut self) -> ODRV_W<ODRVR_SPEC, 16> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv9(&mut self) -> ODRV_W<ODRVR_SPEC, 18> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv10(&mut self) -> ODRV_W<ODRVR_SPEC, 20> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv11(&mut self) -> ODRV_W<ODRVR_SPEC, 22> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv12(&mut self) -> ODRV_W<ODRVR_SPEC, 24> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv13(&mut self) -> ODRV_W<ODRVR_SPEC, 26> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv14(&mut self) -> ODRV_W<ODRVR_SPEC, 28> {
        ODRV_W::new(self)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv15(&mut self) -> ODRV_W<ODRVR_SPEC, 30> {
        ODRV_W::new(self)
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
