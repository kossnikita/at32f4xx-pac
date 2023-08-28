#[doc = "Register `DMARDLADDR` reader"]
pub type R = crate::R<DMARDLADDR_SPEC>;
#[doc = "Register `DMARDLADDR` writer"]
pub type W = crate::W<DMARDLADDR_SPEC>;
#[doc = "Field `SRL` reader - Start of receive list"]
pub type SRL_R = crate::FieldReader<u32>;
#[doc = "Field `SRL` writer - Start of receive list"]
pub type SRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SRL_W<DMARDLADDR_SPEC, 0> {
        SRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmardladdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardladdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARDLADDR_SPEC;
impl crate::RegisterSpec for DMARDLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardladdr::R`](R) reader structure"]
impl crate::Readable for DMARDLADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmardladdr::W`](W) writer structure"]
impl crate::Writable for DMARDLADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMARDLADDR to value 0"]
impl crate::Resettable for DMARDLADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
