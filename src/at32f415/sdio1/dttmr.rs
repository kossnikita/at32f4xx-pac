#[doc = "Register `DTTMR` reader"]
pub type R = crate::R<DTTMR_SPEC>;
#[doc = "Register `DTTMR` writer"]
pub type W = crate::W<DTTMR_SPEC>;
#[doc = "Field `TIMEOUT` reader - Data timeout period"]
pub type TIMEOUT_R = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT` writer - Data timeout period"]
pub type TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<DTTMR_SPEC, 0> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bits 31:0 = DATATIME: Data timeout period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTTMR_SPEC;
impl crate::RegisterSpec for DTTMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttmr::R`](R) reader structure"]
impl crate::Readable for DTTMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dttmr::W`](W) writer structure"]
impl crate::Writable for DTTMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTTMR to value 0"]
impl crate::Resettable for DTTMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
