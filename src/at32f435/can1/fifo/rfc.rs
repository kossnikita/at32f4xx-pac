#[doc = "Register `RFC` reader"]
pub type R = crate::R<RFC_SPEC>;
#[doc = "Field `DTL` reader - Receive FIFO data length"]
pub type DTL_R = crate::FieldReader;
#[doc = "Field `FMN` reader - Receive FIFO filter match number"]
pub type FMN_R = crate::FieldReader;
#[doc = "Field `TS` reader - Receive FIFO time stamp"]
pub type TS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Receive FIFO data length"]
    #[inline(always)]
    pub fn dtl(&self) -> DTL_R {
        DTL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO filter match number"]
    #[inline(always)]
    pub fn fmn(&self) -> FMN_R {
        FMN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Receive FIFO time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFC")
            .field("ts", &self.ts())
            .field("fmn", &self.fmn())
            .field("dtl", &self.dtl())
            .finish()
    }
}
#[doc = "Receive FIFO mailbox data length and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFC_SPEC;
impl crate::RegisterSpec for RFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfc::R`](R) reader structure"]
impl crate::Readable for RFC_SPEC {}
#[doc = "`reset()` method sets RFC to value 0"]
impl crate::Resettable for RFC_SPEC {}
