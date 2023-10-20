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
    #[doc = "0: Output"]
    Output = 0,
    #[doc = "1: Input, C1IN is mapped on C1IFP1"]
    Input = 1,
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
    pub const fn variant(&self) -> Option<C1C_A> {
        match self.bits {
            0 => Some(C1C_A::Output),
            1 => Some(C1C_A::Input),
            _ => None,
        }
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == C1C_A::Output
    }
    #[doc = "Input, C1IN is mapped on C1IFP1"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == C1C_A::Input
    }
}
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1C_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C1C_A>;
impl<'a, REG> C1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(C1C_A::Output)
    }
    #[doc = "Input, C1IN is mapped on C1IFP1"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(C1C_A::Input)
    }
}
#[doc = "Field `CIDIV[1-1]` reader - Channel %s input divider"]
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
    pub const fn variant(&self) -> C1IDIV_A {
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
#[doc = "Field `CIDIV[1-1]` writer - Channel %s input divider"]
pub type CIDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C1IDIV_A>;
impl<'a, REG> CIDIV_W<'a, REG>
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
#[doc = "Field `CDF[1-1]` reader - Channel %s digital filter"]
pub type CDF_R = crate::FieldReader;
#[doc = "Field `CDF[1-1]` writer - Channel %s digital filter"]
pub type CDF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1C_R {
        C1C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Channel [1-1]
input divider\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cidiv(&self, n: u8) -> CIDIV_R {
        assert!(n < 1);
        CIDIV_R::new(((self.bits >> (n * 0 + 2)) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    pub fn c1idiv(&self) -> CIDIV_R {
        CIDIV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Channel [1-1]
digital filter\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cdf(&self, n: u8) -> CDF_R {
        assert!(n < 1);
        CDF_R::new(((self.bits >> (n * 0 + 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    pub fn c1df(&self) -> CDF_R {
        CDF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_INPUT")
            .field("c1df", &format_args!("{}", self.c1df().bits()))
            .field("c1idiv", &format_args!("{}", self.c1idiv().bits()))
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
    pub fn c1c(&mut self) -> C1C_W<CM1_INPUT_SPEC> {
        C1C_W::new(self, 0)
    }
    #[doc = "Channel [1-1]
input divider"]
    #[inline(always)]
    #[must_use]
    pub fn cidiv(&mut self, n: u8) -> CIDIV_W<CM1_INPUT_SPEC> {
        assert!(n < 1);
        CIDIV_W::new(self, n * 0 + 2)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c1idiv(&mut self) -> CIDIV_W<CM1_INPUT_SPEC> {
        CIDIV_W::new(self, 2)
    }
    #[doc = "Channel [1-1]
digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn cdf(&mut self, n: u8) -> CDF_W<CM1_INPUT_SPEC> {
        assert!(n < 1);
        CDF_W::new(self, n * 0 + 4)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c1df(&mut self) -> CDF_W<CM1_INPUT_SPEC> {
        CDF_W::new(self, 4)
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
