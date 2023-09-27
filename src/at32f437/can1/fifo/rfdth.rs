#[doc = "Register `RFDTH` reader"]
pub type R = crate::R<RFDTH_SPEC>;
#[doc = "Field `RFDT[4-7]` reader - Receive FIFO data byte 4"]
pub type RFDT_R = crate::FieldReader;
impl R {
    #[doc = "Receive FIFO data byte 4"]
    #[inline(always)]
    pub unsafe fn rfdt(&self, n: u8) -> RFDT_R {
        RFDT_R::new(((self.bits >> ((n - 4) * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Receive FIFO data byte 4"]
    #[inline(always)]
    pub fn rfdt4(&self) -> RFDT_R {
        RFDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO data byte 4"]
    #[inline(always)]
    pub fn rfdt5(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive FIFO data byte 4"]
    #[inline(always)]
    pub fn rfdt6(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO data byte 4"]
    #[inline(always)]
    pub fn rfdt7(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receive FIFO mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFDTH_SPEC;
impl crate::RegisterSpec for RFDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdth::R`](R) reader structure"]
impl crate::Readable for RFDTH_SPEC {}
#[doc = "`reset()` method sets RFDTH to value 0"]
impl crate::Resettable for RFDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
