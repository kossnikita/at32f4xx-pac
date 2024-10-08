#[doc = "Register `STCTRL` reader"]
pub type R = crate::R<STCTRL_SPEC>;
#[doc = "Register `STCTRL` writer"]
pub type W = crate::W<STCTRL_SPEC>;
#[doc = "Subordinate TMR mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMSEL_A {
    #[doc = "0: Slave mode is disabled"]
    Disabled = 0,
    #[doc = "1: Encoder mode A"]
    EncoderA = 1,
    #[doc = "2: Encoder mode B"]
    EncoderB = 2,
    #[doc = "3: Encoder mode C"]
    EncoderC = 3,
    #[doc = "4: Reset mode - Rising edge of the TRGIN input reinitializes the counter"]
    Reset = 4,
    #[doc = "5: Suspend mode - The counter starts counting when the TRGIN is high"]
    Suspend = 5,
    #[doc = "6: Trigger mode - A trigger event is generated at the rising edge of the TRGIN input"]
    Trigger = 6,
    #[doc = "7: External clock mode A - Rising edge of the TRGIN input clocks the counter"]
    External = 7,
}
impl From<SMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SMSEL_A {}
#[doc = "Field `SMSEL` reader - Subordinate TMR mode selection"]
pub type SMSEL_R = crate::FieldReader<SMSEL_A>;
impl SMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMSEL_A {
        match self.bits {
            0 => SMSEL_A::Disabled,
            1 => SMSEL_A::EncoderA,
            2 => SMSEL_A::EncoderB,
            3 => SMSEL_A::EncoderC,
            4 => SMSEL_A::Reset,
            5 => SMSEL_A::Suspend,
            6 => SMSEL_A::Trigger,
            7 => SMSEL_A::External,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMSEL_A::Disabled
    }
    #[doc = "Encoder mode A"]
    #[inline(always)]
    pub fn is_encoder_a(&self) -> bool {
        *self == SMSEL_A::EncoderA
    }
    #[doc = "Encoder mode B"]
    #[inline(always)]
    pub fn is_encoder_b(&self) -> bool {
        *self == SMSEL_A::EncoderB
    }
    #[doc = "Encoder mode C"]
    #[inline(always)]
    pub fn is_encoder_c(&self) -> bool {
        *self == SMSEL_A::EncoderC
    }
    #[doc = "Reset mode - Rising edge of the TRGIN input reinitializes the counter"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SMSEL_A::Reset
    }
    #[doc = "Suspend mode - The counter starts counting when the TRGIN is high"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SMSEL_A::Suspend
    }
    #[doc = "Trigger mode - A trigger event is generated at the rising edge of the TRGIN input"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SMSEL_A::Trigger
    }
    #[doc = "External clock mode A - Rising edge of the TRGIN input clocks the counter"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SMSEL_A::External
    }
}
#[doc = "Field `SMSEL` writer - Subordinate TMR mode selection"]
pub type SMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMSEL_A, crate::Safe>;
impl<'a, REG> SMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Disabled)
    }
    #[doc = "Encoder mode A"]
    #[inline(always)]
    pub fn encoder_a(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::EncoderA)
    }
    #[doc = "Encoder mode B"]
    #[inline(always)]
    pub fn encoder_b(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::EncoderB)
    }
    #[doc = "Encoder mode C"]
    #[inline(always)]
    pub fn encoder_c(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::EncoderC)
    }
    #[doc = "Reset mode - Rising edge of the TRGIN input reinitializes the counter"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Reset)
    }
    #[doc = "Suspend mode - The counter starts counting when the TRGIN is high"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Suspend)
    }
    #[doc = "Trigger mode - A trigger event is generated at the rising edge of the TRGIN input"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::Trigger)
    }
    #[doc = "External clock mode A - Rising edge of the TRGIN input clocks the counter"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(SMSEL_A::External)
    }
}
#[doc = "Subordinate TMR input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STIS_A {
    #[doc = "0: Internal selection 0"]
    Is0 = 0,
    #[doc = "1: Internal selection 1"]
    Is1 = 1,
    #[doc = "2: Internal selection 2"]
    Is2 = 2,
    #[doc = "3: Internal selection 3"]
    Is3 = 3,
    #[doc = "4: C1IRAW input detector"]
    C1inc = 4,
    #[doc = "5: Filtered input 1"]
    C1if1 = 5,
    #[doc = "6: Filtered input 2"]
    C1if2 = 6,
    #[doc = "7: External input"]
    Ext = 7,
}
impl From<STIS_A> for u8 {
    #[inline(always)]
    fn from(variant: STIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STIS_A {
    type Ux = u8;
}
impl crate::IsEnum for STIS_A {}
#[doc = "Field `STIS` reader - Subordinate TMR input selection"]
pub type STIS_R = crate::FieldReader<STIS_A>;
impl STIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STIS_A {
        match self.bits {
            0 => STIS_A::Is0,
            1 => STIS_A::Is1,
            2 => STIS_A::Is2,
            3 => STIS_A::Is3,
            4 => STIS_A::C1inc,
            5 => STIS_A::C1if1,
            6 => STIS_A::C1if2,
            7 => STIS_A::Ext,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal selection 0"]
    #[inline(always)]
    pub fn is_is0(&self) -> bool {
        *self == STIS_A::Is0
    }
    #[doc = "Internal selection 1"]
    #[inline(always)]
    pub fn is_is1(&self) -> bool {
        *self == STIS_A::Is1
    }
    #[doc = "Internal selection 2"]
    #[inline(always)]
    pub fn is_is2(&self) -> bool {
        *self == STIS_A::Is2
    }
    #[doc = "Internal selection 3"]
    #[inline(always)]
    pub fn is_is3(&self) -> bool {
        *self == STIS_A::Is3
    }
    #[doc = "C1IRAW input detector"]
    #[inline(always)]
    pub fn is_c1inc(&self) -> bool {
        *self == STIS_A::C1inc
    }
    #[doc = "Filtered input 1"]
    #[inline(always)]
    pub fn is_c1if1(&self) -> bool {
        *self == STIS_A::C1if1
    }
    #[doc = "Filtered input 2"]
    #[inline(always)]
    pub fn is_c1if2(&self) -> bool {
        *self == STIS_A::C1if2
    }
    #[doc = "External input"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == STIS_A::Ext
    }
}
#[doc = "Field `STIS` writer - Subordinate TMR input selection"]
pub type STIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, STIS_A, crate::Safe>;
impl<'a, REG> STIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal selection 0"]
    #[inline(always)]
    pub fn is0(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is0)
    }
    #[doc = "Internal selection 1"]
    #[inline(always)]
    pub fn is1(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is1)
    }
    #[doc = "Internal selection 2"]
    #[inline(always)]
    pub fn is2(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is2)
    }
    #[doc = "Internal selection 3"]
    #[inline(always)]
    pub fn is3(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Is3)
    }
    #[doc = "C1IRAW input detector"]
    #[inline(always)]
    pub fn c1inc(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::C1inc)
    }
    #[doc = "Filtered input 1"]
    #[inline(always)]
    pub fn c1if1(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::C1if1)
    }
    #[doc = "Filtered input 2"]
    #[inline(always)]
    pub fn c1if2(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::C1if2)
    }
    #[doc = "External input"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(STIS_A::Ext)
    }
}
#[doc = "Subordinate TMR synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stsr {
    #[doc = "0: Subordinate TMR synchronization is disabled"]
    Disabled = 0,
    #[doc = "1: Subordinate TMR synchronization is disabled"]
    Enabled = 1,
}
impl From<Stsr> for bool {
    #[inline(always)]
    fn from(variant: Stsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STS` reader - Subordinate TMR synchronization"]
pub type STS_R = crate::BitReader<Stsr>;
impl STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stsr {
        match self.bits {
            false => Stsr::Disabled,
            true => Stsr::Enabled,
        }
    }
    #[doc = "Subordinate TMR synchronization is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stsr::Disabled
    }
    #[doc = "Subordinate TMR synchronization is disabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stsr::Enabled
    }
}
#[doc = "Subordinate TMR synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StswWO {
    #[doc = "0: Subordinate TMR synchronization disable"]
    Disable = 0,
    #[doc = "1: Subordinate TMR synchronization enable"]
    Enable = 1,
}
impl From<StswWO> for bool {
    #[inline(always)]
    fn from(variant: StswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STS` writer - Subordinate TMR synchronization"]
pub type STS_W<'a, REG> = crate::BitWriter<'a, REG, StswWO>;
impl<'a, REG> STS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Subordinate TMR synchronization disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(StswWO::Disable)
    }
    #[doc = "Subordinate TMR synchronization enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(StswWO::Enable)
    }
}
#[doc = "External signal filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ESF_A {
    #[doc = "0: No filter, sampling is done at f_DTS"]
    NoFilter = 0,
    #[doc = "1: f_sampling = f_DTS, N=2"]
    N2 = 1,
    #[doc = "2: f_sampling = f_DTS, N=4"]
    N4 = 2,
    #[doc = "3: f_sampling = f_DTS, N=8"]
    N8 = 3,
    #[doc = "4: f_sampling = f_DTS/2, N=6"]
    Div2n6 = 4,
    #[doc = "5: f_sampling = f_DTS/2, N=8"]
    Div2n8 = 5,
    #[doc = "6: f_sampling = f_DTS/4, N=6"]
    Div4n6 = 6,
    #[doc = "7: f_sampling = f_DTS/4, N=8"]
    Div4n8 = 7,
    #[doc = "8: f_sampling = f_DTS/8, N=6"]
    Div8n6 = 8,
    #[doc = "9: f_sampling = f_DTS/8, N=8"]
    Div8n8 = 9,
    #[doc = "10: f_sampling = f_DTS/16, N=5"]
    Div16n5 = 10,
    #[doc = "11: f_sampling = f_DTS/16, N=6"]
    Div16n6 = 11,
    #[doc = "12: f_sampling = f_DTS/16, N=8"]
    Div16n8 = 12,
    #[doc = "13: f_sampling = f_DTS/32, N=5"]
    Div32n5 = 13,
    #[doc = "14: f_sampling = f_DTS/32, N=6"]
    Div32n6 = 14,
    #[doc = "15: f_sampling = f_DTS/32, N=8"]
    Div32n8 = 15,
}
impl From<ESF_A> for u8 {
    #[inline(always)]
    fn from(variant: ESF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ESF_A {
    type Ux = u8;
}
impl crate::IsEnum for ESF_A {}
#[doc = "Field `ESF` reader - External signal filter"]
pub type ESF_R = crate::FieldReader<ESF_A>;
impl ESF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESF_A {
        match self.bits {
            0 => ESF_A::NoFilter,
            1 => ESF_A::N2,
            2 => ESF_A::N4,
            3 => ESF_A::N8,
            4 => ESF_A::Div2n6,
            5 => ESF_A::Div2n8,
            6 => ESF_A::Div4n6,
            7 => ESF_A::Div4n8,
            8 => ESF_A::Div8n6,
            9 => ESF_A::Div8n8,
            10 => ESF_A::Div16n5,
            11 => ESF_A::Div16n6,
            12 => ESF_A::Div16n8,
            13 => ESF_A::Div32n5,
            14 => ESF_A::Div32n6,
            15 => ESF_A::Div32n8,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at f_DTS"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ESF_A::NoFilter
    }
    #[doc = "f_sampling = f_DTS, N=2"]
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        *self == ESF_A::N2
    }
    #[doc = "f_sampling = f_DTS, N=4"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == ESF_A::N4
    }
    #[doc = "f_sampling = f_DTS, N=8"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == ESF_A::N8
    }
    #[doc = "f_sampling = f_DTS/2, N=6"]
    #[inline(always)]
    pub fn is_div2n6(&self) -> bool {
        *self == ESF_A::Div2n6
    }
    #[doc = "f_sampling = f_DTS/2, N=8"]
    #[inline(always)]
    pub fn is_div2n8(&self) -> bool {
        *self == ESF_A::Div2n8
    }
    #[doc = "f_sampling = f_DTS/4, N=6"]
    #[inline(always)]
    pub fn is_div4n6(&self) -> bool {
        *self == ESF_A::Div4n6
    }
    #[doc = "f_sampling = f_DTS/4, N=8"]
    #[inline(always)]
    pub fn is_div4n8(&self) -> bool {
        *self == ESF_A::Div4n8
    }
    #[doc = "f_sampling = f_DTS/8, N=6"]
    #[inline(always)]
    pub fn is_div8n6(&self) -> bool {
        *self == ESF_A::Div8n6
    }
    #[doc = "f_sampling = f_DTS/8, N=8"]
    #[inline(always)]
    pub fn is_div8n8(&self) -> bool {
        *self == ESF_A::Div8n8
    }
    #[doc = "f_sampling = f_DTS/16, N=5"]
    #[inline(always)]
    pub fn is_div16n5(&self) -> bool {
        *self == ESF_A::Div16n5
    }
    #[doc = "f_sampling = f_DTS/16, N=6"]
    #[inline(always)]
    pub fn is_div16n6(&self) -> bool {
        *self == ESF_A::Div16n6
    }
    #[doc = "f_sampling = f_DTS/16, N=8"]
    #[inline(always)]
    pub fn is_div16n8(&self) -> bool {
        *self == ESF_A::Div16n8
    }
    #[doc = "f_sampling = f_DTS/32, N=5"]
    #[inline(always)]
    pub fn is_div32n5(&self) -> bool {
        *self == ESF_A::Div32n5
    }
    #[doc = "f_sampling = f_DTS/32, N=6"]
    #[inline(always)]
    pub fn is_div32n6(&self) -> bool {
        *self == ESF_A::Div32n6
    }
    #[doc = "f_sampling = f_DTS/32, N=8"]
    #[inline(always)]
    pub fn is_div32n8(&self) -> bool {
        *self == ESF_A::Div32n8
    }
}
#[doc = "Field `ESF` writer - External signal filter"]
pub type ESF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ESF_A, crate::Safe>;
impl<'a, REG> ESF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at f_DTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::NoFilter)
    }
    #[doc = "f_sampling = f_DTS, N=2"]
    #[inline(always)]
    pub fn n2(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::N2)
    }
    #[doc = "f_sampling = f_DTS, N=4"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::N4)
    }
    #[doc = "f_sampling = f_DTS, N=8"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::N8)
    }
    #[doc = "f_sampling = f_DTS/2, N=6"]
    #[inline(always)]
    pub fn div2n6(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div2n6)
    }
    #[doc = "f_sampling = f_DTS/2, N=8"]
    #[inline(always)]
    pub fn div2n8(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div2n8)
    }
    #[doc = "f_sampling = f_DTS/4, N=6"]
    #[inline(always)]
    pub fn div4n6(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div4n6)
    }
    #[doc = "f_sampling = f_DTS/4, N=8"]
    #[inline(always)]
    pub fn div4n8(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div4n8)
    }
    #[doc = "f_sampling = f_DTS/8, N=6"]
    #[inline(always)]
    pub fn div8n6(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div8n6)
    }
    #[doc = "f_sampling = f_DTS/8, N=8"]
    #[inline(always)]
    pub fn div8n8(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div8n8)
    }
    #[doc = "f_sampling = f_DTS/16, N=5"]
    #[inline(always)]
    pub fn div16n5(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div16n5)
    }
    #[doc = "f_sampling = f_DTS/16, N=6"]
    #[inline(always)]
    pub fn div16n6(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div16n6)
    }
    #[doc = "f_sampling = f_DTS/16, N=8"]
    #[inline(always)]
    pub fn div16n8(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div16n8)
    }
    #[doc = "f_sampling = f_DTS/32, N=5"]
    #[inline(always)]
    pub fn div32n5(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div32n5)
    }
    #[doc = "f_sampling = f_DTS/32, N=6"]
    #[inline(always)]
    pub fn div32n6(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div32n6)
    }
    #[doc = "f_sampling = f_DTS/32, N=8"]
    #[inline(always)]
    pub fn div32n8(self) -> &'a mut crate::W<REG> {
        self.variant(ESF_A::Div32n8)
    }
}
#[doc = "External signal divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ESDIV_A {
    #[doc = "0: Normal"]
    Normal = 0,
    #[doc = "1: Divided by 2"]
    Div2 = 1,
    #[doc = "2: Divided by 4"]
    Div4 = 2,
    #[doc = "3: Divided by 8"]
    Div8 = 3,
}
impl From<ESDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ESDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ESDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for ESDIV_A {}
#[doc = "Field `ESDIV` reader - External signal divider"]
pub type ESDIV_R = crate::FieldReader<ESDIV_A>;
impl ESDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESDIV_A {
        match self.bits {
            0 => ESDIV_A::Normal,
            1 => ESDIV_A::Div2,
            2 => ESDIV_A::Div4,
            3 => ESDIV_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ESDIV_A::Normal
    }
    #[doc = "Divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ESDIV_A::Div2
    }
    #[doc = "Divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ESDIV_A::Div4
    }
    #[doc = "Divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ESDIV_A::Div8
    }
}
#[doc = "Field `ESDIV` writer - External signal divider"]
pub type ESDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ESDIV_A, crate::Safe>;
impl<'a, REG> ESDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ESDIV_A::Normal)
    }
    #[doc = "Divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ESDIV_A::Div2)
    }
    #[doc = "Divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ESDIV_A::Div4)
    }
    #[doc = "Divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ESDIV_A::Div8)
    }
}
#[doc = "External clock mode B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecmbenr {
    #[doc = "0: External clock mode B is disabled"]
    Disabled = 0,
    #[doc = "1: External clock mode B is disabled"]
    Enabled = 1,
}
impl From<Ecmbenr> for bool {
    #[inline(always)]
    fn from(variant: Ecmbenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECMBEN` reader - External clock mode B enable"]
pub type ECMBEN_R = crate::BitReader<Ecmbenr>;
impl ECMBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecmbenr {
        match self.bits {
            false => Ecmbenr::Disabled,
            true => Ecmbenr::Enabled,
        }
    }
    #[doc = "External clock mode B is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ecmbenr::Disabled
    }
    #[doc = "External clock mode B is disabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ecmbenr::Enabled
    }
}
#[doc = "External clock mode B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcmbenwWO {
    #[doc = "0: External clock mode B disable"]
    Disable = 0,
    #[doc = "1: External clock mode B enable"]
    Enable = 1,
}
impl From<EcmbenwWO> for bool {
    #[inline(always)]
    fn from(variant: EcmbenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECMBEN` writer - External clock mode B enable"]
pub type ECMBEN_W<'a, REG> = crate::BitWriter<'a, REG, EcmbenwWO>;
impl<'a, REG> ECMBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock mode B disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EcmbenwWO::Disable)
    }
    #[doc = "External clock mode B enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EcmbenwWO::Enable)
    }
}
#[doc = "External signal polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESP_A {
    #[doc = "0: High or rising edge"]
    High = 0,
    #[doc = "1: Low or falling edge"]
    Low = 1,
}
impl From<ESP_A> for bool {
    #[inline(always)]
    fn from(variant: ESP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESP` reader - External signal polarity"]
pub type ESP_R = crate::BitReader<ESP_A>;
impl ESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESP_A {
        match self.bits {
            false => ESP_A::High,
            true => ESP_A::Low,
        }
    }
    #[doc = "High or rising edge"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ESP_A::High
    }
    #[doc = "Low or falling edge"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ESP_A::Low
    }
}
#[doc = "Field `ESP` writer - External signal polarity"]
pub type ESP_W<'a, REG> = crate::BitWriter<'a, REG, ESP_A>;
impl<'a, REG> ESP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High or rising edge"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ESP_A::High)
    }
    #[doc = "Low or falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ESP_A::Low)
    }
}
impl R {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    pub fn stis(&self) -> STIS_R {
        STIS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    pub fn esf(&self) -> ESF_R {
        ESF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    pub fn esdiv(&self) -> ESDIV_R {
        ESDIV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    pub fn ecmben(&self) -> ECMBEN_R {
        ECMBEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    pub fn esp(&self) -> ESP_R {
        ESP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCTRL")
            .field("esp", &self.esp())
            .field("ecmben", &self.ecmben())
            .field("esdiv", &self.esdiv())
            .field("esf", &self.esf())
            .field("sts", &self.sts())
            .field("stis", &self.stis())
            .field("smsel", &self.smsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SMSEL_W<STCTRL_SPEC> {
        SMSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stis(&mut self) -> STIS_W<STCTRL_SPEC> {
        STIS_W::new(self, 4)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> STS_W<STCTRL_SPEC> {
        STS_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    #[must_use]
    pub fn esf(&mut self) -> ESF_W<STCTRL_SPEC> {
        ESF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    #[must_use]
    pub fn esdiv(&mut self) -> ESDIV_W<STCTRL_SPEC> {
        ESDIV_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecmben(&mut self) -> ECMBEN_W<STCTRL_SPEC> {
        ECMBEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn esp(&mut self) -> ESP_W<STCTRL_SPEC> {
        ESP_W::new(self, 15)
    }
}
#[doc = "Subordinate TMR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCTRL_SPEC;
impl crate::RegisterSpec for STCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctrl::R`](R) reader structure"]
impl crate::Readable for STCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stctrl::W`](W) writer structure"]
impl crate::Writable for STCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCTRL to value 0"]
impl crate::Resettable for STCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
