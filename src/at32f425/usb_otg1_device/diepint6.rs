#[doc = "Register `DIEPINT6` reader"]
pub type R = crate::R<DIEPINT6_SPEC>;
#[doc = "Register `DIEPINT6` writer"]
pub type W = crate::W<DIEPINT6_SPEC>;
#[doc = "Field `XFERC` reader - Transfer completed interrupt"]
pub type XFERC_R = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed interrupt"]
pub type XFERC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTDISD` reader - Endpoint disabled interrupt"]
pub type EPTDISD_R = crate::BitReader;
#[doc = "Field `EPTDISD` writer - Endpoint disabled interrupt"]
pub type EPTDISD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMEOUT` reader - Timeout condition"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout condition"]
pub type TIMEOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTKNTXFEMP` reader - IN token received when TxFIFO is empty"]
pub type INTKNTXFEMP_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMP` writer - IN token received when TxFIFO is empty"]
pub type INTKNTXFEMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEPTNAK` reader - IN endpoint NAK effective"]
pub type INEPTNAK_R = crate::BitReader;
#[doc = "Field `INEPTNAK` writer - IN endpoint NAK effective"]
pub type INEPTNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFEMP` reader - Transmit FIFO empty"]
pub type TXFEMP_R = crate::BitReader;
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
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XFERC_W<DIEPINT6_SPEC, 0> {
        XFERC_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EPTDISD_W<DIEPINT6_SPEC, 1> {
        EPTDISD_W::new(self)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<DIEPINT6_SPEC, 3> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfemp(&mut self) -> INTKNTXFEMP_W<DIEPINT6_SPEC, 4> {
        INTKNTXFEMP_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn ineptnak(&mut self) -> INEPTNAK_W<DIEPINT6_SPEC, 6> {
        INEPTNAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS device IN endpoint-6 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT6_SPEC;
impl crate::RegisterSpec for DIEPINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint6::R`](R) reader structure"]
impl crate::Readable for DIEPINT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint6::W`](W) writer structure"]
impl crate::Writable for DIEPINT6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT6 to value 0x80"]
impl crate::Resettable for DIEPINT6_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}