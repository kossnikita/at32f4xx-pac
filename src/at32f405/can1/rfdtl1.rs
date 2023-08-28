#[doc = "Register `RFDTL1` reader"]
pub type R = crate::R<RFDTL1_SPEC>;
#[doc = "Field `RFDT0` reader - Receive FIFO data byte 0"]
pub type RFDT0_R = crate::FieldReader;
#[doc = "Field `RFDT1` reader - Receive FIFO data byte 1"]
pub type RFDT1_R = crate::FieldReader;
#[doc = "Field `RFDT2` reader - Receive FIFO data byte 2"]
pub type RFDT2_R = crate::FieldReader;
#[doc = "Field `RFDT3` reader - Receive FIFO data byte 3"]
pub type RFDT3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO data byte 0"]
    #[inline(always)]
    pub fn rfdt0(&self) -> RFDT0_R {
        RFDT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO data byte 1"]
    #[inline(always)]
    pub fn rfdt1(&self) -> RFDT1_R {
        RFDT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive FIFO data byte 2"]
    #[inline(always)]
    pub fn rfdt2(&self) -> RFDT2_R {
        RFDT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO data byte 3"]
    #[inline(always)]
    pub fn rfdt3(&self) -> RFDT3_R {
        RFDT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receive FIFO 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFDTL1_SPEC;
impl crate::RegisterSpec for RFDTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdtl1::R`](R) reader structure"]
impl crate::Readable for RFDTL1_SPEC {}
#[doc = "`reset()` method sets RFDTL1 to value 0"]
impl crate::Resettable for RFDTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
