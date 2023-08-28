#[doc = "Register `DOEPEACHMSK1` reader"]
pub type R = crate::R<DOEPEACHMSK1_SPEC>;
#[doc = "Register `DOEPEACHMSK1` writer"]
pub type W = crate::W<DOEPEACHMSK1_SPEC>;
#[doc = "Field `XFERCMSK` reader - Transfer completed interrupt mask"]
pub type XFERCMSK_R = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed interrupt mask"]
pub type XFERCMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTDISMSK` reader - Endpoint disabled interrupt mask"]
pub type EPTDISMSK_R = crate::BitReader;
#[doc = "Field `EPTDISMSK` writer - Endpoint disabled interrupt mask"]
pub type EPTDISMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHBERRMSK` reader - AHB Error mask"]
pub type AHBERRMSK_R = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - AHB Error mask"]
pub type AHBERRMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETUPMSK` reader - SETUP phase done mask"]
pub type SETUPMSK_R = crate::BitReader;
#[doc = "Field `SETUPMSK` writer - SETUP phase done mask"]
pub type SETUPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTTEPDMSK` reader - OUT token received when endpoint disabled mask"]
pub type OUTTEPDMSK_R = crate::BitReader;
#[doc = "Field `OUTTEPDMSK` writer - OUT token received when endpoint disabled mask"]
pub type OUTTEPDMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STSPHSERCVDMSK` reader - Status Phase Received mask"]
pub type STSPHSERCVDMSK_R = crate::BitReader;
#[doc = "Field `STSPHSERCVDMSK` writer - Status Phase Received mask"]
pub type STSPHSERCVDMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `B2BSETUPMSK` reader - Back-to-back SETUP packets received mask"]
pub type B2BSETUPMSK_R = crate::BitReader;
#[doc = "Field `B2BSETUPMSK` writer - Back-to-back SETUP packets received mask"]
pub type B2BSETUPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTPERRMSK` reader - OUT packet error mask"]
pub type OUTPERRMSK_R = crate::BitReader;
#[doc = "Field `OUTPERRMSK` writer - OUT packet error mask"]
pub type OUTPERRMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BNAOUTMSK` reader - BNA interrupt mask"]
pub type BNAOUTMSK_R = crate::BitReader;
#[doc = "Field `BNAOUTMSK` writer - BNA interrupt mask"]
pub type BNAOUTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBLEERRMSK` reader - Babble Error interrupt mask"]
pub type BBLEERRMSK_R = crate::BitReader;
#[doc = "Field `BBLEERRMSK` writer - Babble Error interrupt mask"]
pub type BBLEERRMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKMSK` reader - NAK interrupt mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK interrupt mask"]
pub type NAKMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYETMSK` reader - NYET interrupt mask"]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMSK` writer - NYET interrupt mask"]
pub type NYETMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfercmsk(&self) -> XFERCMSK_R {
        XFERCMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn eptdismsk(&self) -> EPTDISMSK_R {
        EPTDISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn setupmsk(&self) -> SETUPMSK_R {
        SETUPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn outtepdmsk(&self) -> OUTTEPDMSK_R {
        OUTTEPDMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received mask"]
    #[inline(always)]
    pub fn stsphsercvdmsk(&self) -> STSPHSERCVDMSK_R {
        STSPHSERCVDMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bsetupmsk(&self) -> B2BSETUPMSK_R {
        B2BSETUPMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn outperrmsk(&self) -> OUTPERRMSK_R {
        OUTPERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnaoutmsk(&self) -> BNAOUTMSK_R {
        BNAOUTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Error interrupt mask"]
    #[inline(always)]
    pub fn bbleerrmsk(&self) -> BBLEERRMSK_R {
        BBLEERRMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XFERCMSK_W<DOEPEACHMSK1_SPEC, 0> {
        XFERCMSK_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eptdismsk(&mut self) -> EPTDISMSK_W<DOEPEACHMSK1_SPEC, 1> {
        EPTDISMSK_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<DOEPEACHMSK1_SPEC, 2> {
        AHBERRMSK_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    #[must_use]
    pub fn setupmsk(&mut self) -> SETUPMSK_W<DOEPEACHMSK1_SPEC, 3> {
        SETUPMSK_W::new(self)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    #[must_use]
    pub fn outtepdmsk(&mut self) -> OUTTEPDMSK_W<DOEPEACHMSK1_SPEC, 4> {
        OUTTEPDMSK_W::new(self)
    }
    #[doc = "Bit 5 - Status Phase Received mask"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvdmsk(&mut self) -> STSPHSERCVDMSK_W<DOEPEACHMSK1_SPEC, 5> {
        STSPHSERCVDMSK_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    #[must_use]
    pub fn b2bsetupmsk(&mut self) -> B2BSETUPMSK_W<DOEPEACHMSK1_SPEC, 6> {
        B2BSETUPMSK_W::new(self)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    #[must_use]
    pub fn outperrmsk(&mut self) -> OUTPERRMSK_W<DOEPEACHMSK1_SPEC, 8> {
        OUTPERRMSK_W::new(self)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnaoutmsk(&mut self) -> BNAOUTMSK_W<DOEPEACHMSK1_SPEC, 9> {
        BNAOUTMSK_W::new(self)
    }
    #[doc = "Bit 12 - Babble Error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerrmsk(&mut self) -> BBLEERRMSK_W<DOEPEACHMSK1_SPEC, 12> {
        BBLEERRMSK_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<DOEPEACHMSK1_SPEC, 13> {
        NAKMSK_W::new(self)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<DOEPEACHMSK1_SPEC, 14> {
        NYETMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Each OUT Endpoint 1 Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepeachmsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepeachmsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPEACHMSK1_SPEC;
impl crate::RegisterSpec for DOEPEACHMSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepeachmsk1::R`](R) reader structure"]
impl crate::Readable for DOEPEACHMSK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepeachmsk1::W`](W) writer structure"]
impl crate::Writable for DOEPEACHMSK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPEACHMSK1 to value 0"]
impl crate::Resettable for DOEPEACHMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
