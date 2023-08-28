#[doc = "Register `CFGLR` reader"]
pub type R = crate::R<CFGLR_SPEC>;
#[doc = "Register `CFGLR` writer"]
pub type W = crate::W<CFGLR_SPEC>;
#[doc = "Field `IOMC0` reader - Port n.0 mode configurate bits"]
pub type IOMC0_R = crate::FieldReader;
#[doc = "Field `IOMC0` writer - Port n.0 mode configurate bits"]
pub type IOMC0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC0` reader - Port n.0 function configurate bits"]
pub type IOFC0_R = crate::FieldReader;
#[doc = "Field `IOFC0` writer - Port n.0 function configurate bits"]
pub type IOFC0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC1` reader - Port n.1 mode configurate bits"]
pub type IOMC1_R = crate::FieldReader;
#[doc = "Field `IOMC1` writer - Port n.1 mode configurate bits"]
pub type IOMC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC1` reader - Port n.1 function configurate bits"]
pub type IOFC1_R = crate::FieldReader;
#[doc = "Field `IOFC1` writer - Port n.1 function configurate bits"]
pub type IOFC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC2` reader - Port n.2 mode configurate bits"]
pub type IOMC2_R = crate::FieldReader;
#[doc = "Field `IOMC2` writer - Port n.2 mode configurate bits"]
pub type IOMC2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC2` reader - Port n.2 function configurate bits"]
pub type IOFC2_R = crate::FieldReader;
#[doc = "Field `IOFC2` writer - Port n.2 function configurate bits"]
pub type IOFC2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC3` reader - Port n.3 mode configurate bits"]
pub type IOMC3_R = crate::FieldReader;
#[doc = "Field `IOMC3` writer - Port n.3 mode configurate bits"]
pub type IOMC3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC3` reader - Port n.3 function configurate bits"]
pub type IOFC3_R = crate::FieldReader;
#[doc = "Field `IOFC3` writer - Port n.3 function configurate bits"]
pub type IOFC3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC4` reader - Port n.4 mode configurate bits"]
pub type IOMC4_R = crate::FieldReader;
#[doc = "Field `IOMC4` writer - Port n.4 mode configurate bits"]
pub type IOMC4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC4` reader - Port n.4 function configurate bits"]
pub type IOFC4_R = crate::FieldReader;
#[doc = "Field `IOFC4` writer - Port n.4 function configurate bits"]
pub type IOFC4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC5` reader - Port n.5 mode configurate bits"]
pub type IOMC5_R = crate::FieldReader;
#[doc = "Field `IOMC5` writer - Port n.5 mode configurate bits"]
pub type IOMC5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC5` reader - Port n.5 function configurate bits"]
pub type IOFC5_R = crate::FieldReader;
#[doc = "Field `IOFC5` writer - Port n.5 function configurate bits"]
pub type IOFC5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC6` reader - Port n.6 mode configurate bits"]
pub type IOMC6_R = crate::FieldReader;
#[doc = "Field `IOMC6` writer - Port n.6 mode configurate bits"]
pub type IOMC6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC6` reader - Port n.6 function configurate bits"]
pub type IOFC6_R = crate::FieldReader;
#[doc = "Field `IOFC6` writer - Port n.6 function configurate bits"]
pub type IOFC6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC7` reader - Port n.7 mode configurate bits"]
pub type IOMC7_R = crate::FieldReader;
#[doc = "Field `IOMC7` writer - Port n.7 mode configurate bits"]
pub type IOMC7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC7` reader - Port n.7 function configurate bits"]
pub type IOFC7_R = crate::FieldReader;
#[doc = "Field `IOFC7` writer - Port n.7 function configurate bits"]
pub type IOFC7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port n.0 mode configurate bits"]
    #[inline(always)]
    pub fn iomc0(&self) -> IOMC0_R {
        IOMC0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.0 function configurate bits"]
    #[inline(always)]
    pub fn iofc0(&self) -> IOFC0_R {
        IOFC0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode configurate bits"]
    #[inline(always)]
    pub fn iomc1(&self) -> IOMC1_R {
        IOMC1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 function configurate bits"]
    #[inline(always)]
    pub fn iofc1(&self) -> IOFC1_R {
        IOFC1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode configurate bits"]
    #[inline(always)]
    pub fn iomc2(&self) -> IOMC2_R {
        IOMC2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 function configurate bits"]
    #[inline(always)]
    pub fn iofc2(&self) -> IOFC2_R {
        IOFC2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode configurate bits"]
    #[inline(always)]
    pub fn iomc3(&self) -> IOMC3_R {
        IOMC3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 function configurate bits"]
    #[inline(always)]
    pub fn iofc3(&self) -> IOFC3_R {
        IOFC3_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode configurate bits"]
    #[inline(always)]
    pub fn iomc4(&self) -> IOMC4_R {
        IOMC4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 function configurate bits"]
    #[inline(always)]
    pub fn iofc4(&self) -> IOFC4_R {
        IOFC4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode configurate bits"]
    #[inline(always)]
    pub fn iomc5(&self) -> IOMC5_R {
        IOMC5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 function configurate bits"]
    #[inline(always)]
    pub fn iofc5(&self) -> IOFC5_R {
        IOFC5_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode configurate bits"]
    #[inline(always)]
    pub fn iomc6(&self) -> IOMC6_R {
        IOMC6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 function configurate bits"]
    #[inline(always)]
    pub fn iofc6(&self) -> IOFC6_R {
        IOFC6_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode configurate bits"]
    #[inline(always)]
    pub fn iomc7(&self) -> IOMC7_R {
        IOMC7_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 function configurate bits"]
    #[inline(always)]
    pub fn iofc7(&self) -> IOFC7_R {
        IOFC7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.0 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc0(&mut self) -> IOMC0_W<CFGLR_SPEC, 0> {
        IOMC0_W::new(self)
    }
    #[doc = "Bits 2:3 - Port n.0 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc0(&mut self) -> IOFC0_W<CFGLR_SPEC, 2> {
        IOFC0_W::new(self)
    }
    #[doc = "Bits 4:5 - Port n.1 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc1(&mut self) -> IOMC1_W<CFGLR_SPEC, 4> {
        IOMC1_W::new(self)
    }
    #[doc = "Bits 6:7 - Port n.1 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc1(&mut self) -> IOFC1_W<CFGLR_SPEC, 6> {
        IOFC1_W::new(self)
    }
    #[doc = "Bits 8:9 - Port n.2 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc2(&mut self) -> IOMC2_W<CFGLR_SPEC, 8> {
        IOMC2_W::new(self)
    }
    #[doc = "Bits 10:11 - Port n.2 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc2(&mut self) -> IOFC2_W<CFGLR_SPEC, 10> {
        IOFC2_W::new(self)
    }
    #[doc = "Bits 12:13 - Port n.3 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc3(&mut self) -> IOMC3_W<CFGLR_SPEC, 12> {
        IOMC3_W::new(self)
    }
    #[doc = "Bits 14:15 - Port n.3 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc3(&mut self) -> IOFC3_W<CFGLR_SPEC, 14> {
        IOFC3_W::new(self)
    }
    #[doc = "Bits 16:17 - Port n.4 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc4(&mut self) -> IOMC4_W<CFGLR_SPEC, 16> {
        IOMC4_W::new(self)
    }
    #[doc = "Bits 18:19 - Port n.4 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc4(&mut self) -> IOFC4_W<CFGLR_SPEC, 18> {
        IOFC4_W::new(self)
    }
    #[doc = "Bits 20:21 - Port n.5 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc5(&mut self) -> IOMC5_W<CFGLR_SPEC, 20> {
        IOMC5_W::new(self)
    }
    #[doc = "Bits 22:23 - Port n.5 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc5(&mut self) -> IOFC5_W<CFGLR_SPEC, 22> {
        IOFC5_W::new(self)
    }
    #[doc = "Bits 24:25 - Port n.6 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc6(&mut self) -> IOMC6_W<CFGLR_SPEC, 24> {
        IOMC6_W::new(self)
    }
    #[doc = "Bits 26:27 - Port n.6 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc6(&mut self) -> IOFC6_W<CFGLR_SPEC, 26> {
        IOFC6_W::new(self)
    }
    #[doc = "Bits 28:29 - Port n.7 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc7(&mut self) -> IOMC7_W<CFGLR_SPEC, 28> {
        IOMC7_W::new(self)
    }
    #[doc = "Bits 30:31 - Port n.7 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc7(&mut self) -> IOFC7_W<CFGLR_SPEC, 30> {
        IOFC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO function configurate low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGLR_SPEC;
impl crate::RegisterSpec for CFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfglr::R`](R) reader structure"]
impl crate::Readable for CFGLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfglr::W`](W) writer structure"]
impl crate::Writable for CFGLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGLR to value 0x4444_4444"]
impl crate::Resettable for CFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
