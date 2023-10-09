#[doc = "Register `ISTS` reader"]
pub type R = crate::R<ISTS_SPEC>;
#[doc = "Field `CFDIS` reader - Capture frame done interrupt status"]
pub type CFDIS_R = crate::BitReader;
#[doc = "Field `OVRIS` reader - Data FIFO overrun interrupt status"]
pub type OVRIS_R = crate::BitReader;
#[doc = "Field `ESEIS` reader - Embedded synchronization error interrupt status"]
pub type ESEIS_R = crate::BitReader;
#[doc = "Field `VSIS` reader - Vertical synchronization interrupt status"]
pub type VSIS_R = crate::BitReader;
#[doc = "Field `HSIS` reader - Horizontal synchronization interrupt status"]
pub type HSIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Capture frame done interrupt status"]
    #[inline(always)]
    pub fn cfdis(&self) -> CFDIS_R {
        CFDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt status"]
    #[inline(always)]
    pub fn ovris(&self) -> OVRIS_R {
        OVRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt status"]
    #[inline(always)]
    pub fn eseis(&self) -> ESEIS_R {
        ESEIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt status"]
    #[inline(always)]
    pub fn vsis(&self) -> VSIS_R {
        VSIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt status"]
    #[inline(always)]
    pub fn hsis(&self) -> HSIS_R {
        HSIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTS")
            .field("hsis", &format_args!("{}", self.hsis().bit()))
            .field("vsis", &format_args!("{}", self.vsis().bit()))
            .field("eseis", &format_args!("{}", self.eseis().bit()))
            .field("ovris", &format_args!("{}", self.ovris().bit()))
            .field("cfdis", &format_args!("{}", self.cfdis().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ISTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTS_SPEC;
impl crate::RegisterSpec for ISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for ISTS_SPEC {}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for ISTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
