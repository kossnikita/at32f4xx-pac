#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GINTMSK_SPEC>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GINTMSK_SPEC>;
#[doc = "Field `MODEMISMSK` reader - Mode mismatch interrupt mask"]
pub type MODEMISMSK_R = crate::BitReader;
#[doc = "Field `MODEMISMSK` writer - Mode mismatch interrupt mask"]
pub type MODEMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINTMSK` reader - OTG interrupt mask"]
pub type OTGINTMSK_R = crate::BitReader;
#[doc = "Field `OTGINTMSK` writer - OTG interrupt mask"]
pub type OTGINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFMSK` reader - Start of frame mask"]
pub type SOFMSK_R = crate::BitReader;
#[doc = "Field `SOFMSK` writer - Start of frame mask"]
pub type SOFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLMSK` reader - Receive FIFO non-empty mask"]
pub type RXFLVLMSK_R = crate::BitReader;
#[doc = "Field `RXFLVLMSK` writer - Receive FIFO non-empty mask"]
pub type RXFLVLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPMSK` reader - Non-periodic TxFIFO empty mask"]
pub type NPTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `NPTXFEMPMSK` writer - Non-periodic TxFIFO empty mask"]
pub type NPTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNAKEFFMSK` reader - Global non-periodic IN NAK effective mask"]
pub type GINNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GINNAKEFFMSK` writer - Global non-periodic IN NAK effective mask"]
pub type GINNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Global OUT NAK effective mask"]
pub type GOUTNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFFMSK` writer - Global OUT NAK effective mask"]
pub type GOUTNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLYSUSPMSK` reader - Early suspend mask"]
pub type ERLYSUSPMSK_R = crate::BitReader;
#[doc = "Field `ERLYSUSPMSK` writer - Early suspend mask"]
pub type ERLYSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPMSK` reader - USB suspend mask"]
pub type USBSUSPMSK_R = crate::BitReader;
#[doc = "Field `USBSUSPMSK` writer - USB suspend mask"]
pub type USBSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRSTMSK` reader - USB reset mask"]
pub type USBRSTMSK_R = crate::BitReader;
#[doc = "Field `USBRSTMSK` writer - USB reset mask"]
pub type USBRSTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONEMSK` reader - Enumeration done mask"]
pub type ENUMDONEMSK_R = crate::BitReader;
#[doc = "Field `ENUMDONEMSK` writer - Enumeration done mask"]
pub type ENUMDONEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROPMSK` reader - Isochronous OUT packet dropped interrupt mask"]
pub type ISOOUTDROPMSK_R = crate::BitReader;
#[doc = "Field `ISOOUTDROPMSK` writer - Isochronous OUT packet dropped interrupt mask"]
pub type ISOOUTDROPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMSK` reader - End of periodic frame interrupt mask"]
pub type EOPFMSK_R = crate::BitReader;
#[doc = "Field `EOPFMSK` writer - End of periodic frame interrupt mask"]
pub type EOPFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPTINTMSK` reader - IN endpoints interrupt mask"]
pub type IEPTINTMSK_R = crate::BitReader;
#[doc = "Field `IEPTINTMSK` writer - IN endpoints interrupt mask"]
pub type IEPTINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPTINTMSK` reader - OUT endpoints interrupt mask"]
pub type OEPTINTMSK_R = crate::BitReader;
#[doc = "Field `OEPTINTMSK` writer - OUT endpoints interrupt mask"]
pub type OEPTINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMISOINMSK` reader - Incomplete isochronous IN transfer mask"]
pub type INCOMISOINMSK_R = crate::BitReader;
#[doc = "Field `INCOMISOINMSK` writer - Incomplete isochronous IN transfer mask"]
pub type INCOMISOINMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPIP_INCOMPISOOUTMSK` reader - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type INCOMPIP_INCOMPISOOUTMSK_R = crate::BitReader;
#[doc = "Field `INCOMPIP_INCOMPISOOUTMSK` writer - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type INCOMPIP_INCOMPISOOUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTINTMSK` reader - Host port interrupt mask"]
pub type PRTINTMSK_R = crate::BitReader;
#[doc = "Field `PRTINTMSK` writer - Host port interrupt mask"]
pub type PRTINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCHINTMSK` reader - Host channels interrupt mask"]
pub type HCHINTMSK_R = crate::BitReader;
#[doc = "Field `HCHINTMSK` writer - Host channels interrupt mask"]
pub type HCHINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEMPMSK` reader - Periodic TxFIFO empty mask"]
pub type PTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `PTXFEMPMSK` writer - Periodic TxFIFO empty mask"]
pub type PTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONIDSCHGMSK` reader - Connector ID status change mask"]
pub type CONIDSCHGMSK_R = crate::BitReader;
#[doc = "Field `CONIDSCHGMSK` writer - Connector ID status change mask"]
pub type CONIDSCHGMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONINTMSK` reader - Disconnect detected interrupt mask"]
pub type DISCONINTMSK_R = crate::BitReader;
#[doc = "Field `DISCONINTMSK` writer - Disconnect detected interrupt mask"]
pub type DISCONINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINTMSK` reader - Resume/remote wakeup detected interrupt mask"]
pub type WKUPINTMSK_R = crate::BitReader;
#[doc = "Field `WKUPINTMSK` writer - Resume/remote wakeup detected interrupt mask"]
pub type WKUPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgintmsk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NPTXFEMPMSK_R {
        NPTXFEMPMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GOUTNAKEFFMSK_R {
        GOUTNAKEFFMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn ieptintmsk(&self) -> IEPTINTMSK_R {
        IEPTINTMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oeptintmsk(&self) -> OEPTINTMSK_R {
        OEPTINTMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn incomisoinmsk(&self) -> INCOMISOINMSK_R {
        INCOMISOINMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    pub fn incompip_incompisooutmsk(&self) -> INCOMPIP_INCOMPISOOUTMSK_R {
        INCOMPIP_INCOMPISOOUTMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtintmsk(&self) -> PRTINTMSK_R {
        PRTINTMSK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hchintmsk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn conidschgmsk(&self) -> CONIDSCHGMSK_R {
        CONIDSCHGMSK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn disconintmsk(&self) -> DISCONINTMSK_R {
        DISCONINTMSK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK")
            .field("modemismsk", &self.modemismsk())
            .field("otgintmsk", &self.otgintmsk())
            .field("sofmsk", &self.sofmsk())
            .field("rxflvlmsk", &self.rxflvlmsk())
            .field("nptxfempmsk", &self.nptxfempmsk())
            .field("ginnakeffmsk", &self.ginnakeffmsk())
            .field("goutnakeffmsk", &self.goutnakeffmsk())
            .field("erlysuspmsk", &self.erlysuspmsk())
            .field("usbsuspmsk", &self.usbsuspmsk())
            .field("usbrstmsk", &self.usbrstmsk())
            .field("enumdonemsk", &self.enumdonemsk())
            .field("isooutdropmsk", &self.isooutdropmsk())
            .field("eopfmsk", &self.eopfmsk())
            .field("ieptintmsk", &self.ieptintmsk())
            .field("oeptintmsk", &self.oeptintmsk())
            .field("incomisoinmsk", &self.incomisoinmsk())
            .field("incompip_incompisooutmsk", &self.incompip_incompisooutmsk())
            .field("prtintmsk", &self.prtintmsk())
            .field("hchintmsk", &self.hchintmsk())
            .field("ptxfempmsk", &self.ptxfempmsk())
            .field("conidschgmsk", &self.conidschgmsk())
            .field("disconintmsk", &self.disconintmsk())
            .field("wkupintmsk", &self.wkupintmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W<'_, GINTMSK_SPEC> {
        MODEMISMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgintmsk(&mut self) -> OTGINTMSK_W<'_, GINTMSK_SPEC> {
        OTGINTMSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofmsk(&mut self) -> SOFMSK_W<'_, GINTMSK_SPEC> {
        SOFMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&mut self) -> RXFLVLMSK_W<'_, GINTMSK_SPEC> {
        RXFLVLMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfempmsk(&mut self) -> NPTXFEMPMSK_W<'_, GINTMSK_SPEC> {
        NPTXFEMPMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W<'_, GINTMSK_SPEC> {
        GINNAKEFFMSK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&mut self) -> GOUTNAKEFFMSK_W<'_, GINTMSK_SPEC> {
        GOUTNAKEFFMSK_W::new(self, 7)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W<'_, GINTMSK_SPEC> {
        ERLYSUSPMSK_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W<'_, GINTMSK_SPEC> {
        USBSUSPMSK_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W<'_, GINTMSK_SPEC> {
        USBRSTMSK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W<'_, GINTMSK_SPEC> {
        ENUMDONEMSK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W<'_, GINTMSK_SPEC> {
        ISOOUTDROPMSK_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<'_, GINTMSK_SPEC> {
        EOPFMSK_W::new(self, 15)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn ieptintmsk(&mut self) -> IEPTINTMSK_W<'_, GINTMSK_SPEC> {
        IEPTINTMSK_W::new(self, 18)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oeptintmsk(&mut self) -> OEPTINTMSK_W<'_, GINTMSK_SPEC> {
        OEPTINTMSK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn incomisoinmsk(&mut self) -> INCOMISOINMSK_W<'_, GINTMSK_SPEC> {
        INCOMISOINMSK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    pub fn incompip_incompisooutmsk(&mut self) -> INCOMPIP_INCOMPISOOUTMSK_W<'_, GINTMSK_SPEC> {
        INCOMPIP_INCOMPISOOUTMSK_W::new(self, 21)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtintmsk(&mut self) -> PRTINTMSK_W<'_, GINTMSK_SPEC> {
        PRTINTMSK_W::new(self, 24)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hchintmsk(&mut self) -> HCHINTMSK_W<'_, GINTMSK_SPEC> {
        HCHINTMSK_W::new(self, 25)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfempmsk(&mut self) -> PTXFEMPMSK_W<'_, GINTMSK_SPEC> {
        PTXFEMPMSK_W::new(self, 26)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn conidschgmsk(&mut self) -> CONIDSCHGMSK_W<'_, GINTMSK_SPEC> {
        CONIDSCHGMSK_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn disconintmsk(&mut self) -> DISCONINTMSK_W<'_, GINTMSK_SPEC> {
        DISCONINTMSK_W::new(self, 29)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W<'_, GINTMSK_SPEC> {
        WKUPINTMSK_W::new(self, 31)
    }
}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {}
