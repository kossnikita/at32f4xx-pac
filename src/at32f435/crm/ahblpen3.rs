#[doc = "Register `AHBLPEN3` reader"]
pub type R = crate::R<AHBLPEN3_SPEC>;
#[doc = "Register `AHBLPEN3` writer"]
pub type W = crate::W<AHBLPEN3_SPEC>;
#[doc = "Field `XMCLP` reader - XMC clock enable during sleep mode"]
pub type XMCLP_R = crate::BitReader;
#[doc = "Field `XMCLP` writer - XMC clock enable during sleep mode"]
pub type XMCLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI1LP` reader - QSPI1 clock enable during sleep mode"]
pub type QSPI1LP_R = crate::BitReader;
#[doc = "Field `QSPI1LP` writer - QSPI1 clock enable during sleep mode"]
pub type QSPI1LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPI2LP` reader - QSPI2 clock enable during sleep mode"]
pub type QSPI2LP_R = crate::BitReader;
#[doc = "Field `QSPI2LP` writer - QSPI2 clock enable during sleep mode"]
pub type QSPI2LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO2LP` reader - SDIO2 clock enable during sleep mode"]
pub type SDIO2LP_R = crate::BitReader;
#[doc = "Field `SDIO2LP` writer - SDIO2 clock enable during sleep mode"]
pub type SDIO2LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - XMC clock enable during sleep mode"]
    #[inline(always)]
    pub fn xmclp(&self) -> XMCLP_R {
        XMCLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi1lp(&self) -> QSPI1LP_R {
        QSPI1LP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi2lp(&self) -> QSPI2LP_R {
        QSPI2LP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio2lp(&self) -> SDIO2LP_R {
        SDIO2LP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN3")
            .field("xmclp", &format_args!("{}", self.xmclp().bit()))
            .field("qspi1lp", &format_args!("{}", self.qspi1lp().bit()))
            .field("qspi2lp", &format_args!("{}", self.qspi2lp().bit()))
            .field("sdio2lp", &format_args!("{}", self.sdio2lp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBLPEN3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - XMC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn xmclp(&mut self) -> XMCLP_W<AHBLPEN3_SPEC, 0> {
        XMCLP_W::new(self)
    }
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1lp(&mut self) -> QSPI1LP_W<AHBLPEN3_SPEC, 1> {
        QSPI1LP_W::new(self)
    }
    #[doc = "Bit 14 - QSPI2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn qspi2lp(&mut self) -> QSPI2LP_W<AHBLPEN3_SPEC, 14> {
        QSPI2LP_W::new(self)
    }
    #[doc = "Bit 15 - SDIO2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2lp(&mut self) -> SDIO2LP_W<AHBLPEN3_SPEC, 15> {
        SDIO2LP_W::new(self)
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
#[doc = "AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN3_SPEC;
impl crate::RegisterSpec for AHBLPEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen3::R`](R) reader structure"]
impl crate::Readable for AHBLPEN3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen3::W`](W) writer structure"]
impl crate::Writable for AHBLPEN3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBLPEN3 to value 0xc003"]
impl crate::Resettable for AHBLPEN3_SPEC {
    const RESET_VALUE: Self::Ux = 0xc003;
}
