#[doc = "Register `EXTIC4` reader"]
pub type R = crate::R<EXTIC4_SPEC>;
#[doc = "Register `EXTIC4` writer"]
pub type W = crate::W<EXTIC4_SPEC>;
#[doc = "Field `EXTINT12` reader - EXTINT 12 configuration bits"]
pub type EXTINT12_R = crate::FieldReader;
#[doc = "Field `EXTINT12` writer - EXTINT 12 configuration bits"]
pub type EXTINT12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT13` reader - EXTINT 13 configuration bits"]
pub type EXTINT13_R = crate::FieldReader;
#[doc = "Field `EXTINT13` writer - EXTINT 13 configuration bits"]
pub type EXTINT13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT14` reader - EXTINT 14 configuration bits"]
pub type EXTINT14_R = crate::FieldReader;
#[doc = "Field `EXTINT14` writer - EXTINT 14 configuration bits"]
pub type EXTINT14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT15` reader - EXTINT 15 configuration bits"]
pub type EXTINT15_R = crate::FieldReader;
#[doc = "Field `EXTINT15` writer - EXTINT 15 configuration bits"]
pub type EXTINT15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTINT 12 configuration bits"]
    #[inline(always)]
    pub fn extint12(&self) -> EXTINT12_R {
        EXTINT12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTINT 13 configuration bits"]
    #[inline(always)]
    pub fn extint13(&self) -> EXTINT13_R {
        EXTINT13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTINT 14 configuration bits"]
    #[inline(always)]
    pub fn extint14(&self) -> EXTINT14_R {
        EXTINT14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTINT 15 configuration bits"]
    #[inline(always)]
    pub fn extint15(&self) -> EXTINT15_R {
        EXTINT15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTINT 12 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint12(&mut self) -> EXTINT12_W<EXTIC4_SPEC, 0> {
        EXTINT12_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTINT 13 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint13(&mut self) -> EXTINT13_W<EXTIC4_SPEC, 4> {
        EXTINT13_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTINT 14 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint14(&mut self) -> EXTINT14_W<EXTIC4_SPEC, 8> {
        EXTINT14_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTINT 15 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint15(&mut self) -> EXTINT15_W<EXTIC4_SPEC, 12> {
        EXTINT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extic4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extic4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIC4_SPEC;
impl crate::RegisterSpec for EXTIC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extic4::R`](R) reader structure"]
impl crate::Readable for EXTIC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extic4::W`](W) writer structure"]
impl crate::Writable for EXTIC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIC4 to value 0"]
impl crate::Resettable for EXTIC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
