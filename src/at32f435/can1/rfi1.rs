#[doc = "Register `RFI1` reader"]
pub type R = crate::R<RFI1_SPEC>;
#[doc = "Field `RFFRI` reader - Receive FIFO frame type indication"]
pub type RFFRI_R = crate::BitReader;
#[doc = "Field `RFIDI` reader - Receive FIFO identifier type indication"]
pub type RFIDI_R = crate::BitReader;
#[doc = "Field `RFEID` reader - Receive FIFO extended identifier"]
pub type RFEID_R = crate::FieldReader<u32>;
#[doc = "Field `RFSID` reader - Receive FIFO standard identifier or receive FIFO extended identifier"]
pub type RFSID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - Receive FIFO frame type indication"]
    #[inline(always)]
    pub fn rffri(&self) -> RFFRI_R {
        RFFRI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO identifier type indication"]
    #[inline(always)]
    pub fn rfidi(&self) -> RFIDI_R {
        RFIDI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - Receive FIFO extended identifier"]
    #[inline(always)]
    pub fn rfeid(&self) -> RFEID_R {
        RFEID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Receive FIFO standard identifier or receive FIFO extended identifier"]
    #[inline(always)]
    pub fn rfsid(&self) -> RFSID_R {
        RFSID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "Receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfi1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFI1_SPEC;
impl crate::RegisterSpec for RFI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfi1::R`](R) reader structure"]
impl crate::Readable for RFI1_SPEC {}
#[doc = "`reset()` method sets RFI1 to value 0"]
impl crate::Resettable for RFI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
