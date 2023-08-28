#[doc = "Register `C7PADDR` reader"]
pub type R = crate::R<C7PADDR_SPEC>;
#[doc = "Register `C7PADDR` writer"]
pub type W = crate::W<C7PADDR_SPEC>;
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
    pub fn paddr(&mut self) -> PADDR_W<C7PADDR_SPEC, 0> {
        PADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel 7 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7paddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7paddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C7PADDR_SPEC;
impl crate::RegisterSpec for C7PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c7paddr::R`](R) reader structure"]
impl crate::Readable for C7PADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c7paddr::W`](W) writer structure"]
impl crate::Writable for C7PADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C7PADDR to value 0"]
impl crate::Resettable for C7PADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
