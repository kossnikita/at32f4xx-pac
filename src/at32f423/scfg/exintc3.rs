#[doc = "Register `EXINTC3` reader"]
pub type R = crate::R<EXINTC3_SPEC>;
#[doc = "Register `EXINTC3` writer"]
pub type W = crate::W<EXINTC3_SPEC>;
#[doc = "Field `EXINT8` reader - EXINT 8 configuration bits"]
pub type EXINT8_R = crate::FieldReader;
#[doc = "Field `EXINT8` writer - EXINT 8 configuration bits"]
pub type EXINT8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT9` reader - EXINT 9 configuration bits"]
pub type EXINT9_R = crate::FieldReader;
#[doc = "Field `EXINT9` writer - EXINT 9 configuration bits"]
pub type EXINT9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT10` reader - EXINT 10 configuration bits"]
pub type EXINT10_R = crate::FieldReader;
#[doc = "Field `EXINT10` writer - EXINT 10 configuration bits"]
pub type EXINT10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT11` reader - EXINT 11 configuration bits"]
pub type EXINT11_R = crate::FieldReader;
#[doc = "Field `EXINT11` writer - EXINT 11 configuration bits"]
pub type EXINT11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXINT 8 configuration bits"]
    #[inline(always)]
    pub fn exint8(&self) -> EXINT8_R {
        EXINT8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXINT 9 configuration bits"]
    #[inline(always)]
    pub fn exint9(&self) -> EXINT9_R {
        EXINT9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXINT 10 configuration bits"]
    #[inline(always)]
    pub fn exint10(&self) -> EXINT10_R {
        EXINT10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXINT 11 configuration bits"]
    #[inline(always)]
    pub fn exint11(&self) -> EXINT11_R {
        EXINT11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXINT 8 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint8(&mut self) -> EXINT8_W<EXINTC3_SPEC, 0> {
        EXINT8_W::new(self)
    }
    #[doc = "Bits 4:7 - EXINT 9 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint9(&mut self) -> EXINT9_W<EXINTC3_SPEC, 4> {
        EXINT9_W::new(self)
    }
    #[doc = "Bits 8:11 - EXINT 10 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint10(&mut self) -> EXINT10_W<EXINTC3_SPEC, 8> {
        EXINT10_W::new(self)
    }
    #[doc = "Bits 12:15 - EXINT 11 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint11(&mut self) -> EXINT11_W<EXINTC3_SPEC, 12> {
        EXINT11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC3_SPEC;
impl crate::RegisterSpec for EXINTC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc3::R`](R) reader structure"]
impl crate::Readable for EXINTC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc3::W`](W) writer structure"]
impl crate::Writable for EXINTC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC3 to value 0"]
impl crate::Resettable for EXINTC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
