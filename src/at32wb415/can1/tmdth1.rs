#[doc = "Register `TMDTH1` reader"]
pub type R = crate::R<TMDTH1_SPEC>;
#[doc = "Register `TMDTH1` writer"]
pub type W = crate::W<TMDTH1_SPEC>;
#[doc = "Field `TMDT4` reader - Transmit mailbox data byte 4"]
pub type TMDT4_R = crate::FieldReader;
#[doc = "Field `TMDT4` writer - Transmit mailbox data byte 4"]
pub type TMDT4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TMDT5` reader - Transmit mailbox data byte 5"]
pub type TMDT5_R = crate::FieldReader;
#[doc = "Field `TMDT5` writer - Transmit mailbox data byte 5"]
pub type TMDT5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TMDT6` reader - Transmit mailbox data byte 6"]
pub type TMDT6_R = crate::FieldReader;
#[doc = "Field `TMDT6` writer - Transmit mailbox data byte 6"]
pub type TMDT6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TMDT7` reader - Transmit mailbox data byte 7"]
pub type TMDT7_R = crate::FieldReader;
#[doc = "Field `TMDT7` writer - Transmit mailbox data byte 7"]
pub type TMDT7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt4(&self) -> TMDT4_R {
        TMDT4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 5"]
    #[inline(always)]
    pub fn tmdt5(&self) -> TMDT5_R {
        TMDT5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 6"]
    #[inline(always)]
    pub fn tmdt6(&self) -> TMDT6_R {
        TMDT6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 7"]
    #[inline(always)]
    pub fn tmdt7(&self) -> TMDT7_R {
        TMDT7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt4(&mut self) -> TMDT4_W<TMDTH1_SPEC, 0> {
        TMDT4_W::new(self)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 5"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt5(&mut self) -> TMDT5_W<TMDTH1_SPEC, 8> {
        TMDT5_W::new(self)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 6"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt6(&mut self) -> TMDT6_W<TMDTH1_SPEC, 16> {
        TMDT6_W::new(self)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 7"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt7(&mut self) -> TMDT7_W<TMDTH1_SPEC, 24> {
        TMDT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit mailbox 1 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDTH1_SPEC;
impl crate::RegisterSpec for TMDTH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdth1::R`](R) reader structure"]
impl crate::Readable for TMDTH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdth1::W`](W) writer structure"]
impl crate::Writable for TMDTH1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMDTH1 to value 0"]
impl crate::Resettable for TMDTH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
