#[doc = "Register `SCAL` reader"]
pub type R = crate::R<SCAL_SPEC>;
#[doc = "Register `SCAL` writer"]
pub type W = crate::W<SCAL_SPEC>;
#[doc = "Field `DEC` reader - Decrease ERTC clock"]
pub type DEC_R = crate::FieldReader<u16>;
#[doc = "Field `DEC` writer - Decrease ERTC clock"]
pub type DEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `CAL16` reader - 16 second calibration period"]
pub type CAL16_R = crate::BitReader;
#[doc = "Field `CAL16` writer - 16 second calibration period"]
pub type CAL16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAL8` reader - 8-second calibration period"]
pub type CAL8_R = crate::BitReader;
#[doc = "Field `CAL8` writer - 8-second calibration period"]
pub type CAL8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADD` reader - Add ERTC clock"]
pub type ADD_R = crate::BitReader;
#[doc = "Field `ADD` writer - Add ERTC clock"]
pub type ADD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8 - Decrease ERTC clock"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - 16 second calibration period"]
    #[inline(always)]
    pub fn cal16(&self) -> CAL16_R {
        CAL16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 8-second calibration period"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Add ERTC clock"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Decrease ERTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<SCAL_SPEC, 0> {
        DEC_W::new(self)
    }
    #[doc = "Bit 13 - 16 second calibration period"]
    #[inline(always)]
    #[must_use]
    pub fn cal16(&mut self) -> CAL16_W<SCAL_SPEC, 13> {
        CAL16_W::new(self)
    }
    #[doc = "Bit 14 - 8-second calibration period"]
    #[inline(always)]
    #[must_use]
    pub fn cal8(&mut self) -> CAL8_W<SCAL_SPEC, 14> {
        CAL8_W::new(self)
    }
    #[doc = "Bit 15 - Add ERTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<SCAL_SPEC, 15> {
        ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCAL_SPEC;
impl crate::RegisterSpec for SCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scal::R`](R) reader structure"]
impl crate::Readable for SCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scal::W`](W) writer structure"]
impl crate::Writable for SCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCAL to value 0"]
impl crate::Resettable for SCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
