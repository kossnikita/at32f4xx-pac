#[doc = "Register `SUR` reader"]
pub type R = crate::R<SUR_SPEC>;
#[doc = "Register `SUR` writer"]
pub type W = crate::W<SUR_SPEC>;
#[doc = "Field `FMSU` reader - Frame start unmask"]
pub type FMSU_R = crate::FieldReader;
#[doc = "Field `FMSU` writer - Frame start unmask"]
pub type FMSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNSU` reader - Line start unmask"]
pub type LNSU_R = crate::FieldReader;
#[doc = "Field `LNSU` writer - Line start unmask"]
pub type LNSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LNEU` reader - Line end unmask"]
pub type LNEU_R = crate::FieldReader;
#[doc = "Field `LNEU` writer - Line end unmask"]
pub type LNEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FMEU` reader - Frame end unmask"]
pub type FMEU_R = crate::FieldReader;
#[doc = "Field `FMEU` writer - Frame end unmask"]
pub type FMEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUR")
            .field("fmeu", &format_args!("{}", self.fmeu().bits()))
            .field("lneu", &format_args!("{}", self.lneu().bits()))
            .field("lnsu", &format_args!("{}", self.lnsu().bits()))
            .field("fmsu", &format_args!("{}", self.fmsu().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SUR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start unmask"]
    #[inline(always)]
    #[must_use]
    pub fn fmsu(&mut self) -> FMSU_W<SUR_SPEC> {
        FMSU_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start unmask"]
    #[inline(always)]
    #[must_use]
    pub fn lnsu(&mut self) -> LNSU_W<SUR_SPEC> {
        LNSU_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end unmask"]
    #[inline(always)]
    #[must_use]
    pub fn lneu(&mut self) -> LNEU_W<SUR_SPEC> {
        LNEU_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end unmask"]
    #[inline(always)]
    #[must_use]
    pub fn fmeu(&mut self) -> FMEU_W<SUR_SPEC> {
        FMEU_W::new(self, 24)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUR to value 0"]
impl crate::Resettable for SUR_SPEC {
    const RESET_VALUE: u32 = 0;
}
