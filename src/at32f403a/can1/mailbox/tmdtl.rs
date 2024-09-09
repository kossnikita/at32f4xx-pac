#[doc = "Register `TMDTL` reader"]
pub type R = crate::R<TMDTL_SPEC>;
#[doc = "Register `TMDTL` writer"]
pub type W = crate::W<TMDTL_SPEC>;
#[doc = "Field `TMDT(0-3)` reader - Transmit mailbox data byte %s"]
pub type TMDT_R = crate::FieldReader;
#[doc = "Field `TMDT(0-3)` writer - Transmit mailbox data byte %s"]
pub type TMDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Transmit mailbox data byte (0-3)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TMDT0` field.</div>"]
    #[inline(always)]
    pub fn tmdt(&self, n: u8) -> TMDT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TMDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit mailbox data byte (0-3)"]
    #[inline(always)]
    pub fn tmdt_iter(&self) -> impl Iterator<Item = TMDT_R> + '_ {
        (0..4).map(move |n| TMDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    pub fn tmdt0(&self) -> TMDT_R {
        TMDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    pub fn tmdt1(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    pub fn tmdt2(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    pub fn tmdt3(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMDTL")
            .field("tmdt0", &self.tmdt0())
            .field("tmdt1", &self.tmdt1())
            .field("tmdt2", &self.tmdt2())
            .field("tmdt3", &self.tmdt3())
            .finish()
    }
}
impl W {
    #[doc = "Transmit mailbox data byte (0-3)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TMDT0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt(&mut self, n: u8) -> TMDT_W<TMDTL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TMDT_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt0(&mut self) -> TMDT_W<TMDTL_SPEC> {
        TMDT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt1(&mut self) -> TMDT_W<TMDTL_SPEC> {
        TMDT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt2(&mut self) -> TMDT_W<TMDTL_SPEC> {
        TMDT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt3(&mut self) -> TMDT_W<TMDTL_SPEC> {
        TMDT_W::new(self, 24)
    }
}
#[doc = "Transmit mailbox data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmdtl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmdtl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDTL_SPEC;
impl crate::RegisterSpec for TMDTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdtl::R`](R) reader structure"]
impl crate::Readable for TMDTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdtl::W`](W) writer structure"]
impl crate::Writable for TMDTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDTL to value 0"]
impl crate::Resettable for TMDTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
