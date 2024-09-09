#[doc = "Register `S3LLP` reader"]
pub type R = crate::R<S3LLP_SPEC>;
#[doc = "Register `S3LLP` writer"]
pub type W = crate::W<S3LLP_SPEC>;
#[doc = "Field `LLP` reader - Link list pointer"]
pub type LLP_R = crate::FieldReader<u32>;
#[doc = "Field `LLP` writer - Link list pointer"]
pub type LLP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    pub fn llp(&self) -> LLP_R {
        LLP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S3LLP").field("llp", &self.llp()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    #[must_use]
    pub fn llp(&mut self) -> LLP_W<S3LLP_SPEC> {
        LLP_W::new(self, 0)
    }
}
#[doc = "Stream 3 Link List Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`s3llp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3llp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3LLP_SPEC;
impl crate::RegisterSpec for S3LLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3llp::R`](R) reader structure"]
impl crate::Readable for S3LLP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s3llp::W`](W) writer structure"]
impl crate::Writable for S3LLP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S3LLP to value 0"]
impl crate::Resettable for S3LLP_SPEC {
    const RESET_VALUE: u32 = 0;
}
