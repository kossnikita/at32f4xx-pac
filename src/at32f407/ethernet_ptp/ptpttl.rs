#[doc = "Register `PTPTTL` reader"]
pub type R = crate::R<PTPTTL_SPEC>;
#[doc = "Register `PTPTTL` writer"]
pub type W = crate::W<PTPTTL_SPEC>;
#[doc = "Field `TTLR` reader - Target timestamp low register"]
pub type TTLR_R = crate::FieldReader<u32>;
#[doc = "Field `TTLR` writer - Target timestamp low register"]
pub type TTLR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target timestamp low register"]
    #[inline(always)]
    pub fn ttlr(&self) -> TTLR_R {
        TTLR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTTL")
            .field("ttlr", &format_args!("{}", self.ttlr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PTPTTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target timestamp low register"]
    #[inline(always)]
    #[must_use]
    pub fn ttlr(&mut self) -> TTLR_W<PTPTTL_SPEC> {
        TTLR_W::new(self, 0)
    }
}
#[doc = "Ethernet PTP target time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpttl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpttl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTTL_SPEC;
impl crate::RegisterSpec for PTPTTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpttl::R`](R) reader structure"]
impl crate::Readable for PTPTTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptpttl::W`](W) writer structure"]
impl crate::Writable for PTPTTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTTL to value 0"]
impl crate::Resettable for PTPTTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
