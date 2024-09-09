#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCR_SPEC>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `FMSC` reader - Frame start code"]
pub type FMSC_R = crate::FieldReader;
#[doc = "Field `FMSC` writer - Frame start code"]
pub type FMSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNSC` reader - Line start code"]
pub type LNSC_R = crate::FieldReader;
#[doc = "Field `LNSC` writer - Line start code"]
pub type LNSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNEC` reader - Line end code"]
pub type LNEC_R = crate::FieldReader;
#[doc = "Field `LNEC` writer - Line end code"]
pub type LNEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FMEC` reader - Frame end code"]
pub type FMEC_R = crate::FieldReader;
#[doc = "Field `FMEC` writer - Frame end code"]
pub type FMEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
            .field("fmec", &self.fmec())
            .field("lnec", &self.lnec())
            .field("lnsc", &self.lnsc())
            .field("fmsc", &self.fmsc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start code"]
    #[inline(always)]
    #[must_use]
    pub fn fmsc(&mut self) -> FMSC_W<SCR_SPEC> {
        FMSC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start code"]
    #[inline(always)]
    #[must_use]
    pub fn lnsc(&mut self) -> LNSC_W<SCR_SPEC> {
        LNSC_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end code"]
    #[inline(always)]
    #[must_use]
    pub fn lnec(&mut self) -> LNEC_W<SCR_SPEC> {
        LNEC_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end code"]
    #[inline(always)]
    #[must_use]
    pub fn fmec(&mut self) -> FMEC_W<SCR_SPEC> {
        FMEC_W::new(self, 24)
    }
}
#[doc = "Synchronization code register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
