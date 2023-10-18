#[doc = "Register `CM1_OUTPUT` reader"]
pub type R = crate::R<CM1_OUTPUT_SPEC>;
#[doc = "Register `CM1_OUTPUT` writer"]
pub type W = crate::W<CM1_OUTPUT_SPEC>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1C_R = crate::FieldReader;
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1C_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `COIEN[1-1]` reader - Channel %s output immediately enable"]
pub type COIEN_R = crate::BitReader<C1OIEN_A>;
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
#[doc = "Field `COIEN[1-1]` writer - Channel %s output immediately enable"]
pub type COIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1OIEN_A>;
impl<'a, REG, const O: u8> COIEN_W<'a, REG, O>
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
#[doc = "Field `COBEN[1-1]` reader - Channel %s output buffer enable"]
pub type COBEN_R = crate::BitReader<C1OBENR_A>;
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1OBENR_A {
    #[doc = "0: Channel output buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Channel output buffer is enabled"]
    Enabled = 1,
}
impl From<C1OBENR_A> for bool {
    #[inline(always)]
    fn from(variant: C1OBENR_A) -> Self {
        variant as u8 != 0
    }
}
impl COBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1OBENR_A {
        match self.bits {
            false => C1OBENR_A::Disabled,
            true => C1OBENR_A::Enabled,
        }
    }
    #[doc = "Channel output buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1OBENR_A::Disabled
    }
    #[doc = "Channel output buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1OBENR_A::Enabled
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1OBENW_AW {
    #[doc = "0: Channel output buffer disable"]
    Disable = 0,
    #[doc = "1: Channel output buffer enable"]
    Enable = 1,
}
impl From<C1OBENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1OBENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN[1-1]` writer - Channel %s output buffer enable"]
pub type COBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1OBENW_AW>;
impl<'a, REG, const O: u8> COBEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1OBENW_AW::Disable)
    }
    #[doc = "Channel output buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1OBENW_AW::Enable)
    }
}
#[doc = "Field `COCTRL[1-1]` reader - Channel %s output control"]
pub type COCTRL_R = crate::FieldReader;
#[doc = "Field `COCTRL[1-1]` writer - Channel %s output control"]
pub type COCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `C1OSEN` reader - Channel 1 output switch enable"]
pub type C1OSEN_R = crate::BitReader;
#[doc = "Field `C1OSEN` writer - Channel 1 output switch enable"]
pub type C1OSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1C_R {
        C1C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Channel [1-1]
output immediately enable"]
    #[inline(always)]
    pub unsafe fn coien(&self, n: u8) -> COIEN_R {
        COIEN_R::new(((self.bits >> ((n - 1) * 0 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    pub fn c1oien(&self) -> COIEN_R {
        COIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Channel [1-1]
output buffer enable"]
    #[inline(always)]
    pub unsafe fn coben(&self, n: u8) -> COBEN_R {
        COBEN_R::new(((self.bits >> ((n - 1) * 0 + 3)) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    pub fn c1oben(&self) -> COBEN_R {
        COBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Channel [1-1]
output control"]
    #[inline(always)]
    pub unsafe fn coctrl(&self, n: u8) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> ((n - 1) * 0 + 4)) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    pub fn c1octrl(&self) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 1 output switch enable"]
    #[inline(always)]
    pub fn c1osen(&self) -> C1OSEN_R {
        C1OSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_OUTPUT")
            .field("c1osen", &format_args!("{}", self.c1osen().bit()))
            .field("c1octrl", &format_args!("{}", self.c1octrl().bits()))
            .field("c1oben", &format_args!("{}", self.c1oben().bit()))
            .field("c1oien", &format_args!("{}", self.c1oien().bit()))
            .field("c1c", &format_args!("{}", self.c1c().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CM1_OUTPUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c1c(&mut self) -> C1C_W<CM1_OUTPUT_SPEC, 0> {
        C1C_W::new(self)
    }
    #[doc = "Channel [1-1]
output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coien<const O: u8>(&mut self) -> COIEN_W<CM1_OUTPUT_SPEC, O> {
        COIEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1oien(&mut self) -> COIEN_W<CM1_OUTPUT_SPEC, 2> {
        COIEN_W::new(self)
    }
    #[doc = "Channel [1-1]
output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coben<const O: u8>(&mut self) -> COBEN_W<CM1_OUTPUT_SPEC, O> {
        COBEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1oben(&mut self) -> COBEN_W<CM1_OUTPUT_SPEC, 3> {
        COBEN_W::new(self)
    }
    #[doc = "Channel [1-1]
output control"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coctrl<const O: u8>(&mut self) -> COCTRL_W<CM1_OUTPUT_SPEC, O> {
        COCTRL_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 1 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c1octrl(&mut self) -> COCTRL_W<CM1_OUTPUT_SPEC, 4> {
        COCTRL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 1 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1osen(&mut self) -> C1OSEN_W<CM1_OUTPUT_SPEC, 7> {
        C1OSEN_W::new(self)
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
#[doc = "Channel output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM1_OUTPUT_SPEC;
impl crate::RegisterSpec for CM1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_output::R`](R) reader structure"]
impl crate::Readable for CM1_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm1_output::W`](W) writer structure"]
impl crate::Writable for CM1_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM1_OUTPUT to value 0"]
impl crate::Resettable for CM1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
