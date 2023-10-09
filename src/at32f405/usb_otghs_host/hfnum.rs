#[doc = "Register `HFNUM` reader"]
pub type R = crate::R<HFNUM_SPEC>;
#[doc = "Field `FRNUM` reader - Frame number"]
pub type FRNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FTREM` reader - Frame time remaining"]
pub type FTREM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFNUM")
            .field("frnum", &format_args!("{}", self.frnum().bits()))
            .field("ftrem", &format_args!("{}", self.ftrem().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HFNUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "OTGHS host frame number/frame time remaining register (OTGHS_HFNUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfnum::R`](R) reader structure"]
impl crate::Readable for HFNUM_SPEC {}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
