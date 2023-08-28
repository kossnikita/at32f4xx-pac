#[doc = "Register `TMDTL1` reader"]
pub type R = crate::R<TMDTL1_SPEC>;
#[doc = "Register `TMDTL1` writer"]
pub type W = crate::W<TMDTL1_SPEC>;
#[doc = "Field `TMDT0` reader - Transmit mailbox data byte 0"]
pub type TMDT0_R = crate::FieldReader;
#[doc = "Field `TMDT0` writer - Transmit mailbox data byte 0"]
pub type TMDT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TMDT1` reader - Transmit mailbox data byte 1"]
pub type TMDT1_R = crate::FieldReader;
#[doc = "Field `TMDT1` writer - Transmit mailbox data byte 1"]
pub type TMDT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TMDT2` reader - Transmit mailbox data byte 2"]
pub type TMDT2_R = crate::FieldReader;
#[doc = "Field `TMDT2` writer - Transmit mailbox data byte 2"]
pub type TMDT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TMDT3` reader - Transmit mailbox data byte 3"]
pub type TMDT3_R = crate::FieldReader;
#[doc = "Field `TMDT3` writer - Transmit mailbox data byte 3"]
pub type TMDT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    pub fn tmdt0(&self) -> TMDT0_R {
        TMDT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    pub fn tmdt1(&self) -> TMDT1_R {
        TMDT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    pub fn tmdt2(&self) -> TMDT2_R {
        TMDT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    pub fn tmdt3(&self) -> TMDT3_R {
        TMDT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt0(&mut self) -> TMDT0_W<TMDTL1_SPEC, 0> {
        TMDT0_W::new(self)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt1(&mut self) -> TMDT1_W<TMDTL1_SPEC, 8> {
        TMDT1_W::new(self)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt2(&mut self) -> TMDT2_W<TMDTL1_SPEC, 16> {
        TMDT2_W::new(self)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt3(&mut self) -> TMDT3_W<TMDTL1_SPEC, 24> {
        TMDT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit mailbox 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDTL1_SPEC;
impl crate::RegisterSpec for TMDTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdtl1::R`](R) reader structure"]
impl crate::Readable for TMDTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmdtl1::W`](W) writer structure"]
impl crate::Writable for TMDTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMDTL1 to value 0"]
impl crate::Resettable for TMDTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
