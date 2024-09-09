#[doc = "Register `S7_STRIDE` reader"]
pub type R = crate::R<S7_STRIDE_SPEC>;
#[doc = "Register `S7_STRIDE` writer"]
pub type W = crate::W<S7_STRIDE_SPEC>;
#[doc = "Field `SRCSTD` reader - Source stride"]
pub type SRCSTD_R = crate::FieldReader<u16>;
#[doc = "Field `SRCSTD` writer - Source stride"]
pub type SRCSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DSTSTD` reader - Destination stride"]
pub type DSTSTD_R = crate::FieldReader<u16>;
#[doc = "Field `DSTSTD` writer - Destination stride"]
pub type DSTSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source stride"]
    #[inline(always)]
    pub fn srcstd(&self) -> SRCSTD_R {
        SRCSTD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Destination stride"]
    #[inline(always)]
    pub fn dststd(&self) -> DSTSTD_R {
        DSTSTD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S7_STRIDE")
            .field("srcstd", &self.srcstd())
            .field("dststd", &self.dststd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Source stride"]
    #[inline(always)]
    #[must_use]
    pub fn srcstd(&mut self) -> SRCSTD_W<S7_STRIDE_SPEC> {
        SRCSTD_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Destination stride"]
    #[inline(always)]
    #[must_use]
    pub fn dststd(&mut self) -> DSTSTD_W<S7_STRIDE_SPEC> {
        DSTSTD_W::new(self, 16)
    }
}
#[doc = "Stream 7 2D Transfer Stride\n\nYou can [`read`](crate::Reg::read) this register and get [`s7_stride::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7_stride::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7_STRIDE_SPEC;
impl crate::RegisterSpec for S7_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7_stride::R`](R) reader structure"]
impl crate::Readable for S7_STRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s7_stride::W`](W) writer structure"]
impl crate::Writable for S7_STRIDE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S7_STRIDE to value 0"]
impl crate::Resettable for S7_STRIDE_SPEC {
    const RESET_VALUE: u32 = 0;
}
