#[doc = "Register `EXTIC1` reader"]
pub type R = crate::R<EXTIC1_SPEC>;
#[doc = "Register `EXTIC1` writer"]
pub type W = crate::W<EXTIC1_SPEC>;
#[doc = "Field `EXTINT0` reader - EXTINT 0 configuration bits"]
pub type EXTINT0_R = crate::FieldReader;
#[doc = "Field `EXTINT0` writer - EXTINT 0 configuration bits"]
pub type EXTINT0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT1` reader - EXTINT 1 configuration bits"]
pub type EXTINT1_R = crate::FieldReader;
#[doc = "Field `EXTINT1` writer - EXTINT 1 configuration bits"]
pub type EXTINT1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT2` reader - EXTINT 2 configuration bits"]
pub type EXTINT2_R = crate::FieldReader;
#[doc = "Field `EXTINT2` writer - EXTINT 2 configuration bits"]
pub type EXTINT2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT3` reader - EXTINT 3 configuration bits"]
pub type EXTINT3_R = crate::FieldReader;
#[doc = "Field `EXTINT3` writer - EXTINT 3 configuration bits"]
pub type EXTINT3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTINT 0 configuration bits"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTINT 1 configuration bits"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTINT 2 configuration bits"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTINT 3 configuration bits"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIC1")
            .field("extint3", &self.extint3())
            .field("extint2", &self.extint2())
            .field("extint1", &self.extint1())
            .field("extint0", &self.extint0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTINT 0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> EXTINT0_W<EXTIC1_SPEC> {
        EXTINT0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTINT 1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> EXTINT1_W<EXTIC1_SPEC> {
        EXTINT1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTINT 2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> EXTINT2_W<EXTIC1_SPEC> {
        EXTINT2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTINT 3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> EXTINT3_W<EXTIC1_SPEC> {
        EXTINT3_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`extic1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extic1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIC1_SPEC;
impl crate::RegisterSpec for EXTIC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extic1::R`](R) reader structure"]
impl crate::Readable for EXTIC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extic1::W`](W) writer structure"]
impl crate::Writable for EXTIC1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIC1 to value 0"]
impl crate::Resettable for EXTIC1_SPEC {
    const RESET_VALUE: u32 = 0;
}
