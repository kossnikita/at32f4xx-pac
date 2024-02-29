#[doc = "Register `TCRC` reader"]
pub type R = crate::R<TCRC_SPEC>;
#[doc = "Field `TCRC` reader - Transmit CRC"]
pub type TCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit CRC"]
    #[inline(always)]
    pub fn tcrc(&self) -> TCRC_R {
        TCRC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCRC")
            .field("tcrc", &format_args!("{}", self.tcrc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TCRC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Transmit CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCRC_SPEC;
impl crate::RegisterSpec for TCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcrc::R`](R) reader structure"]
impl crate::Readable for TCRC_SPEC {}
#[doc = "`reset()` method sets TCRC to value 0"]
impl crate::Resettable for TCRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
