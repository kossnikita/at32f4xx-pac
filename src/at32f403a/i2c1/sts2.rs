#[doc = "Register `STS2` reader"]
pub type R = crate::R<STS2_SPEC>;
#[doc = "Field `TRMODE` reader - Transmission mode"]
pub type TRMODE_R = crate::BitReader;
#[doc = "Field `BUSYF` reader - Bus busy"]
pub type BUSYF_R = crate::BitReader;
#[doc = "Field `DIRF` reader - Transmission direction"]
pub type DIRF_R = crate::BitReader;
#[doc = "Field `GCADDRF` reader - General call address reception"]
pub type GCADDRF_R = crate::BitReader;
#[doc = "Field `DEVADDRF` reader - SMBus device address receiving"]
pub type DEVADDRF_R = crate::BitReader;
#[doc = "Field `HOSTADDRF` reader - SMBus host address receiving"]
pub type HOSTADDRF_R = crate::BitReader;
#[doc = "Field `ADDR2F` reader - Received address 2"]
pub type ADDR2F_R = crate::BitReader;
#[doc = "Field `PECVAL` reader - PEC value"]
pub type PECVAL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmission mode"]
    #[inline(always)]
    pub fn trmode(&self) -> TRMODE_R {
        TRMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission direction"]
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address reception"]
    #[inline(always)]
    pub fn gcaddrf(&self) -> GCADDRF_R {
        GCADDRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBus device address receiving"]
    #[inline(always)]
    pub fn devaddrf(&self) -> DEVADDRF_R {
        DEVADDRF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus host address receiving"]
    #[inline(always)]
    pub fn hostaddrf(&self) -> HOSTADDRF_R {
        HOSTADDRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received address 2"]
    #[inline(always)]
    pub fn addr2f(&self) -> ADDR2F_R {
        ADDR2F_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PEC value"]
    #[inline(always)]
    pub fn pecval(&self) -> PECVAL_R {
        PECVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS2")
            .field("pecval", &format_args!("{}", self.pecval().bits()))
            .field("addr2f", &format_args!("{}", self.addr2f().bit()))
            .field("hostaddrf", &format_args!("{}", self.hostaddrf().bit()))
            .field("devaddrf", &format_args!("{}", self.devaddrf().bit()))
            .field("gcaddrf", &format_args!("{}", self.gcaddrf().bit()))
            .field("dirf", &format_args!("{}", self.dirf().bit()))
            .field("busyf", &format_args!("{}", self.busyf().bit()))
            .field("trmode", &format_args!("{}", self.trmode().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS2_SPEC;
impl crate::RegisterSpec for STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for STS2_SPEC {}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for STS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
