#[doc = "Register `RFC1` reader"]
pub type R = crate::R<RFC1_SPEC>;
#[doc = "Field `RFDTL` reader - Receive FIFO data length"]
pub type RFDTL_R = crate::FieldReader;
#[doc = "Field `RFFMN` reader - Receive FIFO filter match number"]
pub type RFFMN_R = crate::FieldReader;
#[doc = "Field `RFTS` reader - Receive FIFO time stamp"]
pub type RFTS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Receive FIFO data length"]
    #[inline(always)]
    pub fn rfdtl(&self) -> RFDTL_R {
        RFDTL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO filter match number"]
    #[inline(always)]
    pub fn rffmn(&self) -> RFFMN_R {
        RFFMN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Receive FIFO time stamp"]
    #[inline(always)]
    pub fn rfts(&self) -> RFTS_R {
        RFTS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO 1 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFC1_SPEC;
impl crate::RegisterSpec for RFC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfc1::R`](R) reader structure"]
impl crate::Readable for RFC1_SPEC {}
#[doc = "`reset()` method sets RFC1 to value 0"]
impl crate::Resettable for RFC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
