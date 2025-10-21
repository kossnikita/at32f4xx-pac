#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `CMDFAILIEN` reader - Command crc fail interrupt enable"]
pub type CMDFAILIEN_R = crate::BitReader;
#[doc = "Field `CMDFAILIEN` writer - Command crc fail interrupt enable"]
pub type CMDFAILIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFAILIEN` reader - Data crc fail interrupt enable"]
pub type DTFAILIEN_R = crate::BitReader;
#[doc = "Field `DTFAILIEN` writer - Data crc fail interrupt enable"]
pub type DTFAILIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUTIEN` reader - Command timeout interrupt enable"]
pub type CMDTIMEOUTIEN_R = crate::BitReader;
#[doc = "Field `CMDTIMEOUTIEN` writer - Command timeout interrupt enable"]
pub type CMDTIMEOUTIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTIMEOUTIEN` reader - Data timeout interrupt enable"]
pub type DTTIMEOUTIEN_R = crate::BitReader;
#[doc = "Field `DTTIMEOUTIEN` writer - Data timeout interrupt enable"]
pub type DTTIMEOUTIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRUIEN` reader - Tx under run interrupt enable"]
pub type TXERRUIEN_R = crate::BitReader;
#[doc = "Field `TXERRUIEN` writer - Tx under run interrupt enable"]
pub type TXERRUIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRUIEN` reader - Rx over run interrupt enable"]
pub type RXERRUIEN_R = crate::BitReader;
#[doc = "Field `RXERRUIEN` writer - Rx over run interrupt enable"]
pub type RXERRUIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRSPCMPLIEN` reader - Command response complete interrupt enable"]
pub type CMDRSPCMPLIEN_R = crate::BitReader;
#[doc = "Field `CMDRSPCMPLIEN` writer - Command response complete interrupt enable"]
pub type CMDRSPCMPLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCMPLIEN` reader - Command sent complete interrupt enable"]
pub type CMDCMPLIEN_R = crate::BitReader;
#[doc = "Field `CMDCMPLIEN` writer - Command sent complete interrupt enable"]
pub type CMDCMPLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMPLIEN` reader - Data sent complete interrupt enable"]
pub type DTCMPLIEN_R = crate::BitReader;
#[doc = "Field `DTCMPLIEN` writer - Data sent complete interrupt enable"]
pub type DTCMPLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBITERRIEN` reader - Start bit error interrupt enable"]
pub type SBITERRIEN_R = crate::BitReader;
#[doc = "Field `SBITERRIEN` writer - Start bit error interrupt enable"]
pub type SBITERRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBLKCMPLIEN` reader - Data block sent complete interrupt enable"]
pub type DTBLKCMPLIEN_R = crate::BitReader;
#[doc = "Field `DTBLKCMPLIEN` writer - Data block sent complete interrupt enable"]
pub type DTBLKCMPLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOCMDIEN` reader - Command acting interrupt enable"]
pub type DOCMDIEN_R = crate::BitReader;
#[doc = "Field `DOCMDIEN` writer - Command acting interrupt enable"]
pub type DOCMDIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOTXIEN` reader - Data transmit acting interrupt enable"]
pub type DOTXIEN_R = crate::BitReader;
#[doc = "Field `DOTXIEN` writer - Data transmit acting interrupt enable"]
pub type DOTXIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORXIEN` reader - Data receive acting interrupt enable"]
pub type DORXIEN_R = crate::BitReader;
#[doc = "Field `DORXIEN` writer - Data receive acting interrupt enable"]
pub type DORXIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFHIEN` reader - Tx buffer half empty interrupt enable"]
pub type TXBUFHIEN_R = crate::BitReader;
#[doc = "Field `TXBUFHIEN` writer - Tx buffer half empty interrupt enable"]
pub type TXBUFHIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFHIEN` reader - Rx buffer half empty interrupt enable"]
pub type RXBUFHIEN_R = crate::BitReader;
#[doc = "Field `RXBUFHIEN` writer - Rx buffer half empty interrupt enable"]
pub type RXBUFHIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFFIEN` reader - Tx buffer full interrupt enable"]
pub type TXBUFFIEN_R = crate::BitReader;
#[doc = "Field `TXBUFFIEN` writer - Tx buffer full interrupt enable"]
pub type TXBUFFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFFIEN` reader - Rx buffer full interrupt enable"]
pub type RXBUFFIEN_R = crate::BitReader;
#[doc = "Field `RXBUFFIEN` writer - Rx buffer full interrupt enable"]
pub type RXBUFFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFEIEN` reader - Tx buffer empty interrupt enable"]
pub type TXBUFEIEN_R = crate::BitReader;
#[doc = "Field `TXBUFEIEN` writer - Tx buffer empty interrupt enable"]
pub type TXBUFEIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFEIEN` reader - Rx buffer empty interrupt enable"]
pub type RXBUFEIEN_R = crate::BitReader;
#[doc = "Field `RXBUFEIEN` writer - Rx buffer empty interrupt enable"]
pub type RXBUFEIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFIEN` reader - Tx buffer data vaild interrupt enable"]
pub type TXBUFIEN_R = crate::BitReader;
#[doc = "Field `TXBUFIEN` writer - Tx buffer data vaild interrupt enable"]
pub type TXBUFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFIEN` reader - Rx buffer data vaild interrupt enable"]
pub type RXBUFIEN_R = crate::BitReader;
#[doc = "Field `RXBUFIEN` writer - Rx buffer data vaild interrupt enable"]
pub type RXBUFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOIFIEN` reader - SD I/O interrupt enable"]
pub type IOIFIEN_R = crate::BitReader;
#[doc = "Field `IOIFIEN` writer - SD I/O interrupt enable"]
pub type IOIFIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command crc fail interrupt enable"]
    #[inline(always)]
    pub fn cmdfailien(&self) -> CMDFAILIEN_R {
        CMDFAILIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data crc fail interrupt enable"]
    #[inline(always)]
    pub fn dtfailien(&self) -> DTFAILIEN_R {
        DTFAILIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtimeoutien(&self) -> CMDTIMEOUTIEN_R {
        CMDTIMEOUTIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttimeoutien(&self) -> DTTIMEOUTIEN_R {
        DTTIMEOUTIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx under run interrupt enable"]
    #[inline(always)]
    pub fn txerruien(&self) -> TXERRUIEN_R {
        TXERRUIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx over run interrupt enable"]
    #[inline(always)]
    pub fn rxerruien(&self) -> RXERRUIEN_R {
        RXERRUIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response complete interrupt enable"]
    #[inline(always)]
    pub fn cmdrspcmplien(&self) -> CMDRSPCMPLIEN_R {
        CMDRSPCMPLIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent complete interrupt enable"]
    #[inline(always)]
    pub fn cmdcmplien(&self) -> CMDCMPLIEN_R {
        CMDCMPLIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtcmplien(&self) -> DTCMPLIEN_R {
        DTCMPLIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn sbiterrien(&self) -> SBITERRIEN_R {
        SBITERRIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtblkcmplien(&self) -> DTBLKCMPLIEN_R {
        DTBLKCMPLIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn docmdien(&self) -> DOCMDIEN_R {
        DOCMDIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn dotxien(&self) -> DOTXIEN_R {
        DOTXIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn dorxien(&self) -> DORXIEN_R {
        DORXIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn txbufhien(&self) -> TXBUFHIEN_R {
        TXBUFHIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufhien(&self) -> RXBUFHIEN_R {
        RXBUFHIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx buffer full interrupt enable"]
    #[inline(always)]
    pub fn txbuffien(&self) -> TXBUFFIEN_R {
        TXBUFFIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbuffien(&self) -> RXBUFFIEN_R {
        RXBUFFIEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbufeien(&self) -> TXBUFEIEN_R {
        TXBUFEIEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufeien(&self) -> RXBUFEIEN_R {
        RXBUFEIEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn txbufien(&self) -> TXBUFIEN_R {
        TXBUFIEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn rxbufien(&self) -> RXBUFIEN_R {
        RXBUFIEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt enable"]
    #[inline(always)]
    pub fn ioifien(&self) -> IOIFIEN_R {
        IOIFIEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("cmdfailien", &self.cmdfailien())
            .field("dtfailien", &self.dtfailien())
            .field("cmdtimeoutien", &self.cmdtimeoutien())
            .field("dttimeoutien", &self.dttimeoutien())
            .field("txerruien", &self.txerruien())
            .field("rxerruien", &self.rxerruien())
            .field("cmdrspcmplien", &self.cmdrspcmplien())
            .field("cmdcmplien", &self.cmdcmplien())
            .field("dtcmplien", &self.dtcmplien())
            .field("sbiterrien", &self.sbiterrien())
            .field("dtblkcmplien", &self.dtblkcmplien())
            .field("docmdien", &self.docmdien())
            .field("dotxien", &self.dotxien())
            .field("dorxien", &self.dorxien())
            .field("txbufhien", &self.txbufhien())
            .field("rxbufhien", &self.rxbufhien())
            .field("txbuffien", &self.txbuffien())
            .field("rxbuffien", &self.rxbuffien())
            .field("txbufeien", &self.txbufeien())
            .field("rxbufeien", &self.rxbufeien())
            .field("txbufien", &self.txbufien())
            .field("rxbufien", &self.rxbufien())
            .field("ioifien", &self.ioifien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Command crc fail interrupt enable"]
    #[inline(always)]
    pub fn cmdfailien(&mut self) -> CMDFAILIEN_W<'_, INTEN_SPEC> {
        CMDFAILIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data crc fail interrupt enable"]
    #[inline(always)]
    pub fn dtfailien(&mut self) -> DTFAILIEN_W<'_, INTEN_SPEC> {
        DTFAILIEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtimeoutien(&mut self) -> CMDTIMEOUTIEN_W<'_, INTEN_SPEC> {
        CMDTIMEOUTIEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttimeoutien(&mut self) -> DTTIMEOUTIEN_W<'_, INTEN_SPEC> {
        DTTIMEOUTIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Tx under run interrupt enable"]
    #[inline(always)]
    pub fn txerruien(&mut self) -> TXERRUIEN_W<'_, INTEN_SPEC> {
        TXERRUIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx over run interrupt enable"]
    #[inline(always)]
    pub fn rxerruien(&mut self) -> RXERRUIEN_W<'_, INTEN_SPEC> {
        RXERRUIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Command response complete interrupt enable"]
    #[inline(always)]
    pub fn cmdrspcmplien(&mut self) -> CMDRSPCMPLIEN_W<'_, INTEN_SPEC> {
        CMDRSPCMPLIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Command sent complete interrupt enable"]
    #[inline(always)]
    pub fn cmdcmplien(&mut self) -> CMDCMPLIEN_W<'_, INTEN_SPEC> {
        CMDCMPLIEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Data sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtcmplien(&mut self) -> DTCMPLIEN_W<'_, INTEN_SPEC> {
        DTCMPLIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn sbiterrien(&mut self) -> SBITERRIEN_W<'_, INTEN_SPEC> {
        SBITERRIEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data block sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtblkcmplien(&mut self) -> DTBLKCMPLIEN_W<'_, INTEN_SPEC> {
        DTBLKCMPLIEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn docmdien(&mut self) -> DOCMDIEN_W<'_, INTEN_SPEC> {
        DOCMDIEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn dotxien(&mut self) -> DOTXIEN_W<'_, INTEN_SPEC> {
        DOTXIEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn dorxien(&mut self) -> DORXIEN_W<'_, INTEN_SPEC> {
        DORXIEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Tx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn txbufhien(&mut self) -> TXBUFHIEN_W<'_, INTEN_SPEC> {
        TXBUFHIEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufhien(&mut self) -> RXBUFHIEN_W<'_, INTEN_SPEC> {
        RXBUFHIEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Tx buffer full interrupt enable"]
    #[inline(always)]
    pub fn txbuffien(&mut self) -> TXBUFFIEN_W<'_, INTEN_SPEC> {
        TXBUFFIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Rx buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbuffien(&mut self) -> RXBUFFIEN_W<'_, INTEN_SPEC> {
        RXBUFFIEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbufeien(&mut self) -> TXBUFEIEN_W<'_, INTEN_SPEC> {
        TXBUFEIEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Rx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufeien(&mut self) -> RXBUFEIEN_W<'_, INTEN_SPEC> {
        RXBUFEIEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Tx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn txbufien(&mut self) -> TXBUFIEN_W<'_, INTEN_SPEC> {
        TXBUFIEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Rx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn rxbufien(&mut self) -> RXBUFIEN_W<'_, INTEN_SPEC> {
        RXBUFIEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SD I/O interrupt enable"]
    #[inline(always)]
    pub fn ioifien(&mut self) -> IOIFIEN_W<'_, INTEN_SPEC> {
        IOIFIEN_W::new(self, 22)
    }
}
#[doc = "SDIO mask register (SDIO_MASK)\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {}
