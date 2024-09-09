#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `DMAEN` reader - DMA handshake enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA handshake enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDIE` reader - Command complete interrupt enable"]
pub type CMDIE_R = crate::BitReader;
#[doc = "Field `CMDIE` writer - Command complete interrupt enable"]
pub type CMDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOTHOD` reader - TxFIFO thod"]
pub type TXFIFOTHOD_R = crate::FieldReader;
#[doc = "Field `TXFIFOTHOD` writer - TxFIFO thod"]
pub type TXFIFOTHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXFIFOTHOD` reader - RxFIFO thod"]
pub type RXFIFOTHOD_R = crate::FieldReader;
#[doc = "Field `RXFIFOTHOD` writer - RxFIFO thod"]
pub type RXFIFOTHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - DMA handshake enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command complete interrupt enable"]
    #[inline(always)]
    pub fn cmdie(&self) -> CMDIE_R {
        CMDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - TxFIFO thod"]
    #[inline(always)]
    pub fn txfifothod(&self) -> TXFIFOTHOD_R {
        TXFIFOTHOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - RxFIFO thod"]
    #[inline(always)]
    pub fn rxfifothod(&self) -> RXFIFOTHOD_R {
        RXFIFOTHOD_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("dmaen", &self.dmaen())
            .field("cmdie", &self.cmdie())
            .field("txfifothod", &self.txfifothod())
            .field("rxfifothod", &self.rxfifothod())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DMA handshake enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CTRL2_SPEC> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Command complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdie(&mut self) -> CMDIE_W<CTRL2_SPEC> {
        CMDIE_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - TxFIFO thod"]
    #[inline(always)]
    #[must_use]
    pub fn txfifothod(&mut self) -> TXFIFOTHOD_W<CTRL2_SPEC> {
        TXFIFOTHOD_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - RxFIFO thod"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifothod(&mut self) -> RXFIFOTHOD_W<CTRL2_SPEC> {
        RXFIFOTHOD_W::new(self, 12)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x01"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
