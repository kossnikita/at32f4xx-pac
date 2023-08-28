#[doc = "Register `DEACHINTMSK` reader"]
pub type R = crate::R<DEACHINTMSK_SPEC>;
#[doc = "Register `DEACHINTMSK` writer"]
pub type W = crate::W<DEACHINTMSK_SPEC>;
#[doc = "Field `ECHINEPINTMSK` reader - Each IN Endpoint Interrupt Mask bits"]
pub type ECHINEPINTMSK_R = crate::FieldReader<u16>;
#[doc = "Field `ECHINEPINTMSK` writer - Each IN Endpoint Interrupt Mask bits"]
pub type ECHINEPINTMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ECHOUTEPINTMSK` reader - Each OUT Endpoint Interrupt Mask bits"]
pub type ECHOUTEPINTMSK_R = crate::FieldReader<u16>;
#[doc = "Field `ECHOUTEPINTMSK` writer - Each OUT Endpoint Interrupt Mask bits"]
pub type ECHOUTEPINTMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Each IN Endpoint Interrupt Mask bits"]
    #[inline(always)]
    pub fn echinepintmsk(&self) -> ECHINEPINTMSK_R {
        ECHINEPINTMSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Each OUT Endpoint Interrupt Mask bits"]
    #[inline(always)]
    pub fn echoutepintmsk(&self) -> ECHOUTEPINTMSK_R {
        ECHOUTEPINTMSK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each IN Endpoint Interrupt Mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn echinepintmsk(&mut self) -> ECHINEPINTMSK_W<DEACHINTMSK_SPEC, 0> {
        ECHINEPINTMSK_W::new(self)
    }
    #[doc = "Bits 16:31 - Each OUT Endpoint Interrupt Mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn echoutepintmsk(&mut self) -> ECHOUTEPINTMSK_W<DEACHINTMSK_SPEC, 16> {
        ECHOUTEPINTMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Each Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINTMSK_SPEC;
impl crate::RegisterSpec for DEACHINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachintmsk::R`](R) reader structure"]
impl crate::Readable for DEACHINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deachintmsk::W`](W) writer structure"]
impl crate::Writable for DEACHINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEACHINTMSK to value 0"]
impl crate::Resettable for DEACHINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
