#[doc = "Register `RTCCAL` reader"]
pub type R = crate::R<RTCCAL_SPEC>;
#[doc = "Register `RTCCAL` writer"]
pub type W = crate::W<RTCCAL_SPEC>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CALVAL_R = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CALVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `CALOUT` reader - Calibration Clock Output"]
pub type CALOUT_R = crate::BitReader;
#[doc = "Field `CALOUT` writer - Calibration Clock Output"]
pub type CALOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTEN` reader - Output enable"]
pub type OUTEN_R = crate::BitReader;
#[doc = "Field `OUTEN` writer - Output enable"]
pub type OUTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTSEL` reader - Output selection"]
pub type OUTSEL_R = crate::BitReader;
#[doc = "Field `OUTSEL` writer - Output selection"]
pub type OUTSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CALVAL_W<RTCCAL_SPEC, 0> {
        CALVAL_W::new(self)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<RTCCAL_SPEC, 7> {
        CALOUT_W::new(self)
    }
    #[doc = "Bit 8 - Output enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<RTCCAL_SPEC, 8> {
        OUTEN_W::new(self)
    }
    #[doc = "Bit 9 - Output selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<RTCCAL_SPEC, 9> {
        OUTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCAL_SPEC;
impl crate::RegisterSpec for RTCCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccal::R`](R) reader structure"]
impl crate::Readable for RTCCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccal::W`](W) writer structure"]
impl crate::Writable for RTCCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCAL to value 0"]
impl crate::Resettable for RTCCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
