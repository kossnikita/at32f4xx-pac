#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `DMAEN` reader - DMA handshake enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA handshake enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMDIE` reader - Command complete interrupt enable"]
pub type CMDIE_R = crate::BitReader;
#[doc = "Field `CMDIE` writer - Command complete interrupt enable"]
pub type CMDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFIFOTHOD` reader - TxFIFO thod"]
pub type TXFIFOTHOD_R = crate::FieldReader;
#[doc = "Field `TXFIFOTHOD` writer - TxFIFO thod"]
pub type TXFIFOTHOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RXFIFOTHOD` reader - RxFIFO thod"]
pub type RXFIFOTHOD_R = crate::FieldReader;
#[doc = "Field `RXFIFOTHOD` writer - RxFIFO thod"]
pub type RXFIFOTHOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
            .field("dmaen", &format_args!("{}", self.dmaen().bit()))
            .field("cmdie", &format_args!("{}", self.cmdie().bit()))
            .field("txfifothod", &format_args!("{}", self.txfifothod().bits()))
            .field("rxfifothod", &format_args!("{}", self.rxfifothod().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DMA handshake enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CTRL2_SPEC, 0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Command complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdie(&mut self) -> CMDIE_W<CTRL2_SPEC, 1> {
        CMDIE_W::new(self)
    }
    #[doc = "Bits 8:9 - TxFIFO thod"]
    #[inline(always)]
    #[must_use]
    pub fn txfifothod(&mut self) -> TXFIFOTHOD_W<CTRL2_SPEC, 8> {
        TXFIFOTHOD_W::new(self)
    }
    #[doc = "Bits 12:13 - RxFIFO thod"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifothod(&mut self) -> RXFIFOTHOD_W<CTRL2_SPEC, 12> {
        RXFIFOTHOD_W::new(self)
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
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x01"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
