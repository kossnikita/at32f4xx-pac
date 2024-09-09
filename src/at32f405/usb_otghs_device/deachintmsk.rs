#[doc = "Register `DEACHINTMSK` reader"]
pub type R = crate::R<DEACHINTMSK_SPEC>;
#[doc = "Register `DEACHINTMSK` writer"]
pub type W = crate::W<DEACHINTMSK_SPEC>;
#[doc = "Field `ECHINEPINTMSK` reader - Each IN Endpoint Interrupt Mask bits"]
pub type ECHINEPINTMSK_R = crate::FieldReader<u16>;
#[doc = "Field `ECHINEPINTMSK` writer - Each IN Endpoint Interrupt Mask bits"]
pub type ECHINEPINTMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ECHOUTEPINTMSK` reader - Each OUT Endpoint Interrupt Mask bits"]
pub type ECHOUTEPINTMSK_R = crate::FieldReader<u16>;
#[doc = "Field `ECHOUTEPINTMSK` writer - Each OUT Endpoint Interrupt Mask bits"]
pub type ECHOUTEPINTMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINTMSK")
            .field("echinepintmsk", &self.echinepintmsk())
            .field("echoutepintmsk", &self.echoutepintmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Each IN Endpoint Interrupt Mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn echinepintmsk(&mut self) -> ECHINEPINTMSK_W<DEACHINTMSK_SPEC> {
        ECHINEPINTMSK_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Each OUT Endpoint Interrupt Mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn echoutepintmsk(&mut self) -> ECHOUTEPINTMSK_W<DEACHINTMSK_SPEC> {
        ECHOUTEPINTMSK_W::new(self, 16)
    }
}
#[doc = "Device Each Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`deachintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINTMSK_SPEC;
impl crate::RegisterSpec for DEACHINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachintmsk::R`](R) reader structure"]
impl crate::Readable for DEACHINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deachintmsk::W`](W) writer structure"]
impl crate::Writable for DEACHINTMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEACHINTMSK to value 0"]
impl crate::Resettable for DEACHINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
