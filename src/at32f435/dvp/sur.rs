#[doc = "Register `SUR` reader"]
pub type R = crate::R<SUR_SPEC>;
#[doc = "Register `SUR` writer"]
pub type W = crate::W<SUR_SPEC>;
#[doc = "Field `FMSU` reader - Frame start unmask"]
pub type FMSU_R = crate::FieldReader;
#[doc = "Field `FMSU` writer - Frame start unmask"]
pub type FMSU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LNSU` reader - Line start unmask"]
pub type LNSU_R = crate::FieldReader;
#[doc = "Field `LNSU` writer - Line start unmask"]
pub type LNSU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LNEU` reader - Line end unmask"]
pub type LNEU_R = crate::FieldReader;
#[doc = "Field `LNEU` writer - Line end unmask"]
pub type LNEU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FMEU` reader - Frame end unmask"]
pub type FMEU_R = crate::FieldReader;
#[doc = "Field `FMEU` writer - Frame end unmask"]
pub type FMEU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame start unmask"]
    #[inline(always)]
    pub fn fmsu(&self) -> FMSU_R {
        FMSU_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start unmask"]
    #[inline(always)]
    pub fn lnsu(&self) -> LNSU_R {
        LNSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end unmask"]
    #[inline(always)]
    pub fn lneu(&self) -> LNEU_R {
        LNEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end unmask"]
    #[inline(always)]
    pub fn fmeu(&self) -> FMEU_R {
        FMEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start unmask"]
    #[inline(always)]
    #[must_use]
    pub fn fmsu(&mut self) -> FMSU_W<SUR_SPEC, 0> {
        FMSU_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start unmask"]
    #[inline(always)]
    #[must_use]
    pub fn lnsu(&mut self) -> LNSU_W<SUR_SPEC, 8> {
        LNSU_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end unmask"]
    #[inline(always)]
    #[must_use]
    pub fn lneu(&mut self) -> LNEU_W<SUR_SPEC, 16> {
        LNEU_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame end unmask"]
    #[inline(always)]
    #[must_use]
    pub fn fmeu(&mut self) -> FMEU_W<SUR_SPEC, 24> {
        FMEU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Synchronization unmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUR_SPEC;
impl crate::RegisterSpec for SUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sur::R`](R) reader structure"]
impl crate::Readable for SUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sur::W`](W) writer structure"]
impl crate::Writable for SUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUR to value 0"]
impl crate::Resettable for SUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
