#[doc = "Register `CCAL` reader"]
pub type R = crate::R<CCAL_SPEC>;
#[doc = "Register `CCAL` writer"]
pub type W = crate::W<CCAL_SPEC>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CALVAL_R = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CALVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CALDIR` reader - Calibration direction"]
pub type CALDIR_R = crate::BitReader;
#[doc = "Field `CALDIR` writer - Calibration direction"]
pub type CALDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CALDIR_R {
        CALDIR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CALVAL_W<CCAL_SPEC, 0> {
        CALVAL_W::new(self)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    #[must_use]
    pub fn caldir(&mut self) -> CALDIR_W<CCAL_SPEC, 7> {
        CALDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCAL_SPEC;
impl crate::RegisterSpec for CCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccal::R`](R) reader structure"]
impl crate::Readable for CCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccal::W`](W) writer structure"]
impl crate::Writable for CCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCAL to value 0"]
impl crate::Resettable for CCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
