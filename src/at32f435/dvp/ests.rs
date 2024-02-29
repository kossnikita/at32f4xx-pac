#[doc = "Register `ESTS` reader"]
pub type R = crate::R<ESTS_SPEC>;
#[doc = "Field `CFDES` reader - Capture frame done event status"]
pub type CFDES_R = crate::BitReader;
#[doc = "Field `OVRES` reader - Data FIFO overrun event status"]
pub type OVRES_R = crate::BitReader;
#[doc = "Field `ESEES` reader - Embedded synchronization error event status"]
pub type ESEES_R = crate::BitReader;
#[doc = "Field `VSES` reader - Vertical synchronization event status"]
pub type VSES_R = crate::BitReader;
#[doc = "Field `HSES` reader - Horizontal synchronization event status"]
pub type HSES_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Capture frame done event status"]
    #[inline(always)]
    pub fn cfdes(&self) -> CFDES_R {
        CFDES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun event status"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded synchronization error event status"]
    #[inline(always)]
    pub fn esees(&self) -> ESEES_R {
        ESEES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical synchronization event status"]
    #[inline(always)]
    pub fn vses(&self) -> VSES_R {
        VSES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Horizontal synchronization event status"]
    #[inline(always)]
    pub fn hses(&self) -> HSES_R {
        HSES_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESTS")
            .field("hses", &format_args!("{}", self.hses().bit()))
            .field("vses", &format_args!("{}", self.vses().bit()))
            .field("esees", &format_args!("{}", self.esees().bit()))
            .field("ovres", &format_args!("{}", self.ovres().bit()))
            .field("cfdes", &format_args!("{}", self.cfdes().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ESTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Event status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESTS_SPEC;
impl crate::RegisterSpec for ESTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ests::R`](R) reader structure"]
impl crate::Readable for ESTS_SPEC {}
#[doc = "`reset()` method sets ESTS to value 0"]
impl crate::Resettable for ESTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
