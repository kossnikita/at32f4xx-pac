#[doc = "Register `REMAP8` reader"]
pub type R = crate::R<REMAP8_SPEC>;
#[doc = "Register `REMAP8` writer"]
pub type W = crate::W<REMAP8_SPEC>;
#[doc = "Field `TMR1_BK1_CMP_GMUX` reader - TMR1 BK1 CMP muxing"]
pub type TMR1_BK1_CMP_GMUX_R = crate::FieldReader<TMR1_BK1_CMP_GMUX_A>;
#[doc = "TMR1 BK1 CMP muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR1_BK1_CMP_GMUX_A {
    #[doc = "0: TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    Mux0 = 0,
    #[doc = "1: TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    Mux1 = 1,
    #[doc = "2: CMP output signal is connected to TMR1 break channel 1"]
    Mux2 = 2,
    #[doc = "3: Either CMP output signal or TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    Mux3 = 3,
}
impl From<TMR1_BK1_CMP_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR1_BK1_CMP_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR1_BK1_CMP_GMUX_A {
    type Ux = u8;
}
impl TMR1_BK1_CMP_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1_BK1_CMP_GMUX_A {
        match self.bits {
            0 => TMR1_BK1_CMP_GMUX_A::Mux0,
            1 => TMR1_BK1_CMP_GMUX_A::Mux1,
            2 => TMR1_BK1_CMP_GMUX_A::Mux2,
            3 => TMR1_BK1_CMP_GMUX_A::Mux3,
            _ => unreachable!(),
        }
    }
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR1_BK1_CMP_GMUX_A::Mux0
    }
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR1_BK1_CMP_GMUX_A::Mux1
    }
    #[doc = "CMP output signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR1_BK1_CMP_GMUX_A::Mux2
    }
    #[doc = "Either CMP output signal or TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == TMR1_BK1_CMP_GMUX_A::Mux3
    }
}
#[doc = "Field `TMR1_BK1_CMP_GMUX` writer - TMR1 BK1 CMP muxing"]
pub type TMR1_BK1_CMP_GMUX_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TMR1_BK1_CMP_GMUX_A>;
impl<'a, REG, const O: u8> TMR1_BK1_CMP_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_BK1_CMP_GMUX_A::Mux0)
    }
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_BK1_CMP_GMUX_A::Mux1)
    }
    #[doc = "CMP output signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_BK1_CMP_GMUX_A::Mux2)
    }
    #[doc = "Either CMP output signal or TMR1_GMUX IO signal is connected to TMR1 break channel 1"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_BK1_CMP_GMUX_A::Mux3)
    }
}
#[doc = "Field `TMR1_CH1_CMP_GMUX` reader - TMR1 CH1 CMP muxing"]
pub type TMR1_CH1_CMP_GMUX_R = crate::FieldReader<TMR1_CH1_CMP_GMUX_A>;
#[doc = "TMR1 CH1 CMP muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR1_CH1_CMP_GMUX_A {
    #[doc = "0: TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    Mux0 = 0,
    #[doc = "1: TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    Mux1 = 1,
    #[doc = "2: CMP output signal is connected to TMR1 channel 1"]
    Mux2 = 2,
    #[doc = "3: Either CMP output signal or TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    Mux3 = 3,
}
impl From<TMR1_CH1_CMP_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR1_CH1_CMP_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR1_CH1_CMP_GMUX_A {
    type Ux = u8;
}
impl TMR1_CH1_CMP_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1_CH1_CMP_GMUX_A {
        match self.bits {
            0 => TMR1_CH1_CMP_GMUX_A::Mux0,
            1 => TMR1_CH1_CMP_GMUX_A::Mux1,
            2 => TMR1_CH1_CMP_GMUX_A::Mux2,
            3 => TMR1_CH1_CMP_GMUX_A::Mux3,
            _ => unreachable!(),
        }
    }
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR1_CH1_CMP_GMUX_A::Mux0
    }
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR1_CH1_CMP_GMUX_A::Mux1
    }
    #[doc = "CMP output signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR1_CH1_CMP_GMUX_A::Mux2
    }
    #[doc = "Either CMP output signal or TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == TMR1_CH1_CMP_GMUX_A::Mux3
    }
}
#[doc = "Field `TMR1_CH1_CMP_GMUX` writer - TMR1 CH1 CMP muxing"]
pub type TMR1_CH1_CMP_GMUX_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TMR1_CH1_CMP_GMUX_A>;
impl<'a, REG, const O: u8> TMR1_CH1_CMP_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_CH1_CMP_GMUX_A::Mux0)
    }
    #[doc = "TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_CH1_CMP_GMUX_A::Mux1)
    }
    #[doc = "CMP output signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_CH1_CMP_GMUX_A::Mux2)
    }
    #[doc = "Either CMP output signal or TMR1_GMUX IO signal is connected to TMR1 channel 1"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_CH1_CMP_GMUX_A::Mux3)
    }
}
#[doc = "Field `TMR2_CH4_CMP_GMUX` reader - TMR2 CH4 CMP muxing"]
pub type TMR2_CH4_CMP_GMUX_R = crate::FieldReader<TMR2_CH4_CMP_GMUX_A>;
#[doc = "TMR2 CH4 CMP muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR2_CH4_CMP_GMUX_A {
    #[doc = "0: TMR2_GMUX IO signal is connected to TMR2 channel 4"]
    Mux0 = 0,
    #[doc = "1: TMR2_GMUX IO signal is connected to TMR2 channel 4"]
    Mux1 = 1,
    #[doc = "2: CMP output signal is connected to TMR2 channel 4"]
    Mux2 = 2,
    #[doc = "3: Either CMP output signal or TMR1_GMUX IO signal is connected to TMR2 channel 4"]
    Mux3 = 3,
}
impl From<TMR2_CH4_CMP_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR2_CH4_CMP_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR2_CH4_CMP_GMUX_A {
    type Ux = u8;
}
impl TMR2_CH4_CMP_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2_CH4_CMP_GMUX_A {
        match self.bits {
            0 => TMR2_CH4_CMP_GMUX_A::Mux0,
            1 => TMR2_CH4_CMP_GMUX_A::Mux1,
            2 => TMR2_CH4_CMP_GMUX_A::Mux2,
            3 => TMR2_CH4_CMP_GMUX_A::Mux3,
            _ => unreachable!(),
        }
    }
    #[doc = "TMR2_GMUX IO signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR2_CH4_CMP_GMUX_A::Mux0
    }
    #[doc = "TMR2_GMUX IO signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR2_CH4_CMP_GMUX_A::Mux1
    }
    #[doc = "CMP output signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR2_CH4_CMP_GMUX_A::Mux2
    }
    #[doc = "Either CMP output signal or TMR1_GMUX IO signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == TMR2_CH4_CMP_GMUX_A::Mux3
    }
}
#[doc = "Field `TMR2_CH4_CMP_GMUX` writer - TMR2 CH4 CMP muxing"]
pub type TMR2_CH4_CMP_GMUX_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TMR2_CH4_CMP_GMUX_A>;
impl<'a, REG, const O: u8> TMR2_CH4_CMP_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMR2_GMUX IO signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_CH4_CMP_GMUX_A::Mux0)
    }
    #[doc = "TMR2_GMUX IO signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_CH4_CMP_GMUX_A::Mux1)
    }
    #[doc = "CMP output signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_CH4_CMP_GMUX_A::Mux2)
    }
    #[doc = "Either CMP output signal or TMR1_GMUX IO signal is connected to TMR2 channel 4"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_CH4_CMP_GMUX_A::Mux3)
    }
}
#[doc = "Field `TMR3_CH1_CMP_GMUX` reader - TMR3 CH1 CMP muxing"]
pub type TMR3_CH1_CMP_GMUX_R = crate::FieldReader<TMR3_CH1_CMP_GMUX_A>;
#[doc = "TMR3 CH1 CMP muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR3_CH1_CMP_GMUX_A {
    #[doc = "0: TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    Mux0 = 0,
    #[doc = "1: TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    Mux1 = 1,
    #[doc = "2: CMP output signal is connected to TMR3 channel 1"]
    Mux2 = 2,
    #[doc = "3: Either CMP output signal or TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    Mux3 = 3,
}
impl From<TMR3_CH1_CMP_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR3_CH1_CMP_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR3_CH1_CMP_GMUX_A {
    type Ux = u8;
}
impl TMR3_CH1_CMP_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3_CH1_CMP_GMUX_A {
        match self.bits {
            0 => TMR3_CH1_CMP_GMUX_A::Mux0,
            1 => TMR3_CH1_CMP_GMUX_A::Mux1,
            2 => TMR3_CH1_CMP_GMUX_A::Mux2,
            3 => TMR3_CH1_CMP_GMUX_A::Mux3,
            _ => unreachable!(),
        }
    }
    #[doc = "TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR3_CH1_CMP_GMUX_A::Mux0
    }
    #[doc = "TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR3_CH1_CMP_GMUX_A::Mux1
    }
    #[doc = "CMP output signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR3_CH1_CMP_GMUX_A::Mux2
    }
    #[doc = "Either CMP output signal or TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == TMR3_CH1_CMP_GMUX_A::Mux3
    }
}
#[doc = "Field `TMR3_CH1_CMP_GMUX` writer - TMR3 CH1 CMP muxing"]
pub type TMR3_CH1_CMP_GMUX_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, TMR3_CH1_CMP_GMUX_A>;
impl<'a, REG, const O: u8> TMR3_CH1_CMP_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_CH1_CMP_GMUX_A::Mux0)
    }
    #[doc = "TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_CH1_CMP_GMUX_A::Mux1)
    }
    #[doc = "CMP output signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_CH1_CMP_GMUX_A::Mux2)
    }
    #[doc = "Either CMP output signal or TMR3_GMUX IO signal is connected to TMR3 channel 1"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_CH1_CMP_GMUX_A::Mux3)
    }
}
impl R {
    #[doc = "Bits 0:1 - TMR1 BK1 CMP muxing"]
    #[inline(always)]
    pub fn tmr1_bk1_cmp_gmux(&self) -> TMR1_BK1_CMP_GMUX_R {
        TMR1_BK1_CMP_GMUX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TMR1 CH1 CMP muxing"]
    #[inline(always)]
    pub fn tmr1_ch1_cmp_gmux(&self) -> TMR1_CH1_CMP_GMUX_R {
        TMR1_CH1_CMP_GMUX_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TMR2 CH4 CMP muxing"]
    #[inline(always)]
    pub fn tmr2_ch4_cmp_gmux(&self) -> TMR2_CH4_CMP_GMUX_R {
        TMR2_CH4_CMP_GMUX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR3 CH1 CMP muxing"]
    #[inline(always)]
    pub fn tmr3_ch1_cmp_gmux(&self) -> TMR3_CH1_CMP_GMUX_R {
        TMR3_CH1_CMP_GMUX_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP8")
            .field(
                "tmr3_ch1_cmp_gmux",
                &format_args!("{}", self.tmr3_ch1_cmp_gmux().bits()),
            )
            .field(
                "tmr2_ch4_cmp_gmux",
                &format_args!("{}", self.tmr2_ch4_cmp_gmux().bits()),
            )
            .field(
                "tmr1_ch1_cmp_gmux",
                &format_args!("{}", self.tmr1_ch1_cmp_gmux().bits()),
            )
            .field(
                "tmr1_bk1_cmp_gmux",
                &format_args!("{}", self.tmr1_bk1_cmp_gmux().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - TMR1 BK1 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_bk1_cmp_gmux(&mut self) -> TMR1_BK1_CMP_GMUX_W<REMAP8_SPEC, 0> {
        TMR1_BK1_CMP_GMUX_W::new(self)
    }
    #[doc = "Bits 2:3 - TMR1 CH1 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_ch1_cmp_gmux(&mut self) -> TMR1_CH1_CMP_GMUX_W<REMAP8_SPEC, 2> {
        TMR1_CH1_CMP_GMUX_W::new(self)
    }
    #[doc = "Bits 4:5 - TMR2 CH4 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_ch4_cmp_gmux(&mut self) -> TMR2_CH4_CMP_GMUX_W<REMAP8_SPEC, 4> {
        TMR2_CH4_CMP_GMUX_W::new(self)
    }
    #[doc = "Bits 6:7 - TMR3 CH1 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_ch1_cmp_gmux(&mut self) -> TMR3_CH1_CMP_GMUX_W<REMAP8_SPEC, 6> {
        TMR3_CH1_CMP_GMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IO MUX remap register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP8_SPEC;
impl crate::RegisterSpec for REMAP8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap8::R`](R) reader structure"]
impl crate::Readable for REMAP8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap8::W`](W) writer structure"]
impl crate::Writable for REMAP8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP8 to value 0"]
impl crate::Resettable for REMAP8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
