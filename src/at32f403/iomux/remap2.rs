#[doc = "Register `REMAP2` reader"]
pub type R = crate::R<REMAP2_SPEC>;
#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<REMAP2_SPEC>;
#[doc = "Field `TMR15_MUX` reader - TMR15 muxing"]
pub type TMR15_MUX_R = crate::BitReader;
#[doc = "Field `TMR15_MUX` writer - TMR15 muxing"]
pub type TMR15_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR9_MUX` reader - TMR9 muxing"]
pub type TMR9_MUX_R = crate::BitReader;
#[doc = "Field `TMR9_MUX` writer - TMR9 muxing"]
pub type TMR9_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR10_MUX` reader - TMR10 muxing"]
pub type TMR10_MUX_R = crate::BitReader;
#[doc = "Field `TMR10_MUX` writer - TMR10 muxing"]
pub type TMR10_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR11_MUX` reader - TMR11 muxing"]
pub type TMR11_MUX_R = crate::BitReader;
#[doc = "Field `TMR11_MUX` writer - TMR11 muxing"]
pub type TMR11_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR13_MUX` reader - TMR13 muxing"]
pub type TMR13_MUX_R = crate::BitReader;
#[doc = "Field `TMR13_MUX` writer - TMR13 muxing"]
pub type TMR13_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR14_MUX` reader - TMR14 muxing"]
pub type TMR14_MUX_R = crate::BitReader;
#[doc = "Field `TMR14_MUX` writer - TMR14 muxing"]
pub type TMR14_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XMC_NADV_MUX` reader - NADV connect/disconnect"]
pub type XMC_NADV_MUX_R = crate::BitReader;
#[doc = "Field `XMC_NADV_MUX` writer - NADV connect/disconnect"]
pub type XMC_NADV_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4_MUX` reader - SPI4 muxing"]
pub type SPI4_MUX_R = crate::BitReader;
#[doc = "Field `SPI4_MUX` writer - SPI4 muxing"]
pub type SPI4_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C3_MUX` reader - I2C3 muxing"]
pub type I2C3_MUX_R = crate::BitReader;
#[doc = "Field `I2C3_MUX` writer - I2C3 muxing"]
pub type I2C3_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO2_MUX` reader - SDIO2 muxing"]
pub type SDIO2_MUX_R = crate::FieldReader;
#[doc = "Field `SDIO2_MUX` writer - SDIO2 muxing"]
pub type SDIO2_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EXT_SPIM_EN_MUX` reader - SPIM enable muxing"]
pub type EXT_SPIM_EN_MUX_R = crate::BitReader;
#[doc = "Field `EXT_SPIM_EN_MUX` writer - SPIM enable muxing"]
pub type EXT_SPIM_EN_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TMR15 muxing"]
    #[inline(always)]
    pub fn tmr15_mux(&self) -> TMR15_MUX_R {
        TMR15_MUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_mux(&self) -> TMR9_MUX_R {
        TMR9_MUX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TMR10 muxing"]
    #[inline(always)]
    pub fn tmr10_mux(&self) -> TMR10_MUX_R {
        TMR10_MUX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TMR11 muxing"]
    #[inline(always)]
    pub fn tmr11_mux(&self) -> TMR11_MUX_R {
        TMR11_MUX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TMR13 muxing"]
    #[inline(always)]
    pub fn tmr13_mux(&self) -> TMR13_MUX_R {
        TMR13_MUX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TMR14 muxing"]
    #[inline(always)]
    pub fn tmr14_mux(&self) -> TMR14_MUX_R {
        TMR14_MUX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn xmc_nadv_mux(&self) -> XMC_NADV_MUX_R {
        XMC_NADV_MUX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 17 - SPI4 muxing"]
    #[inline(always)]
    pub fn spi4_mux(&self) -> SPI4_MUX_R {
        SPI4_MUX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C3 muxing"]
    #[inline(always)]
    pub fn i2c3_mux(&self) -> I2C3_MUX_R {
        I2C3_MUX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - SDIO2 muxing"]
    #[inline(always)]
    pub fn sdio2_mux(&self) -> SDIO2_MUX_R {
        SDIO2_MUX_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - SPIM enable muxing"]
    #[inline(always)]
    pub fn ext_spim_en_mux(&self) -> EXT_SPIM_EN_MUX_R {
        EXT_SPIM_EN_MUX_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP2")
            .field("tmr15_mux", &format_args!("{}", self.tmr15_mux().bit()))
            .field("tmr9_mux", &format_args!("{}", self.tmr9_mux().bit()))
            .field("tmr10_mux", &format_args!("{}", self.tmr10_mux().bit()))
            .field("tmr11_mux", &format_args!("{}", self.tmr11_mux().bit()))
            .field("tmr13_mux", &format_args!("{}", self.tmr13_mux().bit()))
            .field("tmr14_mux", &format_args!("{}", self.tmr14_mux().bit()))
            .field(
                "xmc_nadv_mux",
                &format_args!("{}", self.xmc_nadv_mux().bit()),
            )
            .field("spi4_mux", &format_args!("{}", self.spi4_mux().bit()))
            .field("i2c3_mux", &format_args!("{}", self.i2c3_mux().bit()))
            .field("sdio2_mux", &format_args!("{}", self.sdio2_mux().bits()))
            .field(
                "ext_spim_en_mux",
                &format_args!("{}", self.ext_spim_en_mux().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - TMR15 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr15_mux(&mut self) -> TMR15_MUX_W<REMAP2_SPEC, 0> {
        TMR15_MUX_W::new(self)
    }
    #[doc = "Bit 5 - TMR9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_mux(&mut self) -> TMR9_MUX_W<REMAP2_SPEC, 5> {
        TMR9_MUX_W::new(self)
    }
    #[doc = "Bit 6 - TMR10 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_mux(&mut self) -> TMR10_MUX_W<REMAP2_SPEC, 6> {
        TMR10_MUX_W::new(self)
    }
    #[doc = "Bit 7 - TMR11 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_mux(&mut self) -> TMR11_MUX_W<REMAP2_SPEC, 7> {
        TMR11_MUX_W::new(self)
    }
    #[doc = "Bit 8 - TMR13 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13_mux(&mut self) -> TMR13_MUX_W<REMAP2_SPEC, 8> {
        TMR13_MUX_W::new(self)
    }
    #[doc = "Bit 9 - TMR14 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14_mux(&mut self) -> TMR14_MUX_W<REMAP2_SPEC, 9> {
        TMR14_MUX_W::new(self)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn xmc_nadv_mux(&mut self) -> XMC_NADV_MUX_W<REMAP2_SPEC, 10> {
        XMC_NADV_MUX_W::new(self)
    }
    #[doc = "Bit 17 - SPI4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn spi4_mux(&mut self) -> SPI4_MUX_W<REMAP2_SPEC, 17> {
        SPI4_MUX_W::new(self)
    }
    #[doc = "Bit 18 - I2C3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3_mux(&mut self) -> I2C3_MUX_W<REMAP2_SPEC, 18> {
        I2C3_MUX_W::new(self)
    }
    #[doc = "Bits 19:20 - SDIO2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn sdio2_mux(&mut self) -> SDIO2_MUX_W<REMAP2_SPEC, 19> {
        SDIO2_MUX_W::new(self)
    }
    #[doc = "Bit 21 - SPIM enable muxing"]
    #[inline(always)]
    #[must_use]
    pub fn ext_spim_en_mux(&mut self) -> EXT_SPIM_EN_MUX_W<REMAP2_SPEC, 21> {
        EXT_SPIM_EN_MUX_W::new(self)
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
#[doc = "IO MUX remap register 2 (IOMUX_REMAP2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP2_SPEC;
impl crate::RegisterSpec for REMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap2::R`](R) reader structure"]
impl crate::Readable for REMAP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap2::W`](W) writer structure"]
impl crate::Writable for REMAP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP2 to value 0"]
impl crate::Resettable for REMAP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
