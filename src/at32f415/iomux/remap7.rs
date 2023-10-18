#[doc = "Register `REMAP7` reader"]
pub type R = crate::R<REMAP7_SPEC>;
#[doc = "Register `REMAP7` writer"]
pub type W = crate::W<REMAP7_SPEC>;
#[doc = "Field `ADC1_ETP_GMUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_GMUX_R = crate::BitReader<ADC1_ETP_GMUX_A>;
#[doc = "ADC1 external trigger preempted conversion muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1_ETP_GMUX_A {
    #[doc = "0: ADC1 external trigger preempted conversion is connected to EXINT15"]
    Exint15 = 0,
    #[doc = "1: ADC1 external trigger preempted conversion is connected to TMR1 channel 4"]
    Tmr1ch4 = 1,
}
impl From<ADC1_ETP_GMUX_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_ETP_GMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1_ETP_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC1_ETP_GMUX_A {
        match self.bits {
            false => ADC1_ETP_GMUX_A::Exint15,
            true => ADC1_ETP_GMUX_A::Tmr1ch4,
        }
    }
    #[doc = "ADC1 external trigger preempted conversion is connected to EXINT15"]
    #[inline(always)]
    pub fn is_exint15(&self) -> bool {
        *self == ADC1_ETP_GMUX_A::Exint15
    }
    #[doc = "ADC1 external trigger preempted conversion is connected to TMR1 channel 4"]
    #[inline(always)]
    pub fn is_tmr1ch4(&self) -> bool {
        *self == ADC1_ETP_GMUX_A::Tmr1ch4
    }
}
#[doc = "Field `ADC1_ETP_GMUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADC1_ETP_GMUX_A>;
impl<'a, REG, const O: u8> ADC1_ETP_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC1 external trigger preempted conversion is connected to EXINT15"]
    #[inline(always)]
    pub fn exint15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETP_GMUX_A::Exint15)
    }
    #[doc = "ADC1 external trigger preempted conversion is connected to TMR1 channel 4"]
    #[inline(always)]
    pub fn tmr1ch4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETP_GMUX_A::Tmr1ch4)
    }
}
#[doc = "Field `ADC1_ETO_GMUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_GMUX_R = crate::BitReader<ADC1_ETO_GMUX_A>;
#[doc = "ADC1 external trigger ordinary conversion muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1_ETO_GMUX_A {
    #[doc = "0: ADC1 external trigger ordinary conversion is connected to EXINT11"]
    Exint11 = 0,
    #[doc = "1: ADC1 external trigger ordinary conversion TMR1_TRGO"]
    Tmr1Trgo = 1,
}
impl From<ADC1_ETO_GMUX_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_ETO_GMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1_ETO_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC1_ETO_GMUX_A {
        match self.bits {
            false => ADC1_ETO_GMUX_A::Exint11,
            true => ADC1_ETO_GMUX_A::Tmr1Trgo,
        }
    }
    #[doc = "ADC1 external trigger ordinary conversion is connected to EXINT11"]
    #[inline(always)]
    pub fn is_exint11(&self) -> bool {
        *self == ADC1_ETO_GMUX_A::Exint11
    }
    #[doc = "ADC1 external trigger ordinary conversion TMR1_TRGO"]
    #[inline(always)]
    pub fn is_tmr1_trgo(&self) -> bool {
        *self == ADC1_ETO_GMUX_A::Tmr1Trgo
    }
}
#[doc = "Field `ADC1_ETO_GMUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADC1_ETO_GMUX_A>;
impl<'a, REG, const O: u8> ADC1_ETO_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC1 external trigger ordinary conversion is connected to EXINT11"]
    #[inline(always)]
    pub fn exint11(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETO_GMUX_A::Exint11)
    }
    #[doc = "ADC1 external trigger ordinary conversion TMR1_TRGO"]
    #[inline(always)]
    pub fn tmr1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETO_GMUX_A::Tmr1Trgo)
    }
}
#[doc = "Field `SWJTAG_GMUX` reader - Serial wire JTAG muxing"]
pub type SWJTAG_GMUX_R = crate::FieldReader<SWJTAG_GMUX_A>;
#[doc = "Serial wire JTAG muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWJTAG_GMUX_A {
    #[doc = "0: Supports SWD and JTAG. All SWJTAG pins cannot be used as GPIOs"]
    SwdandJtag = 0,
    #[doc = "1: Supports SWD and JTAG. NJTRST is disabled. PB4 can be used as GPIO"]
    SwdandJtagwithoutNjtrst = 1,
    #[doc = "2: Supports SWD but JTAG is disabled. PA15/PB3/PB4 can be used as GPIOs"]
    Swd = 2,
    #[doc = "4: SWD and JTAG are disabled. All SWJTAG pins can be used as GPIOs"]
    Disable = 4,
}
impl From<SWJTAG_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SWJTAG_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWJTAG_GMUX_A {
    type Ux = u8;
}
impl SWJTAG_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWJTAG_GMUX_A> {
        match self.bits {
            0 => Some(SWJTAG_GMUX_A::SwdandJtag),
            1 => Some(SWJTAG_GMUX_A::SwdandJtagwithoutNjtrst),
            2 => Some(SWJTAG_GMUX_A::Swd),
            4 => Some(SWJTAG_GMUX_A::Disable),
            _ => None,
        }
    }
    #[doc = "Supports SWD and JTAG. All SWJTAG pins cannot be used as GPIOs"]
    #[inline(always)]
    pub fn is_swdand_jtag(&self) -> bool {
        *self == SWJTAG_GMUX_A::SwdandJtag
    }
    #[doc = "Supports SWD and JTAG. NJTRST is disabled. PB4 can be used as GPIO"]
    #[inline(always)]
    pub fn is_swdand_jtagwithout_njtrst(&self) -> bool {
        *self == SWJTAG_GMUX_A::SwdandJtagwithoutNjtrst
    }
    #[doc = "Supports SWD but JTAG is disabled. PA15/PB3/PB4 can be used as GPIOs"]
    #[inline(always)]
    pub fn is_swd(&self) -> bool {
        *self == SWJTAG_GMUX_A::Swd
    }
    #[doc = "SWD and JTAG are disabled. All SWJTAG pins can be used as GPIOs"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWJTAG_GMUX_A::Disable
    }
}
#[doc = "Field `SWJTAG_GMUX` writer - Serial wire JTAG muxing"]
pub type SWJTAG_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SWJTAG_GMUX_A>;
impl<'a, REG, const O: u8> SWJTAG_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Supports SWD and JTAG. All SWJTAG pins cannot be used as GPIOs"]
    #[inline(always)]
    pub fn swdand_jtag(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_GMUX_A::SwdandJtag)
    }
    #[doc = "Supports SWD and JTAG. NJTRST is disabled. PB4 can be used as GPIO"]
    #[inline(always)]
    pub fn swdand_jtagwithout_njtrst(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_GMUX_A::SwdandJtagwithoutNjtrst)
    }
    #[doc = "Supports SWD but JTAG is disabled. PA15/PB3/PB4 can be used as GPIOs"]
    #[inline(always)]
    pub fn swd(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_GMUX_A::Swd)
    }
    #[doc = "SWD and JTAG are disabled. All SWJTAG pins can be used as GPIOs"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_GMUX_A::Disable)
    }
}
#[doc = "Field `PD01_GMUX` reader - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
pub type PD01_GMUX_R = crate::BitReader<PD01_GMUX_A>;
#[doc = "PortD0/PortD1 muxing on OSC_IN/OSC_OUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD01_GMUX_A {
    #[doc = "0: Not PD0 and PD1 mapping"]
    Mux0 = 0,
    #[doc = "1: PD0 is mapped to HEXT_IN, while PD1 to HEXT_OUT"]
    Hext = 1,
}
impl From<PD01_GMUX_A> for bool {
    #[inline(always)]
    fn from(variant: PD01_GMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl PD01_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD01_GMUX_A {
        match self.bits {
            false => PD01_GMUX_A::Mux0,
            true => PD01_GMUX_A::Hext,
        }
    }
    #[doc = "Not PD0 and PD1 mapping"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == PD01_GMUX_A::Mux0
    }
    #[doc = "PD0 is mapped to HEXT_IN, while PD1 to HEXT_OUT"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == PD01_GMUX_A::Hext
    }
}
#[doc = "Field `PD01_GMUX` writer - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
pub type PD01_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PD01_GMUX_A>;
impl<'a, REG, const O: u8> PD01_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not PD0 and PD1 mapping"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(PD01_GMUX_A::Mux0)
    }
    #[doc = "PD0 is mapped to HEXT_IN, while PD1 to HEXT_OUT"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(PD01_GMUX_A::Hext)
    }
}
impl R {
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
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_gmux(&self) -> SWJTAG_GMUX_R {
        SWJTAG_GMUX_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_gmux(&self) -> PD01_GMUX_R {
        PD01_GMUX_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP7")
            .field("pd01_gmux", &format_args!("{}", self.pd01_gmux().bit()))
            .field(
                "swjtag_gmux",
                &format_args!("{}", self.swjtag_gmux().bits()),
            )
            .field(
                "adc1_eto_gmux",
                &format_args!("{}", self.adc1_eto_gmux().bit()),
            )
            .field(
                "adc1_etp_gmux",
                &format_args!("{}", self.adc1_etp_gmux().bit()),
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
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    #[must_use]
    pub fn swjtag_gmux(&mut self) -> SWJTAG_GMUX_W<REMAP7_SPEC, 16> {
        SWJTAG_GMUX_W::new(self)
    }
    #[doc = "Bit 20 - PortD0/PortD1 muxing on OSC_IN/OSC_OUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_gmux(&mut self) -> PD01_GMUX_W<REMAP7_SPEC, 20> {
        PD01_GMUX_W::new(self)
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
#[doc = "IO MUX remap register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
