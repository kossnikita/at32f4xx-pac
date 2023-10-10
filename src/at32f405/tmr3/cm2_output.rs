#[doc = "Register `CM2_OUTPUT` reader"]
pub type R = crate::R<CM2_OUTPUT_SPEC>;
#[doc = "Register `CM2_OUTPUT` writer"]
pub type W = crate::W<CM2_OUTPUT_SPEC>;
#[doc = "Field `C3C` reader - Channel 3 configure"]
pub type C3C_R = crate::FieldReader<C3C_A>;
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
impl C3C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C3C_A {
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
pub type C3C_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, C3C_A>;
impl<'a, REG, const O: u8> C3C_W<'a, REG, O>
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
#[doc = "Field `COIEN[3-4]` reader - Channel %s output immediately enable"]
pub type COIEN_R = crate::BitReader<C3OIEN_A>;
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
impl COIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C3OIEN_A {
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
#[doc = "Field `COIEN[3-4]` writer - Channel %s output immediately enable"]
pub type COIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C3OIEN_A>;
impl<'a, REG, const O: u8> COIEN_W<'a, REG, O>
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
#[doc = "Field `COBEN[3-4]` reader - Channel %s output buffer enable"]
pub type COBEN_R = crate::BitReader<C3OBENR_A>;
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3OBENR_A {
    #[doc = "0: Channel output buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Channel output buffer is enabled"]
    Enabled = 1,
}
impl From<C3OBENR_A> for bool {
    #[inline(always)]
    fn from(variant: C3OBENR_A) -> Self {
        variant as u8 != 0
    }
}
impl COBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C3OBENR_A {
        match self.bits {
            false => C3OBENR_A::Disabled,
            true => C3OBENR_A::Enabled,
        }
    }
    #[doc = "Channel output buffer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C3OBENR_A::Disabled
    }
    #[doc = "Channel output buffer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C3OBENR_A::Enabled
    }
}
#[doc = "Channel %s output buffer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3OBENW_AW {
    #[doc = "0: Channel output buffer disable"]
    Disable = 0,
    #[doc = "1: Channel output buffer enable"]
    Enable = 1,
}
impl From<C3OBENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C3OBENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COBEN[3-4]` writer - Channel %s output buffer enable"]
pub type COBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C3OBENW_AW>;
impl<'a, REG, const O: u8> COBEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C3OBENW_AW::Disable)
    }
    #[doc = "Channel output buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C3OBENW_AW::Enable)
    }
}
#[doc = "Field `COCTRL[3-4]` reader - Channel %s output control"]
pub type COCTRL_R = crate::FieldReader;
#[doc = "Field `COCTRL[3-4]` writer - Channel %s output control"]
pub type COCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `COSEN[3-4]` reader - Channel %s output switch enable"]
pub type COSEN_R = crate::BitReader<C3OSENR_A>;
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3OSENR_A {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disabled = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enabled = 1,
}
impl From<C3OSENR_A> for bool {
    #[inline(always)]
    fn from(variant: C3OSENR_A) -> Self {
        variant as u8 != 0
    }
}
impl COSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C3OSENR_A {
        match self.bits {
            false => C3OSENR_A::Disabled,
            true => C3OSENR_A::Enabled,
        }
    }
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C3OSENR_A::Disabled
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C3OSENR_A::Enabled
    }
}
#[doc = "Channel %s output switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C3OSENW_AW {
    #[doc = "0: CxORAW is not affected by EXT input"]
    Disable = 0,
    #[doc = "1: Once a high level is detect on EXT input, clear CxORAW"]
    Enable = 1,
}
impl From<C3OSENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C3OSENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEN[3-4]` writer - Channel %s output switch enable"]
pub type COSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C3OSENW_AW>;
impl<'a, REG, const O: u8> COSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxORAW is not affected by EXT input"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C3OSENW_AW::Disable)
    }
    #[doc = "Once a high level is detect on EXT input, clear CxORAW"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C3OSENW_AW::Enable)
    }
}
#[doc = "Field `C4C` reader - Channel 4 configure"]
pub type C4C_R = crate::FieldReader<C4C_A>;
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
impl C4C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C4C_A {
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
pub type C4C_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, C4C_A>;
impl<'a, REG, const O: u8> C4C_W<'a, REG, O>
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
    #[doc = "Channel [3-4]
output immediately enable"]
    #[inline(always)]
    pub unsafe fn coien(&self, n: u8) -> COIEN_R {
        COIEN_R::new(((self.bits >> ((n - 3) * 8 + 2)) & 1) != 0)
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
    #[doc = "Channel [3-4]
output buffer enable"]
    #[inline(always)]
    pub unsafe fn coben(&self, n: u8) -> COBEN_R {
        COBEN_R::new(((self.bits >> ((n - 3) * 8 + 3)) & 1) != 0)
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
    #[doc = "Channel [3-4]
output control"]
    #[inline(always)]
    pub unsafe fn coctrl(&self, n: u8) -> COCTRL_R {
        COCTRL_R::new(((self.bits >> ((n - 3) * 8 + 4)) & 7) as u8)
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
    #[doc = "Channel [3-4]
output switch enable"]
    #[inline(always)]
    pub unsafe fn cosen(&self, n: u8) -> COSEN_R {
        COSEN_R::new(((self.bits >> ((n - 3) * 8 + 7)) & 1) != 0)
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
            .field("c3osen", &format_args!("{}", self.c3osen().bit()))
            .field("c4osen", &format_args!("{}", self.c4osen().bit()))
            .field("c3octrl", &format_args!("{}", self.c3octrl().bits()))
            .field("c4octrl", &format_args!("{}", self.c4octrl().bits()))
            .field("c3oben", &format_args!("{}", self.c3oben().bit()))
            .field("c4oben", &format_args!("{}", self.c4oben().bit()))
            .field("c3oien", &format_args!("{}", self.c3oien().bit()))
            .field("c4oien", &format_args!("{}", self.c4oien().bit()))
            .field("c4c", &format_args!("{}", self.c4c().bits()))
            .field("c3c", &format_args!("{}", self.c3c().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CM2_OUTPUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c3c(&mut self) -> C3C_W<CM2_OUTPUT_SPEC, 0> {
        C3C_W::new(self)
    }
    #[doc = "Channel [3-4]
output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coien<const O: u8>(&mut self) -> COIEN_W<CM2_OUTPUT_SPEC, O> {
        COIEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 3 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3oien(&mut self) -> COIEN_W<CM2_OUTPUT_SPEC, 2> {
        COIEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel 4 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4oien(&mut self) -> COIEN_W<CM2_OUTPUT_SPEC, 10> {
        COIEN_W::new(self)
    }
    #[doc = "Channel [3-4]
output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coben<const O: u8>(&mut self) -> COBEN_W<CM2_OUTPUT_SPEC, O> {
        COBEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3oben(&mut self) -> COBEN_W<CM2_OUTPUT_SPEC, 3> {
        COBEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 4 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4oben(&mut self) -> COBEN_W<CM2_OUTPUT_SPEC, 11> {
        COBEN_W::new(self)
    }
    #[doc = "Channel [3-4]
output control"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn coctrl<const O: u8>(&mut self) -> COCTRL_W<CM2_OUTPUT_SPEC, O> {
        COCTRL_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 3 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c3octrl(&mut self) -> COCTRL_W<CM2_OUTPUT_SPEC, 4> {
        COCTRL_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 4 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c4octrl(&mut self) -> COCTRL_W<CM2_OUTPUT_SPEC, 12> {
        COCTRL_W::new(self)
    }
    #[doc = "Channel [3-4]
output switch enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cosen<const O: u8>(&mut self) -> COSEN_W<CM2_OUTPUT_SPEC, O> {
        COSEN_W::new(self)
    }
    #[doc = "Bit 7 - Channel 3 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3osen(&mut self) -> COSEN_W<CM2_OUTPUT_SPEC, 7> {
        COSEN_W::new(self)
    }
    #[doc = "Bit 15 - Channel 4 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4osen(&mut self) -> COSEN_W<CM2_OUTPUT_SPEC, 15> {
        COSEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c4c(&mut self) -> C4C_W<CM2_OUTPUT_SPEC, 8> {
        C4C_W::new(self)
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
#[doc = "Channel output mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm2_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm2_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM2_OUTPUT_SPEC;
impl crate::RegisterSpec for CM2_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm2_output::R`](R) reader structure"]
impl crate::Readable for CM2_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm2_output::W`](W) writer structure"]
impl crate::Writable for CM2_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM2_OUTPUT to value 0"]
impl crate::Resettable for CM2_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
