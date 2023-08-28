#[doc = "Register `CALVAL` reader"]
pub type R = crate::R<CALVAL_SPEC>;
#[doc = "Register `CALVAL` writer"]
pub type W = crate::W<CALVAL_SPEC>;
#[doc = "Field `CALVAL` reader - A/D Calibration value"]
pub type CALVAL_R = crate::FieldReader;
#[doc = "Field `CALVAL` writer - A/D Calibration value"]
pub type CALVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - A/D Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - A/D Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CALVAL_W<CALVAL_SPEC, 0> {
        CALVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Calibration value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALVAL_SPEC;
impl crate::RegisterSpec for CALVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calval::R`](R) reader structure"]
impl crate::Readable for CALVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calval::W`](W) writer structure"]
impl crate::Writable for CALVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALVAL to value 0"]
impl crate::Resettable for CALVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
