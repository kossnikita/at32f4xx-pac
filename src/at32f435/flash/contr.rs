#[doc = "Register `CONTR` reader"]
pub type R = crate::R<CONTR_SPEC>;
#[doc = "Register `CONTR` writer"]
pub type W = crate::W<CONTR_SPEC>;
#[doc = "Field `FCONTR_EN` reader - Flash continue read enable"]
pub type FCONTR_EN_R = crate::BitReader;
#[doc = "Field `FCONTR_EN` writer - Flash continue read enable"]
pub type FCONTR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - Flash continue read enable"]
    #[inline(always)]
    pub fn fcontr_en(&self) -> FCONTR_EN_R {
        FCONTR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Flash continue read enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcontr_en(&mut self) -> FCONTR_EN_W<CONTR_SPEC, 31> {
        FCONTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash continue read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`contr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`contr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTR_SPEC;
impl crate::RegisterSpec for CONTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`contr::R`](R) reader structure"]
impl crate::Readable for CONTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`contr::W`](W) writer structure"]
impl crate::Writable for CONTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTR to value 0x80"]
impl crate::Resettable for CONTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}