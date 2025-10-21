#[doc = "Register `CM1_OUTPUT` reader"]
pub type R = crate::R<CM1_OUTPUT_SPEC>;
#[doc = "Register `CM1_OUTPUT` writer"]
pub type W = crate::W<CM1_OUTPUT_SPEC>;
#[doc = "Channel 1 configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1C_A {
    #[doc = "0: C1IN channel is configured as output"]
    Output = 0,
    #[doc = "1: Input, C1IN is mapped on C1IFP1"]
    C1ifp1 = 1,
    #[doc = "2: Input, C1IN is mapped on C2IFP1"]
    C2ifp1 = 2,
    #[doc = "3: Input, C1IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    Stis = 3,
}
impl From<C1C_A> for u8 {
    #[inline(always)]
    fn from(variant: C1C_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1C_A {
    type Ux = u8;
}
impl crate::IsEnum for C1C_A {}
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1C_R = crate::FieldReader<C1C_A>;
impl C1C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1C_A {
        match self.bits {
            0 => C1C_A::Output,
            1 => C1C_A::C1ifp1,
            2 => C1C_A::C2ifp1,
            3 => C1C_A::Stis,
            _ => unreachable!(),
        }
    }
    #[doc = "C1IN channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C1C_A::Output
    }
    #[doc = "Input, C1IN is mapped on C1IFP1"]
    #[inline(always)]
    pub fn is_c1ifp1(&self) -> bool {
        *self == C1C_A::C1ifp1
    }
    #[doc = "Input, C1IN is mapped on C2IFP1"]
    #[inline(always)]
    pub fn is_c2ifp1(&self) -> bool {
        *self == C1C_A::C2ifp1
    }
    #[doc = "Input, C1IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn is_stis(&self) -> bool {
        *self == C1C_A::Stis
    }
}
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C1C_A, crate::Safe>;
impl<'a, REG> C1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "C1IN channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C1C_A::Output)
    }
    #[doc = "Input, C1IN is mapped on C1IFP1"]
    #[inline(always)]
    pub fn c1ifp1(self) -> &'a mut crate::W<REG> {
        self.variant(C1C_A::C1ifp1)
    }
    #[doc = "Input, C1IN is mapped on C2IFP1"]
    #[inline(always)]
    pub fn c2ifp1(self) -> &'a mut crate::W<REG> {
        self.variant(C1C_A::C2ifp1)
    }
    #[doc = "Input, C1IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn stis(self) -> &'a mut crate::W<REG> {
        self.variant(C1C_A::Stis)
    }
}
#[doc = "Channel %s output immediately enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1OIEN_A {
    #[doc = "0: Need to compare the CVAL with CxDT before generating an output"]
    Compare = 0,
    #[doc = "1: No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    Immediately = 1,
}
impl From<C1OIEN_A> for bool {
    #[inline(always)]
    fn from(variant: C1OIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COIEN(1-2)` reader - Channel %s output immediately enable"]
pub type COIEN_R = crate::BitReader<C1OIEN_A>;
impl COIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1OIEN_A {
        match self.bits {
            false => C1OIEN_A::Compare,
            true => C1OIEN_A::Immediately,
        }
    }
    #[doc = "Need to compare the CVAL with CxDT before generating an output"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == C1OIEN_A::Compare
    }
    #[doc = "No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == C1OIEN_A::Immediately
    }
}
#[doc = "Field `COIEN(1-2)` writer - Channel %s output immediately enable"]
pub type COIEN_W<'a, REG> = crate::BitWriter<'a, REG, C1OIEN_A>;
impl<'a, REG> COIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Need to compare the CVAL with CxDT before generating an output"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(C1OIEN_A::Compare)
    }
    #[doc = "No need to compare the CVAL and CxDT. An output is generated immediately when a trigger event occurs."]
    #[inline(always)]
    pub fn immediately(self) -> &'a mut crate::W<REG> {
        self.variant(C1OIEN_A::Immediately)
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1obenr {
    #[doc = "0: Channel output buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Channel output buffer is enabled"]
    Enabled = 1,
}
impl From<C1obenr> for bool {
    #[inline(always)]
    fn from(variant: C1obenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN(1-2)` reader - Channel %s output buffer enable"]
pub type COBEN_R = crate::BitReader<C1obenr>;
impl COBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1obenr {
        match self.bits {
            false => C1obenr::Disabled,
            true => C1obenr::Enabled,
        }
    }
    #[doc = "Channel output buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1obenr::Disabled
    }
    #[doc = "Channel output buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1obenr::Enabled
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1obenwWO {
    #[doc = "0: Channel output buffer disable"]
    Disable = 0,
    #[doc = "1: Channel output buffer enable"]
    Enable = 1,
}
impl From<C1obenwWO> for bool {
    #[inline(always)]
    fn from(variant: C1obenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN(1-2)` writer - Channel %s output buffer enable"]
pub type COBEN_W<'a, REG> = crate::BitWriter<'a, REG, C1obenwWO>;
impl<'a, REG> COBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1obenwWO::Disable)
    }
    #[doc = "Channel output buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1obenwWO::Enable)
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
#[doc = "Field `COCTRL(1-2)` reader - Channel %s output control"]
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
#[doc = "Field `COCTRL(1-2)` writer - Channel %s output control"]
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
pub enum C1osenr {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disabled = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enabled = 1,
}
impl From<C1osenr> for bool {
    #[inline(always)]
    fn from(variant: C1osenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN(1-2)` reader - Channel %s output switch enable"]
pub type COSEN_R = crate::BitReader<C1osenr>;
impl COSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1osenr {
        match self.bits {
            false => C1osenr::Disabled,
            true => C1osenr::Enabled,
        }
    }
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1osenr::Disabled
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1osenr::Enabled
    }
}
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1osenwWO {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disable = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enable = 1,
}
impl From<C1osenwWO> for bool {
    #[inline(always)]
    fn from(variant: C1osenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN(1-2)` writer - Channel %s output switch enable"]
pub type COSEN_W<'a, REG> = crate::BitWriter<'a, REG, C1osenwWO>;
impl<'a, REG> COSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1osenwWO::Disable)
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1osenwWO::Enable)
    }
}
#[doc = "Channel 2 configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C2C_A {
    #[doc = "0: C2IN channel is configured as output"]
    Output = 0,
    #[doc = "1: Input, C2IN is mapped on C2IFP2"]
    C2ifp2 = 1,
    #[doc = "2: Input, C2IN is mapped on C1IFP2"]
    C1ifp2 = 2,
    #[doc = "3: Input, C2IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    Stis = 3,
}
impl From<C2C_A> for u8 {
    #[inline(always)]
    fn from(variant: C2C_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C2C_A {
    type Ux = u8;
}
impl crate::IsEnum for C2C_A {}
#[doc = "Field `C2C` reader - Channel 2 configure"]
pub type C2C_R = crate::FieldReader<C2C_A>;
impl C2C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C2C_A {
        match self.bits {
            0 => C2C_A::Output,
            1 => C2C_A::C2ifp2,
            2 => C2C_A::C1ifp2,
            3 => C2C_A::Stis,
            _ => unreachable!(),
        }
    }
    #[doc = "C2IN channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C2C_A::Output
    }
    #[doc = "Input, C2IN is mapped on C2IFP2"]
    #[inline(always)]
    pub fn is_c2ifp2(&self) -> bool {
        *self == C2C_A::C2ifp2
    }
    #[doc = "Input, C2IN is mapped on C1IFP2"]
    #[inline(always)]
    pub fn is_c1ifp2(&self) -> bool {
        *self == C2C_A::C1ifp2
    }
    #[doc = "Input, C2IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn is_stis(&self) -> bool {
        *self == C2C_A::Stis
    }
}
#[doc = "Field `C2C` writer - Channel 2 configure"]
pub type C2C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C2C_A, crate::Safe>;
impl<'a, REG> C2C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "C2IN channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C2C_A::Output)
    }
    #[doc = "Input, C2IN is mapped on C2IFP2"]
    #[inline(always)]
    pub fn c2ifp2(self) -> &'a mut crate::W<REG> {
        self.variant(C2C_A::C2ifp2)
    }
    #[doc = "Input, C2IN is mapped on C1IFP2"]
    #[inline(always)]
    pub fn c1ifp2(self) -> &'a mut crate::W<REG> {
        self.variant(C2C_A::C1ifp2)
    }
    #[doc = "Input, C2IN is mapped on STCI. This mode works only when the internal trigger input is selected by STIS."]
    #[inline(always)]
    pub fn stis(self) -> &'a mut crate::W<REG> {
        self.variant(C2C_A::Stis)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1C_R {
        C1C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Channel (1-2) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&self, n: u8) -> COIEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COIEN_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-2) output immediately enable"]
    #[inline(always)]
    pub fn coien_iter(&self) -> impl Iterator<Item = COIEN_R> + '_ {
        (0..2).map(move |n| COIEN_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    pub fn c1oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 output immediately enable"]
    #[inline(always)]
    pub fn c2oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Channel (1-2) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&self, n: u8) -> COBEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COBEN_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-2) output buffer enable"]
    #[inline(always)]
    pub fn coben_iter(&self) -> impl Iterator<Item = COBEN_R> + '_ {
        (0..2).map(move |n| COBEN_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    pub fn c1oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 output buffer enable"]
    #[inline(always)]
    pub fn c2oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Channel (1-2) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&self, n: u8) -> COCTRL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COCTRL_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-2) output control"]
    #[inline(always)]
    pub fn coctrl_iter(&self) -> impl Iterator<Item = COCTRL_R> + '_ {
        (0..2).map(move |n| COCTRL_R::new(((self.bits >> (n * 8 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    pub fn c1octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 2 output control"]
    #[inline(always)]
    pub fn c2octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Channel (1-2) output switch enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OSEN` field.</div>"]
    #[inline(always)]
    pub fn cosen(&self, n: u8) -> COSEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COSEN_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-2) output switch enable"]
    #[inline(always)]
    pub fn cosen_iter(&self) -> impl Iterator<Item = COSEN_R> + '_ {
        (0..2).map(move |n| COSEN_R::new(((self.bits >> (n * 8 + 7)) & 1) != 0))
    }
    #[doc = "Bit 7 - Channel 1 output switch enable"]
    #[inline(always)]
    pub fn c1osen(&self) -> COSEN_R {
        COSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 2 output switch enable"]
    #[inline(always)]
    pub fn c2osen(&self) -> COSEN_R {
        COSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    pub fn c2c(&self) -> C2C_R {
        C2C_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_OUTPUT")
            .field("c1osen", &self.c1osen())
            .field("c2osen", &self.c2osen())
            .field("c1octrl", &self.c1octrl())
            .field("c2octrl", &self.c2octrl())
            .field("c1oben", &self.c1oben())
            .field("c2oben", &self.c2oben())
            .field("c1oien", &self.c1oien())
            .field("c2oien", &self.c2oien())
            .field("c2c", &self.c2c())
            .field("c1c", &self.c1c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&mut self) -> C1C_W<'_, CM1_OUTPUT_SPEC> {
        C1C_W::new(self, 0)
    }
    #[doc = "Channel (1-2) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&mut self, n: u8) -> COIEN_W<'_, CM1_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COIEN_W::new(self, n * 8 + 2)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    pub fn c1oien(&mut self) -> COIEN_W<'_, CM1_OUTPUT_SPEC> {
        COIEN_W::new(self, 2)
    }
    #[doc = "Bit 10 - Channel 2 output immediately enable"]
    #[inline(always)]
    pub fn c2oien(&mut self) -> COIEN_W<'_, CM1_OUTPUT_SPEC> {
        COIEN_W::new(self, 10)
    }
    #[doc = "Channel (1-2) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&mut self, n: u8) -> COBEN_W<'_, CM1_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COBEN_W::new(self, n * 8 + 3)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    pub fn c1oben(&mut self) -> COBEN_W<'_, CM1_OUTPUT_SPEC> {
        COBEN_W::new(self, 3)
    }
    #[doc = "Bit 11 - Channel 2 output buffer enable"]
    #[inline(always)]
    pub fn c2oben(&mut self) -> COBEN_W<'_, CM1_OUTPUT_SPEC> {
        COBEN_W::new(self, 11)
    }
    #[doc = "Channel (1-2) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&mut self, n: u8) -> COCTRL_W<'_, CM1_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COCTRL_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    pub fn c1octrl(&mut self) -> COCTRL_W<'_, CM1_OUTPUT_SPEC> {
        COCTRL_W::new(self, 4)
    }
    #[doc = "Bits 12:14 - Channel 2 output control"]
    #[inline(always)]
    pub fn c2octrl(&mut self) -> COCTRL_W<'_, CM1_OUTPUT_SPEC> {
        COCTRL_W::new(self, 12)
    }
    #[doc = "Channel (1-2) output switch enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OSEN` field.</div>"]
    #[inline(always)]
    pub fn cosen(&mut self, n: u8) -> COSEN_W<'_, CM1_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        COSEN_W::new(self, n * 8 + 7)
    }
    #[doc = "Bit 7 - Channel 1 output switch enable"]
    #[inline(always)]
    pub fn c1osen(&mut self) -> COSEN_W<'_, CM1_OUTPUT_SPEC> {
        COSEN_W::new(self, 7)
    }
    #[doc = "Bit 15 - Channel 2 output switch enable"]
    #[inline(always)]
    pub fn c2osen(&mut self) -> COSEN_W<'_, CM1_OUTPUT_SPEC> {
        COSEN_W::new(self, 15)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    pub fn c2c(&mut self) -> C2C_W<'_, CM1_OUTPUT_SPEC> {
        C2C_W::new(self, 8)
    }
}
#[doc = "Channel output mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`cm1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM1_OUTPUT_SPEC;
impl crate::RegisterSpec for CM1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_output::R`](R) reader structure"]
impl crate::Readable for CM1_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm1_output::W`](W) writer structure"]
impl crate::Writable for CM1_OUTPUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CM1_OUTPUT to value 0"]
impl crate::Resettable for CM1_OUTPUT_SPEC {}
