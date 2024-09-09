#[doc = "Register `EXTIC2` reader"]
pub type R = crate::R<EXTIC2_SPEC>;
#[doc = "Register `EXTIC2` writer"]
pub type W = crate::W<EXTIC2_SPEC>;
#[doc = "Field `EXTINT4` reader - EXTINT 4 configuration bits"]
pub type EXTINT4_R = crate::FieldReader;
#[doc = "Field `EXTINT4` writer - EXTINT 4 configuration bits"]
pub type EXTINT4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT5` reader - EXTINT 5 configuration bits"]
pub type EXTINT5_R = crate::FieldReader;
#[doc = "Field `EXTINT5` writer - EXTINT 5 configuration bits"]
pub type EXTINT5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT6` reader - EXTINT 6 configuration bits"]
pub type EXTINT6_R = crate::FieldReader;
#[doc = "Field `EXTINT6` writer - EXTINT 6 configuration bits"]
pub type EXTINT6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT7` reader - EXTINT 7 configuration bits"]
pub type EXTINT7_R = crate::FieldReader;
#[doc = "Field `EXTINT7` writer - EXTINT 7 configuration bits"]
pub type EXTINT7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTINT 4 configuration bits"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTINT 5 configuration bits"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTINT 6 configuration bits"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTINT 7 configuration bits"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIC2")
            .field("extint7", &self.extint7())
            .field("extint6", &self.extint6())
            .field("extint5", &self.extint5())
            .field("extint4", &self.extint4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTINT 4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint4(&mut self) -> EXTINT4_W<EXTIC2_SPEC> {
        EXTINT4_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTINT 5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint5(&mut self) -> EXTINT5_W<EXTIC2_SPEC> {
        EXTINT5_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTINT 6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint6(&mut self) -> EXTINT6_W<EXTIC2_SPEC> {
        EXTINT6_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTINT 7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint7(&mut self) -> EXTINT7_W<EXTIC2_SPEC> {
        EXTINT7_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`extic2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extic2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIC2_SPEC;
impl crate::RegisterSpec for EXTIC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extic2::R`](R) reader structure"]
impl crate::Readable for EXTIC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extic2::W`](W) writer structure"]
impl crate::Writable for EXTIC2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIC2 to value 0"]
impl crate::Resettable for EXTIC2_SPEC {
    const RESET_VALUE: u32 = 0;
}
