#[doc = "Register `REMAP7` reader"]
pub type R = crate::R<REMAP7_SPEC>;
#[doc = "Register `REMAP7` writer"]
pub type W = crate::W<REMAP7_SPEC>;
#[doc = "Field `EXT_SPIM_GMUX` reader - SPIM muxing"]
pub type EXT_SPIM_GMUX_R = crate::FieldReader;
#[doc = "Field `EXT_SPIM_GMUX` writer - SPIM muxing"]
pub type EXT_SPIM_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXT_SPIM_GEN` reader - SPIM enable"]
pub type EXT_SPIM_GEN_R = crate::BitReader;
#[doc = "Field `EXT_SPIM_GEN` writer - SPIM enable"]
pub type EXT_SPIM_GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETP_GMUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_GMUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETP_GMUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETO_GMUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_GMUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETO_GMUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETP_GMUX` reader - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_GMUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETP_GMUX` writer - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETO_GMUX` reader - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_GMUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETO_GMUX` writer - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJTAG_GMUX` reader - Serial wire JTAG muxing"]
pub type SWJTAG_GMUX_R = crate::FieldReader;
#[doc = "Field `SWJTAG_GMUX` writer - Serial wire JTAG muxing"]
pub type SWJTAG_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD01_GMUX` reader - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
pub type PD01_GMUX_R = crate::BitReader;
#[doc = "Field `PD01_GMUX` writer - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
pub type PD01_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMC_GMUX` reader - XMC muxing"]
pub type XMC_GMUX_R = crate::FieldReader;
#[doc = "Field `XMC_GMUX` writer - XMC muxing"]
pub type XMC_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XMC_NADV_GMUX` reader - XMC_NADV muxing"]
pub type XMC_NADV_GMUX_R = crate::BitReader;
#[doc = "Field `XMC_NADV_GMUX` writer - XMC_NADV muxing"]
pub type XMC_NADV_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SPIM muxing"]
    #[inline(always)]
    pub fn ext_spim_gmux(&self) -> EXT_SPIM_GMUX_R {
        EXT_SPIM_GMUX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SPIM enable"]
    #[inline(always)]
    pub fn ext_spim_gen(&self) -> EXT_SPIM_GEN_R {
        EXT_SPIM_GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_gmux(&self) -> ADC1_ETP_GMUX_R {
        ADC1_ETP_GMUX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_gmux(&self) -> ADC1_ETO_GMUX_R {
        ADC1_ETO_GMUX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_gmux(&self) -> ADC2_ETP_GMUX_R {
        ADC2_ETP_GMUX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_gmux(&self) -> ADC2_ETO_GMUX_R {
        ADC2_ETO_GMUX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_gmux(&self) -> SWJTAG_GMUX_R {
        SWJTAG_GMUX_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_gmux(&self) -> PD01_GMUX_R {
        PD01_GMUX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - XMC muxing"]
    #[inline(always)]
    pub fn xmc_gmux(&self) -> XMC_GMUX_R {
        XMC_GMUX_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - XMC_NADV muxing"]
    #[inline(always)]
    pub fn xmc_nadv_gmux(&self) -> XMC_NADV_GMUX_R {
        XMC_NADV_GMUX_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP7")
            .field("ext_spim_gmux", &self.ext_spim_gmux())
            .field("ext_spim_gen", &self.ext_spim_gen())
            .field("adc1_etp_gmux", &self.adc1_etp_gmux())
            .field("adc1_eto_gmux", &self.adc1_eto_gmux())
            .field("adc2_etp_gmux", &self.adc2_etp_gmux())
            .field("adc2_eto_gmux", &self.adc2_eto_gmux())
            .field("swjtag_gmux", &self.swjtag_gmux())
            .field("pd01_gmux", &self.pd01_gmux())
            .field("xmc_gmux", &self.xmc_gmux())
            .field("xmc_nadv_gmux", &self.xmc_nadv_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SPIM muxing"]
    #[inline(always)]
    #[must_use]
    pub fn ext_spim_gmux(&mut self) -> EXT_SPIM_GMUX_W<REMAP7_SPEC> {
        EXT_SPIM_GMUX_W::new(self, 0)
    }
    #[doc = "Bit 3 - SPIM enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_spim_gen(&mut self) -> EXT_SPIM_GEN_W<REMAP7_SPEC> {
        EXT_SPIM_GEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etp_gmux(&mut self) -> ADC1_ETP_GMUX_W<REMAP7_SPEC> {
        ADC1_ETP_GMUX_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_eto_gmux(&mut self) -> ADC1_ETO_GMUX_W<REMAP7_SPEC> {
        ADC1_ETO_GMUX_W::new(self, 5)
    }
    #[doc = "Bit 8 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etp_gmux(&mut self) -> ADC2_ETP_GMUX_W<REMAP7_SPEC> {
        ADC2_ETP_GMUX_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_eto_gmux(&mut self) -> ADC2_ETO_GMUX_W<REMAP7_SPEC> {
        ADC2_ETO_GMUX_W::new(self, 9)
    }
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    #[must_use]
    pub fn swjtag_gmux(&mut self) -> SWJTAG_GMUX_W<REMAP7_SPEC> {
        SWJTAG_GMUX_W::new(self, 16)
    }
    #[doc = "Bit 20 - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_gmux(&mut self) -> PD01_GMUX_W<REMAP7_SPEC> {
        PD01_GMUX_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - XMC muxing"]
    #[inline(always)]
    #[must_use]
    pub fn xmc_gmux(&mut self) -> XMC_GMUX_W<REMAP7_SPEC> {
        XMC_GMUX_W::new(self, 24)
    }
    #[doc = "Bit 27 - XMC_NADV muxing"]
    #[inline(always)]
    #[must_use]
    pub fn xmc_nadv_gmux(&mut self) -> XMC_NADV_GMUX_W<REMAP7_SPEC> {
        XMC_NADV_GMUX_W::new(self, 27)
    }
}
#[doc = "IO MUX remap register 7 (IOMUX_REMAP7)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP7_SPEC;
impl crate::RegisterSpec for REMAP7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap7::R`](R) reader structure"]
impl crate::Readable for REMAP7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap7::W`](W) writer structure"]
impl crate::Writable for REMAP7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP7 to value 0"]
impl crate::Resettable for REMAP7_SPEC {
    const RESET_VALUE: u32 = 0;
}
