#[doc = "Register `CM1_INPUT` reader"]
pub type R = crate::R<CM1_INPUT_SPEC>;
#[doc = "Register `CM1_INPUT` writer"]
pub type W = crate::W<CM1_INPUT_SPEC>;
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
impl crate::IsEnum for C1IDIV_A {}
#[doc = "Field `CIDIV(1-2)` reader - Channel %s input divider"]
pub type CIDIV_R = crate::FieldReader<C1IDIV_A>;
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
#[doc = "Field `CIDIV(1-2)` writer - Channel %s input divider"]
pub type CIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C1IDIV_A, crate::Safe>;
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
#[doc = "Channel %s digital filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDF_A {
    #[doc = "0: No filter, sampling is done at f_DTS"]
    NoFilter = 0,
    #[doc = "1: f_sampling = f_DTS, N=2"]
    N2 = 1,
    #[doc = "2: f_sampling = f_DTS, N=4"]
    N4 = 2,
    #[doc = "3: f_sampling = f_DTS, N=8"]
    N8 = 3,
    #[doc = "4: f_sampling = f_DTS/2, N=6"]
    Div2n6 = 4,
    #[doc = "5: f_sampling = f_DTS/2, N=8"]
    Div2n8 = 5,
    #[doc = "6: f_sampling = f_DTS/4, N=6"]
    Div4n6 = 6,
    #[doc = "7: f_sampling = f_DTS/4, N=8"]
    Div4n8 = 7,
    #[doc = "8: f_sampling = f_DTS/8, N=6"]
    Div8n6 = 8,
    #[doc = "9: f_sampling = f_DTS/8, N=8"]
    Div8n8 = 9,
    #[doc = "10: f_sampling = f_DTS/16, N=5"]
    Div16n5 = 10,
    #[doc = "11: f_sampling = f_DTS/16, N=6"]
    Div16n6 = 11,
    #[doc = "12: f_sampling = f_DTS/16, N=8"]
    Div16n8 = 12,
    #[doc = "13: f_sampling = f_DTS/32, N=5"]
    Div32n5 = 13,
    #[doc = "14: f_sampling = f_DTS/32, N=6"]
    Div32n6 = 14,
    #[doc = "15: f_sampling = f_DTS/32, N=8"]
    Div32n8 = 15,
}
impl From<CDF_A> for u8 {
    #[inline(always)]
    fn from(variant: CDF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDF_A {
    type Ux = u8;
}
impl crate::IsEnum for CDF_A {}
#[doc = "Field `CDF(1-2)` reader - Channel %s digital filter"]
pub type CDF_R = crate::FieldReader<CDF_A>;
impl CDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CDF_A {
        match self.bits {
            0 => CDF_A::NoFilter,
            1 => CDF_A::N2,
            2 => CDF_A::N4,
            3 => CDF_A::N8,
            4 => CDF_A::Div2n6,
            5 => CDF_A::Div2n8,
            6 => CDF_A::Div4n6,
            7 => CDF_A::Div4n8,
            8 => CDF_A::Div8n6,
            9 => CDF_A::Div8n8,
            10 => CDF_A::Div16n5,
            11 => CDF_A::Div16n6,
            12 => CDF_A::Div16n8,
            13 => CDF_A::Div32n5,
            14 => CDF_A::Div32n6,
            15 => CDF_A::Div32n8,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at f_DTS"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == CDF_A::NoFilter
    }
    #[doc = "f_sampling = f_DTS, N=2"]
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        *self == CDF_A::N2
    }
    #[doc = "f_sampling = f_DTS, N=4"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == CDF_A::N4
    }
    #[doc = "f_sampling = f_DTS, N=8"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == CDF_A::N8
    }
    #[doc = "f_sampling = f_DTS/2, N=6"]
    #[inline(always)]
    pub fn is_div2n6(&self) -> bool {
        *self == CDF_A::Div2n6
    }
    #[doc = "f_sampling = f_DTS/2, N=8"]
    #[inline(always)]
    pub fn is_div2n8(&self) -> bool {
        *self == CDF_A::Div2n8
    }
    #[doc = "f_sampling = f_DTS/4, N=6"]
    #[inline(always)]
    pub fn is_div4n6(&self) -> bool {
        *self == CDF_A::Div4n6
    }
    #[doc = "f_sampling = f_DTS/4, N=8"]
    #[inline(always)]
    pub fn is_div4n8(&self) -> bool {
        *self == CDF_A::Div4n8
    }
    #[doc = "f_sampling = f_DTS/8, N=6"]
    #[inline(always)]
    pub fn is_div8n6(&self) -> bool {
        *self == CDF_A::Div8n6
    }
    #[doc = "f_sampling = f_DTS/8, N=8"]
    #[inline(always)]
    pub fn is_div8n8(&self) -> bool {
        *self == CDF_A::Div8n8
    }
    #[doc = "f_sampling = f_DTS/16, N=5"]
    #[inline(always)]
    pub fn is_div16n5(&self) -> bool {
        *self == CDF_A::Div16n5
    }
    #[doc = "f_sampling = f_DTS/16, N=6"]
    #[inline(always)]
    pub fn is_div16n6(&self) -> bool {
        *self == CDF_A::Div16n6
    }
    #[doc = "f_sampling = f_DTS/16, N=8"]
    #[inline(always)]
    pub fn is_div16n8(&self) -> bool {
        *self == CDF_A::Div16n8
    }
    #[doc = "f_sampling = f_DTS/32, N=5"]
    #[inline(always)]
    pub fn is_div32n5(&self) -> bool {
        *self == CDF_A::Div32n5
    }
    #[doc = "f_sampling = f_DTS/32, N=6"]
    #[inline(always)]
    pub fn is_div32n6(&self) -> bool {
        *self == CDF_A::Div32n6
    }
    #[doc = "f_sampling = f_DTS/32, N=8"]
    #[inline(always)]
    pub fn is_div32n8(&self) -> bool {
        *self == CDF_A::Div32n8
    }
}
#[doc = "Field `CDF(1-2)` writer - Channel %s digital filter"]
pub type CDF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CDF_A, crate::Safe>;
impl<'a, REG> CDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at f_DTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::NoFilter)
    }
    #[doc = "f_sampling = f_DTS, N=2"]
    #[inline(always)]
    pub fn n2(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::N2)
    }
    #[doc = "f_sampling = f_DTS, N=4"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::N4)
    }
    #[doc = "f_sampling = f_DTS, N=8"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::N8)
    }
    #[doc = "f_sampling = f_DTS/2, N=6"]
    #[inline(always)]
    pub fn div2n6(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div2n6)
    }
    #[doc = "f_sampling = f_DTS/2, N=8"]
    #[inline(always)]
    pub fn div2n8(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div2n8)
    }
    #[doc = "f_sampling = f_DTS/4, N=6"]
    #[inline(always)]
    pub fn div4n6(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div4n6)
    }
    #[doc = "f_sampling = f_DTS/4, N=8"]
    #[inline(always)]
    pub fn div4n8(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div4n8)
    }
    #[doc = "f_sampling = f_DTS/8, N=6"]
    #[inline(always)]
    pub fn div8n6(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div8n6)
    }
    #[doc = "f_sampling = f_DTS/8, N=8"]
    #[inline(always)]
    pub fn div8n8(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div8n8)
    }
    #[doc = "f_sampling = f_DTS/16, N=5"]
    #[inline(always)]
    pub fn div16n5(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div16n5)
    }
    #[doc = "f_sampling = f_DTS/16, N=6"]
    #[inline(always)]
    pub fn div16n6(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div16n6)
    }
    #[doc = "f_sampling = f_DTS/16, N=8"]
    #[inline(always)]
    pub fn div16n8(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div16n8)
    }
    #[doc = "f_sampling = f_DTS/32, N=5"]
    #[inline(always)]
    pub fn div32n5(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div32n5)
    }
    #[doc = "f_sampling = f_DTS/32, N=6"]
    #[inline(always)]
    pub fn div32n6(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div32n6)
    }
    #[doc = "f_sampling = f_DTS/32, N=8"]
    #[inline(always)]
    pub fn div32n8(self) -> &'a mut crate::W<REG> {
        self.variant(CDF_A::Div32n8)
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
    #[doc = "Channel (1-2) input divider"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IDIV` field.</div>"]
    #[inline(always)]
    pub fn cidiv(&self, n: u8) -> CIDIV_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CIDIV_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-2) input divider"]
    #[inline(always)]
    pub fn cidiv_iter(&self) -> impl Iterator<Item = CIDIV_R> + '_ {
        (0..2).map(move |n| CIDIV_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8))
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
    #[doc = "Channel (1-2) digital filter"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1DF` field.</div>"]
    #[inline(always)]
    pub fn cdf(&self, n: u8) -> CDF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CDF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-2) digital filter"]
    #[inline(always)]
    pub fn cdf_iter(&self) -> impl Iterator<Item = CDF_R> + '_ {
        (0..2).map(move |n| CDF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8))
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
            .field("c1df", &self.c1df())
            .field("c2df", &self.c2df())
            .field("c1idiv", &self.c1idiv())
            .field("c2idiv", &self.c2idiv())
            .field("c2c", &self.c2c())
            .field("c1c", &self.c1c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c1c(&mut self) -> C1C_W<CM1_INPUT_SPEC> {
        C1C_W::new(self, 0)
    }
    #[doc = "Channel (1-2) input divider"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IDIV` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn cidiv(&mut self, n: u8) -> CIDIV_W<CM1_INPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CIDIV_W::new(self, n * 8 + 2)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c1idiv(&mut self) -> CIDIV_W<CM1_INPUT_SPEC> {
        CIDIV_W::new(self, 2)
    }
    #[doc = "Bits 10:11 - Channel 2 input divider"]
    #[inline(always)]
    #[must_use]
    pub fn c2idiv(&mut self) -> CIDIV_W<CM1_INPUT_SPEC> {
        CIDIV_W::new(self, 10)
    }
    #[doc = "Channel (1-2) digital filter"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1DF` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn cdf(&mut self, n: u8) -> CDF_W<CM1_INPUT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CDF_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c1df(&mut self) -> CDF_W<CM1_INPUT_SPEC> {
        CDF_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - Channel 2 digital filter"]
    #[inline(always)]
    #[must_use]
    pub fn c2df(&mut self) -> CDF_W<CM1_INPUT_SPEC> {
        CDF_W::new(self, 12)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    #[must_use]
    pub fn c2c(&mut self) -> C2C_W<CM1_INPUT_SPEC> {
        C2C_W::new(self, 8)
    }
}
#[doc = "Channel input mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM1_INPUT_SPEC;
impl crate::RegisterSpec for CM1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_input::R`](R) reader structure"]
impl crate::Readable for CM1_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm1_input::W`](W) writer structure"]
impl crate::Writable for CM1_INPUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM1_INPUT to value 0"]
impl crate::Resettable for CM1_INPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
