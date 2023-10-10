#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCR_SPEC>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `FMSC` reader - Frame start code"]
pub type FMSC_R = crate::FieldReader;
#[doc = "Field `FMSC` writer - Frame start code"]
pub type FMSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LNSC` reader - Line start code"]
pub type LNSC_R = crate::FieldReader;
#[doc = "Field `LNSC` writer - Line start code"]
pub type LNSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LNEC` reader - Line end code"]
pub type LNEC_R = crate::FieldReader;
#[doc = "Field `LNEC` writer - Line end code"]
pub type LNEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FMEC` reader - Frame end code"]
pub type FMEC_R = crate::FieldReader;
#[doc = "Field `FMEC` writer - Frame end code"]
pub type FMEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame start code"]
    #[inline(always)]
    pub fn fmsc(&self) -> FMSC_R {
        FMSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start code"]
    #[inline(always)]
    pub fn lnsc(&self) -> LNSC_R {
        LNSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end code"]
    #[inline(always)]
    pub fn lnec(&self) -> LNEC_R {
        LNEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end code"]
    #[inline(always)]
    pub fn fmec(&self) -> FMEC_R {
        FMEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("fmec", &format_args!("{}", self.fmec().bits()))
            .field("lnec", &format_args!("{}", self.lnec().bits()))
            .field("lnsc", &format_args!("{}", self.lnsc().bits()))
            .field("fmsc", &format_args!("{}", self.fmsc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start code"]
    #[inline(always)]
    #[must_use]
    pub fn fmsc(&mut self) -> FMSC_W<SCR_SPEC, 0> {
        FMSC_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start code"]
    #[inline(always)]
    #[must_use]
    pub fn lnsc(&mut self) -> LNSC_W<SCR_SPEC, 8> {
        LNSC_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end code"]
    #[inline(always)]
    #[must_use]
    pub fn lnec(&mut self) -> LNEC_W<SCR_SPEC, 16> {
        LNEC_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame end code"]
    #[inline(always)]
    #[must_use]
    pub fn fmec(&mut self) -> FMEC_W<SCR_SPEC, 24> {
        FMEC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
