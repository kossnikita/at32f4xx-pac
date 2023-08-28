#[doc = "Register `DEACHINT` reader"]
pub type R = crate::R<DEACHINT_SPEC>;
#[doc = "Register `DEACHINT` writer"]
pub type W = crate::W<DEACHINT_SPEC>;
#[doc = "Field `ECHINEPINT` reader - Each IN Endpoint Interrupt bits"]
pub type ECHINEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `ECHINEPINT` writer - Each IN Endpoint Interrupt bits"]
pub type ECHINEPINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ECHOUTEPINT` reader - Each OUT Endpoint Interrupt bits"]
pub type ECHOUTEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `ECHOUTEPINT` writer - Each OUT Endpoint Interrupt bits"]
pub type ECHOUTEPINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Each IN Endpoint Interrupt bits"]
    #[inline(always)]
    pub fn echinepint(&self) -> ECHINEPINT_R {
        ECHINEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Each OUT Endpoint Interrupt bits"]
    #[inline(always)]
    pub fn echoutepint(&self) -> ECHOUTEPINT_R {
        ECHOUTEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each IN Endpoint Interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn echinepint(&mut self) -> ECHINEPINT_W<DEACHINT_SPEC, 0> {
        ECHINEPINT_W::new(self)
    }
    #[doc = "Bits 16:31 - Each OUT Endpoint Interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn echoutepint(&mut self) -> ECHOUTEPINT_W<DEACHINT_SPEC, 16> {
        ECHOUTEPINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Each Endpoints Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINT_SPEC;
impl crate::RegisterSpec for DEACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachint::R`](R) reader structure"]
impl crate::Readable for DEACHINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deachint::W`](W) writer structure"]
impl crate::Writable for DEACHINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEACHINT to value 0"]
impl crate::Resettable for DEACHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}