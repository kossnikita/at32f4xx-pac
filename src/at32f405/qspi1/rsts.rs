#[doc = "Register `RSTS` reader"]
pub type R = crate::R<RSTS_SPEC>;
#[doc = "Field `SPISTS` reader - SPI read status"]
pub type SPISTS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SPI read status"]
    #[inline(always)]
    pub fn spists(&self) -> SPISTS_R {
        SPISTS_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTS")
            .field("spists", &self.spists())
            .finish()
    }
}
#[doc = "SPI read status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTS_SPEC;
impl crate::RegisterSpec for RSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsts::R`](R) reader structure"]
impl crate::Readable for RSTS_SPEC {}
#[doc = "`reset()` method sets RSTS to value 0"]
impl crate::Resettable for RSTS_SPEC {}
