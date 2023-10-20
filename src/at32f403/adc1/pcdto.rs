#[doc = "Register `PCDTO%s` reader"]
pub type R = crate::R<PCDTO_SPEC>;
#[doc = "Register `PCDTO%s` writer"]
pub type W = crate::W<PCDTO_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PCDTO_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data offset for Preempted channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDTO_SPEC;
impl crate::RegisterSpec for PCDTO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pcdto::R`](R) reader structure"]
impl crate::Readable for PCDTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcdto::W`](W) writer structure"]
impl crate::Writable for PCDTO_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDTO%s to value 0"]
impl crate::Resettable for PCDTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
