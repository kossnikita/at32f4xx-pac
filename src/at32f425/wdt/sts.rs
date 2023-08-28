#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Field `DIVF` reader - Division value update complete flag"]
pub type DIVF_R = crate::BitReader;
#[doc = "Field `RLDF` reader - Reload value update complete flag"]
pub type RLDF_R = crate::BitReader;
#[doc = "Field `WINF` reader - Window value update complete flag"]
pub type WINF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Division value update complete flag"]
    #[inline(always)]
    pub fn divf(&self) -> DIVF_R {
        DIVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reload value update complete flag"]
    #[inline(always)]
    pub fn rldf(&self) -> RLDF_R {
        RLDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window value update complete flag"]
    #[inline(always)]
    pub fn winf(&self) -> WINF_R {
        WINF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
