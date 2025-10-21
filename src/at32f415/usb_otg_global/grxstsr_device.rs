#[doc = "Register `GRXSTSR_Device` reader"]
pub type R = crate::R<GRXSTSR_DEVICE_SPEC>;
#[doc = "Field `EPTNUM` reader - Endpoint number"]
pub type EPTNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PKTSTS_R = crate::FieldReader;
#[doc = "Field `FN` reader - Frame number"]
pub type FN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn eptnum(&self) -> EPTNUM_R {
        EPTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSR_Device")
            .field("eptnum", &self.eptnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("fn_", &self.fn_())
            .finish()
    }
}
#[doc = "OTGFS Receive status debug read(Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr_device::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXSTSR_DEVICE_SPEC;
impl crate::RegisterSpec for GRXSTSR_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsr_device::R`](R) reader structure"]
impl crate::Readable for GRXSTSR_DEVICE_SPEC {}
#[doc = "`reset()` method sets GRXSTSR_Device to value 0"]
impl crate::Resettable for GRXSTSR_DEVICE_SPEC {}
