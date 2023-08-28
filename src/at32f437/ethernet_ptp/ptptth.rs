#[doc = "Register `PTPTTH` reader"]
pub type R = crate::R<PTPTTH_SPEC>;
#[doc = "Register `PTPTTH` writer"]
pub type W = crate::W<PTPTTH_SPEC>;
#[doc = "Field `TTSR` reader - Target time seconds register"]
pub type TTSR_R = crate::FieldReader<u32>;
#[doc = "Field `TTSR` writer - Target time seconds register"]
pub type TTSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Target time seconds register"]
    #[inline(always)]
    pub fn ttsr(&self) -> TTSR_R {
        TTSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target time seconds register"]
    #[inline(always)]
    #[must_use]
    pub fn ttsr(&mut self) -> TTSR_W<PTPTTH_SPEC, 0> {
        TTSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet PTP target time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTTH_SPEC;
impl crate::RegisterSpec for PTPTTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptth::R`](R) reader structure"]
impl crate::Readable for PTPTTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptth::W`](W) writer structure"]
impl crate::Writable for PTPTTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTTH to value 0"]
impl crate::Resettable for PTPTTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
