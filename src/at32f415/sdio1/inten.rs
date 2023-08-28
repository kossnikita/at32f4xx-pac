#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `CMDFAIL` reader - CCRCFAILIE"]
pub type CMDFAIL_R = crate::BitReader;
#[doc = "Field `CMDFAIL` writer - CCRCFAILIE"]
pub type CMDFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTFAIL` reader - DCRCFAILIE"]
pub type DTFAIL_R = crate::BitReader;
#[doc = "Field `DTFAIL` writer - DCRCFAILIE"]
pub type DTFAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDTIMEOUT` reader - CTIMEOUTIE"]
pub type CMDTIMEOUT_R = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` writer - CTIMEOUTIE"]
pub type CMDTIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTTIMEOUT` reader - DTIMEOUTIE"]
pub type DTTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTTIMEOUT` writer - DTIMEOUTIE"]
pub type DTTIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXERRU` reader - TXUNDERRIE"]
pub type TXERRU_R = crate::BitReader;
#[doc = "Field `TXERRU` writer - TXUNDERRIE"]
pub type TXERRU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXERRU` reader - RXOVERRIE"]
pub type RXERRU_R = crate::BitReader;
#[doc = "Field `RXERRU` writer - RXOVERRIE"]
pub type RXERRU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDRSPCMPL` reader - CMDRENDIE"]
pub type CMDRSPCMPL_R = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` writer - CMDRENDIE"]
pub type CMDRSPCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDCMPL` reader - CMDSENTIE"]
pub type CMDCMPL_R = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - CMDSENTIE"]
pub type CMDCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTCMPL` reader - DATAENDIE"]
pub type DTCMPL_R = crate::BitReader;
#[doc = "Field `DTCMPL` writer - DATAENDIE"]
pub type DTCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SBITERR` reader - STBITERRIE"]
pub type SBITERR_R = crate::BitReader;
#[doc = "Field `SBITERR` writer - STBITERRIE"]
pub type SBITERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTBLKCMPL` reader - DBACKENDIE"]
pub type DTBLKCMPL_R = crate::BitReader;
#[doc = "Field `DTBLKCMPL` writer - DBACKENDIE"]
pub type DTBLKCMPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOCMD` reader - CMDACTIE"]
pub type DOCMD_R = crate::BitReader;
#[doc = "Field `DOCMD` writer - CMDACTIE"]
pub type DOCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOTX` reader - TXACTIE"]
pub type DOTX_R = crate::BitReader;
#[doc = "Field `DOTX` writer - TXACTIE"]
pub type DOTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DORX` reader - RXACTIE"]
pub type DORX_R = crate::BitReader;
#[doc = "Field `DORX` writer - RXACTIE"]
pub type DORX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBUF_H` reader - TXFIFOHEIE"]
pub type TXBUF_H_R = crate::BitReader;
#[doc = "Field `TXBUF_H` writer - TXFIFOHEIE"]
pub type TXBUF_H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUF_H` reader - RXFIFOHFIE"]
pub type RXBUF_H_R = crate::BitReader;
#[doc = "Field `RXBUF_H` writer - RXFIFOHFIE"]
pub type RXBUF_H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBUF_F` reader - TXFIFOFIE"]
pub type TXBUF_F_R = crate::BitReader;
#[doc = "Field `TXBUF_F` writer - TXFIFOFIE"]
pub type TXBUF_F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUF_F` reader - RXFIFOFIE"]
pub type RXBUF_F_R = crate::BitReader;
#[doc = "Field `RXBUF_F` writer - RXFIFOFIE"]
pub type RXBUF_F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBUF_E` reader - TXFIFOEIE"]
pub type TXBUF_E_R = crate::BitReader;
#[doc = "Field `TXBUF_E` writer - TXFIFOEIE"]
pub type TXBUF_E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUF_E` reader - RXFIFOEIE"]
pub type RXBUF_E_R = crate::BitReader;
#[doc = "Field `RXBUF_E` writer - RXFIFOEIE"]
pub type RXBUF_E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXBUF` reader - TXDAVLIE"]
pub type TXBUF_R = crate::BitReader;
#[doc = "Field `TXBUF` writer - TXDAVLIE"]
pub type TXBUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBUF` reader - RXDAVLIE"]
pub type RXBUF_R = crate::BitReader;
#[doc = "Field `RXBUF` writer - RXDAVLIE"]
pub type RXBUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOIF` reader - SDIOITIE"]
pub type SDIOIF_R = crate::BitReader;
#[doc = "Field `SDIOIF` writer - SDIOITIE"]
pub type SDIOIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfail(&mut self) -> CMDFAIL_W<INTEN_SPEC, 0> {
        CMDFAIL_W::new(self)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtfail(&mut self) -> DTFAIL_W<INTEN_SPEC, 1> {
        DTFAIL_W::new(self)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtimeout(&mut self) -> CMDTIMEOUT_W<INTEN_SPEC, 2> {
        CMDTIMEOUT_W::new(self)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dttimeout(&mut self) -> DTTIMEOUT_W<INTEN_SPEC, 3> {
        DTTIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txerru(&mut self) -> TXERRU_W<INTEN_SPEC, 4> {
        TXERRU_W::new(self)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxerru(&mut self) -> RXERRU_W<INTEN_SPEC, 5> {
        RXERRU_W::new(self)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrspcmpl(&mut self) -> CMDRSPCMPL_W<INTEN_SPEC, 6> {
        CMDRSPCMPL_W::new(self)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmpl(&mut self) -> CMDCMPL_W<INTEN_SPEC, 7> {
        CMDCMPL_W::new(self)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmpl(&mut self) -> DTCMPL_W<INTEN_SPEC, 8> {
        DTCMPL_W::new(self)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn sbiterr(&mut self) -> SBITERR_W<INTEN_SPEC, 9> {
        SBITERR_W::new(self)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtblkcmpl(&mut self) -> DTBLKCMPL_W<INTEN_SPEC, 10> {
        DTBLKCMPL_W::new(self)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn docmd(&mut self) -> DOCMD_W<INTEN_SPEC, 11> {
        DOCMD_W::new(self)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dotx(&mut self) -> DOTX_W<INTEN_SPEC, 12> {
        DOTX_W::new(self)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dorx(&mut self) -> DORX_W<INTEN_SPEC, 13> {
        DORX_W::new(self)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf_h(&mut self) -> TXBUF_H_W<INTEN_SPEC, 14> {
        TXBUF_H_W::new(self)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf_h(&mut self) -> RXBUF_H_W<INTEN_SPEC, 15> {
        RXBUF_H_W::new(self)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf_f(&mut self) -> TXBUF_F_W<INTEN_SPEC, 16> {
        TXBUF_F_W::new(self)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf_f(&mut self) -> RXBUF_F_W<INTEN_SPEC, 17> {
        RXBUF_F_W::new(self)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf_e(&mut self) -> TXBUF_E_W<INTEN_SPEC, 18> {
        TXBUF_E_W::new(self)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf_e(&mut self) -> RXBUF_E_W<INTEN_SPEC, 19> {
        RXBUF_E_W::new(self)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf(&mut self) -> TXBUF_W<INTEN_SPEC, 20> {
        TXBUF_W::new(self)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf(&mut self) -> RXBUF_W<INTEN_SPEC, 21> {
        RXBUF_W::new(self)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    #[must_use]
    pub fn sdioif(&mut self) -> SDIOIF_W<INTEN_SPEC, 22> {
        SDIOIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDIO interrupt enable register (SDIO_INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
