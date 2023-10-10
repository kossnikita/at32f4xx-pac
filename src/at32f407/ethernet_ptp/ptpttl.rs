#[doc = "Register `PTPTTL` reader"]
pub type R = crate::R<PTPTTL_SPEC>;
#[doc = "Register `PTPTTL` writer"]
pub type W = crate::W<PTPTTL_SPEC>;
#[doc = "Field `TTLR` reader - Target timestamp low register"]
pub type TTLR_R = crate::FieldReader<u32>;
#[doc = "Field `TTLR` writer - Target timestamp low register"]
pub type TTLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target timestamp low register"]
    #[inline(always)]
    #[must_use]
    pub fn ttlr(&mut self) -> TTLR_W<PTPTTL_SPEC, 0> {
        TTLR_W::new(self)
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
#[doc = "Ethernet PTP target time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpttl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpttl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTTL_SPEC;
impl crate::RegisterSpec for PTPTTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpttl::R`](R) reader structure"]
impl crate::Readable for PTPTTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptpttl::W`](W) writer structure"]
impl crate::Writable for PTPTTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTTL to value 0"]
impl crate::Resettable for PTPTTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
