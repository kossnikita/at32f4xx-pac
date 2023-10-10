#[doc = "Register `PTPTSHUD` reader"]
pub type R = crate::R<PTPTSHUD_SPEC>;
#[doc = "Register `PTPTSHUD` writer"]
pub type W = crate::W<PTPTSHUD_SPEC>;
#[doc = "Field `TS` reader - Timestamp second"]
pub type TS_R = crate::FieldReader<u32>;
#[doc = "Field `TS` writer - Timestamp second"]
pub type TS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSHUD")
            .field("ts", &format_args!("{}", self.ts().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PTPTSHUD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<PTPTSHUD_SPEC, 0> {
        TS_W::new(self)
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
#[doc = "Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSHUD_SPEC;
impl crate::RegisterSpec for PTPTSHUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshud::R`](R) reader structure"]
impl crate::Readable for PTPTSHUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptshud::W`](W) writer structure"]
impl crate::Writable for PTPTSHUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTSHUD to value 0"]
impl crate::Resettable for PTPTSHUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
