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
#[doc = "Field `C3OIEN` reader - Channel 3 output immediately enable"]
pub type C3OIEN_R = crate::BitReader;
#[doc = "Field `C3OIEN` writer - Channel 3 output immediately enable"]
pub type C3OIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3OBEN` reader - Channel 3 output buffer enable"]
pub type C3OBEN_R = crate::BitReader;
#[doc = "Field `C3OBEN` writer - Channel 3 output buffer enable"]
pub type C3OBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3OCTRL` reader - Channel 3 output control"]
pub type C3OCTRL_R = crate::FieldReader;
#[doc = "Field `C3OCTRL` writer - Channel 3 output control"]
pub type C3OCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `C3OSEN` reader - Channel 3 output switch enable"]
pub type C3OSEN_R = crate::BitReader;
#[doc = "Field `C3OSEN` writer - Channel 3 output switch enable"]
pub type C3OSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
#[doc = "Field `C4OIEN` reader - Channel 4 output immediately enable"]
pub type C4OIEN_R = crate::BitReader;
#[doc = "Field `C4OIEN` writer - Channel 4 output immediately enable"]
pub type C4OIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C4OBEN` reader - Channel 4 output buffer enable"]
pub type C4OBEN_R = crate::BitReader;
#[doc = "Field `C4OBEN` writer - Channel 4 output buffer enable"]
pub type C4OBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C4OCTRL` reader - Channel 4 output control"]
pub type C4OCTRL_R = crate::FieldReader;
#[doc = "Field `C4OCTRL` writer - Channel 4 output control"]
pub type C4OCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `C4OSEN` reader - Channel 4 output switch enable"]
pub type C4OSEN_R = crate::BitReader;
#[doc = "Field `C4OSEN` writer - Channel 4 output switch enable"]
pub type C4OSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    pub fn c3c(&self) -> C3C_R {
        C3C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 3 output immediately enable"]
    #[inline(always)]
    pub fn c3oien(&self) -> C3OIEN_R {
        C3OIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 output buffer enable"]
    #[inline(always)]
    pub fn c3oben(&self) -> C3OBEN_R {
        C3OBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 3 output control"]
    #[inline(always)]
    pub fn c3octrl(&self) -> C3OCTRL_R {
        C3OCTRL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 3 output switch enable"]
    #[inline(always)]
    pub fn c3osen(&self) -> C3OSEN_R {
        C3OSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    pub fn c4c(&self) -> C4C_R {
        C4C_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 4 output immediately enable"]
    #[inline(always)]
    pub fn c4oien(&self) -> C4OIEN_R {
        C4OIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 4 output buffer enable"]
    #[inline(always)]
    pub fn c4oben(&self) -> C4OBEN_R {
        C4OBEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 4 output control"]
    #[inline(always)]
    pub fn c4octrl(&self) -> C4OCTRL_R {
        C4OCTRL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Channel 4 output switch enable"]
    #[inline(always)]
    pub fn c4osen(&self) -> C4OSEN_R {
        C4OSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c3c(&mut self) -> C3C_W<CM2_OUTPUT_SPEC, 0> {
        C3C_W::new(self)
    }
    #[doc = "Bit 2 - Channel 3 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3oien(&mut self) -> C3OIEN_W<CM2_OUTPUT_SPEC, 2> {
        C3OIEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3oben(&mut self) -> C3OBEN_W<CM2_OUTPUT_SPEC, 3> {
        C3OBEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 3 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c3octrl(&mut self) -> C3OCTRL_W<CM2_OUTPUT_SPEC, 4> {
        C3OCTRL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 3 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3osen(&mut self) -> C3OSEN_W<CM2_OUTPUT_SPEC, 7> {
        C3OSEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c4c(&mut self) -> C4C_W<CM2_OUTPUT_SPEC, 8> {
        C4C_W::new(self)
    }
    #[doc = "Bit 10 - Channel 4 output immediately enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4oien(&mut self) -> C4OIEN_W<CM2_OUTPUT_SPEC, 10> {
        C4OIEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 4 output buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4oben(&mut self) -> C4OBEN_W<CM2_OUTPUT_SPEC, 11> {
        C4OBEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 4 output control"]
    #[inline(always)]
    #[must_use]
    pub fn c4octrl(&mut self) -> C4OCTRL_W<CM2_OUTPUT_SPEC, 12> {
        C4OCTRL_W::new(self)
    }
    #[doc = "Bit 15 - Channel 4 output switch enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4osen(&mut self) -> C4OSEN_W<CM2_OUTPUT_SPEC, 15> {
        C4OSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
