#[doc = "Register `EXTIC3` reader"]
pub type R = crate::R<EXTIC3_SPEC>;
#[doc = "Register `EXTIC3` writer"]
pub type W = crate::W<EXTIC3_SPEC>;
#[doc = "Field `EXTINT8` reader - EXTINT 8 configuration bits"]
pub type EXTINT8_R = crate::FieldReader;
#[doc = "Field `EXTINT8` writer - EXTINT 8 configuration bits"]
pub type EXTINT8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT9` reader - EXTINT 9 configuration bits"]
pub type EXTINT9_R = crate::FieldReader;
#[doc = "Field `EXTINT9` writer - EXTINT 9 configuration bits"]
pub type EXTINT9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT10` reader - EXTINT 10 configuration bits"]
pub type EXTINT10_R = crate::FieldReader;
#[doc = "Field `EXTINT10` writer - EXTINT 10 configuration bits"]
pub type EXTINT10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTINT11` reader - EXTINT 11 configuration bits"]
pub type EXTINT11_R = crate::FieldReader;
#[doc = "Field `EXTINT11` writer - EXTINT 11 configuration bits"]
pub type EXTINT11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTINT 8 configuration bits"]
    #[inline(always)]
    pub fn extint8(&self) -> EXTINT8_R {
        EXTINT8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTINT 9 configuration bits"]
    #[inline(always)]
    pub fn extint9(&self) -> EXTINT9_R {
        EXTINT9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTINT 10 configuration bits"]
    #[inline(always)]
    pub fn extint10(&self) -> EXTINT10_R {
        EXTINT10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTINT 11 configuration bits"]
    #[inline(always)]
    pub fn extint11(&self) -> EXTINT11_R {
        EXTINT11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTIC3")
            .field("extint11", &self.extint11())
            .field("extint10", &self.extint10())
            .field("extint9", &self.extint9())
            .field("extint8", &self.extint8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTINT 8 configuration bits"]
    #[inline(always)]
    pub fn extint8(&mut self) -> EXTINT8_W<'_, EXTIC3_SPEC> {
        EXTINT8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTINT 9 configuration bits"]
    #[inline(always)]
    pub fn extint9(&mut self) -> EXTINT9_W<'_, EXTIC3_SPEC> {
        EXTINT9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTINT 10 configuration bits"]
    #[inline(always)]
    pub fn extint10(&mut self) -> EXTINT10_W<'_, EXTIC3_SPEC> {
        EXTINT10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTINT 11 configuration bits"]
    #[inline(always)]
    pub fn extint11(&mut self) -> EXTINT11_W<'_, EXTIC3_SPEC> {
        EXTINT11_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`extic3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extic3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIC3_SPEC;
impl crate::RegisterSpec for EXTIC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extic3::R`](R) reader structure"]
impl crate::Readable for EXTIC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extic3::W`](W) writer structure"]
impl crate::Writable for EXTIC3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIC3 to value 0"]
impl crate::Resettable for EXTIC3_SPEC {}
