#[doc = "Register `EXTIC1` reader"]
pub type R = crate::R<EXTIC1_SPEC>;
#[doc = "Register `EXTIC1` writer"]
pub type W = crate::W<EXTIC1_SPEC>;
#[doc = "Field `EXTINT0` reader - EXTINT 0 configuration bits"]
pub type EXTINT0_R = crate::FieldReader;
#[doc = "Field `EXTINT0` writer - EXTINT 0 configuration bits"]
pub type EXTINT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT1` reader - EXTINT 1 configuration bits"]
pub type EXTINT1_R = crate::FieldReader;
#[doc = "Field `EXTINT1` writer - EXTINT 1 configuration bits"]
pub type EXTINT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT2` reader - EXTINT 2 configuration bits"]
pub type EXTINT2_R = crate::FieldReader;
#[doc = "Field `EXTINT2` writer - EXTINT 2 configuration bits"]
pub type EXTINT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTINT3` reader - EXTINT 3 configuration bits"]
pub type EXTINT3_R = crate::FieldReader;
#[doc = "Field `EXTINT3` writer - EXTINT 3 configuration bits"]
pub type EXTINT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTINT 0 configuration bits"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTINT 1 configuration bits"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTINT 2 configuration bits"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTINT 3 configuration bits"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTINT 0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> EXTINT0_W<EXTIC1_SPEC, 0> {
        EXTINT0_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTINT 1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> EXTINT1_W<EXTIC1_SPEC, 4> {
        EXTINT1_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTINT 2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> EXTINT2_W<EXTIC1_SPEC, 8> {
        EXTINT2_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTINT 3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> EXTINT3_W<EXTIC1_SPEC, 12> {
        EXTINT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extic1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extic1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIC1_SPEC;
impl crate::RegisterSpec for EXTIC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extic1::R`](R) reader structure"]
impl crate::Readable for EXTIC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extic1::W`](W) writer structure"]
impl crate::Writable for EXTIC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTIC1 to value 0"]
impl crate::Resettable for EXTIC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}