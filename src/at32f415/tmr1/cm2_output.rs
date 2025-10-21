#[doc = "Register `CM2_OUTPUT` reader"]
pub type R = crate::R<CM2_OUTPUT_SPEC>;
#[doc = "Register `CM2_OUTPUT` writer"]
pub type W = crate::W<CM2_OUTPUT_SPEC>;
#[doc = "Channel 3 configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C3C_A {
    #[doc = "0: C3IN channel is configured as output"]
    Output = 0,
    #[doc = "1: Input, C3IN is mapped on C3IFP3"]
    C3ifp3 = 1,
    #[doc = "2: Input, C3IN is mapped on C4IFP3"]
    C4ifp3 = 2,
    #[doc = "3: Input, C3IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    Stis = 3,
}
impl From<C3C_A> for u8 {
    #[inline(always)]
    fn from(variant: C3C_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C3C_A {
    type Ux = u8;
}
impl crate::IsEnum for C3C_A {}
#[doc = "Field `C3C` reader - Channel 3 configure"]
pub type C3C_R = crate::FieldReader<C3C_A>;
impl C3C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C3C_A {
        match self.bits {
            0 => C3C_A::Output,
            1 => C3C_A::C3ifp3,
            2 => C3C_A::C4ifp3,
            3 => C3C_A::Stis,
            _ => unreachable!(),
        }
    }
    #[doc = "C3IN channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C3C_A::Output
    }
    #[doc = "Input, C3IN is mapped on C3IFP3"]
    #[inline(always)]
    pub fn is_c3ifp3(&self) -> bool {
        *self == C3C_A::C3ifp3
    }
    #[doc = "Input, C3IN is mapped on C4IFP3"]
    #[inline(always)]
    pub fn is_c4ifp3(&self) -> bool {
        *self == C3C_A::C4ifp3
    }
    #[doc = "Input, C3IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn is_stis(&self) -> bool {
        *self == C3C_A::Stis
    }
}
#[doc = "Field `C3C` writer - Channel 3 configure"]
pub type C3C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C3C_A, crate::Safe>;
impl<'a, REG> C3C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "C3IN channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C3C_A::Output)
    }
    #[doc = "Input, C3IN is mapped on C3IFP3"]
    #[inline(always)]
    pub fn c3ifp3(self) -> &'a mut crate::W<REG> {
        self.variant(C3C_A::C3ifp3)
    }
    #[doc = "Input, C3IN is mapped on C4IFP3"]
    #[inline(always)]
    pub fn c4ifp3(self) -> &'a mut crate::W<REG> {
        self.variant(C3C_A::C4ifp3)
    }
    #[doc = "Input, C3IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn stis(self) -> &'a mut crate::W<REG> {
        self.variant(C3C_A::Stis)
    }
}
#[doc = "Channel %s output immediately enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3OIEN_A {
    #[doc = "0: Need to compare the CVAL with CxDT before generating an output"]
    Compare = 0,
    #[doc = "1: No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    Immediately = 1,
}
impl From<C3OIEN_A> for bool {
    #[inline(always)]
    fn from(variant: C3OIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COIEN(3-4)` reader - Channel %s output immediately enable"]
pub type COIEN_R = crate::BitReader<C3OIEN_A>;
impl COIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C3OIEN_A {
        match self.bits {
            false => C3OIEN_A::Compare,
            true => C3OIEN_A::Immediately,
        }
    }
    #[doc = "Need to compare the CVAL with CxDT before generating an output"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == C3OIEN_A::Compare
    }
    #[doc = "No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == C3OIEN_A::Immediately
    }
}
#[doc = "Field `COIEN(3-4)` writer - Channel %s output immediately enable"]
pub type COIEN_W<'a, REG> = crate::BitWriter<'a, REG, C3OIEN_A>;
impl<'a, REG> COIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Need to compare the CVAL with CxDT before generating an output"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(C3OIEN_A::Compare)
    }
    #[doc = "No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    #[inline(always)]
    pub fn immediately(self) -> &'a mut crate::W<REG> {
        self.variant(C3OIEN_A::Immediately)
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3obenr {
    #[doc = "0: Channel output buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Channel output buffer is enabled"]
    Enabled = 1,
}
impl From<C3obenr> for bool {
    #[inline(always)]
    fn from(variant: C3obenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN(3-4)` reader - Channel %s output buffer enable"]
pub type COBEN_R = crate::BitReader<C3obenr>;
impl COBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C3obenr {
        match self.bits {
            false => C3obenr::Disabled,
            true => C3obenr::Enabled,
        }
    }
    #[doc = "Channel output buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C3obenr::Disabled
    }
    #[doc = "Channel output buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C3obenr::Enabled
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3obenwWO {
    #[doc = "0: Channel output buffer disable"]
    Disable = 0,
    #[doc = "1: Channel output buffer enable"]
    Enable = 1,
}
impl From<C3obenwWO> for bool {
    #[inline(always)]
    fn from(variant: C3obenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN(3-4)` writer - Channel %s output buffer enable"]
pub type COBEN_W<'a, REG> = crate::BitWriter<'a, REG, C3obenwWO>;
impl<'a, REG> COBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C3obenwWO::Disable)
    }
    #[doc = "Channel output buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C3obenwWO::Enable)
    }
}
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
impl crate::IsEnum for COCTRL_A {}
#[doc = "Field `COCTRL(3-4)` reader - Channel %s output control"]
pub type COCTRL_R = crate::FieldReader<COCTRL_A>;
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
#[doc = "Field `COCTRL(3-4)` writer - Channel %s output control"]
pub type COCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COCTRL_A, crate::Safe>;
impl<'a, REG> COCTRL_W<'a, REG>
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
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3osenr {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disabled = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enabled = 1,
}
impl From<C3osenr> for bool {
    #[inline(always)]
    fn from(variant: C3osenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN(3-4)` reader - Channel %s output switch enable"]
pub type COSEN_R = crate::BitReader<C3osenr>;
impl COSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C3osenr {
        match self.bits {
            false => C3osenr::Disabled,
            true => C3osenr::Enabled,
        }
    }
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C3osenr::Disabled
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C3osenr::Enabled
    }
}
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3osenwWO {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disable = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enable = 1,
}
impl From<C3osenwWO> for bool {
    #[inline(always)]
    fn from(variant: C3osenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN(3-4)` writer - Channel %s output switch enable"]
pub type COSEN_W<'a, REG> = crate::BitWriter<'a, REG, C3osenwWO>;
impl<'a, REG> COSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C3osenwWO::Disable)
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C3osenwWO::Enable)
    }
}
#[doc = "Channel 4 configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C4C_A {
    #[doc = "0: C4IN channel is configured as output"]
    Output = 0,
    #[doc = "1: Input, C4IN is mapped on C4IFP4"]
    C4ifp4 = 1,
    #[doc = "2: Input, C4IN is mapped on C3IFP4"]
    C3ifp4 = 2,
    #[doc = "3: Input, C4IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    Stis = 3,
}
impl From<C4C_A> for u8 {
    #[inline(always)]
    fn from(variant: C4C_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C4C_A {
    type Ux = u8;
}
impl crate::IsEnum for C4C_A {}
#[doc = "Field `C4C` reader - Channel 4 configure"]
pub type C4C_R = crate::FieldReader<C4C_A>;
impl C4C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C4C_A {
        match self.bits {
            0 => C4C_A::Output,
            1 => C4C_A::C4ifp4,
            2 => C4C_A::C3ifp4,
            3 => C4C_A::Stis,
            _ => unreachable!(),
        }
    }
    #[doc = "C4IN channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C4C_A::Output
    }
    #[doc = "Input, C4IN is mapped on C4IFP4"]
    #[inline(always)]
    pub fn is_c4ifp4(&self) -> bool {
        *self == C4C_A::C4ifp4
    }
    #[doc = "Input, C4IN is mapped on C3IFP4"]
    #[inline(always)]
    pub fn is_c3ifp4(&self) -> bool {
        *self == C4C_A::C3ifp4
    }
    #[doc = "Input, C4IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn is_stis(&self) -> bool {
        *self == C4C_A::Stis
    }
}
#[doc = "Field `C4C` writer - Channel 4 configure"]
pub type C4C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C4C_A, crate::Safe>;
impl<'a, REG> C4C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "C4IN channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C4C_A::Output)
    }
    #[doc = "Input, C4IN is mapped on C4IFP4"]
    #[inline(always)]
    pub fn c4ifp4(self) -> &'a mut crate::W<REG> {
        self.variant(C4C_A::C4ifp4)
    }
    #[doc = "Input, C4IN is mapped on C3IFP4"]
    #[inline(always)]
    pub fn c3ifp4(self) -> &'a mut crate::W<REG> {
        self.variant(C4C_A::C3ifp4)
    }
    #[doc = "Input, C4IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn stis(self) -> &'a mut crate::W<REG> {
        self.variant(C4C_A::Stis)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    pub fn c3c(&self) -> C3C_R {
        C3C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Channel (3-4) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&self, n: u8) -> COIEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COIEN_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (3-4) output immediately enable"]
    #[inline(always)]
    pub fn coien_iter(&self) -> impl Iterator<Item = COIEN_R> + '_ {
        (0..2).map(move |n| COIEN_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Channel 3 output immediately enable"]
    #[inline(always)]
    pub fn c3oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 4 output immediately enable"]
    #[inline(always)]
    pub fn c4oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Channel (3-4) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&self, n: u8) -> COBEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COBEN_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (3-4) output buffer enable"]
    #[inline(always)]
    pub fn coben_iter(&self) -> impl Iterator<Item = COBEN_R> + '_ {
        (0..2).map(move |n| COBEN_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 3 output buffer enable"]
    #[inline(always)]
    pub fn c3oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 4 output buffer enable"]
    #[inline(always)]
    pub fn c4oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Channel (3-4) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&self, n: u8) -> COCTRL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COCTRL_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (3-4) output control"]
    #[inline(always)]
    pub fn coctrl_iter(&self) -> impl Iterator<Item = COCTRL_R> + '_ {
        (0..2).map(move |n| COCTRL_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Channel 3 output control"]
    #[inline(always)]
    pub fn c3octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 output control"]
    #[inline(always)]
    pub fn c4octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Channel (3-4) output switch enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OSEN` field.</div>"]
    #[inline(always)]
    pub fn cosen(&self, n: u8) -> COSEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COSEN_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (3-4) output switch enable"]
    #[inline(always)]
    pub fn cosen_iter(&self) -> impl Iterator<Item = COSEN_R> + '_ {
        (0..2).map(move |n| COSEN_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    #[doc = "Bit 7 - Channel 3 output switch enable"]
    #[inline(always)]
    pub fn c3osen(&self) -> COSEN_R {
        COSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 output switch enable"]
    #[inline(always)]
    pub fn c4osen(&self) -> COSEN_R {
        COSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    pub fn c4c(&self) -> C4C_R {
        C4C_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM2_OUTPUT")
            .field("c3osen", &self.c3osen())
            .field("c4osen", &self.c4osen())
            .field("c3octrl", &self.c3octrl())
            .field("c4octrl", &self.c4octrl())
            .field("c3oben", &self.c3oben())
            .field("c4oben", &self.c4oben())
            .field("c3oien", &self.c3oien())
            .field("c4oien", &self.c4oien())
            .field("c4c", &self.c4c())
            .field("c3c", &self.c3c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    pub fn c3c(&mut self) -> C3C_W<'_, CM2_OUTPUT_SPEC> {
        C3C_W::new(self, 0)
    }
    #[doc = "Channel (3-4) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&mut self, n: u8) -> COIEN_W<'_, CM2_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COIEN_W::new(self, n * 8 + 2)
    }
    #[doc = "Bit 2 - Channel 3 output immediately enable"]
    #[inline(always)]
    pub fn c3oien(&mut self) -> COIEN_W<'_, CM2_OUTPUT_SPEC> {
        COIEN_W::new(self, 2)
    }
    #[doc = "Bit 10 - Channel 4 output immediately enable"]
    #[inline(always)]
    pub fn c4oien(&mut self) -> COIEN_W<'_, CM2_OUTPUT_SPEC> {
        COIEN_W::new(self, 10)
    }
    #[doc = "Channel (3-4) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&mut self, n: u8) -> COBEN_W<'_, CM2_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COBEN_W::new(self, n * 8 + 3)
    }
    #[doc = "Bit 3 - Channel 3 output buffer enable"]
    #[inline(always)]
    pub fn c3oben(&mut self) -> COBEN_W<'_, CM2_OUTPUT_SPEC> {
        COBEN_W::new(self, 3)
    }
    #[doc = "Bit 11 - Channel 4 output buffer enable"]
    #[inline(always)]
    pub fn c4oben(&mut self) -> COBEN_W<'_, CM2_OUTPUT_SPEC> {
        COBEN_W::new(self, 11)
    }
    #[doc = "Channel (3-4) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&mut self, n: u8) -> COCTRL_W<'_, CM2_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COCTRL_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:6 - Channel 3 output control"]
    #[inline(always)]
    pub fn c3octrl(&mut self) -> COCTRL_W<'_, CM2_OUTPUT_SPEC> {
        COCTRL_W::new(self, 4)
    }
    #[doc = "Bits 12:14 - Channel 4 output control"]
    #[inline(always)]
    pub fn c4octrl(&mut self) -> COCTRL_W<'_, CM2_OUTPUT_SPEC> {
        COCTRL_W::new(self, 12)
    }
    #[doc = "Channel (3-4) output switch enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C3OSEN` field.</div>"]
    #[inline(always)]
    pub fn cosen(&mut self, n: u8) -> COSEN_W<'_, CM2_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COSEN_W::new(self, n * 8 + 7)
    }
    #[doc = "Bit 7 - Channel 3 output switch enable"]
    #[inline(always)]
    pub fn c3osen(&mut self) -> COSEN_W<'_, CM2_OUTPUT_SPEC> {
        COSEN_W::new(self, 7)
    }
    #[doc = "Bit 15 - Channel 4 output switch enable"]
    #[inline(always)]
    pub fn c4osen(&mut self) -> COSEN_W<'_, CM2_OUTPUT_SPEC> {
        COSEN_W::new(self, 15)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    pub fn c4c(&mut self) -> C4C_W<'_, CM2_OUTPUT_SPEC> {
        C4C_W::new(self, 8)
    }
}
#[doc = "Channel output mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cm2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM2_OUTPUT_SPEC;
impl crate::RegisterSpec for CM2_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm2_output::R`](R) reader structure"]
impl crate::Readable for CM2_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm2_output::W`](W) writer structure"]
impl crate::Writable for CM2_OUTPUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CM2_OUTPUT to value 0"]
impl crate::Resettable for CM2_OUTPUT_SPEC {}
