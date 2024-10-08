#[doc = "Register `CVAL` reader"]
pub type R = crate::R<CVAL_SPEC>;
#[doc = "Register `CVAL` writer"]
pub type W = crate::W<CVAL_SPEC>;
#[doc = "Field `CVAL` reader - Counter value"]
pub type CVAL_R = crate::FieldReader<u32>;
#[doc = "Field `CVAL` writer - Counter value"]
pub type CVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter value"]
    #[inline(always)]
    pub fn cval(&self) -> CVAL_R {
        CVAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CVAL").field("cval", &self.cval()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cval(&mut self) -> CVAL_W<CVAL_SPEC> {
        CVAL_W::new(self, 0)
    }
}
#[doc = "Counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`cval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CVAL_SPEC;
impl crate::RegisterSpec for CVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cval::R`](R) reader structure"]
impl crate::Readable for CVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cval::W`](W) writer structure"]
impl crate::Writable for CVAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CVAL to value 0"]
impl crate::Resettable for CVAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
