#[doc = "Register `EXINTC2` reader"]
pub type R = crate::R<EXINTC2_SPEC>;
#[doc = "Register `EXINTC2` writer"]
pub type W = crate::W<EXINTC2_SPEC>;
#[doc = "Field `EXINT4` reader - EXINT 4 configuration bits"]
pub type EXINT4_R = crate::FieldReader;
#[doc = "Field `EXINT4` writer - EXINT 4 configuration bits"]
pub type EXINT4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT5` reader - EXINT 5 configuration bits"]
pub type EXINT5_R = crate::FieldReader;
#[doc = "Field `EXINT5` writer - EXINT 5 configuration bits"]
pub type EXINT5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT6` reader - EXINT 6 configuration bits"]
pub type EXINT6_R = crate::FieldReader;
#[doc = "Field `EXINT6` writer - EXINT 6 configuration bits"]
pub type EXINT6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT7` reader - EXINT 7 configuration bits"]
pub type EXINT7_R = crate::FieldReader;
#[doc = "Field `EXINT7` writer - EXINT 7 configuration bits"]
pub type EXINT7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXINT 4 configuration bits"]
    #[inline(always)]
    pub fn exint4(&self) -> EXINT4_R {
        EXINT4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXINT 5 configuration bits"]
    #[inline(always)]
    pub fn exint5(&self) -> EXINT5_R {
        EXINT5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXINT 6 configuration bits"]
    #[inline(always)]
    pub fn exint6(&self) -> EXINT6_R {
        EXINT6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXINT 7 configuration bits"]
    #[inline(always)]
    pub fn exint7(&self) -> EXINT7_R {
        EXINT7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC2")
            .field("exint7", &self.exint7())
            .field("exint6", &self.exint6())
            .field("exint5", &self.exint5())
            .field("exint4", &self.exint4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - EXINT 4 configuration bits"]
    #[inline(always)]
    pub fn exint4(&mut self) -> EXINT4_W<'_, EXINTC2_SPEC> {
        EXINT4_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXINT 5 configuration bits"]
    #[inline(always)]
    pub fn exint5(&mut self) -> EXINT5_W<'_, EXINTC2_SPEC> {
        EXINT5_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXINT 6 configuration bits"]
    #[inline(always)]
    pub fn exint6(&mut self) -> EXINT6_W<'_, EXINTC2_SPEC> {
        EXINT6_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXINT 7 configuration bits"]
    #[inline(always)]
    pub fn exint7(&mut self) -> EXINT7_W<'_, EXINTC2_SPEC> {
        EXINT7_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC2_SPEC;
impl crate::RegisterSpec for EXINTC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc2::R`](R) reader structure"]
impl crate::Readable for EXINTC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc2::W`](W) writer structure"]
impl crate::Writable for EXINTC2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC2 to value 0"]
impl crate::Resettable for EXINTC2_SPEC {}
