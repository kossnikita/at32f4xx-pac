#[doc = "Register `TMDTH` reader"]
pub type R = crate::R<TMDTH_SPEC>;
#[doc = "Register `TMDTH` writer"]
pub type W = crate::W<TMDTH_SPEC>;
#[doc = "Field `TMDT[4-7]` reader - Transmit mailbox data byte 4"]
pub type TMDT_R = crate::FieldReader;
#[doc = "Field `TMDT[4-7]` writer - Transmit mailbox data byte 4"]
pub type TMDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Transmit mailbox data byte 4\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn tmdt(&self, n: u8) -> TMDT_R {
        assert!(n < 4);
        TMDT_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt4(&self) -> TMDT_R {
        TMDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt5(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt6(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt7(&self) -> TMDT_R {
        TMDT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMDTH")
            .field("tmdt4", &format_args!("{}", self.tmdt4().bits()))
            .field("tmdt5", &format_args!("{}", self.tmdt5().bits()))
            .field("tmdt6", &format_args!("{}", self.tmdt6().bits()))
            .field("tmdt7", &format_args!("{}", self.tmdt7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TMDTH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Transmit mailbox data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt(&mut self, n: u8) -> TMDT_W<TMDTH_SPEC> {
        assert!(n < 4);
        TMDT_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt4(&mut self) -> TMDT_W<TMDTH_SPEC> {
        TMDT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt5(&mut self) -> TMDT_W<TMDTH_SPEC> {
        TMDT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt6(&mut self) -> TMDT_W<TMDTH_SPEC> {
        TMDT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt7(&mut self) -> TMDT_W<TMDTH_SPEC> {
        TMDT_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit mailbox data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDTH_SPEC;
impl crate::RegisterSpec for TMDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdth::R`](R) reader structure"]
impl crate::Readable for TMDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdth::W`](W) writer structure"]
impl crate::Writable for TMDTH_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMDTH to value 0"]
impl crate::Resettable for TMDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
