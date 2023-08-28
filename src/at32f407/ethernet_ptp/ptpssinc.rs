#[doc = "Register `PTPSSINC` reader"]
pub type R = crate::R<PTPSSINC_SPEC>;
#[doc = "Register `PTPSSINC` writer"]
pub type W = crate::W<PTPSSINC_SPEC>;
#[doc = "Field `SSIV` reader - Sub-second increment value"]
pub type SSIV_R = crate::FieldReader;
#[doc = "Field `SSIV` writer - Sub-second increment value"]
pub type SSIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sub-second increment value"]
    #[inline(always)]
    pub fn ssiv(&self) -> SSIV_R {
        SSIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-second increment value"]
    #[inline(always)]
    #[must_use]
    pub fn ssiv(&mut self) -> SSIV_W<PTPSSINC_SPEC, 0> {
        SSIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssinc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssinc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPSSINC_SPEC;
impl crate::RegisterSpec for PTPSSINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpssinc::R`](R) reader structure"]
impl crate::Readable for PTPSSINC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptpssinc::W`](W) writer structure"]
impl crate::Writable for PTPSSINC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPSSINC to value 0"]
impl crate::Resettable for PTPSSINC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
