#[doc = "Register `CFGHR` reader"]
pub type R = crate::R<CFGHR_SPEC>;
#[doc = "Register `CFGHR` writer"]
pub type W = crate::W<CFGHR_SPEC>;
#[doc = "Field `IOMC8` reader - Port n.8 mode configurate bits"]
pub type IOMC8_R = crate::FieldReader;
#[doc = "Field `IOMC8` writer - Port n.8 mode configurate bits"]
pub type IOMC8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC8` reader - Port n.8 function configurate bits"]
pub type IOFC8_R = crate::FieldReader;
#[doc = "Field `IOFC8` writer - Port n.8 function configurate bits"]
pub type IOFC8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC9` reader - Port n.9 mode configurate bits"]
pub type IOMC9_R = crate::FieldReader;
#[doc = "Field `IOMC9` writer - Port n.9 mode configurate bits"]
pub type IOMC9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC9` reader - Port n.9 function configurate bits"]
pub type IOFC9_R = crate::FieldReader;
#[doc = "Field `IOFC9` writer - Port n.9 function configurate bits"]
pub type IOFC9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC10` reader - Port n.10 mode configurate bits"]
pub type IOMC10_R = crate::FieldReader;
#[doc = "Field `IOMC10` writer - Port n.10 mode configurate bits"]
pub type IOMC10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC10` reader - Port n.10 function configurate bits"]
pub type IOFC10_R = crate::FieldReader;
#[doc = "Field `IOFC10` writer - Port n.10 function configurate bits"]
pub type IOFC10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC11` reader - Port n.11 mode configurate bits"]
pub type IOMC11_R = crate::FieldReader;
#[doc = "Field `IOMC11` writer - Port n.11 mode configurate bits"]
pub type IOMC11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC11` reader - Port n.11 function configurate bits"]
pub type IOFC11_R = crate::FieldReader;
#[doc = "Field `IOFC11` writer - Port n.11 function configurate bits"]
pub type IOFC11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC12` reader - Port n.12 mode configurate bits"]
pub type IOMC12_R = crate::FieldReader;
#[doc = "Field `IOMC12` writer - Port n.12 mode configurate bits"]
pub type IOMC12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC12` reader - Port n.12 function configurate bits"]
pub type IOFC12_R = crate::FieldReader;
#[doc = "Field `IOFC12` writer - Port n.12 function configurate bits"]
pub type IOFC12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC13` reader - Port n.13 mode configurate bits"]
pub type IOMC13_R = crate::FieldReader;
#[doc = "Field `IOMC13` writer - Port n.13 mode configurate bits"]
pub type IOMC13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC13` reader - Port n.13 function configurate bits"]
pub type IOFC13_R = crate::FieldReader;
#[doc = "Field `IOFC13` writer - Port n.13 function configurate bits"]
pub type IOFC13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC14` reader - Port n.14 mode configurate bits"]
pub type IOMC14_R = crate::FieldReader;
#[doc = "Field `IOMC14` writer - Port n.14 mode configurate bits"]
pub type IOMC14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC14` reader - Port n.14 function configurate bits"]
pub type IOFC14_R = crate::FieldReader;
#[doc = "Field `IOFC14` writer - Port n.14 function configurate bits"]
pub type IOFC14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOMC15` reader - Port n.15 mode configurate bits"]
pub type IOMC15_R = crate::FieldReader;
#[doc = "Field `IOMC15` writer - Port n.15 mode configurate bits"]
pub type IOMC15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IOFC15` reader - Port n.15 function configurate bits"]
pub type IOFC15_R = crate::FieldReader;
#[doc = "Field `IOFC15` writer - Port n.15 function configurate bits"]
pub type IOFC15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc8(&self) -> IOMC8_R {
        IOMC8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc8(&self) -> IOFC8_R {
        IOFC8_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.9 mode configurate bits"]
    #[inline(always)]
    pub fn iomc9(&self) -> IOMC9_R {
        IOMC9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.9 function configurate bits"]
    #[inline(always)]
    pub fn iofc9(&self) -> IOFC9_R {
        IOFC9_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.10 mode configurate bits"]
    #[inline(always)]
    pub fn iomc10(&self) -> IOMC10_R {
        IOMC10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.10 function configurate bits"]
    #[inline(always)]
    pub fn iofc10(&self) -> IOFC10_R {
        IOFC10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.11 mode configurate bits"]
    #[inline(always)]
    pub fn iomc11(&self) -> IOMC11_R {
        IOMC11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.11 function configurate bits"]
    #[inline(always)]
    pub fn iofc11(&self) -> IOFC11_R {
        IOFC11_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.12 mode configurate bits"]
    #[inline(always)]
    pub fn iomc12(&self) -> IOMC12_R {
        IOMC12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.12 function configurate bits"]
    #[inline(always)]
    pub fn iofc12(&self) -> IOFC12_R {
        IOFC12_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.13 mode configurate bits"]
    #[inline(always)]
    pub fn iomc13(&self) -> IOMC13_R {
        IOMC13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.13 function configurate bits"]
    #[inline(always)]
    pub fn iofc13(&self) -> IOFC13_R {
        IOFC13_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.14 mode configurate bits"]
    #[inline(always)]
    pub fn iomc14(&self) -> IOMC14_R {
        IOMC14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.14 function configurate bits"]
    #[inline(always)]
    pub fn iofc14(&self) -> IOFC14_R {
        IOFC14_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.15 mode configurate bits"]
    #[inline(always)]
    pub fn iomc15(&self) -> IOMC15_R {
        IOMC15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.15 function configurate bits"]
    #[inline(always)]
    pub fn iofc15(&self) -> IOFC15_R {
        IOFC15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc8(&mut self) -> IOMC8_W<CFGHR_SPEC, 0> {
        IOMC8_W::new(self)
    }
    #[doc = "Bits 2:3 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc8(&mut self) -> IOFC8_W<CFGHR_SPEC, 2> {
        IOFC8_W::new(self)
    }
    #[doc = "Bits 4:5 - Port n.9 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc9(&mut self) -> IOMC9_W<CFGHR_SPEC, 4> {
        IOMC9_W::new(self)
    }
    #[doc = "Bits 6:7 - Port n.9 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc9(&mut self) -> IOFC9_W<CFGHR_SPEC, 6> {
        IOFC9_W::new(self)
    }
    #[doc = "Bits 8:9 - Port n.10 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc10(&mut self) -> IOMC10_W<CFGHR_SPEC, 8> {
        IOMC10_W::new(self)
    }
    #[doc = "Bits 10:11 - Port n.10 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc10(&mut self) -> IOFC10_W<CFGHR_SPEC, 10> {
        IOFC10_W::new(self)
    }
    #[doc = "Bits 12:13 - Port n.11 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc11(&mut self) -> IOMC11_W<CFGHR_SPEC, 12> {
        IOMC11_W::new(self)
    }
    #[doc = "Bits 14:15 - Port n.11 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc11(&mut self) -> IOFC11_W<CFGHR_SPEC, 14> {
        IOFC11_W::new(self)
    }
    #[doc = "Bits 16:17 - Port n.12 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc12(&mut self) -> IOMC12_W<CFGHR_SPEC, 16> {
        IOMC12_W::new(self)
    }
    #[doc = "Bits 18:19 - Port n.12 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc12(&mut self) -> IOFC12_W<CFGHR_SPEC, 18> {
        IOFC12_W::new(self)
    }
    #[doc = "Bits 20:21 - Port n.13 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc13(&mut self) -> IOMC13_W<CFGHR_SPEC, 20> {
        IOMC13_W::new(self)
    }
    #[doc = "Bits 22:23 - Port n.13 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc13(&mut self) -> IOFC13_W<CFGHR_SPEC, 22> {
        IOFC13_W::new(self)
    }
    #[doc = "Bits 24:25 - Port n.14 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc14(&mut self) -> IOMC14_W<CFGHR_SPEC, 24> {
        IOMC14_W::new(self)
    }
    #[doc = "Bits 26:27 - Port n.14 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc14(&mut self) -> IOFC14_W<CFGHR_SPEC, 26> {
        IOFC14_W::new(self)
    }
    #[doc = "Bits 28:29 - Port n.15 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc15(&mut self) -> IOMC15_W<CFGHR_SPEC, 28> {
        IOMC15_W::new(self)
    }
    #[doc = "Bits 30:31 - Port n.15 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc15(&mut self) -> IOFC15_W<CFGHR_SPEC, 30> {
        IOFC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO function configurate high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfghr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfghr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGHR_SPEC;
impl crate::RegisterSpec for CFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfghr::R`](R) reader structure"]
impl crate::Readable for CFGHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfghr::W`](W) writer structure"]
impl crate::Writable for CFGHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGHR to value 0x4444_4444"]
impl crate::Resettable for CFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
