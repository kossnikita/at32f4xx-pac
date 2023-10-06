#[doc = "Register `CPOLY` reader"]
pub type R = crate::R<CPOLY_SPEC>;
#[doc = "Register `CPOLY` writer"]
pub type W = crate::W<CPOLY_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CPOLY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpoly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpoly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPOLY_SPEC;
impl crate::RegisterSpec for CPOLY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cpoly::R`](R) reader structure"]
impl crate::Readable for CPOLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpoly::W`](W) writer structure"]
impl crate::Writable for CPOLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPOLY to value 0x07"]
impl crate::Resettable for CPOLY_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
