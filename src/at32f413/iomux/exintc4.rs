#[doc = "Register `EXINTC4` reader"]
pub type R = crate::R<EXINTC4_SPEC>;
#[doc = "Register `EXINTC4` writer"]
pub type W = crate::W<EXINTC4_SPEC>;
#[doc = "Field `EXINT12` reader - Configure EXINT12 source"]
pub type EXINT12_R = crate::FieldReader;
#[doc = "Field `EXINT12` writer - Configure EXINT12 source"]
pub type EXINT12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT13` reader - Configure EXINT13 source"]
pub type EXINT13_R = crate::FieldReader;
#[doc = "Field `EXINT13` writer - Configure EXINT13 source"]
pub type EXINT13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT14` reader - Configure EXINT14 source"]
pub type EXINT14_R = crate::FieldReader;
#[doc = "Field `EXINT14` writer - Configure EXINT14 source"]
pub type EXINT14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT15` reader - Configure EXINT15 source"]
pub type EXINT15_R = crate::FieldReader;
#[doc = "Field `EXINT15` writer - Configure EXINT15 source"]
pub type EXINT15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT12 source"]
    #[inline(always)]
    pub fn exint12(&self) -> EXINT12_R {
        EXINT12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT13 source"]
    #[inline(always)]
    pub fn exint13(&self) -> EXINT13_R {
        EXINT13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT14 source"]
    #[inline(always)]
    pub fn exint14(&self) -> EXINT14_R {
        EXINT14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT15 source"]
    #[inline(always)]
    pub fn exint15(&self) -> EXINT15_R {
        EXINT15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC4")
            .field("exint12", &self.exint12())
            .field("exint13", &self.exint13())
            .field("exint14", &self.exint14())
            .field("exint15", &self.exint15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT12 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint12(&mut self) -> EXINT12_W<EXINTC4_SPEC> {
        EXINT12_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT13 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint13(&mut self) -> EXINT13_W<EXINTC4_SPEC> {
        EXINT13_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT14 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint14(&mut self) -> EXINT14_W<EXINTC4_SPEC> {
        EXINT14_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT15 source"]
    #[inline(always)]
    #[must_use]
    pub fn exint15(&mut self) -> EXINT15_W<EXINTC4_SPEC> {
        EXINT15_W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC4_SPEC;
impl crate::RegisterSpec for EXINTC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc4::R`](R) reader structure"]
impl crate::Readable for EXINTC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc4::W`](W) writer structure"]
impl crate::Writable for EXINTC4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXINTC4 to value 0"]
impl crate::Resettable for EXINTC4_SPEC {
    const RESET_VALUE: u32 = 0;
}
