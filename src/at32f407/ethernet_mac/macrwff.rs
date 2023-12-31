#[doc = "Register `MACRWFF` reader"]
pub type R = crate::R<MACRWFF_SPEC>;
#[doc = "Register `MACRWFF` writer"]
pub type W = crate::W<MACRWFF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACRWFF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "Ethernet MAC remote wakeup frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRWFF_SPEC;
impl crate::RegisterSpec for MACRWFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrwff::R`](R) reader structure"]
impl crate::Readable for MACRWFF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macrwff::W`](W) writer structure"]
impl crate::Writable for MACRWFF_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACRWFF to value 0"]
impl crate::Resettable for MACRWFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
