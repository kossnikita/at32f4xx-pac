#[doc = "Register `CMDSTS` reader"]
pub type R = crate::R<CMDSTS_SPEC>;
#[doc = "Field `CMDSTS` reader - Command complete status"]
pub type CMDSTS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command complete status"]
    #[inline(always)]
    pub fn cmdsts(&self) -> CMDSTS_R {
        CMDSTS_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDSTS")
            .field("cmdsts", &self.cmdsts())
            .finish()
    }
}
#[doc = "CMD status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDSTS_SPEC;
impl crate::RegisterSpec for CMDSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdsts::R`](R) reader structure"]
impl crate::Readable for CMDSTS_SPEC {}
#[doc = "`reset()` method sets CMDSTS to value 0"]
impl crate::Resettable for CMDSTS_SPEC {}
