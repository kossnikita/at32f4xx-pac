#[doc = "Register `RFDTH1` reader"]
pub type R = crate::R<RFDTH1_SPEC>;
#[doc = "Field `RFDT4` reader - Receive FIFO data byte 4"]
pub type RFDT4_R = crate::FieldReader;
#[doc = "Field `RFDT5` reader - Receive FIFO data byte 5"]
pub type RFDT5_R = crate::FieldReader;
#[doc = "Field `RFDT6` reader - Receive FIFO data byte 6"]
pub type RFDT6_R = crate::FieldReader;
#[doc = "Field `RFDT7` reader - Receive FIFO data byte 7"]
pub type RFDT7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO data byte 4"]
    #[inline(always)]
    pub fn rfdt4(&self) -> RFDT4_R {
        RFDT4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO data byte 5"]
    #[inline(always)]
    pub fn rfdt5(&self) -> RFDT5_R {
        RFDT5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive FIFO data byte 6"]
    #[inline(always)]
    pub fn rfdt6(&self) -> RFDT6_R {
        RFDT6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO data byte 7"]
    #[inline(always)]
    pub fn rfdt7(&self) -> RFDT7_R {
        RFDT7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receive FIFO 1 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFDTH1_SPEC;
impl crate::RegisterSpec for RFDTH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdth1::R`](R) reader structure"]
impl crate::Readable for RFDTH1_SPEC {}
#[doc = "`reset()` method sets RFDTH1 to value 0"]
impl crate::Resettable for RFDTH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
