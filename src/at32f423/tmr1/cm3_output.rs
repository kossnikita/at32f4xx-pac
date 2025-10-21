#[doc = "Register `CM3_OUTPUT` reader"]
pub type R = crate::R<CM3_OUTPUT_SPEC>;
#[doc = "Register `CM3_OUTPUT` writer"]
pub type W = crate::W<CM3_OUTPUT_SPEC>;
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
#[doc = "Field `COIEN(5-5)` reader - Channel %s output immediately enable"]
pub type COIEN_R = crate::BitReader<C5OIEN_A>;
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
#[doc = "Field `COIEN(5-5)` writer - Channel %s output immediately enable"]
pub type COIEN_W<'a, REG> = crate::BitWriter<'a, REG, C5OIEN_A>;
impl<'a, REG> COIEN_W<'a, REG>
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
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5obenr {
    #[doc = "0: Channel output buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Channel output buffer is enabled"]
    Enabled = 1,
}
impl From<C5obenr> for bool {
    #[inline(always)]
    fn from(variant: C5obenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN(5-5)` reader - Channel %s output buffer enable"]
pub type COBEN_R = crate::BitReader<C5obenr>;
impl COBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C5obenr {
        match self.bits {
            false => C5obenr::Disabled,
            true => C5obenr::Enabled,
        }
    }
    #[doc = "Channel output buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C5obenr::Disabled
    }
    #[doc = "Channel output buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C5obenr::Enabled
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5obenwWO {
    #[doc = "0: Channel output buffer disable"]
    Disable = 0,
    #[doc = "1: Channel output buffer enable"]
    Enable = 1,
}
impl From<C5obenwWO> for bool {
    #[inline(always)]
    fn from(variant: C5obenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN(5-5)` writer - Channel %s output buffer enable"]
pub type COBEN_W<'a, REG> = crate::BitWriter<'a, REG, C5obenwWO>;
impl<'a, REG> COBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C5obenwWO::Disable)
    }
    #[doc = "Channel output buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C5obenwWO::Enable)
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
#[doc = "Field `COCTRL(5-5)` reader - Channel %s output control"]
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
#[doc = "Field `COCTRL(5-5)` writer - Channel %s output control"]
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
pub enum C5osenr {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disabled = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enabled = 1,
}
impl From<C5osenr> for bool {
    #[inline(always)]
    fn from(variant: C5osenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN(5-5)` reader - Channel %s output switch enable"]
pub type COSEN_R = crate::BitReader<C5osenr>;
impl COSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C5osenr {
        match self.bits {
            false => C5osenr::Disabled,
            true => C5osenr::Enabled,
        }
    }
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C5osenr::Disabled
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C5osenr::Enabled
    }
}
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C5osenwWO {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disable = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enable = 1,
}
impl From<C5osenwWO> for bool {
    #[inline(always)]
    fn from(variant: C5osenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN(5-5)` writer - Channel %s output switch enable"]
pub type COSEN_W<'a, REG> = crate::BitWriter<'a, REG, C5osenwWO>;
impl<'a, REG> COSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C5osenwWO::Disable)
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C5osenwWO::Enable)
    }
}
impl R {
    #[doc = "Channel (5-5) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&self, n: u8) -> COIEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COIEN_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (5-5) output immediately enable"]
    #[inline(always)]
    pub fn coien_iter(&self) -> impl Iterator<Item = COIEN_R> + '_ {
        (0..1).map(move |n| COIEN_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    pub fn c5oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Channel (5-5) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&self, n: u8) -> COBEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COBEN_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (5-5) output buffer enable"]
    #[inline(always)]
    pub fn coben_iter(&self) -> impl Iterator<Item = COBEN_R> + '_ {
        (0..1).map(move |n| COBEN_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    pub fn c5oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Channel (5-5) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&self, n: u8) -> COCTRL_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COCTRL_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (5-5) output control"]
    #[inline(always)]
    pub fn coctrl_iter(&self) -> impl Iterator<Item = COCTRL_R> + '_ {
        (0..1).map(move |n| COCTRL_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    pub fn c5octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Channel (5-5) output switch enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OSEN` field.</div>"]
    #[inline(always)]
    pub fn cosen(&self, n: u8) -> COSEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COSEN_R::new(((self.bits >> (n * 0 + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (5-5) output switch enable"]
    #[inline(always)]
    pub fn cosen_iter(&self) -> impl Iterator<Item = COSEN_R> + '_ {
        (0..1).map(move |n| COSEN_R::new(((self.bits >> (n * 0 + 7)) & 1) != 0))
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
            .field("c5osen", &self.c5osen())
            .field("c5octrl", &self.c5octrl())
            .field("c5oben", &self.c5oben())
            .field("c5oien", &self.c5oien())
            .finish()
    }
}
impl W {
    #[doc = "Channel (5-5) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&mut self, n: u8) -> COIEN_W<'_, CM3_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COIEN_W::new(self, n * 0 + 2)
    }
    #[doc = "Bit 2 - Channel 5 output immediately enable"]
    #[inline(always)]
    pub fn c5oien(&mut self) -> COIEN_W<'_, CM3_OUTPUT_SPEC> {
        COIEN_W::new(self, 2)
    }
    #[doc = "Channel (5-5) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&mut self, n: u8) -> COBEN_W<'_, CM3_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COBEN_W::new(self, n * 0 + 3)
    }
    #[doc = "Bit 3 - Channel 5 output buffer enable"]
    #[inline(always)]
    pub fn c5oben(&mut self) -> COBEN_W<'_, CM3_OUTPUT_SPEC> {
        COBEN_W::new(self, 3)
    }
    #[doc = "Channel (5-5) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&mut self, n: u8) -> COCTRL_W<'_, CM3_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COCTRL_W::new(self, n * 0 + 4)
    }
    #[doc = "Bits 4:6 - Channel 5 output control"]
    #[inline(always)]
    pub fn c5octrl(&mut self) -> COCTRL_W<'_, CM3_OUTPUT_SPEC> {
        COCTRL_W::new(self, 4)
    }
    #[doc = "Channel (5-5) output switch enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C5OSEN` field.</div>"]
    #[inline(always)]
    pub fn cosen(&mut self, n: u8) -> COSEN_W<'_, CM3_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COSEN_W::new(self, n * 0 + 7)
    }
    #[doc = "Bit 7 - Channel 5 output switch enable"]
    #[inline(always)]
    pub fn c5osen(&mut self) -> COSEN_W<'_, CM3_OUTPUT_SPEC> {
        COSEN_W::new(self, 7)
    }
}
#[doc = "Channel output mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`cm3_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm3_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM3_OUTPUT_SPEC;
impl crate::RegisterSpec for CM3_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm3_output::R`](R) reader structure"]
impl crate::Readable for CM3_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm3_output::W`](W) writer structure"]
impl crate::Writable for CM3_OUTPUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CM3_OUTPUT to value 0"]
impl crate::Resettable for CM3_OUTPUT_SPEC {}
