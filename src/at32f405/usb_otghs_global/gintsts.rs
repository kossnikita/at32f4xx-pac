#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GINTSTS_SPEC>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GINTSTS_SPEC>;
#[doc = "Field `CURMOD` reader - Current mode of operation"]
pub type CURMOD_R = crate::BitReader;
#[doc = "Field `MODEMIS` reader - Mode mismatch interrupt"]
pub type MODEMIS_R = crate::BitReader;
#[doc = "Field `MODEMIS` writer - Mode mismatch interrupt"]
pub type MODEMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTG interrupt"]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - RxFIFO non-empty"]
pub type RXFLVL_R = crate::BitReader;
#[doc = "Field `NPTXFEMP` reader - Non-periodic TxFIFO empty"]
pub type NPTXFEMP_R = crate::BitReader;
#[doc = "Field `GINNAKEFF` reader - Global IN non-periodic NAK effective"]
pub type GINNAKEFF_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFF` reader - Global OUT NAK effective"]
pub type GOUTNAKEFF_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` reader - Early suspend"]
pub type ERLYSUSP_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` writer - Early suspend"]
pub type ERLYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - USB suspend"]
pub type USBSUSP_R = crate::BitReader;
#[doc = "Field `USBSUSP` writer - USB suspend"]
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset"]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONE` reader - Enumeration done"]
pub type ENUMDONE_R = crate::BitReader;
#[doc = "Field `ENUMDONE` writer - Enumeration done"]
pub type ENUMDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROP` reader - Isochronous OUT packet dropped interrupt"]
pub type ISOOUTDROP_R = crate::BitReader;
#[doc = "Field `ISOOUTDROP` writer - Isochronous OUT packet dropped interrupt"]
pub type ISOOUTDROP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of periodic frame interrupt"]
pub type EOPF_R = crate::BitReader;
#[doc = "Field `EOPF` writer - End of periodic frame interrupt"]
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPTINT` reader - IN endpoint interrupt"]
pub type IEPTINT_R = crate::BitReader;
#[doc = "Field `OEPTINT` reader - OUT endpoint interrupt"]
pub type OEPTINT_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` reader - Incomplete isochronous IN transfer"]
pub type INCOMPISOIN_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` writer - Incomplete isochronous IN transfer"]
pub type INCOMPISOIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPIP_INCOMPISOOUT` reader - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
pub type INCOMPIP_INCOMPISOOUT_R = crate::BitReader;
#[doc = "Field `INCOMPIP_INCOMPISOOUT` writer - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
pub type INCOMPIP_INCOMPISOOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTINT` reader - Host port interrupt"]
pub type PRTINT_R = crate::BitReader;
#[doc = "Field `HCHINT` reader - Host channels interrupt"]
pub type HCHINT_R = crate::BitReader;
#[doc = "Field `PTXFEMP` reader - Periodic TxFIFO empty"]
pub type PTXFEMP_R = crate::BitReader;
#[doc = "Field `CONIDSCHG` reader - Connector ID status change"]
pub type CONIDSCHG_R = crate::BitReader;
#[doc = "Field `CONIDSCHG` writer - Connector ID status change"]
pub type CONIDSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONINT` reader - Disconnect detected interrupt"]
pub type DISCONINT_R = crate::BitReader;
#[doc = "Field `DISCONINT` writer - Disconnect detected interrupt"]
pub type DISCONINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - Resume/remote wakeup detected interrupt"]
pub type WKUPINT_R = crate::BitReader;
#[doc = "Field `WKUPINT` writer - Resume/remote wakeup detected interrupt"]
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current mode of operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn modemis(&self) -> MODEMIS_R {
        MODEMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO non-empty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfemp(&self) -> NPTXFEMP_R {
        NPTXFEMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN non-periodic NAK effective"]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GINNAKEFF_R {
        GINNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn erlysusp(&self) -> ERLYSUSP_R {
        ERLYSUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdone(&self) -> ENUMDONE_R {
        ENUMDONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isooutdrop(&self) -> ISOOUTDROP_R {
        ISOOUTDROP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt"]
    #[inline(always)]
    pub fn ieptint(&self) -> IEPTINT_R {
        IEPTINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oeptint(&self) -> OEPTINT_R {
        OEPTINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn incompisoin(&self) -> INCOMPISOIN_R {
        INCOMPISOIN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
    #[inline(always)]
    pub fn incompip_incompisoout(&self) -> INCOMPIP_INCOMPISOOUT_R {
        INCOMPIP_INCOMPISOOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt"]
    #[inline(always)]
    pub fn prtint(&self) -> PRTINT_R {
        PRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt"]
    #[inline(always)]
    pub fn hchint(&self) -> HCHINT_R {
        HCHINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfemp(&self) -> PTXFEMP_R {
        PTXFEMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn conidschg(&self) -> CONIDSCHG_R {
        CONIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn disconint(&self) -> DISCONINT_R {
        DISCONINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("curmod", &format_args!("{}", self.curmod().bit()))
            .field("modemis", &format_args!("{}", self.modemis().bit()))
            .field("otgint", &format_args!("{}", self.otgint().bit()))
            .field("sof", &format_args!("{}", self.sof().bit()))
            .field("rxflvl", &format_args!("{}", self.rxflvl().bit()))
            .field("nptxfemp", &format_args!("{}", self.nptxfemp().bit()))
            .field("ginnakeff", &format_args!("{}", self.ginnakeff().bit()))
            .field("goutnakeff", &format_args!("{}", self.goutnakeff().bit()))
            .field("erlysusp", &format_args!("{}", self.erlysusp().bit()))
            .field("usbsusp", &format_args!("{}", self.usbsusp().bit()))
            .field("usbrst", &format_args!("{}", self.usbrst().bit()))
            .field("enumdone", &format_args!("{}", self.enumdone().bit()))
            .field("isooutdrop", &format_args!("{}", self.isooutdrop().bit()))
            .field("eopf", &format_args!("{}", self.eopf().bit()))
            .field("ieptint", &format_args!("{}", self.ieptint().bit()))
            .field("oeptint", &format_args!("{}", self.oeptint().bit()))
            .field("incompisoin", &format_args!("{}", self.incompisoin().bit()))
            .field(
                "incompip_incompisoout",
                &format_args!("{}", self.incompip_incompisoout().bit()),
            )
            .field("prtint", &format_args!("{}", self.prtint().bit()))
            .field("hchint", &format_args!("{}", self.hchint().bit()))
            .field("ptxfemp", &format_args!("{}", self.ptxfemp().bit()))
            .field("conidschg", &format_args!("{}", self.conidschg().bit()))
            .field("disconint", &format_args!("{}", self.disconint().bit()))
            .field("wkupint", &format_args!("{}", self.wkupint().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GINTSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn modemis(&mut self) -> MODEMIS_W<GINTSTS_SPEC> {
        MODEMIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<GINTSTS_SPEC> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    #[must_use]
    pub fn erlysusp(&mut self) -> ERLYSUSP_W<GINTSTS_SPEC> {
        ERLYSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> USBSUSP_W<GINTSTS_SPEC> {
        USBSUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<GINTSTS_SPEC> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    #[must_use]
    pub fn enumdone(&mut self) -> ENUMDONE_W<GINTSTS_SPEC> {
        ENUMDONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isooutdrop(&mut self) -> ISOOUTDROP_W<GINTSTS_SPEC> {
        ISOOUTDROP_W::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EOPF_W<GINTSTS_SPEC> {
        EOPF_W::new(self, 15)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incompisoin(&mut self) -> INCOMPISOIN_W<GINTSTS_SPEC> {
        INCOMPISOIN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn incompip_incompisoout(&mut self) -> INCOMPIP_INCOMPISOOUT_W<GINTSTS_SPEC> {
        INCOMPIP_INCOMPISOOUT_W::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    #[must_use]
    pub fn conidschg(&mut self) -> CONIDSCHG_W<GINTSTS_SPEC> {
        CONIDSCHG_W::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disconint(&mut self) -> DISCONINT_W<GINTSTS_SPEC> {
        DISCONINT_W::new(self, 29)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkupint(&mut self) -> WKUPINT_W<GINTSTS_SPEC> {
        WKUPINT_W::new(self, 31)
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
#[doc = "OTGHS core interrupt register (OTGHS_GINTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GINTSTS to value 0x0400_0020"]
impl crate::Resettable for GINTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0020;
}
