#[doc = "Register `DMADT` reader"]
pub type R = crate::R<DMADT_SPEC>;
#[doc = "Register `DMADT` writer"]
pub type W = crate::W<DMADT_SPEC>;
#[doc = "Field `DMADT` reader - DMA data register"]
pub type DMADT_R = crate::FieldReader<u16>;
#[doc = "Field `DMADT` writer - DMA data register"]
pub type DMADT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA data register"]
    #[inline(always)]
    pub fn dmadt(&self) -> DMADT_R {
        DMADT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA data register"]
    #[inline(always)]
    #[must_use]
    pub fn dmadt(&mut self) -> DMADT_W<DMADT_SPEC, 0> {
        DMADT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmadt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMADT_SPEC;
impl crate::RegisterSpec for DMADT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadt::R`](R) reader structure"]
impl crate::Readable for DMADT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmadt::W`](W) writer structure"]
impl crate::Writable for DMADT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMADT to value 0"]
impl crate::Resettable for DMADT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
