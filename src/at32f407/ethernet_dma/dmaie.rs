#[doc = "Register `DMAIE` reader"]
pub type R = crate::R<DMAIE_SPEC>;
#[doc = "Register `DMAIE` writer"]
pub type W = crate::W<DMAIE_SPEC>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSE` reader - Transmit stopped enable"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit stopped enable"]
pub type TSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TUE` reader - Transmit buffer unavailable enable"]
pub type TUE_R = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit buffer unavailable enable"]
pub type TUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TJE` reader - Transmit jabber timeout enable"]
pub type TJE_R = crate::BitReader;
#[doc = "Field `TJE` writer - Transmit jabber timeout enable"]
pub type TJE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVE` reader - Overflow interrupt enable"]
pub type OVE_R = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow interrupt enable"]
pub type OVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UNE` reader - Underflow interrupt enable"]
pub type UNE_R = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow interrupt enable"]
pub type UNE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBUE` reader - Receive buffer unavailable enable"]
pub type RBUE_R = crate::BitReader;
#[doc = "Field `RBUE` writer - Receive buffer unavailable enable"]
pub type RBUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSE` reader - Receive stopped enable"]
pub type RSE_R = crate::BitReader;
#[doc = "Field `RSE` writer - Receive stopped enable"]
pub type RSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWTE` reader - receive watchdog timeout enable"]
pub type RWTE_R = crate::BitReader;
#[doc = "Field `RWTE` writer - receive watchdog timeout enable"]
pub type RWTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIE` reader - Early transmit interrupt enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Early transmit interrupt enable"]
pub type EIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBEE` reader - Fatal bus error enable"]
pub type FBEE_R = crate::BitReader;
#[doc = "Field `FBEE` writer - Fatal bus error enable"]
pub type FBEE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERE` reader - Early receive interrupt enable"]
pub type ERE_R = crate::BitReader;
#[doc = "Field `ERE` writer - Early receive interrupt enable"]
pub type ERE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AIE` reader - Abnormal interrupt enable"]
pub type AIE_R = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal interrupt enable"]
pub type AIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NIE` reader - Normal interrupt enable"]
pub type NIE_R = crate::BitReader;
#[doc = "Field `NIE` writer - Normal interrupt enable"]
pub type NIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit stopped enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable"]
    #[inline(always)]
    pub fn tue(&self) -> TUE_R {
        TUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable"]
    #[inline(always)]
    pub fn tje(&self) -> TJE_R {
        TJE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ove(&self) -> OVE_R {
        OVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn une(&self) -> UNE_R {
        UNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable"]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive stopped enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive watchdog timeout enable"]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error enable"]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn ere(&self) -> ERE_R {
        ERE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<DMAIE_SPEC, 0> {
        TIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit stopped enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<DMAIE_SPEC, 1> {
        TSE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable"]
    #[inline(always)]
    #[must_use]
    pub fn tue(&mut self) -> TUE_W<DMAIE_SPEC, 2> {
        TUE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn tje(&mut self) -> TJE_W<DMAIE_SPEC, 3> {
        TJE_W::new(self)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OVE_W<DMAIE_SPEC, 4> {
        OVE_W::new(self)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn une(&mut self) -> UNE_W<DMAIE_SPEC, 5> {
        UNE_W::new(self)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<DMAIE_SPEC, 6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbue(&mut self) -> RBUE_W<DMAIE_SPEC, 7> {
        RBUE_W::new(self)
    }
    #[doc = "Bit 8 - Receive stopped enable"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<DMAIE_SPEC, 8> {
        RSE_W::new(self)
    }
    #[doc = "Bit 9 - receive watchdog timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwte(&mut self) -> RWTE_W<DMAIE_SPEC, 9> {
        RWTE_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<DMAIE_SPEC, 10> {
        EIE_W::new(self)
    }
    #[doc = "Bit 13 - Fatal bus error enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbee(&mut self) -> FBEE_W<DMAIE_SPEC, 13> {
        FBEE_W::new(self)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ere(&mut self) -> ERE_W<DMAIE_SPEC, 14> {
        ERE_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<DMAIE_SPEC, 15> {
        AIE_W::new(self)
    }
    #[doc = "Bit 16 - Normal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<DMAIE_SPEC, 16> {
        NIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAIE_SPEC;
impl crate::RegisterSpec for DMAIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaie::R`](R) reader structure"]
impl crate::Readable for DMAIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaie::W`](W) writer structure"]
impl crate::Writable for DMAIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAIE to value 0"]
impl crate::Resettable for DMAIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
