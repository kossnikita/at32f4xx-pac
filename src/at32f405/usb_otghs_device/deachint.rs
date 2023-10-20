#[doc = "Register `DEACHINT` reader"]
pub type R = crate::R<DEACHINT_SPEC>;
#[doc = "Register `DEACHINT` writer"]
pub type W = crate::W<DEACHINT_SPEC>;
#[doc = "Field `ECHINEPINT` reader - Each IN Endpoint Interrupt bits"]
pub type ECHINEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `ECHINEPINT` writer - Each IN Endpoint Interrupt bits"]
pub type ECHINEPINT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ECHOUTEPINT` reader - Each OUT Endpoint Interrupt bits"]
pub type ECHOUTEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `ECHOUTEPINT` writer - Each OUT Endpoint Interrupt bits"]
pub type ECHOUTEPINT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINT")
            .field("echinepint", &format_args!("{}", self.echinepint().bits()))
            .field(
                "echoutepint",
                &format_args!("{}", self.echoutepint().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEACHINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each IN Endpoint Interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn echinepint(&mut self) -> ECHINEPINT_W<DEACHINT_SPEC> {
        ECHINEPINT_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Each OUT Endpoint Interrupt bits"]
    #[inline(always)]
    #[must_use]
    pub fn echoutepint(&mut self) -> ECHOUTEPINT_W<DEACHINT_SPEC> {
        ECHOUTEPINT_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
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
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEACHINT to value 0"]
impl crate::Resettable for DEACHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
