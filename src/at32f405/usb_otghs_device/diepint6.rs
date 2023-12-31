#[doc = "Register `DIEPINT6` reader"]
pub type R = crate::R<DIEPINT6_SPEC>;
#[doc = "Register `DIEPINT6` writer"]
pub type W = crate::W<DIEPINT6_SPEC>;
#[doc = "Field `XFERC` reader - Transfer completed interrupt"]
pub type XFERC_R = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed interrupt"]
pub type XFERC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISD` reader - Endpoint disabled interrupt"]
pub type EPTDISD_R = crate::BitReader;
#[doc = "Field `EPTDISD` writer - Endpoint disabled interrupt"]
pub type EPTDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB Error interrupt"]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB Error interrupt"]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Timeout condition"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout condition"]
pub type TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMP` reader - IN token received when TxFIFO is empty"]
pub type INTKNTXFEMP_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMP` writer - IN token received when TxFIFO is empty"]
pub type INTKNTXFEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPTNAK` reader - IN endpoint NAK effective"]
pub type INEPTNAK_R = crate::BitReader;
#[doc = "Field `INEPTNAK` writer - IN endpoint NAK effective"]
pub type INEPTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMP` reader - Transmit FIFO empty"]
pub type TXFEMP_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` reader - Fifo Underrun"]
pub type TXFIFOUNDRN_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` writer - Fifo Underrun"]
pub type TXFIFOUNDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTPT` reader - NAK interrupt"]
pub type NAKINTPT_R = crate::BitReader;
#[doc = "Field `NAKINTPT` writer - NAK interrupt"]
pub type NAKINTPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETINTPT` reader - NYET interrupt"]
pub type NYETINTPT_R = crate::BitReader;
#[doc = "Field `NYETINTPT` writer - NYET interrupt"]
pub type NYETINTPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xferc(&self) -> XFERC_R {
        XFERC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn eptdisd(&self) -> EPTDISD_R {
        EPTDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error interrupt"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn intkntxfemp(&self) -> INTKNTXFEMP_R {
        INTKNTXFEMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn ineptnak(&self) -> INEPTNAK_R {
        INEPTNAK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfemp(&self) -> TXFEMP_R {
        TXFEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoundrn(&self) -> TXFIFOUNDRN_R {
        TXFIFOUNDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nakintpt(&self) -> NAKINTPT_R {
        NAKINTPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyetintpt(&self) -> NYETINTPT_R {
        NYETINTPT_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT6")
            .field("xferc", &format_args!("{}", self.xferc().bit()))
            .field("eptdisd", &format_args!("{}", self.eptdisd().bit()))
            .field("ahberr", &format_args!("{}", self.ahberr().bit()))
            .field("timeout", &format_args!("{}", self.timeout().bit()))
            .field("intkntxfemp", &format_args!("{}", self.intkntxfemp().bit()))
            .field("ineptnak", &format_args!("{}", self.ineptnak().bit()))
            .field("txfemp", &format_args!("{}", self.txfemp().bit()))
            .field("txfifoundrn", &format_args!("{}", self.txfifoundrn().bit()))
            .field("pktdrpsts", &format_args!("{}", self.pktdrpsts().bit()))
            .field("nakintpt", &format_args!("{}", self.nakintpt().bit()))
            .field("nyetintpt", &format_args!("{}", self.nyetintpt().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPINT6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XFERC_W<DIEPINT6_SPEC> {
        XFERC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EPTDISD_W<DIEPINT6_SPEC> {
        EPTDISD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<DIEPINT6_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<DIEPINT6_SPEC> {
        TIMEOUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfemp(&mut self) -> INTKNTXFEMP_W<DIEPINT6_SPEC> {
        INTKNTXFEMP_W::new(self, 4)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn ineptnak(&mut self) -> INEPTNAK_W<DIEPINT6_SPEC> {
        INEPTNAK_W::new(self, 6)
    }
    #[doc = "Bit 8 - Fifo Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoundrn(&mut self) -> TXFIFOUNDRN_W<DIEPINT6_SPEC> {
        TXFIFOUNDRN_W::new(self, 8)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DIEPINT6_SPEC> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintpt(&mut self) -> NAKINTPT_W<DIEPINT6_SPEC> {
        NAKINTPT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyetintpt(&mut self) -> NYETINTPT_W<DIEPINT6_SPEC> {
        NYETINTPT_W::new(self, 14)
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
#[doc = "OTGHS device IN endpoint-6 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT6_SPEC;
impl crate::RegisterSpec for DIEPINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint6::R`](R) reader structure"]
impl crate::Readable for DIEPINT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint6::W`](W) writer structure"]
impl crate::Writable for DIEPINT6_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT6 to value 0x80"]
impl crate::Resettable for DIEPINT6_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
