#[doc = "Register `AHBEN3` reader"]
pub type R = crate::R<AHBEN3_SPEC>;
#[doc = "Register `AHBEN3` writer"]
pub type W = crate::W<AHBEN3_SPEC>;
#[doc = "Field `XMC` reader - XMC clock enable"]
pub type XMC_R = crate::BitReader;
#[doc = "Field `XMC` writer - XMC clock enable"]
pub type XMC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI1` reader - QSPI1 clock enable"]
pub type QSPI1_R = crate::BitReader;
#[doc = "Field `QSPI1` writer - QSPI1 clock enable"]
pub type QSPI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI2` reader - QSPI2 clock enable"]
pub type QSPI2_R = crate::BitReader;
#[doc = "Field `QSPI2` writer - QSPI2 clock enable"]
pub type QSPI2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO2` reader - SDIO 2 clock enable"]
pub type SDIO2_R = crate::BitReader;
#[doc = "Field `SDIO2` writer - SDIO 2 clock enable"]
pub type SDIO2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    pub fn xmc(&self) -> XMC_R {
        XMC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1(&self) -> QSPI1_R {
        QSPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    pub fn qspi2(&self) -> QSPI2_R {
        QSPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    pub fn sdio2(&self) -> SDIO2_R {
        SDIO2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN3")
            .field("xmc", &format_args!("{}", self.xmc().bit()))
            .field("qspi1", &format_args!("{}", self.qspi1().bit()))
            .field("qspi2", &format_args!("{}", self.qspi2().bit()))
            .field("sdio2", &format_args!("{}", self.sdio2().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBEN3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmc(&mut self) -> XMC_W<AHBEN3_SPEC, 0> {
        XMC_W::new(self)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1(&mut self) -> QSPI1_W<AHBEN3_SPEC, 1> {
        QSPI1_W::new(self)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi2(&mut self) -> QSPI2_W<AHBEN3_SPEC, 14> {
        QSPI2_W::new(self)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2(&mut self) -> SDIO2_W<AHBEN3_SPEC, 15> {
        SDIO2_W::new(self)
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
