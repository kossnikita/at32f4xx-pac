#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Division value update complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIVF_A {
    #[doc = "0: Division value update complete"]
    Complete = 0,
    #[doc = "1: Division value update is in process"]
    InProgress = 1,
}
impl From<DIVF_A> for bool {
    #[inline(always)]
    fn from(variant: DIVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVF` reader - Division value update complete flag"]
pub type DIVF_R = crate::BitReader<DIVF_A>;
impl DIVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVF_A {
        match self.bits {
            false => DIVF_A::Complete,
            true => DIVF_A::InProgress,
        }
    }
    #[doc = "Division value update complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == DIVF_A::Complete
    }
    #[doc = "Division value update is in process"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == DIVF_A::InProgress
    }
}
#[doc = "Reload value update complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDF_A {
    #[doc = "0: Reload value update complete"]
    Complete = 0,
    #[doc = "1: Reload value update is in process"]
    InProgress = 1,
}
impl From<RLDF_A> for bool {
    #[inline(always)]
    fn from(variant: RLDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDF` reader - Reload value update complete flag"]
pub type RLDF_R = crate::BitReader<RLDF_A>;
impl RLDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RLDF_A {
        match self.bits {
            false => RLDF_A::Complete,
            true => RLDF_A::InProgress,
        }
    }
    #[doc = "Reload value update complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == RLDF_A::Complete
    }
    #[doc = "Reload value update is in process"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RLDF_A::InProgress
    }
}
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("divf", &self.divf())
            .field("rldf", &self.rldf())
            .finish()
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
