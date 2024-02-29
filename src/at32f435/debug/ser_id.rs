#[doc = "Register `SER_ID` reader"]
pub type R = crate::R<SER_ID_SPEC>;
#[doc = "Field `REV_ID` reader - version ID"]
pub type REV_ID_R = crate::FieldReader;
#[doc = "Field `SER_ID` reader - series ID"]
pub type SER_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - version ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - series ID"]
    #[inline(always)]
    pub fn ser_id(&self) -> SER_ID_R {
        SER_ID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SER_ID")
            .field("rev_id", &format_args!("{}", self.rev_id().bits()))
            .field("ser_id", &format_args!("{}", self.ser_id().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SER_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SERIES ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ser_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SER_ID_SPEC;
impl crate::RegisterSpec for SER_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser_id::R`](R) reader structure"]
impl crate::Readable for SER_ID_SPEC {}
#[doc = "`reset()` method sets SER_ID to value 0"]
impl crate::Resettable for SER_ID_SPEC {
    const RESET_VALUE: u32 = 0;
}
