#[doc = "Register `REMAP4` reader"]
pub type R = crate::R<REMAP4_SPEC>;
#[doc = "Register `REMAP4` writer"]
pub type W = crate::W<REMAP4_SPEC>;
#[doc = "Field `TMR1_GMUX` reader - TMR1 muxing"]
pub type TMR1_GMUX_R = crate::FieldReader<TMR1_GMUX_A>;
#[doc = "TMR1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR1_GMUX_A {
    #[doc = "0: EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PB12, CH1C/PB13, CH2C/PB14, CH3C/PB15"]
    Mux0 = 0,
    #[doc = "1: EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    Mux1 = 1,
    #[doc = "2: EXT/PA0, CH1/PC6, CH2/PC7, CH3/PC8, CH4/PC9, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    Mux2 = 2,
}
impl From<TMR1_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR1_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR1_GMUX_A {
    type Ux = u8;
}
impl TMR1_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR1_GMUX_A> {
        match self.bits {
            0 => Some(TMR1_GMUX_A::Mux0),
            1 => Some(TMR1_GMUX_A::Mux1),
            2 => Some(TMR1_GMUX_A::Mux2),
            _ => None,
        }
    }
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PB12, CH1C/PB13, CH2C/PB14, CH3C/PB15"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR1_GMUX_A::Mux0
    }
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR1_GMUX_A::Mux1
    }
    #[doc = "EXT/PA0, CH1/PC6, CH2/PC7, CH3/PC8, CH4/PC9, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR1_GMUX_A::Mux2
    }
}
#[doc = "Field `TMR1_GMUX` writer - TMR1 muxing"]
pub type TMR1_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, TMR1_GMUX_A>;
impl<'a, REG, const O: u8> TMR1_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PB12, CH1C/PB13, CH2C/PB14, CH3C/PB15"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_GMUX_A::Mux0)
    }
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_GMUX_A::Mux1)
    }
    #[doc = "EXT/PA0, CH1/PC6, CH2/PC7, CH3/PC8, CH4/PC9, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_GMUX_A::Mux2)
    }
}
#[doc = "Field `TMR2_GMUX` reader - TMR2 muxing"]
pub type TMR2_GMUX_R = crate::FieldReader<TMR2_GMUX_A>;
#[doc = "TMR2 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR2_GMUX_A {
    #[doc = "0: CH1_EXT/PA0 CH2/PA1 CH3/PA2 CH4/PA3"]
    Mux0 = 0,
    #[doc = "1: CH1_EXT/PA15 CH2/PB3 CH3/PA2 CH4/PA3"]
    Mux1 = 1,
    #[doc = "2: CH1_EXT/PA0 CH2/PA1 CH3/PB10 CH4/PB11"]
    Mux2 = 2,
    #[doc = "3: CH1_EXT/PA15 CH2/PB3 CH3/PB10 CH4/PB11"]
    Mux3 = 3,
}
impl From<TMR2_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR2_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR2_GMUX_A {
    type Ux = u8;
}
impl TMR2_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR2_GMUX_A> {
        match self.bits {
            0 => Some(TMR2_GMUX_A::Mux0),
            1 => Some(TMR2_GMUX_A::Mux1),
            2 => Some(TMR2_GMUX_A::Mux2),
            3 => Some(TMR2_GMUX_A::Mux3),
            _ => None,
        }
    }
    #[doc = "CH1_EXT/PA0 CH2/PA1 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR2_GMUX_A::Mux0
    }
    #[doc = "CH1_EXT/PA15 CH2/PB3 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR2_GMUX_A::Mux1
    }
    #[doc = "CH1_EXT/PA0 CH2/PA1 CH3/PB10 CH4/PB11"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR2_GMUX_A::Mux2
    }
    #[doc = "CH1_EXT/PA15 CH2/PB3 CH3/PB10 CH4/PB11"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == TMR2_GMUX_A::Mux3
    }
}
#[doc = "Field `TMR2_GMUX` writer - TMR2 muxing"]
pub type TMR2_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TMR2_GMUX_A>;
impl<'a, REG, const O: u8> TMR2_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1_EXT/PA0 CH2/PA1 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_GMUX_A::Mux0)
    }
    #[doc = "CH1_EXT/PA15 CH2/PB3 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_GMUX_A::Mux1)
    }
    #[doc = "CH1_EXT/PA0 CH2/PA1 CH3/PB10 CH4/PB11"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_GMUX_A::Mux2)
    }
    #[doc = "CH1_EXT/PA15 CH2/PB3 CH3/PB10 CH4/PB11"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_GMUX_A::Mux3)
    }
}
#[doc = "Field `TMR3_GMUX` reader - TMR3 muxing"]
pub type TMR3_GMUX_R = crate::FieldReader<TMR3_GMUX_A>;
#[doc = "TMR3 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR3_GMUX_A {
    #[doc = "0: CH1/PA6 CH2/PA7 CH3/PB0 CH4/PB1"]
    Mux0 = 0,
    #[doc = "1: CH1/PB4 CH2/PB5 CH3/PB0 CH4/PB1"]
    Mux1 = 1,
}
impl From<TMR3_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR3_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR3_GMUX_A {
    type Ux = u8;
}
impl TMR3_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR3_GMUX_A> {
        match self.bits {
            0 => Some(TMR3_GMUX_A::Mux0),
            1 => Some(TMR3_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "CH1/PA6 CH2/PA7 CH3/PB0 CH4/PB1"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR3_GMUX_A::Mux0
    }
    #[doc = "CH1/PB4 CH2/PB5 CH3/PB0 CH4/PB1"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR3_GMUX_A::Mux1
    }
}
#[doc = "Field `TMR3_GMUX` writer - TMR3 muxing"]
pub type TMR3_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, TMR3_GMUX_A>;
impl<'a, REG, const O: u8> TMR3_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1/PA6 CH2/PA7 CH3/PB0 CH4/PB1"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_GMUX_A::Mux0)
    }
    #[doc = "CH1/PB4 CH2/PB5 CH3/PB0 CH4/PB1"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_GMUX_A::Mux1)
    }
}
#[doc = "Field `TMR5_GMUX` reader - TMR5 muxing"]
pub type TMR5_GMUX_R = crate::FieldReader<TMR5_GMUX_A>;
#[doc = "TMR5 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR5_GMUX_A {
    #[doc = "0: CH1/PA0 CH2/PA1 CH3/PA2 CH4/PA3"]
    Mux0 = 0,
    #[doc = "1: CH1/PF4 CH2/PF5 CH3/PA2 CH4/PA3"]
    Mux1 = 1,
}
impl From<TMR5_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR5_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR5_GMUX_A {
    type Ux = u8;
}
impl TMR5_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR5_GMUX_A> {
        match self.bits {
            0 => Some(TMR5_GMUX_A::Mux0),
            1 => Some(TMR5_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "CH1/PA0 CH2/PA1 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR5_GMUX_A::Mux0
    }
    #[doc = "CH1/PF4 CH2/PF5 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR5_GMUX_A::Mux1
    }
}
#[doc = "Field `TMR5_GMUX` writer - TMR5 muxing"]
pub type TMR5_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TMR5_GMUX_A>;
impl<'a, REG, const O: u8> TMR5_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1/PA0 CH2/PA1 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_GMUX_A::Mux0)
    }
    #[doc = "CH1/PF4 CH2/PF5 CH3/PA2 CH4/PA3"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_GMUX_A::Mux1)
    }
}
#[doc = "Field `TMR5CH4_GMUX` reader - TMR5CH4 muxing"]
pub type TMR5CH4_GMUX_R = crate::BitReader<TMR5CH4_GMUX_A>;
#[doc = "TMR5CH4 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR5CH4_GMUX_A {
    #[doc = "0: TMR5_CH4 is connected to PA3"]
    Pa3 = 0,
    #[doc = "1: LICK is connected to TMR5_CH4 to get calibration"]
    Lick = 1,
}
impl From<TMR5CH4_GMUX_A> for bool {
    #[inline(always)]
    fn from(variant: TMR5CH4_GMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR5CH4_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR5CH4_GMUX_A {
        match self.bits {
            false => TMR5CH4_GMUX_A::Pa3,
            true => TMR5CH4_GMUX_A::Lick,
        }
    }
    #[doc = "TMR5_CH4 is connected to PA3"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == TMR5CH4_GMUX_A::Pa3
    }
    #[doc = "LICK is connected to TMR5_CH4 to get calibration"]
    #[inline(always)]
    pub fn is_lick(&self) -> bool {
        *self == TMR5CH4_GMUX_A::Lick
    }
}
#[doc = "Field `TMR5CH4_GMUX` writer - TMR5CH4 muxing"]
pub type TMR5CH4_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMR5CH4_GMUX_A>;
impl<'a, REG, const O: u8> TMR5CH4_GMUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TMR5_CH4 is connected to PA3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5CH4_GMUX_A::Pa3)
    }
    #[doc = "LICK is connected to TMR5_CH4 to get calibration"]
    #[inline(always)]
    pub fn lick(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5CH4_GMUX_A::Lick)
    }
}
impl R {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_gmux(&self) -> TMR1_GMUX_R {
        TMR1_GMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_gmux(&self) -> TMR2_GMUX_R {
        TMR2_GMUX_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_gmux(&self) -> TMR3_GMUX_R {
        TMR3_GMUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - TMR5 muxing"]
    #[inline(always)]
    pub fn tmr5_gmux(&self) -> TMR5_GMUX_R {
        TMR5_GMUX_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - TMR5CH4 muxing"]
    #[inline(always)]
    pub fn tmr5ch4_gmux(&self) -> TMR5CH4_GMUX_R {
        TMR5CH4_GMUX_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP4")
            .field(
                "tmr5ch4_gmux",
                &format_args!("{}", self.tmr5ch4_gmux().bit()),
            )
            .field("tmr5_gmux", &format_args!("{}", self.tmr5_gmux().bits()))
            .field("tmr3_gmux", &format_args!("{}", self.tmr3_gmux().bits()))
            .field("tmr2_gmux", &format_args!("{}", self.tmr2_gmux().bits()))
            .field("tmr1_gmux", &format_args!("{}", self.tmr1_gmux().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_gmux(&mut self) -> TMR1_GMUX_W<REMAP4_SPEC, 0> {
        TMR1_GMUX_W::new(self)
    }
    #[doc = "Bits 4:6 - TMR2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_gmux(&mut self) -> TMR2_GMUX_W<REMAP4_SPEC, 4> {
        TMR2_GMUX_W::new(self)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_gmux(&mut self) -> TMR3_GMUX_W<REMAP4_SPEC, 8> {
        TMR3_GMUX_W::new(self)
    }
    #[doc = "Bits 16:18 - TMR5 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5_gmux(&mut self) -> TMR5_GMUX_W<REMAP4_SPEC, 16> {
        TMR5_GMUX_W::new(self)
    }
    #[doc = "Bit 19 - TMR5CH4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5ch4_gmux(&mut self) -> TMR5CH4_GMUX_W<REMAP4_SPEC, 19> {
        TMR5CH4_GMUX_W::new(self)
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
#[doc = "IO MUX remap register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP4_SPEC;
impl crate::RegisterSpec for REMAP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap4::R`](R) reader structure"]
impl crate::Readable for REMAP4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap4::W`](W) writer structure"]
impl crate::Writable for REMAP4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP4 to value 0"]
impl crate::Resettable for REMAP4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
