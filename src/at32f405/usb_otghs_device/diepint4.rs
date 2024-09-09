#[doc = "Register `DIEPINT4` reader"]
pub type R = crate::R<DIEPINT4_SPEC>;
#[doc = "Register `DIEPINT4` writer"]
pub type W = crate::W<DIEPINT4_SPEC>;
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
        f.debug_struct("DIEPINT4")
            .field("xferc", &self.xferc())
            .field("eptdisd", &self.eptdisd())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("ineptnak", &self.ineptnak())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("nakintpt", &self.nakintpt())
            .field("nyetintpt", &self.nyetintpt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XFERC_W<DIEPINT4_SPEC> {
        XFERC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EPTDISD_W<DIEPINT4_SPEC> {
        EPTDISD_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<DIEPINT4_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<DIEPINT4_SPEC> {
        TIMEOUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfemp(&mut self) -> INTKNTXFEMP_W<DIEPINT4_SPEC> {
        INTKNTXFEMP_W::new(self, 4)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn ineptnak(&mut self) -> INEPTNAK_W<DIEPINT4_SPEC> {
        INEPTNAK_W::new(self, 6)
    }
    #[doc = "Bit 8 - Fifo Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoundrn(&mut self) -> TXFIFOUNDRN_W<DIEPINT4_SPEC> {
        TXFIFOUNDRN_W::new(self, 8)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DIEPINT4_SPEC> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintpt(&mut self) -> NAKINTPT_W<DIEPINT4_SPEC> {
        NAKINTPT_W::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyetintpt(&mut self) -> NYETINTPT_W<DIEPINT4_SPEC> {
        NYETINTPT_W::new(self, 14)
    }
}
#[doc = "OTGHS device IN endpoint-4 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT4_SPEC;
impl crate::RegisterSpec for DIEPINT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint4::R`](R) reader structure"]
impl crate::Readable for DIEPINT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint4::W`](W) writer structure"]
impl crate::Writable for DIEPINT4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT4 to value 0x80"]
impl crate::Resettable for DIEPINT4_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
