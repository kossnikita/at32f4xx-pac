#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `CMDFAIL` reader - CCRCFAILIE"]
pub type CMDFAIL_R = crate::BitReader;
#[doc = "Field `CMDFAIL` writer - CCRCFAILIE"]
pub type CMDFAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFAIL` reader - DCRCFAILIE"]
pub type DTFAIL_R = crate::BitReader;
#[doc = "Field `DTFAIL` writer - DCRCFAILIE"]
pub type DTFAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUT` reader - CTIMEOUTIE"]
pub type CMDTIMEOUT_R = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` writer - CTIMEOUTIE"]
pub type CMDTIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTIMEOUT` reader - DTIMEOUTIE"]
pub type DTTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTTIMEOUT` writer - DTIMEOUTIE"]
pub type DTTIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRU` reader - TXUNDERRIE"]
pub type TXERRU_R = crate::BitReader;
#[doc = "Field `TXERRU` writer - TXUNDERRIE"]
pub type TXERRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRU` reader - RXOVERRIE"]
pub type RXERRU_R = crate::BitReader;
#[doc = "Field `RXERRU` writer - RXOVERRIE"]
pub type RXERRU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRSPCMPL` reader - CMDRENDIE"]
pub type CMDRSPCMPL_R = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` writer - CMDRENDIE"]
pub type CMDRSPCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCMPL` reader - CMDSENTIE"]
pub type CMDCMPL_R = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - CMDSENTIE"]
pub type CMDCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMPL` reader - DATAENDIE"]
pub type DTCMPL_R = crate::BitReader;
#[doc = "Field `DTCMPL` writer - DATAENDIE"]
pub type DTCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBITERR` reader - STBITERRIE"]
pub type SBITERR_R = crate::BitReader;
#[doc = "Field `SBITERR` writer - STBITERRIE"]
pub type SBITERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBLKCMPL` reader - DBACKENDIE"]
pub type DTBLKCMPL_R = crate::BitReader;
#[doc = "Field `DTBLKCMPL` writer - DBACKENDIE"]
pub type DTBLKCMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOCMD` reader - CMDACTIE"]
pub type DOCMD_R = crate::BitReader;
#[doc = "Field `DOCMD` writer - CMDACTIE"]
pub type DOCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOTX` reader - TXACTIE"]
pub type DOTX_R = crate::BitReader;
#[doc = "Field `DOTX` writer - TXACTIE"]
pub type DOTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORX` reader - RXACTIE"]
pub type DORX_R = crate::BitReader;
#[doc = "Field `DORX` writer - RXACTIE"]
pub type DORX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF_H` reader - TXFIFOHEIE"]
pub type TXBUF_H_R = crate::BitReader;
#[doc = "Field `TXBUF_H` writer - TXFIFOHEIE"]
pub type TXBUF_H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF_H` reader - RXFIFOHFIE"]
pub type RXBUF_H_R = crate::BitReader;
#[doc = "Field `RXBUF_H` writer - RXFIFOHFIE"]
pub type RXBUF_H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF_F` reader - TXFIFOFIE"]
pub type TXBUF_F_R = crate::BitReader;
#[doc = "Field `TXBUF_F` writer - TXFIFOFIE"]
pub type TXBUF_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF_F` reader - RXFIFOFIE"]
pub type RXBUF_F_R = crate::BitReader;
#[doc = "Field `RXBUF_F` writer - RXFIFOFIE"]
pub type RXBUF_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF_E` reader - TXFIFOEIE"]
pub type TXBUF_E_R = crate::BitReader;
#[doc = "Field `TXBUF_E` writer - TXFIFOEIE"]
pub type TXBUF_E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF_E` reader - RXFIFOEIE"]
pub type RXBUF_E_R = crate::BitReader;
#[doc = "Field `RXBUF_E` writer - RXFIFOEIE"]
pub type RXBUF_E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF` reader - TXDAVLIE"]
pub type TXBUF_R = crate::BitReader;
#[doc = "Field `TXBUF` writer - TXDAVLIE"]
pub type TXBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF` reader - RXDAVLIE"]
pub type RXBUF_R = crate::BitReader;
#[doc = "Field `RXBUF` writer - RXDAVLIE"]
pub type RXBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOIF` reader - SDIOITIE"]
pub type SDIOIF_R = crate::BitReader;
#[doc = "Field `SDIOIF` writer - SDIOITIE"]
pub type SDIOIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CMDFAIL_R {
        CMDFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dtfail(&self) -> DTFAIL_R {
        DTFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CMDTIMEOUT_R {
        CMDTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DTTIMEOUT_R {
        DTTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txerru(&self) -> TXERRU_R {
        TXERRU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxerru(&self) -> RXERRU_R {
        RXERRU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CMDRSPCMPL_R {
        CMDRSPCMPL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CMDCMPL_R {
        CMDCMPL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DTCMPL_R {
        DTCMPL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SBITERR_R {
        SBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DTBLKCMPL_R {
        DTBLKCMPL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn docmd(&self) -> DOCMD_R {
        DOCMD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn dotx(&self) -> DOTX_R {
        DOTX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn dorx(&self) -> DORX_R {
        DORX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txbuf_h(&self) -> TXBUF_H_R {
        TXBUF_H_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxbuf_h(&self) -> RXBUF_H_R {
        RXBUF_H_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txbuf_f(&self) -> TXBUF_F_R {
        TXBUF_F_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxbuf_f(&self) -> RXBUF_F_R {
        RXBUF_F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txbuf_e(&self) -> TXBUF_E_R {
        TXBUF_E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxbuf_e(&self) -> RXBUF_E_R {
        RXBUF_E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txbuf(&self) -> TXBUF_R {
        TXBUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxbuf(&self) -> RXBUF_R {
        RXBUF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioif(&self) -> SDIOIF_R {
        SDIOIF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("cmdfail", &self.cmdfail())
            .field("dtfail", &self.dtfail())
            .field("cmdtimeout", &self.cmdtimeout())
            .field("dttimeout", &self.dttimeout())
            .field("txerru", &self.txerru())
            .field("rxerru", &self.rxerru())
            .field("cmdrspcmpl", &self.cmdrspcmpl())
            .field("cmdcmpl", &self.cmdcmpl())
            .field("dtcmpl", &self.dtcmpl())
            .field("sbiterr", &self.sbiterr())
            .field("dtblkcmpl", &self.dtblkcmpl())
            .field("docmd", &self.docmd())
            .field("dotx", &self.dotx())
            .field("dorx", &self.dorx())
            .field("txbuf_h", &self.txbuf_h())
            .field("rxbuf_h", &self.rxbuf_h())
            .field("txbuf_f", &self.txbuf_f())
            .field("rxbuf_f", &self.rxbuf_f())
            .field("txbuf_e", &self.txbuf_e())
            .field("rxbuf_e", &self.rxbuf_e())
            .field("txbuf", &self.txbuf())
            .field("rxbuf", &self.rxbuf())
            .field("sdioif", &self.sdioif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn cmdfail(&mut self) -> CMDFAIL_W<'_, INTEN_SPEC> {
        CMDFAIL_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dtfail(&mut self) -> DTFAIL_W<'_, INTEN_SPEC> {
        DTFAIL_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn cmdtimeout(&mut self) -> CMDTIMEOUT_W<'_, INTEN_SPEC> {
        CMDTIMEOUT_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dttimeout(&mut self) -> DTTIMEOUT_W<'_, INTEN_SPEC> {
        DTTIMEOUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txerru(&mut self) -> TXERRU_W<'_, INTEN_SPEC> {
        TXERRU_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxerru(&mut self) -> RXERRU_W<'_, INTEN_SPEC> {
        RXERRU_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrspcmpl(&mut self) -> CMDRSPCMPL_W<'_, INTEN_SPEC> {
        CMDRSPCMPL_W::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdcmpl(&mut self) -> CMDCMPL_W<'_, INTEN_SPEC> {
        CMDCMPL_W::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dtcmpl(&mut self) -> DTCMPL_W<'_, INTEN_SPEC> {
        DTCMPL_W::new(self, 8)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn sbiterr(&mut self) -> SBITERR_W<'_, INTEN_SPEC> {
        SBITERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dtblkcmpl(&mut self) -> DTBLKCMPL_W<'_, INTEN_SPEC> {
        DTBLKCMPL_W::new(self, 10)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn docmd(&mut self) -> DOCMD_W<'_, INTEN_SPEC> {
        DOCMD_W::new(self, 11)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn dotx(&mut self) -> DOTX_W<'_, INTEN_SPEC> {
        DOTX_W::new(self, 12)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn dorx(&mut self) -> DORX_W<'_, INTEN_SPEC> {
        DORX_W::new(self, 13)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txbuf_h(&mut self) -> TXBUF_H_W<'_, INTEN_SPEC> {
        TXBUF_H_W::new(self, 14)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxbuf_h(&mut self) -> RXBUF_H_W<'_, INTEN_SPEC> {
        RXBUF_H_W::new(self, 15)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txbuf_f(&mut self) -> TXBUF_F_W<'_, INTEN_SPEC> {
        TXBUF_F_W::new(self, 16)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxbuf_f(&mut self) -> RXBUF_F_W<'_, INTEN_SPEC> {
        RXBUF_F_W::new(self, 17)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txbuf_e(&mut self) -> TXBUF_E_W<'_, INTEN_SPEC> {
        TXBUF_E_W::new(self, 18)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxbuf_e(&mut self) -> RXBUF_E_W<'_, INTEN_SPEC> {
        RXBUF_E_W::new(self, 19)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txbuf(&mut self) -> TXBUF_W<'_, INTEN_SPEC> {
        TXBUF_W::new(self, 20)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxbuf(&mut self) -> RXBUF_W<'_, INTEN_SPEC> {
        RXBUF_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioif(&mut self) -> SDIOIF_W<'_, INTEN_SPEC> {
        SDIOIF_W::new(self, 22)
    }
}
#[doc = "SDIO interrupt enable register (SDIO_INTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
