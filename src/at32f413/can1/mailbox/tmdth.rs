#[doc = "Register `TMDTH` reader"]
pub type R = crate::R<TMDTH_SPEC>;
#[doc = "Register `TMDTH` writer"]
pub type W = crate::W<TMDTH_SPEC>;
#[doc = "Field `TMDT(4-7)` reader - Transmit mailbox data byte %s"]
pub type TMDT_R = crate::FieldReader;
#[doc = "Field `TMDT(4-7)` writer - Transmit mailbox data byte %s"]
pub type TMDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Transmit mailbox data byte (4-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TMDT4` field.</div>"]
    #[inline(always)]
    pub fn tmdt(&self, n: u8) -> TMDT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TMDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox data byte (4-7)"]
    #[inline(always)]
    pub fn tmdt_iter(&self) -> impl Iterator<Item = TMDT_R> + '_ {
        (0..4).map(move |n| TMDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt4(&self) -> TMDT_R {
        TMDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 5"]
    #[inline(always)]
    pub fn tmdt5(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 6"]
    #[inline(always)]
    pub fn tmdt6(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 7"]
    #[inline(always)]
    pub fn tmdt7(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMDTH")
            .field("tmdt4", &self.tmdt4())
            .field("tmdt5", &self.tmdt5())
            .field("tmdt6", &self.tmdt6())
            .field("tmdt7", &self.tmdt7())
            .finish()
    }
}
impl W {
    #[doc = "Transmit mailbox data byte (4-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TMDT4` field.</div>"]
    #[inline(always)]
    pub fn tmdt(&mut self, n: u8) -> TMDT_W<'_, TMDTH_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TMDT_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt4(&mut self) -> TMDT_W<'_, TMDTH_SPEC> {
        TMDT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 5"]
    #[inline(always)]
    pub fn tmdt5(&mut self) -> TMDT_W<'_, TMDTH_SPEC> {
        TMDT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 6"]
    #[inline(always)]
    pub fn tmdt6(&mut self) -> TMDT_W<'_, TMDTH_SPEC> {
        TMDT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 7"]
    #[inline(always)]
    pub fn tmdt7(&mut self) -> TMDT_W<'_, TMDTH_SPEC> {
        TMDT_W::new(self, 24)
    }
}
#[doc = "Transmit mailbox data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmdth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmdth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDTH_SPEC;
impl crate::RegisterSpec for TMDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdth::R`](R) reader structure"]
impl crate::Readable for TMDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdth::W`](W) writer structure"]
impl crate::Writable for TMDTH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMDTH to value 0"]
impl crate::Resettable for TMDTH_SPEC {}
