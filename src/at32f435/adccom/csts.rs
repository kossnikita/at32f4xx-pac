#[doc = "Register `CSTS` reader"]
pub type R = crate::R<CSTS_SPEC>;
#[doc = "Field `VMOR1` reader - Voltage monitoring out of range flag of ADC1"]
pub type VMOR1_R = crate::BitReader;
#[doc = "Field `OCCE1` reader - Ordinary channels conversion end flag of ADC1"]
pub type OCCE1_R = crate::BitReader;
#[doc = "Field `PCCE1` reader - Preempted channels conversion end flag of ADC1"]
pub type PCCE1_R = crate::BitReader;
#[doc = "Field `PCCS1` reader - Preempted channel conversion start flag of ADC1"]
pub type PCCS1_R = crate::BitReader;
#[doc = "Field `OCCS1` reader - Ordinary channel conversion start flag of ADC1"]
pub type OCCS1_R = crate::BitReader;
#[doc = "Field `OCCO1` reader - Ordinary channel conversion overflow flag of ADC1"]
pub type OCCO1_R = crate::BitReader;
#[doc = "Field `RDY1` reader - ADC ready to conversion flag of ADC1"]
pub type RDY1_R = crate::BitReader;
#[doc = "Field `VMOR2` reader - Voltage monitoring out of range flag of ADC2"]
pub type VMOR2_R = crate::BitReader;
#[doc = "Field `OCCE2` reader - Ordinary channels conversion end flag of ADC2"]
pub type OCCE2_R = crate::BitReader;
#[doc = "Field `PCCE2` reader - Preempted channels conversion end flag of ADC2"]
pub type PCCE2_R = crate::BitReader;
#[doc = "Field `PCCS2` reader - Preempted channel conversion start flag of ADC2"]
pub type PCCS2_R = crate::BitReader;
#[doc = "Field `OCCS2` reader - Ordinary channel conversion start flag of ADC2"]
pub type OCCS2_R = crate::BitReader;
#[doc = "Field `OCCO2` reader - Ordinary channel conversion overflow flag of ADC2"]
pub type OCCO2_R = crate::BitReader;
#[doc = "Field `RDY2` reader - ADC ready to conversion flag of ADC2"]
pub type RDY2_R = crate::BitReader;
#[doc = "Field `VMOR3` reader - Voltage monitoring out of range flag of ADC3"]
pub type VMOR3_R = crate::BitReader;
#[doc = "Field `OCCE3` reader - Ordinary channels conversion end flag of ADC3"]
pub type OCCE3_R = crate::BitReader;
#[doc = "Field `PCCE3` reader - Preempted channels conversion end flag of ADC3"]
pub type PCCE3_R = crate::BitReader;
#[doc = "Field `PCCS3` reader - Preempted channel conversion start flag of ADC3"]
pub type PCCS3_R = crate::BitReader;
#[doc = "Field `OCCS3` reader - Ordinary channel conversion start flag of ADC3"]
pub type OCCS3_R = crate::BitReader;
#[doc = "Field `OCCO3` reader - Ordinary channel conversion overflow flag of ADC3"]
pub type OCCO3_R = crate::BitReader;
#[doc = "Field `RDY3` reader - ADC ready to conversion flag of ADC3"]
pub type RDY3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage monitoring out of range flag of ADC1"]
    #[inline(always)]
    pub fn vmor1(&self) -> VMOR1_R {
        VMOR1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag of ADC1"]
    #[inline(always)]
    pub fn occe1(&self) -> OCCE1_R {
        OCCE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag of ADC1"]
    #[inline(always)]
    pub fn pcce1(&self) -> PCCE1_R {
        PCCE1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag of ADC1"]
    #[inline(always)]
    pub fn pccs1(&self) -> PCCS1_R {
        PCCS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag of ADC1"]
    #[inline(always)]
    pub fn occs1(&self) -> OCCS1_R {
        OCCS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag of ADC1"]
    #[inline(always)]
    pub fn occo1(&self) -> OCCO1_R {
        OCCO1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC ready to conversion flag of ADC1"]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage monitoring out of range flag of ADC2"]
    #[inline(always)]
    pub fn vmor2(&self) -> VMOR2_R {
        VMOR2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ordinary channels conversion end flag of ADC2"]
    #[inline(always)]
    pub fn occe2(&self) -> OCCE2_R {
        OCCE2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Preempted channels conversion end flag of ADC2"]
    #[inline(always)]
    pub fn pcce2(&self) -> PCCE2_R {
        PCCE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Preempted channel conversion start flag of ADC2"]
    #[inline(always)]
    pub fn pccs2(&self) -> PCCS2_R {
        PCCS2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ordinary channel conversion start flag of ADC2"]
    #[inline(always)]
    pub fn occs2(&self) -> OCCS2_R {
        OCCS2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ordinary channel conversion overflow flag of ADC2"]
    #[inline(always)]
    pub fn occo2(&self) -> OCCO2_R {
        OCCO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC ready to conversion flag of ADC2"]
    #[inline(always)]
    pub fn rdy2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage monitoring out of range flag of ADC3"]
    #[inline(always)]
    pub fn vmor3(&self) -> VMOR3_R {
        VMOR3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ordinary channels conversion end flag of ADC3"]
    #[inline(always)]
    pub fn occe3(&self) -> OCCE3_R {
        OCCE3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Preempted channels conversion end flag of ADC3"]
    #[inline(always)]
    pub fn pcce3(&self) -> PCCE3_R {
        PCCE3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Preempted channel conversion start flag of ADC3"]
    #[inline(always)]
    pub fn pccs3(&self) -> PCCS3_R {
        PCCS3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Ordinary channel conversion start flag of ADC3"]
    #[inline(always)]
    pub fn occs3(&self) -> OCCS3_R {
        OCCS3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ordinary channel conversion overflow flag of ADC3"]
    #[inline(always)]
    pub fn occo3(&self) -> OCCO3_R {
        OCCO3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC ready to conversion flag of ADC3"]
    #[inline(always)]
    pub fn rdy3(&self) -> RDY3_R {
        RDY3_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSTS")
            .field("rdy3", &format_args!("{}", self.rdy3().bit()))
            .field("occo3", &format_args!("{}", self.occo3().bit()))
            .field("occs3", &format_args!("{}", self.occs3().bit()))
            .field("pccs3", &format_args!("{}", self.pccs3().bit()))
            .field("pcce3", &format_args!("{}", self.pcce3().bit()))
            .field("occe3", &format_args!("{}", self.occe3().bit()))
            .field("vmor3", &format_args!("{}", self.vmor3().bit()))
            .field("rdy2", &format_args!("{}", self.rdy2().bit()))
            .field("occo2", &format_args!("{}", self.occo2().bit()))
            .field("occs2", &format_args!("{}", self.occs2().bit()))
            .field("pccs2", &format_args!("{}", self.pccs2().bit()))
            .field("pcce2", &format_args!("{}", self.pcce2().bit()))
            .field("occe2", &format_args!("{}", self.occe2().bit()))
            .field("vmor2", &format_args!("{}", self.vmor2().bit()))
            .field("rdy1", &format_args!("{}", self.rdy1().bit()))
            .field("occo1", &format_args!("{}", self.occo1().bit()))
            .field("occs1", &format_args!("{}", self.occs1().bit()))
            .field("pccs1", &format_args!("{}", self.pccs1().bit()))
            .field("pcce1", &format_args!("{}", self.pcce1().bit()))
            .field("occe1", &format_args!("{}", self.occe1().bit()))
            .field("vmor1", &format_args!("{}", self.vmor1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSTS_SPEC;
impl crate::RegisterSpec for CSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csts::R`](R) reader structure"]
impl crate::Readable for CSTS_SPEC {}
#[doc = "`reset()` method sets CSTS to value 0"]
impl crate::Resettable for CSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
