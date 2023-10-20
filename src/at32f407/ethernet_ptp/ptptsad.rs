#[doc = "Register `PTPTSAD` reader"]
pub type R = crate::R<PTPTSAD_SPEC>;
#[doc = "Register `PTPTSAD` writer"]
pub type W = crate::W<PTPTSAD_SPEC>;
#[doc = "Field `TAR` reader - Timestamp addend register"]
pub type TAR_R = crate::FieldReader<u32>;
#[doc = "Field `TAR` writer - Timestamp addend register"]
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp addend register"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSAD")
            .field("tar", &format_args!("{}", self.tar().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PTPTSAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp addend register"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<PTPTSAD_SPEC> {
        TAR_W::new(self, 0)
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
#[doc = "Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSAD_SPEC;
impl crate::RegisterSpec for PTPTSAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsad::R`](R) reader structure"]
impl crate::Readable for PTPTSAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptsad::W`](W) writer structure"]
impl crate::Writable for PTPTSAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTSAD to value 0"]
impl crate::Resettable for PTPTSAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
