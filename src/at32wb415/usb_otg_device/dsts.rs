#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DSTS_SPEC>;
#[doc = "Field `SUSPSTS` reader - Suspend status"]
pub type SUSPSTS_R = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - Enumerated speed"]
pub type ENUMSPD_R = crate::FieldReader;
#[doc = "Field `ETICERR` reader - Erratic error"]
pub type ETICERR_R = crate::BitReader;
#[doc = "Field `SOFFN` reader - Frame number of the received SOF"]
pub type SOFFN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Suspend status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic error"]
    #[inline(always)]
    pub fn eticerr(&self) -> ETICERR_R {
        ETICERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTS")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("eticerr", &self.eticerr())
            .field("soffn", &self.soffn())
            .finish()
    }
}
#[doc = "OTGFS device status register (OTGFS_DSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DSTS_SPEC {}
#[doc = "`reset()` method sets DSTS to value 0x10"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
