#[doc = "Register `CM1_INPUT` reader"]
pub type R = crate::R<CM1_INPUT_SPEC>;
#[doc = "Register `CM1_INPUT` writer"]
pub type W = crate::W<CM1_INPUT_SPEC>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1C_R = crate::FieldReader<C1C_A>;
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
impl C1C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1C_A {
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
pub type C1C_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, C1C_A>;
impl<'a, REG, const O: u8> C1C_W<'a, REG, O>
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
#[doc = "Field `CIDIV[1-2]` reader - Channel %s input divider"]
pub type CIDIV_R = crate::FieldReader<C1IDIV_A>;
#[doc = "Channel %s input divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1IDIV_A {
    #[doc = "0: No divider. An input capture is generated at each active edge."]
    Every = 0,
    #[doc = "1: An input compare is generated every 2 active edges"]
    Div2 = 1,
    #[doc = "2: An input compare is generated every 4 active edges"]
    Div4 = 2,
    #[doc = "3: An input compare is generated every 8 active edges"]
    Div8 = 3,
}
impl From<C1IDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: C1IDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1IDIV_A {
    type Ux = u8;
}
impl CIDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1IDIV_A {
        match self.bits {
            0 => C1IDIV_A::Every,
            1 => C1IDIV_A::Div2,
            2 => C1IDIV_A::Div4,
            3 => C1IDIV_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "No divider. An input capture is generated at each active edge."]
    #[inline(always)]
    pub fn is_every(&self) -> bool {
        *self == C1IDIV_A::Every
    }
    #[doc = "An input compare is generated every 2 active edges"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == C1IDIV_A::Div2
    }
    #[doc = "An input compare is generated every 4 active edges"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == C1IDIV_A::Div4
    }
    #[doc = "An input compare is generated every 8 active edges"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == C1IDIV_A::Div8
    }
}
#[doc = "Field `CIDIV[1-2]` writer - Channel %s input divider"]
pub type CIDIV_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, C1IDIV_A>;
impl<'a, REG, const O: u8> CIDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No divider. An input capture is generated at each active edge."]
    #[inline(always)]
    pub fn every(self) -> &'a mut crate::W<REG> {
        self.variant(C1IDIV_A::Every)
    }
    #[doc = "An input compare is generated every 2 active edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(C1IDIV_A::Div2)
    }
    #[doc = "An input compare is generated every 4 active edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(C1IDIV_A::Div4)
    }
    #[doc = "An input compare is generated every 8 active edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(C1IDIV_A::Div8)
    }
}
#[doc = "Field `CDF[1-2]` reader - Channel %s digital filter"]
pub type CDF_R = crate::FieldReader;
#[doc = "Field `CDF[1-2]` writer - Channel %s digital filter"]
pub type CDF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `C2C` reader - Channel 2 configure"]
pub type C2C_R = crate::FieldReader<C2C_A>;
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
impl C2C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C2C_A {
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
pub type C2C_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, C2C_A>;
impl<'a, REG, const O: u8> C2C_W<'a, REG, O>
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
    #[doc = "Channel [1-2]
input divider"]
    #[inline(always)]
    pub unsafe fn cidiv(&self, n: u8) -> CIDIV_R {
        CIDIV_R::new(((self.bits >> ((n - 1) * 8 + 2)) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    pub fn c1idiv(&self) -> CIDIV_R {
        CIDIV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 2 input divider"]
    #[inline(always)]
    pub fn c2idiv(&self) -> CIDIV_R {
        CIDIV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Channel [1-2]
digital filter"]
    #[inline(always)]
    pub unsafe fn cdf(&self, n: u8) -> CDF_R {
        CDF_R::new(((self.bits >> ((n - 1) * 8 + 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    pub fn c1df(&self) -> CDF_R {
        CDF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 2 digital filter"]
    #[inline(always)]
    pub fn c2df(&self) -> CDF_R {
        CDF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    pub fn c2c(&self) -> C2C_R {
        C2C_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_INPUT")
            .field("c1df", &format_args!("{}", self.c1df().bits()))
            .field("c2df", &format_args!("{}", self.c2df().bits()))
            .field("c1idiv", &format_args!("{}", self.c1idiv().bits()))
            .field("c2idiv", &format_args!("{}", self.c2idiv().bits()))
            .field("c2c", &format_args!("{}", self.c2c().bits()))
            .field("c1c", &format_args!("{}", self.c1c().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CM1_INPUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c1c(&mut self) -> C1C_W<CM1_INPUT_SPEC, 0> {
        C1C_W::new(self)
    }
    #[doc = "Channel [1-2]
input divider"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cidiv<const O: u8>(&mut self) -> CIDIV_W<CM1_INPUT_SPEC, O> {
        CIDIV_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c1idiv(&mut self) -> CIDIV_W<CM1_INPUT_SPEC, 2> {
        CIDIV_W::new(self)
    }
    #[doc = "Bits 10:11 - Channel 2 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c2idiv(&mut self) -> CIDIV_W<CM1_INPUT_SPEC, 10> {
        CIDIV_W::new(self)
    }
    #[doc = "Channel [1-2]
digital filter"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cdf<const O: u8>(&mut self) -> CDF_W<CM1_INPUT_SPEC, O> {
        CDF_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c1df(&mut self) -> CDF_W<CM1_INPUT_SPEC, 4> {
        CDF_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 2 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c2df(&mut self) -> CDF_W<CM1_INPUT_SPEC, 12> {
        CDF_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c2c(&mut self) -> C2C_W<CM1_INPUT_SPEC, 8> {
        C2C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel input mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM1_INPUT_SPEC;
impl crate::RegisterSpec for CM1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_input::R`](R) reader structure"]
impl crate::Readable for CM1_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm1_input::W`](W) writer structure"]
impl crate::Writable for CM1_INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM1_INPUT to value 0"]
impl crate::Resettable for CM1_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
