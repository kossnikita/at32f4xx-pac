#[doc = "Register `REMAP7` reader"]
pub type R = crate::R<REMAP7_SPEC>;
#[doc = "Register `REMAP7` writer"]
pub type W = crate::W<REMAP7_SPEC>;
#[doc = "Field `EXT_SPIM_GMUX` reader - SPIM muxing"]
pub type EXT_SPIM_GMUX_R = crate::FieldReader;
#[doc = "Field `EXT_SPIM_GMUX` writer - SPIM muxing"]
pub type EXT_SPIM_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `EXT_SPIM_GEN` reader - SPIM enable"]
pub type EXT_SPIM_GEN_R = crate::BitReader;
#[doc = "Field `EXT_SPIM_GEN` writer - SPIM enable"]
pub type EXT_SPIM_GEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1_ETP_GMUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_GMUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETP_GMUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC1_ETO_GMUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_GMUX_R = crate::BitReader;
#[doc = "Field `ADC1_ETO_GMUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2_ETP_GMUX` reader - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_GMUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETP_GMUX` writer - ADC2 external trigger preempted conversion muxing"]
pub type ADC2_ETP_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC2_ETO_GMUX` reader - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_GMUX_R = crate::BitReader;
#[doc = "Field `ADC2_ETO_GMUX` writer - ADC2 external trigger ordinary conversion muxing"]
pub type ADC2_ETO_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWJTAG_GMUX` reader - Serial wire JTAG muxing"]
pub type SWJTAG_GMUX_R = crate::FieldReader;
#[doc = "Field `SWJTAG_GMUX` writer - Serial wire JTAG muxing"]
pub type SWJTAG_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PD01_GMUX` reader - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
pub type PD01_GMUX_R = crate::BitReader;
#[doc = "Field `PD01_GMUX` writer - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
pub type PD01_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XMC_GMUX` reader - XMC muxing"]
pub type XMC_GMUX_R = crate::FieldReader;
#[doc = "Field `XMC_GMUX` writer - XMC muxing"]
pub type XMC_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `XMC_NADV_GMUX` reader - XMC_NADV muxing"]
pub type XMC_NADV_GMUX_R = crate::BitReader;
#[doc = "Field `XMC_NADV_GMUX` writer - XMC_NADV muxing"]
pub type XMC_NADV_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
            .field(
                "ext_spim_gmux",
                &format_args!("{}", self.ext_spim_gmux().bits()),
            )
            .field(
                "ext_spim_gen",
                &format_args!("{}", self.ext_spim_gen().bit()),
            )
            .field(
                "adc1_etp_gmux",
                &format_args!("{}", self.adc1_etp_gmux().bit()),
            )
            .field(
                "adc1_eto_gmux",
                &format_args!("{}", self.adc1_eto_gmux().bit()),
            )
            .field(
                "adc2_etp_gmux",
                &format_args!("{}", self.adc2_etp_gmux().bit()),
            )
            .field(
                "adc2_eto_gmux",
                &format_args!("{}", self.adc2_eto_gmux().bit()),
            )
            .field(
                "swjtag_gmux",
                &format_args!("{}", self.swjtag_gmux().bits()),
            )
            .field("pd01_gmux", &format_args!("{}", self.pd01_gmux().bit()))
            .field("xmc_gmux", &format_args!("{}", self.xmc_gmux().bits()))
            .field(
                "xmc_nadv_gmux",
                &format_args!("{}", self.xmc_nadv_gmux().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPIM muxing"]
    #[inline(always)]
    #[must_use]
    pub fn ext_spim_gmux(&mut self) -> EXT_SPIM_GMUX_W<REMAP7_SPEC, 0> {
        EXT_SPIM_GMUX_W::new(self)
    }
    #[doc = "Bit 3 - SPIM enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_spim_gen(&mut self) -> EXT_SPIM_GEN_W<REMAP7_SPEC, 3> {
        EXT_SPIM_GEN_W::new(self)
    }
    #[doc = "Bit 4 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etp_gmux(&mut self) -> ADC1_ETP_GMUX_W<REMAP7_SPEC, 4> {
        ADC1_ETP_GMUX_W::new(self)
    }
    #[doc = "Bit 5 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_eto_gmux(&mut self) -> ADC1_ETO_GMUX_W<REMAP7_SPEC, 5> {
        ADC1_ETO_GMUX_W::new(self)
    }
    #[doc = "Bit 8 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_etp_gmux(&mut self) -> ADC2_ETP_GMUX_W<REMAP7_SPEC, 8> {
        ADC2_ETP_GMUX_W::new(self)
    }
    #[doc = "Bit 9 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_eto_gmux(&mut self) -> ADC2_ETO_GMUX_W<REMAP7_SPEC, 9> {
        ADC2_ETO_GMUX_W::new(self)
    }
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    #[must_use]
    pub fn swjtag_gmux(&mut self) -> SWJTAG_GMUX_W<REMAP7_SPEC, 16> {
        SWJTAG_GMUX_W::new(self)
    }
    #[doc = "Bit 20 - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_gmux(&mut self) -> PD01_GMUX_W<REMAP7_SPEC, 20> {
        PD01_GMUX_W::new(self)
    }
    #[doc = "Bits 24:26 - XMC muxing"]
    #[inline(always)]
    #[must_use]
    pub fn xmc_gmux(&mut self) -> XMC_GMUX_W<REMAP7_SPEC, 24> {
        XMC_GMUX_W::new(self)
    }
    #[doc = "Bit 27 - XMC_NADV muxing"]
    #[inline(always)]
    #[must_use]
    pub fn xmc_nadv_gmux(&mut self) -> XMC_NADV_GMUX_W<REMAP7_SPEC, 27> {
        XMC_NADV_GMUX_W::new(self)
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
#[doc = "IO MUX remap register 7 (IOMUX_REMAP7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP7_SPEC;
impl crate::RegisterSpec for REMAP7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap7::R`](R) reader structure"]
impl crate::Readable for REMAP7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap7::W`](W) writer structure"]
impl crate::Writable for REMAP7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP7 to value 0"]
impl crate::Resettable for REMAP7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
