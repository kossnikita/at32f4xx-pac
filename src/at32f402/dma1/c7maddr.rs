#[doc = "Register `C7MADDR` reader"]
pub type R = crate::R<C7MADDR_SPEC>;
#[doc = "Register `C7MADDR` writer"]
pub type W = crate::W<C7MADDR_SPEC>;
#[doc = "Field `MADDR` reader - Memory address"]
pub type MADDR_R = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory address"]
pub type MADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&self) -> MADDR_R {
        MADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MADDR_W<C7MADDR_SPEC, 0> {
        MADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel 7 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7maddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7maddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C7MADDR_SPEC;
impl crate::RegisterSpec for C7MADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c7maddr::R`](R) reader structure"]
impl crate::Readable for C7MADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c7maddr::W`](W) writer structure"]
impl crate::Writable for C7MADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C7MADDR to value 0"]
impl crate::Resettable for C7MADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
