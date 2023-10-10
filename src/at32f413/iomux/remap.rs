#[doc = "Register `REMAP` reader"]
pub type R = crate::R<REMAP_SPEC>;
#[doc = "Register `REMAP` writer"]
pub type W = crate::W<REMAP_SPEC>;
#[doc = "Field `SPI1_MUX0` reader - SPI1 muxing bit0"]
pub type SPI1_MUX0_R = crate::BitReader;
#[doc = "Field `SPI1_MUX0` writer - SPI1 muxing bit0"]
pub type SPI1_MUX0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1_MUX` reader - I2C1 muxing"]
pub type I2C1_MUX_R = crate::BitReader;
#[doc = "Field `I2C1_MUX` writer - I2C1 muxing"]
pub type I2C1_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1_MUX` reader - USART1 muxing"]
pub type USART1_MUX_R = crate::BitReader;
#[doc = "Field `USART1_MUX` writer - USART1 muxing"]
pub type USART1_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3_MUX` reader - USART3 muxing"]
pub type USART3_MUX_R = crate::FieldReader;
#[doc = "Field `USART3_MUX` writer - USART3 muxing"]
pub type USART3_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TMR1_MUX` reader - TMR1 muxing"]
pub type TMR1_MUX_R = crate::FieldReader;
#[doc = "Field `TMR1_MUX` writer - TMR1 muxing"]
pub type TMR1_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TMR2_MUX` reader - TMR2 muxing"]
pub type TMR2_MUX_R = crate::FieldReader;
#[doc = "Field `TMR2_MUX` writer - TMR2 muxing"]
pub type TMR2_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TMR3_MUX` reader - TMR3 muxing"]
pub type TMR3_MUX_R = crate::FieldReader;
#[doc = "Field `TMR3_MUX` writer - TMR3 muxing"]
pub type TMR3_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CAN_MUX` reader - CAN1 muxing"]
pub type CAN_MUX_R = crate::FieldReader;
#[doc = "Field `CAN_MUX` writer - CAN1 muxing"]
pub type CAN_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PD01_MUX` reader - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type PD01_MUX_R = crate::BitReader;
#[doc = "Field `PD01_MUX` writer - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type PD01_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR5CH4_MUX` reader - TMR5 channel4 internal muxing"]
pub type TMR5CH4_MUX_R = crate::BitReader;
#[doc = "Field `TMR5CH4_MUX` writer - TMR5 channel4 internal muxing"]
pub type TMR5CH4_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1_ETP_MUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_MUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETP_MUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1_ETO_MUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_MUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETO_MUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2_ETP_MUX` reader - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_MUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETP_MUX` writer - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2_ETO_MUX` reader - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_MUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETO_MUX` writer - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWJTAG_MUX` reader - SWD JTAG muxing"]
pub type SWJTAG_MUX_R = crate::FieldReader;
#[doc = "Field `SWJTAG_MUX` writer - SWD JTAG muxing"]
pub type SWJTAG_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI1_MUX1` reader - SPI1 muxing bit1"]
pub type SPI1_MUX1_R = crate::BitReader;
#[doc = "Field `SPI1_MUX1` writer - SPI1 muxing bit1"]
pub type SPI1_MUX1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    pub fn spi1_mux0(&self) -> SPI1_MUX0_R {
        SPI1_MUX0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_mux(&self) -> I2C1_MUX_R {
        I2C1_MUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_mux(&self) -> USART1_MUX_R {
        USART1_MUX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_mux(&self) -> USART3_MUX_R {
        USART3_MUX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_mux(&self) -> TMR1_MUX_R {
        TMR1_MUX_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_mux(&self) -> TMR2_MUX_R {
        TMR2_MUX_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_mux(&self) -> TMR3_MUX_R {
        TMR3_MUX_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    pub fn can_mux(&self) -> CAN_MUX_R {
        CAN_MUX_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_mux(&self) -> PD01_MUX_R {
        PD01_MUX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_mux(&self) -> TMR5CH4_MUX_R {
        TMR5CH4_MUX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_mux(&self) -> ADC1_ETP_MUX_R {
        ADC1_ETP_MUX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_mux(&self) -> ADC1_ETO_MUX_R {
        ADC1_ETO_MUX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_mux(&self) -> ADC2_ETP_MUX_R {
        ADC2_ETP_MUX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_mux(&self) -> ADC2_ETO_MUX_R {
        ADC2_ETO_MUX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_mux(&self) -> SWJTAG_MUX_R {
        SWJTAG_MUX_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    pub fn spi1_mux1(&self) -> SPI1_MUX1_R {
        SPI1_MUX1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP")
            .field("spi1_mux0", &format_args!("{}", self.spi1_mux0().bit()))
            .field("i2c1_mux", &format_args!("{}", self.i2c1_mux().bit()))
            .field("usart1_mux", &format_args!("{}", self.usart1_mux().bit()))
            .field("usart3_mux", &format_args!("{}", self.usart3_mux().bits()))
            .field("tmr1_mux", &format_args!("{}", self.tmr1_mux().bits()))
            .field("tmr2_mux", &format_args!("{}", self.tmr2_mux().bits()))
            .field("tmr3_mux", &format_args!("{}", self.tmr3_mux().bits()))
            .field("can_mux", &format_args!("{}", self.can_mux().bits()))
            .field("pd01_mux", &format_args!("{}", self.pd01_mux().bit()))
            .field("tmr5ch4_mux", &format_args!("{}", self.tmr5ch4_mux().bit()))
            .field(
                "adc1_etp_mux",
                &format_args!("{}", self.adc1_etp_mux().bit()),
            )
            .field(
                "adc1_eto_mux",
                &format_args!("{}", self.adc1_eto_mux().bit()),
            )
            .field(
                "adc2_etp_mux",
                &format_args!("{}", self.adc2_etp_mux().bit()),
            )
            .field(
                "adc2_eto_mux",
                &format_args!("{}", self.adc2_eto_mux().bit()),
            )
            .field("swjtag_mux", &format_args!("{}", self.swjtag_mux().bits()))
            .field("spi1_mux1", &format_args!("{}", self.spi1_mux1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_mux0(&mut self) -> SPI1_MUX0_W<REMAP_SPEC, 0> {
        SPI1_MUX0_W::new(self)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_mux(&mut self) -> I2C1_MUX_W<REMAP_SPEC, 1> {
        I2C1_MUX_W::new(self)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_mux(&mut self) -> USART1_MUX_W<REMAP_SPEC, 2> {
        USART1_MUX_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_mux(&mut self) -> USART3_MUX_W<REMAP_SPEC, 4> {
        USART3_MUX_W::new(self)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_mux(&mut self) -> TMR1_MUX_W<REMAP_SPEC, 6> {
        TMR1_MUX_W::new(self)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_mux(&mut self) -> TMR2_MUX_W<REMAP_SPEC, 8> {
        TMR2_MUX_W::new(self)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_mux(&mut self) -> TMR3_MUX_W<REMAP_SPEC, 10> {
        TMR3_MUX_W::new(self)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can_mux(&mut self) -> CAN_MUX_W<REMAP_SPEC, 13> {
        CAN_MUX_W::new(self)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_mux(&mut self) -> PD01_MUX_W<REMAP_SPEC, 15> {
        PD01_MUX_W::new(self)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5ch4_mux(&mut self) -> TMR5CH4_MUX_W<REMAP_SPEC, 16> {
        TMR5CH4_MUX_W::new(self)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etp_mux(&mut self) -> ADC1_ETP_MUX_W<REMAP_SPEC, 17> {
        ADC1_ETP_MUX_W::new(self)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_eto_mux(&mut self) -> ADC1_ETO_MUX_W<REMAP_SPEC, 18> {
        ADC1_ETO_MUX_W::new(self)
    }
    #[doc = "Bit 19 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etp_mux(&mut self) -> ADC2_ETP_MUX_W<REMAP_SPEC, 19> {
        ADC2_ETP_MUX_W::new(self)
    }
    #[doc = "Bit 20 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_eto_mux(&mut self) -> ADC2_ETO_MUX_W<REMAP_SPEC, 20> {
        ADC2_ETO_MUX_W::new(self)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    #[must_use]
    pub fn swjtag_mux(&mut self) -> SWJTAG_MUX_W<REMAP_SPEC, 24> {
        SWJTAG_MUX_W::new(self)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_mux1(&mut self) -> SPI1_MUX1_W<REMAP_SPEC, 31> {
        SPI1_MUX1_W::new(self)
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
#[doc = "IO MUX remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP_SPEC;
impl crate::RegisterSpec for REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap::R`](R) reader structure"]
impl crate::Readable for REMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap::W`](W) writer structure"]
impl crate::Writable for REMAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for REMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
