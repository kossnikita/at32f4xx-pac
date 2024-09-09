#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GOTGINT_SPEC>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GOTGINT_SPEC>;
#[doc = "Field `SESENDDET` reader - VBUS is deasserted"]
pub type SESENDDET_R = crate::BitReader;
#[doc = "Field `SESENDDET` writer - VBUS is deasserted"]
pub type SESENDDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - VBUS is deasserted"]
    #[inline(always)]
    pub fn sesenddet(&self) -> SESENDDET_R {
        SESENDDET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
            .field("sesenddet", &self.sesenddet())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - VBUS is deasserted"]
    #[inline(always)]
    #[must_use]
    pub fn sesenddet(&mut self) -> SESENDDET_W<GOTGINT_SPEC> {
        SESENDDET_W::new(self, 2)
    }
}
#[doc = "OTGHS interrupt register (OTGHS_GOTGINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
