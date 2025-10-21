#[doc = "Register `LDOOV` reader"]
pub type R = crate::R<LDOOV_SPEC>;
#[doc = "Register `LDOOV` writer"]
pub type W = crate::W<LDOOV_SPEC>;
#[doc = "Field `LDOOVSEL` reader - LDO output voltage select"]
pub type LDOOVSEL_R = crate::FieldReader;
#[doc = "Field `LDOOVSEL` writer - LDO output voltage select"]
pub type LDOOVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldoovsel(&self) -> LDOOVSEL_R {
        LDOOVSEL_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDOOV")
            .field("ldoovsel", &self.ldoovsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldoovsel(&mut self) -> LDOOVSEL_W<'_, LDOOV_SPEC> {
        LDOOVSEL_W::new(self, 0)
    }
}
#[doc = "LDO output voltage register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoov::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoov::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDOOV_SPEC;
impl crate::RegisterSpec for LDOOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoov::R`](R) reader structure"]
impl crate::Readable for LDOOV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldoov::W`](W) writer structure"]
impl crate::Writable for LDOOV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDOOV to value 0"]
impl crate::Resettable for LDOOV_SPEC {}
