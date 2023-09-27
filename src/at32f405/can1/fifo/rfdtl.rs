#[doc = "Register `RFDTL` reader"]
pub type R = crate::R<RFDTL_SPEC>;
#[doc = "Field `RFDT[0-3]` reader - Receive FIFO data byte %s"]
pub type RFDT_R = crate::FieldReader;
impl R {
    #[doc = "Receive FIFO data byte [0-3]"]
    #[inline(always)]
    pub unsafe fn rfdt(&self, n: u8) -> RFDT_R {
        RFDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Receive FIFO data byte 0"]
    #[inline(always)]
    pub fn rfdt0(&self) -> RFDT_R {
        RFDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO data byte 1"]
    #[inline(always)]
    pub fn rfdt1(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive FIFO data byte 2"]
    #[inline(always)]
    pub fn rfdt2(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO data byte 3"]
    #[inline(always)]
    pub fn rfdt3(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receive FIFO mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFDTL_SPEC;
impl crate::RegisterSpec for RFDTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdtl::R`](R) reader structure"]
impl crate::Readable for RFDTL_SPEC {}
#[doc = "`reset()` method sets RFDTL to value 0"]
impl crate::Resettable for RFDTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
