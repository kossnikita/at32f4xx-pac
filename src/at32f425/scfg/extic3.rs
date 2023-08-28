#[doc = "Register `EXTIC3` reader"]
pub type R = crate::R<EXTIC3_SPEC>;
#[doc = "Register `EXTIC3` writer"]
pub type W = crate::W<EXTIC3_SPEC>;
#[doc = "Field `EXTINT8` reader - EXTINT 8 configuration bits"]
pub type EXTINT8_R = crate::FieldReader;
#[doc = "Field `EXTINT8` writer - EXTINT 8 configuration bits"]
pub type EXTINT8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT9` reader - EXTINT 9 configuration bits"]
pub type EXTINT9_R = crate::FieldReader;
#[doc = "Field `EXTINT9` writer - EXTINT 9 configuration bits"]
pub type EXTINT9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT10` reader - EXTINT 10 configuration bits"]
pub type EXTINT10_R = crate::FieldReader;
#[doc = "Field `EXTINT10` writer - EXTINT 10 configuration bits"]
pub type EXTINT10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT11` reader - EXTINT 11 configuration bits"]
pub type EXTINT11_R = crate::FieldReader;
#[doc = "Field `EXTINT11` writer - EXTINT 11 configuration bits"]
pub type EXTINT11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTINT 8 configuration bits"]
    #[inline(always)]
    pub fn extint8(&self) -> EXTINT8_R {
        EXTINT8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTINT 9 configuration bits"]
    #[inline(always)]
    pub fn extint9(&self) -> EXTINT9_R {
        EXTINT9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTINT 10 configuration bits"]
    #[inline(always)]
    pub fn extint10(&self) -> EXTINT10_R {
        EXTINT10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTINT 11 configuration bits"]
    #[inline(always)]
    pub fn extint11(&self) -> EXTINT11_R {
        EXTINT11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTINT 8 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint8(&mut self) -> EXTINT8_W<EXTIC3_SPEC, 0> {
        EXTINT8_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTINT 9 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint9(&mut self) -> EXTINT9_W<EXTIC3_SPEC, 4> {
        EXTINT9_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTINT 10 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint10(&mut self) -> EXTINT10_W<EXTIC3_SPEC, 8> {
        EXTINT10_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTINT 11 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint11(&mut self) -> EXTINT11_W<EXTIC3_SPEC, 12> {
        EXTINT11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "external interrupt configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extic3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extic3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIC3_SPEC;
impl crate::RegisterSpec for EXTIC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extic3::R`](R) reader structure"]
impl crate::Readable for EXTIC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extic3::W`](W) writer structure"]
impl crate::Writable for EXTIC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIC3 to value 0"]
impl crate::Resettable for EXTIC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
