#[doc = "Register `AHBRST3` reader"]
pub type R = crate::R<AHBRST3_SPEC>;
#[doc = "Register `AHBRST3` writer"]
pub type W = crate::W<AHBRST3_SPEC>;
#[doc = "Field `XMCRST` reader - XMC reset"]
pub type XMCRST_R = crate::BitReader;
#[doc = "Field `XMCRST` writer - XMC reset"]
pub type XMCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI1RST` reader - QSPI1 reset"]
pub type QSPI1RST_R = crate::BitReader;
#[doc = "Field `QSPI1RST` writer - QSPI1 reset"]
pub type QSPI1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI2RST` reader - QSPI2 reset"]
pub type QSPI2RST_R = crate::BitReader;
#[doc = "Field `QSPI2RST` writer - QSPI2 reset"]
pub type QSPI2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO2RST` reader - SDIO2 reset"]
pub type SDIO2RST_R = crate::BitReader;
#[doc = "Field `SDIO2RST` writer - SDIO2 reset"]
pub type SDIO2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - XMC reset"]
    #[inline(always)]
    pub fn xmcrst(&self) -> XMCRST_R {
        XMCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 reset"]
    #[inline(always)]
    pub fn qspi1rst(&self) -> QSPI1RST_R {
        QSPI1RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 reset"]
    #[inline(always)]
    pub fn qspi2rst(&self) -> QSPI2RST_R {
        QSPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO2 reset"]
    #[inline(always)]
    pub fn sdio2rst(&self) -> SDIO2RST_R {
        SDIO2RST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn xmcrst(&mut self) -> XMCRST_W<AHBRST3_SPEC, 0> {
        XMCRST_W::new(self)
    }
    #[doc = "Bit 1 - QSPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1rst(&mut self) -> QSPI1RST_W<AHBRST3_SPEC, 1> {
        QSPI1RST_W::new(self)
    }
    #[doc = "Bit 14 - QSPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspi2rst(&mut self) -> QSPI2RST_W<AHBRST3_SPEC, 14> {
        QSPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SDIO2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2rst(&mut self) -> SDIO2RST_W<AHBRST3_SPEC, 15> {
        SDIO2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB peripheral reset register 3 (CRM_AHBRST3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST3_SPEC;
impl crate::RegisterSpec for AHBRST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst3::R`](R) reader structure"]
impl crate::Readable for AHBRST3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst3::W`](W) writer structure"]
impl crate::Writable for AHBRST3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST3 to value 0"]
impl crate::Resettable for AHBRST3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
