#[doc = "Register `RFDTH` reader"]
pub type R = crate::R<RFDTH_SPEC>;
#[doc = "Field `RFDT(4-7)` reader - Receive FIFO data byte %s"]
pub type RFDT_R = crate::FieldReader;
impl R {
    #[doc = "Receive FIFO data byte (4-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RFDT4` field.</div>"]
    #[inline(always)]
    pub fn rfdt(&self, n: u8) -> RFDT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RFDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Receive FIFO data byte (4-7)"]
    #[inline(always)]
    pub fn rfdt_iter(&self) -> impl Iterator<Item = RFDT_R> + '_ {
        (0..4).map(move |n| RFDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Receive FIFO data byte 4"]
    #[inline(always)]
    pub fn rfdt4(&self) -> RFDT_R {
        RFDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO data byte 5"]
    #[inline(always)]
    pub fn rfdt5(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive FIFO data byte 6"]
    #[inline(always)]
    pub fn rfdt6(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO data byte 7"]
    #[inline(always)]
    pub fn rfdt7(&self) -> RFDT_R {
        RFDT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFDTH")
            .field("rfdt4", &self.rfdt4())
            .field("rfdt5", &self.rfdt5())
            .field("rfdt6", &self.rfdt6())
            .field("rfdt7", &self.rfdt7())
            .finish()
    }
}
#[doc = "Receive FIFO mailbox data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfdth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFDTH_SPEC;
impl crate::RegisterSpec for RFDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdth::R`](R) reader structure"]
impl crate::Readable for RFDTH_SPEC {}
#[doc = "`reset()` method sets RFDTH to value 0"]
impl crate::Resettable for RFDTH_SPEC {}
