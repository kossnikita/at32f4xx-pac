#[doc = "Register `S2PADDR` reader"]
pub type R = crate::R<S2PADDR_SPEC>;
#[doc = "Register `S2PADDR` writer"]
pub type W = crate::W<S2PADDR_SPEC>;
#[doc = "Field `PADDR` reader - Peripheral address"]
pub type PADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral address"]
pub type PADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn paddr(&mut self) -> PADDR_W<S2PADDR_SPEC, 0> {
        PADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "stream 2 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2paddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2paddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2PADDR_SPEC;
impl crate::RegisterSpec for S2PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2paddr::R`](R) reader structure"]
impl crate::Readable for S2PADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s2paddr::W`](W) writer structure"]
impl crate::Writable for S2PADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S2PADDR to value 0"]
impl crate::Resettable for S2PADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
