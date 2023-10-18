#[doc = "Register `CM3_OUTPUT` reader"]
pub type R = crate::R<CM3_OUTPUT_SPEC>;
#[doc = "Register `CM3_OUTPUT` writer"]
pub type W = crate::W<CM3_OUTPUT_SPEC>;
#[doc = "Field `COIEN[5-5]` reader - Channel %s output immediately enable"]
pub type COIEN_R = crate::BitReader<C5OIEN_A>;
#[doc = "Channel %s output immediately enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5OIEN_A {
    #[doc = "0: Need to compare the CVAL with CxDT before generating an output"]
    Compare = 0,
    #[doc = "1: No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    Immediately = 1,
}
impl From<C5OIEN_A> for bool {
    #[inline(always)]
    fn from(variant: C5OIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl COIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C5OIEN_A {
        match self.bits {
            false => C5OIEN_A::Compare,
            true => C5OIEN_A::Immediately,
        }
    }
    #[doc = "Need to compare the CVAL with CxDT before generating an output"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == C5OIEN_A::Compare
    }
    #[doc = "No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == C5OIEN_A::Immediately
    }
}
#[doc = "Field `COIEN[5-5]` writer - Channel %s output immediately enable"]
pub type COIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C5OIEN_A>;
impl<'a, REG, const O: u8> COIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Need to compare the CVAL with CxDT before generating an output"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(C5OIEN_A::Compare)
    }
    #[doc = "No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    #[inline(always)]
    pub fn immediately(self) -> &'a mut crate::W<REG> {
        self.variant(C5OIEN_A::Immediately)
    }
}
#[doc = "Field `COBEN[5-5]` reader - Channel %s output buffer enable"]
pub type COBEN_R = crate::BitReader<C5OBENR_A>;
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5OBENR_A {
    #[doc = "0: Channel output buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Channel output buffer is enabled"]
    Enabled = 1,
}
impl From<C5OBENR_A> for bool {
    #[inline(always)]
    fn from(variant: C5OBENR_A) -> Self {
        variant as u8 != 0
    }
}
impl COBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C5OBENR_A {
        match self.bits {
            false => C5OBENR_A::Disabled,
            true => C5OBENR_A::Enabled,
        }
    }
    #[doc = "Channel output buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C5OBENR_A::Disabled
    }
    #[doc = "Channel output buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C5OBENR_A::Enabled
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5OBENW_AW {
    #[doc = "0: Channel output buffer disable"]
    Disable = 0,
    #[doc = "1: Channel output buffer enable"]
    Enable = 1,
}
impl From<C5OBENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C5OBENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN[5-5]` writer - Channel %s output buffer enable"]
pub type COBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C5OBENW_AW>;
impl<'a, REG, const O: u8> COBEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C5OBENW_AW::Disable)
    }
    #[doc = "Channel output buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C5OBENW_AW::Enable)
    }
}
#[doc = "Field `COCTRL[5-5]` reader - Channel %s output control"]
pub type COCTRL_R = crate::FieldReader<COCTRL_A>;
#[doc = "Channel %s output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COCTRL_A {
    #[doc = "0: Disconnected. CxORAW is disconnected from CxOUT"]
    Disconnected = 0,
    #[doc = "1: CxORAW is high when TMRx_CVAL=TMRx_CxDT"]
    High = 1,
    #[doc = "2: CxORAW is low when TMRx_CVAL=TMRx_CxDT"]
    Low = 2,
    #[doc = "3: Switch CxORAW level when TMRx_CVAL=TMRx_CxDT"]
    Toggle = 3,
    #[doc = "4: CxORAW is forced low"]
    ForceLow = 4,
    #[doc = "5: CxORAW is forced high"]
    ForceHigh = 5,
    #[doc = "6: PWM mode A"]
    PwmA = 6,
    #[doc = "7: PWM mode B"]
    PwmB = 7,
}
impl From<COCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: COCTRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COCTRL_A {
    type Ux = u8;
}
impl COCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COCTRL_A {
        match self.bits {
            0 => COCTRL_A::Disconnected,
            1 => COCTRL_A::High,
            2 => COCTRL_A::Low,
            3 => COCTRL_A::Toggle,
            4 => COCTRL_A::ForceLow,
            5 => COCTRL_A::ForceHigh,
            6 => COCTRL_A::PwmA,
            7 => COCTRL_A::PwmB,
            _ => unreachable!(),
        }
    }
    #[doc = "Disconnected. CxORAW is disconnected from CxOUT"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == COCTRL_A::Disconnected
    }
    #[doc = "CxORAW is high when TMRx_CVAL=TMRx_CxDT"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COCTRL_A::High
    }
    #[doc = "CxORAW is low when TMRx_CVAL=TMRx_CxDT"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COCTRL_A::Low
    }
    #[doc = "Switch CxORAW level when TMRx_CVAL=TMRx_CxDT"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == COCTRL_A::Toggle
    }
    #[doc = "CxORAW is forced low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == COCTRL_A::ForceLow
    }
    #[doc = "CxORAW is forced high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == COCTRL_A::ForceHigh
    }
    #[doc = "PWM mode A"]
    #[inline(always)]
    pub fn is_pwm_a(&self) -> bool {
        *self == COCTRL_A::PwmA
    }
    #[doc = "PWM mode B"]
    #[inline(always)]
    pub fn is_pwm_b(&self) -> bool {
        *self == COCTRL_A::PwmB
    }
}
#[doc = "Field `COCTRL[5-5]` writer - Channel %s output control"]
pub type COCTRL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, COCTRL_A>;
impl<'a, REG, const O: u8> COCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disconnected. CxORAW is disconnected from CxOUT"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::Disconnected)
    }
    #[doc = "CxORAW is high when TMRx_CVAL=TMRx_CxDT"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::High)
    }
    #[doc = "CxORAW is low when TMRx_CVAL=TMRx_CxDT"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::Low)
    }
    #[doc = "Switch CxORAW level when TMRx_CVAL=TMRx_CxDT"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::Toggle)
    }
    #[doc = "CxORAW is forced low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::ForceLow)
    }
    #[doc = "CxORAW is forced high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::ForceHigh)
    }
    #[doc = "PWM mode A"]
    #[inline(always)]
    pub fn pwm_a(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::PwmA)
    }
    #[doc = "PWM mode B"]
    #[inline(always)]
    pub fn pwm_b(self) -> &'a mut crate::W<REG> {
        self.variant(COCTRL_A::PwmB)
    }
}
#[doc = "Field `COSEN[5-5]` reader - Channel %s output switch enable"]
pub type COSEN_R = crate::BitReader<C5OSENR_A>;
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5OSENR_A {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disabled = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enabled = 1,
}
impl From<C5OSENR_A> for bool {
    #[inline(always)]
    fn from(variant: C5OSENR_A) -> Self {
        variant as u8 != 0
    }
}
impl COSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C5OSENR_A {
        match self.bits {
            false => C5OSENR_A::Disabled,
            true => C5OSENR_A::Enabled,
        }
    }
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C5OSENR_A::Disabled
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C5OSENR_A::Enabled
    }
}
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5OSENW_AW {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disable = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enable = 1,
}
impl From<C5OSENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C5OSENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN[5-5]` writer - Channel %s output switch enable"]
pub type COSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C5OSENW_AW>;
impl<'a, REG, const O: u8> COSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C5OSENW_AW::Disable)
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C5OSENW_AW::Enable)
    }
}
impl R {
    #[doc = "Channel [5-5]
output immediately enable"]
    #[inline(always)]
    pub unsafe fn coien(&self, n: u8) -> COIEN_R {
        COIEN_R::new(((self.bits >> ((n - 5) * 0 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    pub fn c5oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Channel [5-5]
output buffer enable"]
    #[inline(always)]
    pub unsafe fn coben(&self, n: u8) -> COBEN_R {
        COBEN_R::new(((self.bits >> ((n - 5) * 0 + 3)) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    pub fn c5oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Channel [5-5]
output control"]
    #[inline(always)]
    pub unsafe fn coctrl(&self, n: u8) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> ((n - 5) * 0 + 4)) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    pub fn c5octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Channel [5-5]
output switch enable"]
    #[inline(always)]
    pub unsafe fn cosen(&self, n: u8) -> COSEN_R {
        COSEN_R::new(((self.bits >> ((n - 5) * 0 + 7)) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 5 output switch enable"]
    #[inline(always)]
    pub fn c5osen(&self) -> COSEN_R {
        COSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM3_OUTPUT")
            .field("c5osen", &format_args!("{}", self.c5osen().bit()))
            .field("c5octrl", &format_args!("{}", self.c5octrl().bits()))
            .field("c5oben", &format_args!("{}", self.c5oben().bit()))
            .field("c5oien", &format_args!("{}", self.c5oien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CM3_OUTPUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Channel [5-5]
output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coien<const O: u8>(&mut self) -> COIEN_W<CM3_OUTPUT_SPEC, O> {
        COIEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5oien(&mut self) -> COIEN_W<CM3_OUTPUT_SPEC, 2> {
        COIEN_W::new(self)
    }
    #[doc = "Channel [5-5]
output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coben<const O: u8>(&mut self) -> COBEN_W<CM3_OUTPUT_SPEC, O> {
        COBEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5oben(&mut self) -> COBEN_W<CM3_OUTPUT_SPEC, 3> {
        COBEN_W::new(self)
    }
    #[doc = "Channel [5-5]
output control"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coctrl<const O: u8>(&mut self) -> COCTRL_W<CM3_OUTPUT_SPEC, O> {
        COCTRL_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c5octrl(&mut self) -> COCTRL_W<CM3_OUTPUT_SPEC, 4> {
        COCTRL_W::new(self)
    }
    #[doc = "Channel [5-5]
output switch enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cosen<const O: u8>(&mut self) -> COSEN_W<CM3_OUTPUT_SPEC, O> {
        COSEN_W::new(self)
    }
    #[doc = "Bit 7 - Channel 5 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c5osen(&mut self) -> COSEN_W<CM3_OUTPUT_SPEC, 7> {
        COSEN_W::new(self)
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
#[doc = "Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm3_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm3_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM3_OUTPUT_SPEC;
impl crate::RegisterSpec for CM3_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm3_output::R`](R) reader structure"]
impl crate::Readable for CM3_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm3_output::W`](W) writer structure"]
impl crate::Writable for CM3_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM3_OUTPUT to value 0"]
impl crate::Resettable for CM3_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
