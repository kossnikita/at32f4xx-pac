#[doc = "Register `AHBEN1` reader"]
pub type R = crate::R<AHBEN1_SPEC>;
#[doc = "Register `AHBEN1` writer"]
pub type W = crate::W<AHBEN1_SPEC>;
#[doc = "Field `GPIOAEN` reader - IO A clock enable"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - IO A clock enable"]
pub type GPIOAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOBEN` reader - IO B clock enable"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - IO B clock enable"]
pub type GPIOBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOCEN` reader - IO C clock enable"]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - IO C clock enable"]
pub type GPIOCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIODEN` reader - IO D clock enable"]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - IO D clock enable"]
pub type GPIODEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOEEN` reader - IO E clock enable"]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - IO E clock enable"]
pub type GPIOEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOFEN` reader - IO F clock enable"]
pub type GPIOFEN_R = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - IO F clock enable"]
pub type GPIOFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOGEN` reader - IO G clock enable"]
pub type GPIOGEN_R = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - IO G clock enable"]
pub type GPIOGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOHEN` reader - IO H clock enable"]
pub type GPIOHEN_R = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - IO H clock enable"]
pub type GPIOHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDMAEN` reader - DMA1 clock enable"]
pub type EDMAEN_R = crate::BitReader;
#[doc = "Field `EDMAEN` writer - DMA1 clock enable"]
pub type EDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type DMA2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACEN` reader - EMAC clock enable"]
pub type EMACEN_R = crate::BitReader;
#[doc = "Field `EMACEN` writer - EMAC clock enable"]
pub type EMACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACTXEN` reader - EMAC Tx clock enable"]
pub type EMACTXEN_R = crate::BitReader;
#[doc = "Field `EMACTXEN` writer - EMAC Tx clock enable"]
pub type EMACTXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACRXEN` reader - EMAC Rx clock enable"]
pub type EMACRXEN_R = crate::BitReader;
#[doc = "Field `EMACRXEN` writer - EMAC Rx clock enable"]
pub type EMACRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMACPTPEN` reader - EMAC PTP clock enable"]
pub type EMACPTPEN_R = crate::BitReader;
#[doc = "Field `EMACPTPEN` writer - EMAC PTP clock enable"]
pub type EMACPTPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGFS2EN` reader - OTGFS2 clock enable"]
pub type OTGFS2EN_R = crate::BitReader;
#[doc = "Field `OTGFS2EN` writer - OTGFS2 clock enable"]
pub type OTGFS2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IO A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    pub fn edmaen(&self) -> EDMAEN_R {
        EDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EMAC clock enable"]
    #[inline(always)]
    pub fn emacen(&self) -> EMACEN_R {
        EMACEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable"]
    #[inline(always)]
    pub fn emactxen(&self) -> EMACTXEN_R {
        EMACTXEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable"]
    #[inline(always)]
    pub fn emacrxen(&self) -> EMACRXEN_R {
        EMACRXEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable"]
    #[inline(always)]
    pub fn emacptpen(&self) -> EMACPTPEN_R {
        EMACPTPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable"]
    #[inline(always)]
    pub fn otgfs2en(&self) -> OTGFS2EN_R {
        OTGFS2EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHBEN1_SPEC, 0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - IO B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHBEN1_SPEC, 1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - IO C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHBEN1_SPEC, 2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - IO D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHBEN1_SPEC, 3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - IO E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHBEN1_SPEC, 4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 5 - IO F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHBEN1_SPEC, 5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 6 - IO G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHBEN1_SPEC, 6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 7 - IO H clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHBEN1_SPEC, 7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBEN1_SPEC, 12> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn edmaen(&mut self) -> EDMAEN_W<AHBEN1_SPEC, 21> {
        EDMAEN_W::new(self)
    }
    #[doc = "Bit 22 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBEN1_SPEC, 22> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 24 - DMA2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<AHBEN1_SPEC, 24> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 25 - EMAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacen(&mut self) -> EMACEN_W<AHBEN1_SPEC, 25> {
        EMACEN_W::new(self)
    }
    #[doc = "Bit 26 - EMAC Tx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emactxen(&mut self) -> EMACTXEN_W<AHBEN1_SPEC, 26> {
        EMACTXEN_W::new(self)
    }
    #[doc = "Bit 27 - EMAC Rx clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacrxen(&mut self) -> EMACRXEN_W<AHBEN1_SPEC, 27> {
        EMACRXEN_W::new(self)
    }
    #[doc = "Bit 28 - EMAC PTP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn emacptpen(&mut self) -> EMACPTPEN_W<AHBEN1_SPEC, 28> {
        EMACPTPEN_W::new(self)
    }
    #[doc = "Bit 29 - OTGFS2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs2en(&mut self) -> OTGFS2EN_W<AHBEN1_SPEC, 29> {
        OTGFS2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB Peripheral Clock enable register 1 (CRM_AHBEN1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN1_SPEC;
impl crate::RegisterSpec for AHBEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben1::R`](R) reader structure"]
impl crate::Readable for AHBEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben1::W`](W) writer structure"]
impl crate::Writable for AHBEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN1 to value 0"]
impl crate::Resettable for AHBEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
