#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DIEPMSK_SPEC>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DIEPMSK_SPEC>;
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
#[doc = "Field `TIMEOUTMSK` reader - Timeout condition mask (Non-isochronous endpoints)"]
pub type TIMEOUTMSK_R = crate::BitReader;
#[doc = "Field `TIMEOUTMSK` writer - Timeout condition mask (Non-isochronous endpoints)"]
pub type TIMEOUTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTKNTXFEMPMSK` reader - IN token received when TxFIFO empty mask"]
pub type INTKNTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMPMSK` writer - IN token received when TxFIFO empty mask"]
pub type INTKNTXFEMPMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTKNEPTMISMSK` reader - IN token received with EP mismatch mask"]
pub type INTKNEPTMISMSK_R = crate::BitReader;
#[doc = "Field `INTKNEPTMISMSK` writer - IN token received with EP mismatch mask"]
pub type INTKNEPTMISMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEPTNAKMSK` reader - IN endpoint NAK effective mask"]
pub type INEPTNAKMSK_R = crate::BitReader;
#[doc = "Field `INEPTNAKMSK` writer - IN endpoint NAK effective mask"]
pub type INEPTNAKMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFIFOUDRMSK` reader - FIFO underrun mask"]
pub type TXFIFOUDRMSK_R = crate::BitReader;
#[doc = "Field `TXFIFOUDRMSK` writer - FIFO underrun mask"]
pub type TXFIFOUDRMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BNAINMSK` reader - BNA interrupt mask"]
pub type BNAINMSK_R = crate::BitReader;
#[doc = "Field `BNAINMSK` writer - BNA interrupt mask"]
pub type BNAINMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKMSK` reader - NAK interrupt mask"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK interrupt mask"]
pub type NAKMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn intkneptmismsk(&self) -> INTKNEPTMISMSK_R {
        INTKNEPTMISMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn ineptnakmsk(&self) -> INEPTNAKMSK_R {
        INEPTNAKMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfifoudrmsk(&self) -> TXFIFOUDRMSK_R {
        TXFIFOUDRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnainmsk(&self) -> BNAINMSK_R {
        BNAINMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field("xfercmsk", &format_args!("{}", self.xfercmsk().bit()))
            .field("eptdismsk", &format_args!("{}", self.eptdismsk().bit()))
            .field("ahberrmsk", &format_args!("{}", self.ahberrmsk().bit()))
            .field("timeoutmsk", &format_args!("{}", self.timeoutmsk().bit()))
            .field(
                "intkntxfempmsk",
                &format_args!("{}", self.intkntxfempmsk().bit()),
            )
            .field(
                "intkneptmismsk",
                &format_args!("{}", self.intkneptmismsk().bit()),
            )
            .field("ineptnakmsk", &format_args!("{}", self.ineptnakmsk().bit()))
            .field(
                "txfifoudrmsk",
                &format_args!("{}", self.txfifoudrmsk().bit()),
            )
            .field("bnainmsk", &format_args!("{}", self.bnainmsk().bit()))
            .field("nakmsk", &format_args!("{}", self.nakmsk().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XFERCMSK_W<DIEPMSK_SPEC, 0> {
        XFERCMSK_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eptdismsk(&mut self) -> EPTDISMSK_W<DIEPMSK_SPEC, 1> {
        EPTDISMSK_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error mask"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<DIEPMSK_SPEC, 2> {
        AHBERRMSK_W::new(self)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutmsk(&mut self) -> TIMEOUTMSK_W<DIEPMSK_SPEC, 3> {
        TIMEOUTMSK_W::new(self)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfempmsk(&mut self) -> INTKNTXFEMPMSK_W<DIEPMSK_SPEC, 4> {
        INTKNTXFEMPMSK_W::new(self)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    #[must_use]
    pub fn intkneptmismsk(&mut self) -> INTKNEPTMISMSK_W<DIEPMSK_SPEC, 5> {
        INTKNEPTMISMSK_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn ineptnakmsk(&mut self) -> INEPTNAKMSK_W<DIEPMSK_SPEC, 6> {
        INEPTNAKMSK_W::new(self)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoudrmsk(&mut self) -> TXFIFOUDRMSK_W<DIEPMSK_SPEC, 8> {
        TXFIFOUDRMSK_W::new(self)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnainmsk(&mut self) -> BNAINMSK_W<DIEPMSK_SPEC, 9> {
        BNAINMSK_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<DIEPMSK_SPEC, 13> {
        NAKMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGHS device IN endpoint common interrupt mask register (OTGHS_DIEPMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
