#[doc = "Register `CM1_OUTPUT` reader"]
pub type R = crate::R<CM1_OUTPUT_SPEC>;
#[doc = "Register `CM1_OUTPUT` writer"]
pub type W = crate::W<CM1_OUTPUT_SPEC>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1C_R = crate::FieldReader;
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1C_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
#[doc = "Field `COIEN(1-1)` reader - Channel %s output immediately enable"]
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
#[doc = "Field `COIEN(1-1)` writer - Channel %s output immediately enable"]
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
#[doc = "Field `COBEN(1-1)` reader - Channel %s output buffer enable"]
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
#[doc = "Field `COBEN(1-1)` writer - Channel %s output buffer enable"]
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
#[doc = "Field `COCTRL(1-1)` reader - Channel %s output control"]
pub type COCTRL_R = crate::FieldReader;
#[doc = "Field `COCTRL(1-1)` writer - Channel %s output control"]
pub type COCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1C_R {
        C1C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Channel (1-1) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&self, n: u8) -> COIEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COIEN_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) output immediately enable"]
    #[inline(always)]
    pub fn coien_iter(&self) -> impl Iterator<Item = COIEN_R> + '_ {
        (0..1).map(move |n| COIEN_R::new(((self.bits >> (n * 0 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    pub fn c1oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Channel (1-1) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&self, n: u8) -> COBEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COBEN_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) output buffer enable"]
    #[inline(always)]
    pub fn coben_iter(&self) -> impl Iterator<Item = COBEN_R> + '_ {
        (0..1).map(move |n| COBEN_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    pub fn c1oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Channel (1-1) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&self, n: u8) -> COCTRL_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COCTRL_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) output control"]
    #[inline(always)]
    pub fn coctrl_iter(&self) -> impl Iterator<Item = COCTRL_R> + '_ {
        (0..1).map(move |n| COCTRL_R::new(((self.bits >> (n * 0 + 4)) & 7) as u8))
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    pub fn c1octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_OUTPUT")
            .field("c1octrl", &self.c1octrl())
            .field("c1oben", &self.c1oben())
            .field("c1oien", &self.c1oien())
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
    #[doc = "Channel (1-1) output immediately enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OIEN` field.</div>"]
    #[inline(always)]
    pub fn coien(&mut self, n: u8) -> COIEN_W<'_, CM1_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COIEN_W::new(self, n * 0 + 2)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    pub fn c1oien(&mut self) -> COIEN_W<'_, CM1_OUTPUT_SPEC> {
        COIEN_W::new(self, 2)
    }
    #[doc = "Channel (1-1) output buffer enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OBEN` field.</div>"]
    #[inline(always)]
    pub fn coben(&mut self, n: u8) -> COBEN_W<'_, CM1_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COBEN_W::new(self, n * 0 + 3)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    pub fn c1oben(&mut self) -> COBEN_W<'_, CM1_OUTPUT_SPEC> {
        COBEN_W::new(self, 3)
    }
    #[doc = "Channel (1-1) output control"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1OCTRL` field.</div>"]
    #[inline(always)]
    pub fn coctrl(&mut self, n: u8) -> COCTRL_W<'_, CM1_OUTPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        COCTRL_W::new(self, n * 0 + 4)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    pub fn c1octrl(&mut self) -> COCTRL_W<'_, CM1_OUTPUT_SPEC> {
        COCTRL_W::new(self, 4)
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
