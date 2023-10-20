#[doc = "Register `CM2_INPUT` reader"]
pub type R = crate::R<CM2_INPUT_SPEC>;
#[doc = "Register `CM2_INPUT` writer"]
pub type W = crate::W<CM2_INPUT_SPEC>;
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
pub type C3C_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C3C_A>;
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
#[doc = "Field `CIDIV[3-4]` reader - Channel %s input divider"]
pub type CIDIV_R = crate::FieldReader<C3IDIV_A>;
#[doc = "Channel %s input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C3IDIV_A {
    #[doc = "0: No divider. An input capture is generated at each active edge."]
    Every = 0,
    #[doc = "1: An input compare is generated every 2 active edges"]
    Div2 = 1,
    #[doc = "2: An input compare is generated every 4 active edges"]
    Div4 = 2,
    #[doc = "3: An input compare is generated every 8 active edges"]
    Div8 = 3,
}
impl From<C3IDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: C3IDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C3IDIV_A {
    type Ux = u8;
}
impl CIDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C3IDIV_A {
        match self.bits {
            0 => C3IDIV_A::Every,
            1 => C3IDIV_A::Div2,
            2 => C3IDIV_A::Div4,
            3 => C3IDIV_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "No divider. An input capture is generated at each active edge."]
    #[inline(always)]
    pub fn is_every(&self) -> bool {
        *self == C3IDIV_A::Every
    }
    #[doc = "An input compare is generated every 2 active edges"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == C3IDIV_A::Div2
    }
    #[doc = "An input compare is generated every 4 active edges"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == C3IDIV_A::Div4
    }
    #[doc = "An input compare is generated every 8 active edges"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == C3IDIV_A::Div8
    }
}
#[doc = "Field `CIDIV[3-4]` writer - Channel %s input divider"]
pub type CIDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C3IDIV_A>;
impl<'a, REG> CIDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No divider. An input capture is generated at each active edge."]
    #[inline(always)]
    pub fn every(self) -> &'a mut crate::W<REG> {
        self.variant(C3IDIV_A::Every)
    }
    #[doc = "An input compare is generated every 2 active edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(C3IDIV_A::Div2)
    }
    #[doc = "An input compare is generated every 4 active edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(C3IDIV_A::Div4)
    }
    #[doc = "An input compare is generated every 8 active edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(C3IDIV_A::Div8)
    }
}
#[doc = "Field `CDF[3-4]` reader - Channel %s digital filter"]
pub type CDF_R = crate::FieldReader;
#[doc = "Field `CDF[3-4]` writer - Channel %s digital filter"]
pub type CDF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
pub type C4C_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C4C_A>;
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
    #[doc = "Channel [3-4]
input divider\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cidiv(&self, n: u8) -> CIDIV_R {
        assert!(n < 2);
        CIDIV_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 3 input divider"]
    #[inline(always)]
    pub fn c3idiv(&self) -> CIDIV_R {
        CIDIV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 4 input divider"]
    #[inline(always)]
    pub fn c4idiv(&self) -> CIDIV_R {
        CIDIV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Channel [3-4]
digital filter\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cdf(&self, n: u8) -> CDF_R {
        assert!(n < 2);
        CDF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 3 digital filter"]
    #[inline(always)]
    pub fn c3df(&self) -> CDF_R {
        CDF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 4 digital filter"]
    #[inline(always)]
    pub fn c4df(&self) -> CDF_R {
        CDF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    pub fn c4c(&self) -> C4C_R {
        C4C_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM2_INPUT")
            .field("c3df", &format_args!("{}", self.c3df().bits()))
            .field("c4df", &format_args!("{}", self.c4df().bits()))
            .field("c3idiv", &format_args!("{}", self.c3idiv().bits()))
            .field("c4idiv", &format_args!("{}", self.c4idiv().bits()))
            .field("c4c", &format_args!("{}", self.c4c().bits()))
            .field("c3c", &format_args!("{}", self.c3c().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CM2_INPUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c3c(&mut self) -> C3C_W<CM2_INPUT_SPEC> {
        C3C_W::new(self, 0)
    }
    #[doc = "Channel [3-4]
input divider"]
    #[inline(always)]
    #[must_use]
    pub fn cidiv(&mut self, n: u8) -> CIDIV_W<CM2_INPUT_SPEC> {
        assert!(n < 2);
        CIDIV_W::new(self, n * 8 + 2)
    }
    #[doc = "Bits 2:3 - Channel 3 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c3idiv(&mut self) -> CIDIV_W<CM2_INPUT_SPEC> {
        CIDIV_W::new(self, 2)
    }
    #[doc = "Bits 10:11 - Channel 4 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c4idiv(&mut self) -> CIDIV_W<CM2_INPUT_SPEC> {
        CIDIV_W::new(self, 10)
    }
    #[doc = "Channel [3-4]
digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn cdf(&mut self, n: u8) -> CDF_W<CM2_INPUT_SPEC> {
        assert!(n < 2);
        CDF_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:7 - Channel 3 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c3df(&mut self) -> CDF_W<CM2_INPUT_SPEC> {
        CDF_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - Channel 4 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c4df(&mut self) -> CDF_W<CM2_INPUT_SPEC> {
        CDF_W::new(self, 12)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c4c(&mut self) -> C4C_W<CM2_INPUT_SPEC> {
        C4C_W::new(self, 8)
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
#[doc = "Channel input mode register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm2_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm2_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM2_INPUT_SPEC;
impl crate::RegisterSpec for CM2_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm2_input::R`](R) reader structure"]
impl crate::Readable for CM2_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm2_input::W`](W) writer structure"]
impl crate::Writable for CM2_INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM2_INPUT to value 0"]
impl crate::Resettable for CM2_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
