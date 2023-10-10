#[doc = "Register `REMAP2` reader"]
pub type R = crate::R<REMAP2_SPEC>;
#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<REMAP2_SPEC>;
#[doc = "Field `TMR9_MUX` reader - TMR9 muxing"]
pub type TMR9_MUX_R = crate::BitReader;
#[doc = "Field `TMR9_MUX` writer - TMR9 muxing"]
pub type TMR9_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 5 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_mux(&self) -> TMR9_MUX_R {
        TMR9_MUX_R::new(((self.bits >> 5) & 1) != 0)
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
            .field("tmr9_mux", &format_args!("{}", self.tmr9_mux().bit()))
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
    #[doc = "Bit 5 - TMR9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_mux(&mut self) -> TMR9_MUX_W<REMAP2_SPEC, 5> {
        TMR9_MUX_W::new(self)
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
