#[doc = "Register `EXINTC1` reader"]
pub type R = crate::R<EXINTC1_SPEC>;
#[doc = "Register `EXINTC1` writer"]
pub type W = crate::W<EXINTC1_SPEC>;
#[doc = "Field `EXINT0` reader - EXINT 0 configuration bits"]
pub type EXINT0_R = crate::FieldReader;
#[doc = "Field `EXINT0` writer - EXINT 0 configuration bits"]
pub type EXINT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT1` reader - EXINT 1 configuration bits"]
pub type EXINT1_R = crate::FieldReader;
#[doc = "Field `EXINT1` writer - EXINT 1 configuration bits"]
pub type EXINT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT2` reader - EXINT 2 configuration bits"]
pub type EXINT2_R = crate::FieldReader;
#[doc = "Field `EXINT2` writer - EXINT 2 configuration bits"]
pub type EXINT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXINT3` reader - EXINT 3 configuration bits"]
pub type EXINT3_R = crate::FieldReader;
#[doc = "Field `EXINT3` writer - EXINT 3 configuration bits"]
pub type EXINT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXINT 0 configuration bits"]
    #[inline(always)]
    pub fn exint0(&self) -> EXINT0_R {
        EXINT0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXINT 1 configuration bits"]
    #[inline(always)]
    pub fn exint1(&self) -> EXINT1_R {
        EXINT1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXINT 2 configuration bits"]
    #[inline(always)]
    pub fn exint2(&self) -> EXINT2_R {
        EXINT2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXINT 3 configuration bits"]
    #[inline(always)]
    pub fn exint3(&self) -> EXINT3_R {
        EXINT3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXINT 0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint0(&mut self) -> EXINT0_W<EXINTC1_SPEC, 0> {
        EXINT0_W::new(self)
    }
    #[doc = "Bits 4:7 - EXINT 1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint1(&mut self) -> EXINT1_W<EXINTC1_SPEC, 4> {
        EXINT1_W::new(self)
    }
    #[doc = "Bits 8:11 - EXINT 2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint2(&mut self) -> EXINT2_W<EXINTC1_SPEC, 8> {
        EXINT2_W::new(self)
    }
    #[doc = "Bits 12:15 - EXINT 3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exint3(&mut self) -> EXINT3_W<EXINTC1_SPEC, 12> {
        EXINT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC1_SPEC;
impl crate::RegisterSpec for EXINTC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc1::R`](R) reader structure"]
impl crate::Readable for EXINTC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc1::W`](W) writer structure"]
impl crate::Writable for EXINTC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC1 to value 0"]
impl crate::Resettable for EXINTC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}