#[doc = "Register `REMAP` reader"]
pub type R = crate::R<REMAP_SPEC>;
#[doc = "Register `REMAP` writer"]
pub type W = crate::W<REMAP_SPEC>;
#[doc = "Field `SPI1_MUX0` reader - SPI1 muxing bit0"]
pub type SPI1_MUX0_R = crate::BitReader;
#[doc = "Field `SPI1_MUX0` writer - SPI1 muxing bit0"]
pub type SPI1_MUX0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_MUX` reader - I2C1 muxing"]
pub type I2C1_MUX_R = crate::BitReader;
#[doc = "Field `I2C1_MUX` writer - I2C1 muxing"]
pub type I2C1_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_MUX` reader - USART1 muxing"]
pub type USART1_MUX_R = crate::BitReader;
#[doc = "Field `USART1_MUX` writer - USART1 muxing"]
pub type USART1_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2_MUX` reader - USART2 muxing"]
pub type USART2_MUX_R = crate::BitReader;
#[doc = "Field `USART2_MUX` writer - USART2 muxing"]
pub type USART2_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3_MUX` reader - USART3 muxing"]
pub type USART3_MUX_R = crate::FieldReader;
#[doc = "Field `USART3_MUX` writer - USART3 muxing"]
pub type USART3_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR1_MUX` reader - TMR1 muxing"]
pub type TMR1_MUX_R = crate::FieldReader;
#[doc = "Field `TMR1_MUX` writer - TMR1 muxing"]
pub type TMR1_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR2_MUX` reader - TMR2 muxing"]
pub type TMR2_MUX_R = crate::FieldReader;
#[doc = "Field `TMR2_MUX` writer - TMR2 muxing"]
pub type TMR2_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR3_MUX` reader - TMR3 muxing"]
pub type TMR3_MUX_R = crate::FieldReader;
#[doc = "Field `TMR3_MUX` writer - TMR3 muxing"]
pub type TMR3_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR4_MUX` reader - TMR4 muxing"]
pub type TMR4_MUX_R = crate::BitReader;
#[doc = "Field `TMR4_MUX` writer - TMR4 muxing"]
pub type TMR4_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN_MUX` reader - CAN1 muxing"]
pub type CAN_MUX_R = crate::FieldReader;
#[doc = "Field `CAN_MUX` writer - CAN1 muxing"]
pub type CAN_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD01_MUX` reader - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type PD01_MUX_R = crate::BitReader;
#[doc = "Field `PD01_MUX` writer - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type PD01_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5CH4_MUX` reader - TMR5 channel4 internal muxing"]
pub type TMR5CH4_MUX_R = crate::BitReader;
#[doc = "Field `TMR5CH4_MUX` writer - TMR5 channel4 internal muxing"]
pub type TMR5CH4_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETP_MUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_MUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETP_MUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETO_MUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_MUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETO_MUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETP_MUX` reader - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_MUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETP_MUX` writer - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETO_MUX` reader - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_MUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETO_MUX` writer - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJTAG_MUX` reader - SWD JTAG muxing"]
pub type SWJTAG_MUX_R = crate::FieldReader;
#[doc = "Field `SWJTAG_MUX` writer - SWD JTAG muxing"]
pub type SWJTAG_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI1_MUX1` reader - SPI1 muxing bit1"]
pub type SPI1_MUX1_R = crate::BitReader;
#[doc = "Field `SPI1_MUX1` writer - SPI1 muxing bit1"]
pub type SPI1_MUX1_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - USART2 muxing"]
    #[inline(always)]
    pub fn usart2_mux(&self) -> USART2_MUX_R {
        USART2_MUX_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 12 - TMR4 muxing"]
    #[inline(always)]
    pub fn tmr4_mux(&self) -> TMR4_MUX_R {
        TMR4_MUX_R::new(((self.bits >> 12) & 1) != 0)
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
            .field("spi1_mux0", &self.spi1_mux0())
            .field("i2c1_mux", &self.i2c1_mux())
            .field("usart1_mux", &self.usart1_mux())
            .field("usart2_mux", &self.usart2_mux())
            .field("usart3_mux", &self.usart3_mux())
            .field("tmr1_mux", &self.tmr1_mux())
            .field("tmr2_mux", &self.tmr2_mux())
            .field("tmr3_mux", &self.tmr3_mux())
            .field("tmr4_mux", &self.tmr4_mux())
            .field("can_mux", &self.can_mux())
            .field("pd01_mux", &self.pd01_mux())
            .field("tmr5ch4_mux", &self.tmr5ch4_mux())
            .field("adc1_etp_mux", &self.adc1_etp_mux())
            .field("adc1_eto_mux", &self.adc1_eto_mux())
            .field("adc2_etp_mux", &self.adc2_etp_mux())
            .field("adc2_eto_mux", &self.adc2_eto_mux())
            .field("swjtag_mux", &self.swjtag_mux())
            .field("spi1_mux1", &self.spi1_mux1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    pub fn spi1_mux0(&mut self) -> SPI1_MUX0_W<'_, REMAP_SPEC> {
        SPI1_MUX0_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_mux(&mut self) -> I2C1_MUX_W<'_, REMAP_SPEC> {
        I2C1_MUX_W::new(self, 1)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_mux(&mut self) -> USART1_MUX_W<'_, REMAP_SPEC> {
        USART1_MUX_W::new(self, 2)
    }
    #[doc = "Bit 3 - USART2 muxing"]
    #[inline(always)]
    pub fn usart2_mux(&mut self) -> USART2_MUX_W<'_, REMAP_SPEC> {
        USART2_MUX_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_mux(&mut self) -> USART3_MUX_W<'_, REMAP_SPEC> {
        USART3_MUX_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_mux(&mut self) -> TMR1_MUX_W<'_, REMAP_SPEC> {
        TMR1_MUX_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_mux(&mut self) -> TMR2_MUX_W<'_, REMAP_SPEC> {
        TMR2_MUX_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_mux(&mut self) -> TMR3_MUX_W<'_, REMAP_SPEC> {
        TMR3_MUX_W::new(self, 10)
    }
    #[doc = "Bit 12 - TMR4 muxing"]
    #[inline(always)]
    pub fn tmr4_mux(&mut self) -> TMR4_MUX_W<'_, REMAP_SPEC> {
        TMR4_MUX_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    pub fn can_mux(&mut self) -> CAN_MUX_W<'_, REMAP_SPEC> {
        CAN_MUX_W::new(self, 13)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_mux(&mut self) -> PD01_MUX_W<'_, REMAP_SPEC> {
        PD01_MUX_W::new(self, 15)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_mux(&mut self) -> TMR5CH4_MUX_W<'_, REMAP_SPEC> {
        TMR5CH4_MUX_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_mux(&mut self) -> ADC1_ETP_MUX_W<'_, REMAP_SPEC> {
        ADC1_ETP_MUX_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_mux(&mut self) -> ADC1_ETO_MUX_W<'_, REMAP_SPEC> {
        ADC1_ETO_MUX_W::new(self, 18)
    }
    #[doc = "Bit 19 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_mux(&mut self) -> ADC2_ETP_MUX_W<'_, REMAP_SPEC> {
        ADC2_ETP_MUX_W::new(self, 19)
    }
    #[doc = "Bit 20 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_mux(&mut self) -> ADC2_ETO_MUX_W<'_, REMAP_SPEC> {
        ADC2_ETO_MUX_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_mux(&mut self) -> SWJTAG_MUX_W<'_, REMAP_SPEC> {
        SWJTAG_MUX_W::new(self, 24)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    pub fn spi1_mux1(&mut self) -> SPI1_MUX1_W<'_, REMAP_SPEC> {
        SPI1_MUX1_W::new(self, 31)
    }
}
#[doc = "IO MUX remap register (IOMUX_REMAP)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP_SPEC;
impl crate::RegisterSpec for REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap::R`](R) reader structure"]
impl crate::Readable for REMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap::W`](W) writer structure"]
impl crate::Writable for REMAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for REMAP_SPEC {}
