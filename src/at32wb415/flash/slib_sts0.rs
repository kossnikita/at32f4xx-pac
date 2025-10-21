#[doc = "Register `SLIB_STS0` reader"]
pub type R = crate::R<SLIB_STS0_SPEC>;
#[doc = "Field `BTM_AP_ENF` reader - Boot memory store application code enabled flag"]
pub type BTM_AP_ENF_R = crate::BitReader;
#[doc = "Field `EM_SLIB_ENF` reader - Extension memory sLib enabled flag"]
pub type EM_SLIB_ENF_R = crate::BitReader;
#[doc = "Field `SLIB_ENF` reader - sLib enabled flag"]
pub type SLIB_ENF_R = crate::BitReader;
#[doc = "Field `EM_SLIB_DAT_SS` reader - Extension memory sLib data start sector"]
pub type EM_SLIB_DAT_SS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Boot memory store application code enabled flag"]
    #[inline(always)]
    pub fn btm_ap_enf(&self) -> BTM_AP_ENF_R {
        BTM_AP_ENF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Extension memory sLib enabled flag"]
    #[inline(always)]
    pub fn em_slib_enf(&self) -> EM_SLIB_ENF_R {
        EM_SLIB_ENF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sLib enabled flag"]
    #[inline(always)]
    pub fn slib_enf(&self) -> SLIB_ENF_R {
        SLIB_ENF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Extension memory sLib data start sector"]
    #[inline(always)]
    pub fn em_slib_dat_ss(&self) -> EM_SLIB_DAT_SS_R {
        EM_SLIB_DAT_SS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS0")
            .field("btm_ap_enf", &self.btm_ap_enf())
            .field("em_slib_enf", &self.em_slib_enf())
            .field("slib_enf", &self.slib_enf())
            .field("em_slib_dat_ss", &self.em_slib_dat_ss())
            .finish()
    }
}
#[doc = "sLib status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_sts0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_STS0_SPEC;
impl crate::RegisterSpec for SLIB_STS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts0::R`](R) reader structure"]
impl crate::Readable for SLIB_STS0_SPEC {}
#[doc = "`reset()` method sets SLIB_STS0 to value 0"]
impl crate::Resettable for SLIB_STS0_SPEC {}
