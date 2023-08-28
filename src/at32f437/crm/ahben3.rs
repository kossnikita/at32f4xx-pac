#[doc = "Register `AHBEN3` reader"]
pub type R = crate::R<AHBEN3_SPEC>;
#[doc = "Register `AHBEN3` writer"]
pub type W = crate::W<AHBEN3_SPEC>;
#[doc = "Field `XMCEN` reader - XMC clock enable"]
pub type XMCEN_R = crate::BitReader;
#[doc = "Field `XMCEN` writer - XMC clock enable"]
pub type XMCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI1EN` reader - QSPI1 clock enable"]
pub type QSPI1EN_R = crate::BitReader;
#[doc = "Field `QSPI1EN` writer - QSPI1 clock enable"]
pub type QSPI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI2EN` reader - QSPI2 clock enable"]
pub type QSPI2EN_R = crate::BitReader;
#[doc = "Field `QSPI2EN` writer - QSPI2 clock enable"]
pub type QSPI2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO2EN` reader - SDIO 2 clock enable"]
pub type SDIO2EN_R = crate::BitReader;
#[doc = "Field `SDIO2EN` writer - SDIO 2 clock enable"]
pub type SDIO2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    pub fn xmcen(&self) -> XMCEN_R {
        XMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1en(&self) -> QSPI1EN_R {
        QSPI1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    pub fn qspi2en(&self) -> QSPI2EN_R {
        QSPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    pub fn sdio2en(&self) -> SDIO2EN_R {
        SDIO2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmcen(&mut self) -> XMCEN_W<AHBEN3_SPEC, 0> {
        XMCEN_W::new(self)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1en(&mut self) -> QSPI1EN_W<AHBEN3_SPEC, 1> {
        QSPI1EN_W::new(self)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi2en(&mut self) -> QSPI2EN_W<AHBEN3_SPEC, 14> {
        QSPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2en(&mut self) -> SDIO2EN_W<AHBEN3_SPEC, 15> {
        SDIO2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB peripheral clock enable register 3 (CRM_AHBEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN3_SPEC;
impl crate::RegisterSpec for AHBEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben3::R`](R) reader structure"]
impl crate::Readable for AHBEN3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben3::W`](W) writer structure"]
impl crate::Writable for AHBEN3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN3 to value 0"]
impl crate::Resettable for AHBEN3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
