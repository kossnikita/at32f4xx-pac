#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CALON` reader - Internal high-speed clock calibration ready"]
pub type CALON_R = crate::BitReader;
#[doc = "Field `CALON` writer - Internal high-speed clock calibration ready"]
pub type CALON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENTRIM` reader - Enable trim"]
pub type ENTRIM_R = crate::BitReader;
#[doc = "Field `ENTRIM` writer - Enable trim"]
pub type ENTRIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIEN` reader - RSLOST error interrupt enable"]
pub type EIEN_R = crate::BitReader;
#[doc = "Field `EIEN` writer - RSLOST error interrupt enable"]
pub type EIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALRDYIEN` reader - CALRDY interrupt enable"]
pub type CALRDYIEN_R = crate::BitReader;
#[doc = "Field `CALRDYIEN` writer - CALRDY interrupt enable"]
pub type CALRDYIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STEP` reader - STEP"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `STEP` writer - STEP"]
pub type STEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock calibration ready"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    pub fn entrim(&self) -> ENTRIM_R {
        ENTRIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RSLOST error interrupt enable"]
    #[inline(always)]
    pub fn eien(&self) -> EIEN_R {
        EIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CALRDY interrupt enable"]
    #[inline(always)]
    pub fn calrdyien(&self) -> CALRDYIEN_R {
        CALRDYIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - STEP"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock calibration ready"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<CTRL1_SPEC, 0> {
        CALON_W::new(self)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    #[must_use]
    pub fn entrim(&mut self) -> ENTRIM_W<CTRL1_SPEC, 1> {
        ENTRIM_W::new(self)
    }
    #[doc = "Bit 4 - RSLOST error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eien(&mut self) -> EIEN_W<CTRL1_SPEC, 4> {
        EIEN_W::new(self)
    }
    #[doc = "Bit 5 - CALRDY interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn calrdyien(&mut self) -> CALRDYIEN_W<CTRL1_SPEC, 5> {
        CALRDYIEN_W::new(self)
    }
    #[doc = "Bits 8:11 - STEP"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<CTRL1_SPEC, 8> {
        STEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0100"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
