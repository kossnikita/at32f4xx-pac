#[doc = "Register `PTPTTH` reader"]
pub type R = crate::R<PTPTTH_SPEC>;
#[doc = "Register `PTPTTH` writer"]
pub type W = crate::W<PTPTTH_SPEC>;
#[doc = "Field `TTSR` reader - Target time seconds register"]
pub type TTSR_R = crate::FieldReader<u32>;
#[doc = "Field `TTSR` writer - Target time seconds register"]
pub type TTSR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target time seconds register"]
    #[inline(always)]
    pub fn ttsr(&self) -> TTSR_R {
        TTSR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTTH")
            .field("ttsr", &self.ttsr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Target time seconds register"]
    #[inline(always)]
    #[must_use]
    pub fn ttsr(&mut self) -> TTSR_W<PTPTTH_SPEC> {
        TTSR_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP target time high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptptth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTTH_SPEC;
impl crate::RegisterSpec for PTPTTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptth::R`](R) reader structure"]
impl crate::Readable for PTPTTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptth::W`](W) writer structure"]
impl crate::Writable for PTPTTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTTH to value 0"]
impl crate::Resettable for PTPTTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
