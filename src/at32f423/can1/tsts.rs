#[doc = "Register `TSTS` reader"]
pub type R = crate::R<TSTS_SPEC>;
#[doc = "Register `TSTS` writer"]
pub type W = crate::W<TSTS_SPEC>;
#[doc = "Field `TM0TCF` reader - Transmit mailbox 0 transmission complete flag"]
pub type TM0TCF_R = crate::BitReader;
#[doc = "Field `TM0TCF` writer - Transmit mailbox 0 transmission complete flag"]
pub type TM0TCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM0TSF` reader - Transmit mailbox 0 transmission success flag"]
pub type TM0TSF_R = crate::BitReader;
#[doc = "Field `TM0TSF` writer - Transmit mailbox 0 transmission success flag"]
pub type TM0TSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM0ALF` reader - Transmit mailbox 0 arbitration lost flag"]
pub type TM0ALF_R = crate::BitReader;
#[doc = "Field `TM0ALF` writer - Transmit mailbox 0 arbitration lost flag"]
pub type TM0ALF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM0TEF` reader - Transmit mailbox 0 transmission error flag"]
pub type TM0TEF_R = crate::BitReader;
#[doc = "Field `TM0TEF` writer - Transmit mailbox 0 transmission error flag"]
pub type TM0TEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM0CT` reader - Transmit mailbox 0 cancel transmission"]
pub type TM0CT_R = crate::BitReader;
#[doc = "Field `TM0CT` writer - Transmit mailbox 0 cancel transmission"]
pub type TM0CT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM1TCF` reader - Transmit mailbox 1 transmission complete flag"]
pub type TM1TCF_R = crate::BitReader;
#[doc = "Field `TM1TCF` writer - Transmit mailbox 1 transmission complete flag"]
pub type TM1TCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM1TSF` reader - Transmit mailbox 1 transmission success flag"]
pub type TM1TSF_R = crate::BitReader;
#[doc = "Field `TM1TSF` writer - Transmit mailbox 1 transmission success flag"]
pub type TM1TSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM1ALF` reader - Transmit mailbox 1 arbitration lost flag"]
pub type TM1ALF_R = crate::BitReader;
#[doc = "Field `TM1ALF` writer - Transmit mailbox 1 arbitration lost flag"]
pub type TM1ALF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM1TEF` reader - Transmit mailbox 1 transmission error flag"]
pub type TM1TEF_R = crate::BitReader;
#[doc = "Field `TM1TEF` writer - Transmit mailbox 1 transmission error flag"]
pub type TM1TEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM1CT` reader - Transmit mailbox 1 cancel transmission"]
pub type TM1CT_R = crate::BitReader;
#[doc = "Field `TM1CT` writer - Transmit mailbox 1 cancel transmission"]
pub type TM1CT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM2TCF` reader - transmit mailbox 2 transmission complete flag"]
pub type TM2TCF_R = crate::BitReader;
#[doc = "Field `TM2TCF` writer - transmit mailbox 2 transmission complete flag"]
pub type TM2TCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM2TSF` reader - Transmit mailbox 2 transmission success flag"]
pub type TM2TSF_R = crate::BitReader;
#[doc = "Field `TM2TSF` writer - Transmit mailbox 2 transmission success flag"]
pub type TM2TSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM2ALF` reader - Transmit mailbox 2 arbitration lost flag"]
pub type TM2ALF_R = crate::BitReader;
#[doc = "Field `TM2ALF` writer - Transmit mailbox 2 arbitration lost flag"]
pub type TM2ALF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM2TEF` reader - Transmit mailbox 2 transmission error flag"]
pub type TM2TEF_R = crate::BitReader;
#[doc = "Field `TM2TEF` writer - Transmit mailbox 2 transmission error flag"]
pub type TM2TEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TM2CT` reader - Transmit mailbox 2 cancel transmission"]
pub type TM2CT_R = crate::BitReader;
#[doc = "Field `TM2CT` writer - Transmit mailbox 2 cancel transmission"]
pub type TM2CT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMNR` reader - Transmit Mailbox number record"]
pub type TMNR_R = crate::FieldReader;
#[doc = "Field `TM0EF` reader - Transmit mailbox 0 empty flag"]
pub type TM0EF_R = crate::BitReader;
#[doc = "Field `TM1EF` reader - Transmit mailbox 1 empty flag"]
pub type TM1EF_R = crate::BitReader;
#[doc = "Field `TM2EF` reader - Transmit mailbox 2 empty flag"]
pub type TM2EF_R = crate::BitReader;
#[doc = "Field `TM0LPF` reader - Transmit mailbox 0 lowest priority flag"]
pub type TM0LPF_R = crate::BitReader;
#[doc = "Field `TM1LPF` reader - Transmit mailbox 1 lowest priority flag"]
pub type TM1LPF_R = crate::BitReader;
#[doc = "Field `TM2LPF` reader - Transmit mailbox 2 lowest priority flag"]
pub type TM2LPF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit mailbox 0 transmission complete flag"]
    #[inline(always)]
    pub fn tm0tcf(&self) -> TM0TCF_R {
        TM0TCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mailbox 0 transmission success flag"]
    #[inline(always)]
    pub fn tm0tsf(&self) -> TM0TSF_R {
        TM0TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit mailbox 0 arbitration lost flag"]
    #[inline(always)]
    pub fn tm0alf(&self) -> TM0ALF_R {
        TM0ALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit mailbox 0 transmission error flag"]
    #[inline(always)]
    pub fn tm0tef(&self) -> TM0TEF_R {
        TM0TEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit mailbox 0 cancel transmission"]
    #[inline(always)]
    pub fn tm0ct(&self) -> TM0CT_R {
        TM0CT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit mailbox 1 transmission complete flag"]
    #[inline(always)]
    pub fn tm1tcf(&self) -> TM1TCF_R {
        TM1TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit mailbox 1 transmission success flag"]
    #[inline(always)]
    pub fn tm1tsf(&self) -> TM1TSF_R {
        TM1TSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit mailbox 1 arbitration lost flag"]
    #[inline(always)]
    pub fn tm1alf(&self) -> TM1ALF_R {
        TM1ALF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit mailbox 1 transmission error flag"]
    #[inline(always)]
    pub fn tm1tef(&self) -> TM1TEF_R {
        TM1TEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit mailbox 1 cancel transmission"]
    #[inline(always)]
    pub fn tm1ct(&self) -> TM1CT_R {
        TM1CT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - transmit mailbox 2 transmission complete flag"]
    #[inline(always)]
    pub fn tm2tcf(&self) -> TM2TCF_R {
        TM2TCF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit mailbox 2 transmission success flag"]
    #[inline(always)]
    pub fn tm2tsf(&self) -> TM2TSF_R {
        TM2TSF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit mailbox 2 arbitration lost flag"]
    #[inline(always)]
    pub fn tm2alf(&self) -> TM2ALF_R {
        TM2ALF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit mailbox 2 transmission error flag"]
    #[inline(always)]
    pub fn tm2tef(&self) -> TM2TEF_R {
        TM2TEF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit mailbox 2 cancel transmission"]
    #[inline(always)]
    pub fn tm2ct(&self) -> TM2CT_R {
        TM2CT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transmit Mailbox number record"]
    #[inline(always)]
    pub fn tmnr(&self) -> TMNR_R {
        TMNR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty flag"]
    #[inline(always)]
    pub fn tm0ef(&self) -> TM0EF_R {
        TM0EF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty flag"]
    #[inline(always)]
    pub fn tm1ef(&self) -> TM1EF_R {
        TM1EF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty flag"]
    #[inline(always)]
    pub fn tm2ef(&self) -> TM2EF_R {
        TM2EF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 lowest priority flag"]
    #[inline(always)]
    pub fn tm0lpf(&self) -> TM0LPF_R {
        TM0LPF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 lowest priority flag"]
    #[inline(always)]
    pub fn tm1lpf(&self) -> TM1LPF_R {
        TM1LPF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit mailbox 2 lowest priority flag"]
    #[inline(always)]
    pub fn tm2lpf(&self) -> TM2LPF_R {
        TM2LPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox 0 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tcf(&mut self) -> TM0TCF_W<TSTS_SPEC, 0> {
        TM0TCF_W::new(self)
    }
    #[doc = "Bit 1 - Transmit mailbox 0 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tsf(&mut self) -> TM0TSF_W<TSTS_SPEC, 1> {
        TM0TSF_W::new(self)
    }
    #[doc = "Bit 2 - Transmit mailbox 0 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0alf(&mut self) -> TM0ALF_W<TSTS_SPEC, 2> {
        TM0ALF_W::new(self)
    }
    #[doc = "Bit 3 - Transmit mailbox 0 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm0tef(&mut self) -> TM0TEF_W<TSTS_SPEC, 3> {
        TM0TEF_W::new(self)
    }
    #[doc = "Bit 7 - Transmit mailbox 0 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm0ct(&mut self) -> TM0CT_W<TSTS_SPEC, 7> {
        TM0CT_W::new(self)
    }
    #[doc = "Bit 8 - Transmit mailbox 1 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tcf(&mut self) -> TM1TCF_W<TSTS_SPEC, 8> {
        TM1TCF_W::new(self)
    }
    #[doc = "Bit 9 - Transmit mailbox 1 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tsf(&mut self) -> TM1TSF_W<TSTS_SPEC, 9> {
        TM1TSF_W::new(self)
    }
    #[doc = "Bit 10 - Transmit mailbox 1 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1alf(&mut self) -> TM1ALF_W<TSTS_SPEC, 10> {
        TM1ALF_W::new(self)
    }
    #[doc = "Bit 11 - Transmit mailbox 1 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm1tef(&mut self) -> TM1TEF_W<TSTS_SPEC, 11> {
        TM1TEF_W::new(self)
    }
    #[doc = "Bit 15 - Transmit mailbox 1 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm1ct(&mut self) -> TM1CT_W<TSTS_SPEC, 15> {
        TM1CT_W::new(self)
    }
    #[doc = "Bit 16 - transmit mailbox 2 transmission complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tcf(&mut self) -> TM2TCF_W<TSTS_SPEC, 16> {
        TM2TCF_W::new(self)
    }
    #[doc = "Bit 17 - Transmit mailbox 2 transmission success flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tsf(&mut self) -> TM2TSF_W<TSTS_SPEC, 17> {
        TM2TSF_W::new(self)
    }
    #[doc = "Bit 18 - Transmit mailbox 2 arbitration lost flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2alf(&mut self) -> TM2ALF_W<TSTS_SPEC, 18> {
        TM2ALF_W::new(self)
    }
    #[doc = "Bit 19 - Transmit mailbox 2 transmission error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tm2tef(&mut self) -> TM2TEF_W<TSTS_SPEC, 19> {
        TM2TEF_W::new(self)
    }
    #[doc = "Bit 23 - Transmit mailbox 2 cancel transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tm2ct(&mut self) -> TM2CT_W<TSTS_SPEC, 23> {
        TM2CT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSTS_SPEC;
impl crate::RegisterSpec for TSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsts::R`](R) reader structure"]
impl crate::Readable for TSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsts::W`](W) writer structure"]
impl crate::Writable for TSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSTS to value 0x1c00_0000"]
impl crate::Resettable for TSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c00_0000;
}
