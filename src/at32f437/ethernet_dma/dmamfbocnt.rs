#[doc = "Register `DMAMFBOCNT` reader"]
pub type R = crate::R<DMAMFBOCNT_SPEC>;
#[doc = "Field `MFC` reader - Missed frames control"]
pub type MFC_R = crate::FieldReader<u16>;
#[doc = "Field `OBMFC` reader - Overflow bit for missed frame counter"]
pub type OBMFC_R = crate::BitReader;
#[doc = "Field `OFC` reader - Overflow frame counter"]
pub type OFC_R = crate::FieldReader<u16>;
#[doc = "Field `OBFOC` reader - Overflow bit for FIFO overflow counter"]
pub type OBFOC_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Missed frames control"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn obmfc(&self) -> OBMFC_R {
        OBMFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Overflow frame counter"]
    #[inline(always)]
    pub fn ofc(&self) -> OFC_R {
        OFC_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn obfoc(&self) -> OBFOC_R {
        OBFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamfbocnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMFBOCNT_SPEC;
impl crate::RegisterSpec for DMAMFBOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamfbocnt::R`](R) reader structure"]
impl crate::Readable for DMAMFBOCNT_SPEC {}
#[doc = "`reset()` method sets DMAMFBOCNT to value 0"]
impl crate::Resettable for DMAMFBOCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
