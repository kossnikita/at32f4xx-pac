#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "System clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKSEL_A {
    #[doc = "0: System clock from HICK"]
    Hick = 0,
    #[doc = "1: System clock from HEXT"]
    Hext = 1,
    #[doc = "2: System clock from PLL"]
    Pll = 2,
}
impl From<SCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCLKSEL_A {
    type Ux = u8;
}
#[doc = "Field `SCLKSEL` reader - System clock select"]
pub type SCLKSEL_R = crate::FieldReader<SCLKSEL_A>;
impl SCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SCLKSEL_A> {
        match self.bits {
            0 => Some(SCLKSEL_A::Hick),
            1 => Some(SCLKSEL_A::Hext),
            2 => Some(SCLKSEL_A::Pll),
            _ => None,
        }
    }
    #[doc = "System clock from HICK"]
    #[inline(always)]
    pub fn is_hick(&self) -> bool {
        *self == SCLKSEL_A::Hick
    }
    #[doc = "System clock from HEXT"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == SCLKSEL_A::Hext
    }
    #[doc = "System clock from PLL"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SCLKSEL_A::Pll
    }
}
#[doc = "Field `SCLKSEL` writer - System clock select"]
pub type SCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SCLKSEL_A>;
impl<'a, REG> SCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock from HICK"]
    #[inline(always)]
    pub fn hick(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::Hick)
    }
    #[doc = "System clock from HEXT"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::Hext)
    }
    #[doc = "System clock from PLL"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::Pll)
    }
}
#[doc = "System Clock select Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKSTS_A {
    #[doc = "0: System clock from HICK"]
    Hick = 0,
    #[doc = "1: System clock from HEXT"]
    Hext = 1,
    #[doc = "2: System clock from PLL"]
    Pll = 2,
}
impl From<SCLKSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKSTS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCLKSTS_A {
    type Ux = u8;
}
#[doc = "Field `SCLKSTS` reader - System Clock select Status"]
pub type SCLKSTS_R = crate::FieldReader<SCLKSTS_A>;
impl SCLKSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SCLKSTS_A> {
        match self.bits {
            0 => Some(SCLKSTS_A::Hick),
            1 => Some(SCLKSTS_A::Hext),
            2 => Some(SCLKSTS_A::Pll),
            _ => None,
        }
    }
    #[doc = "System clock from HICK"]
    #[inline(always)]
    pub fn is_hick(&self) -> bool {
        *self == SCLKSTS_A::Hick
    }
    #[doc = "System clock from HEXT"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == SCLKSTS_A::Hext
    }
    #[doc = "System clock from PLL"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SCLKSTS_A::Pll
    }
}
#[doc = "AHB division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHBDIV_A {
    #[doc = "0: The SCLK is used as AHB clock"]
    Sclk = 0,
    #[doc = "8: SCLK divided by 2"]
    Div2 = 8,
    #[doc = "9: SCLK divided by 4"]
    Div4 = 9,
    #[doc = "10: SCLK divided by 8"]
    Div8 = 10,
    #[doc = "11: SCLK divided by 16"]
    Div16 = 11,
    #[doc = "12: SCLK divided by 64"]
    Div64 = 12,
    #[doc = "13: SCLK divided by 128"]
    Div128 = 13,
    #[doc = "14: SCLK divided by 256"]
    Div256 = 14,
    #[doc = "15: SCLK divided by 512"]
    Div512 = 15,
}
impl From<AHBDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AHBDIV_A {
    type Ux = u8;
}
#[doc = "Field `AHBDIV` reader - AHB division"]
pub type AHBDIV_R = crate::FieldReader<AHBDIV_A>;
impl AHBDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AHBDIV_A> {
        match self.bits {
            0 => Some(AHBDIV_A::Sclk),
            8 => Some(AHBDIV_A::Div2),
            9 => Some(AHBDIV_A::Div4),
            10 => Some(AHBDIV_A::Div8),
            11 => Some(AHBDIV_A::Div16),
            12 => Some(AHBDIV_A::Div64),
            13 => Some(AHBDIV_A::Div128),
            14 => Some(AHBDIV_A::Div256),
            15 => Some(AHBDIV_A::Div512),
            _ => None,
        }
    }
    #[doc = "The SCLK is used as AHB clock"]
    #[inline(always)]
    pub fn is_sclk(&self) -> bool {
        *self == AHBDIV_A::Sclk
    }
    #[doc = "SCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AHBDIV_A::Div2
    }
    #[doc = "SCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AHBDIV_A::Div4
    }
    #[doc = "SCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AHBDIV_A::Div8
    }
    #[doc = "SCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == AHBDIV_A::Div16
    }
    #[doc = "SCLK divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == AHBDIV_A::Div64
    }
    #[doc = "SCLK divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == AHBDIV_A::Div128
    }
    #[doc = "SCLK divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == AHBDIV_A::Div256
    }
    #[doc = "SCLK divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == AHBDIV_A::Div512
    }
}
#[doc = "Field `AHBDIV` writer - AHB division"]
pub type AHBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AHBDIV_A>;
impl<'a, REG> AHBDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The SCLK is used as AHB clock"]
    #[inline(always)]
    pub fn sclk(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Sclk)
    }
    #[doc = "SCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div2)
    }
    #[doc = "SCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div4)
    }
    #[doc = "SCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div8)
    }
    #[doc = "SCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div16)
    }
    #[doc = "SCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div64)
    }
    #[doc = "SCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div128)
    }
    #[doc = "SCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div256)
    }
    #[doc = "SCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div512)
    }
}
#[doc = "APB1 division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APB1DIV_A {
    #[doc = "0: The divided HCLK is used as APB clock"]
    Hclk = 0,
    #[doc = "4: HCLK divided by 2"]
    Div2 = 4,
    #[doc = "5: HCLK divided by 4"]
    Div4 = 5,
    #[doc = "6: HCLK divided by 8"]
    Div8 = 6,
    #[doc = "7: HCLK divided by 16"]
    Div16 = 7,
}
impl From<APB1DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: APB1DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APB1DIV_A {
    type Ux = u8;
}
#[doc = "Field `APB1DIV` reader - APB1 division"]
pub type APB1DIV_R = crate::FieldReader<APB1DIV_A>;
impl APB1DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<APB1DIV_A> {
        match self.bits {
            0 => Some(APB1DIV_A::Hclk),
            4 => Some(APB1DIV_A::Div2),
            5 => Some(APB1DIV_A::Div4),
            6 => Some(APB1DIV_A::Div8),
            7 => Some(APB1DIV_A::Div16),
            _ => None,
        }
    }
    #[doc = "The divided HCLK is used as APB clock"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == APB1DIV_A::Hclk
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == APB1DIV_A::Div2
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == APB1DIV_A::Div4
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == APB1DIV_A::Div8
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == APB1DIV_A::Div16
    }
}
#[doc = "Field `APB1DIV` writer - APB1 division"]
pub type APB1DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, APB1DIV_A>;
impl<'a, REG> APB1DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The divided HCLK is used as APB clock"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Hclk)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div16)
    }
}
#[doc = "Field `APB2DIV` reader - APB2 division"]
pub use APB1DIV_R as APB2DIV_R;
#[doc = "Field `APB2DIV` writer - APB2 division"]
pub use APB1DIV_W as APB2DIV_W;
#[doc = "Field `ADCDIV1_0` reader - ADC division bit1 and bit0"]
pub type ADCDIV1_0_R = crate::FieldReader;
#[doc = "Field `ADCDIV1_0` writer - ADC division bit1 and bit0"]
pub type ADCDIV1_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLRCS` reader - PLL reference clock select"]
pub type PLLRCS_R = crate::BitReader;
#[doc = "Field `PLLRCS` writer - PLL reference clock select"]
pub type PLLRCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "HEXT division selection for PLL entry clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLHEXTDIV_A {
    #[doc = "0: HEXT"]
    Hext = 0,
    #[doc = "1: HEXT/2"]
    Div2 = 1,
}
impl From<PLLHEXTDIV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLHEXTDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLHEXTDIV` reader - HEXT division selection for PLL entry clock"]
pub type PLLHEXTDIV_R = crate::BitReader<PLLHEXTDIV_A>;
impl PLLHEXTDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLHEXTDIV_A {
        match self.bits {
            false => PLLHEXTDIV_A::Hext,
            true => PLLHEXTDIV_A::Div2,
        }
    }
    #[doc = "HEXT"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == PLLHEXTDIV_A::Hext
    }
    #[doc = "HEXT/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLHEXTDIV_A::Div2
    }
}
#[doc = "Field `PLLHEXTDIV` writer - HEXT division selection for PLL entry clock"]
pub type PLLHEXTDIV_W<'a, REG> = crate::BitWriter<'a, REG, PLLHEXTDIV_A>;
impl<'a, REG> PLLHEXTDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HEXT"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(PLLHEXTDIV_A::Hext)
    }
    #[doc = "HEXT/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLHEXTDIV_A::Div2)
    }
}
#[doc = "Field `PLLMULT3_0` reader - PLL Multiplication Factor bit3 to bit0"]
pub type PLLMULT3_0_R = crate::FieldReader;
#[doc = "Field `PLLMULT3_0` writer - PLL Multiplication Factor bit3 to bit0"]
pub type PLLMULT3_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKOUT_SEL` reader - Clock output selection bit2 to bit0"]
pub type CLKOUT_SEL_R = crate::FieldReader;
#[doc = "Field `CLKOUT_SEL` writer - Clock output selection bit2 to bit0"]
pub type CLKOUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADCDIV2` reader - ADC division bit2"]
pub type ADCDIV2_R = crate::BitReader;
#[doc = "Field `ADCDIV2` writer - ADC division bit2"]
pub type ADCDIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLMULT5_4` reader - PLL Multiplication Factor bit5 and bit4"]
pub type PLLMULT5_4_R = crate::FieldReader;
#[doc = "Field `PLLMULT5_4` writer - PLL Multiplication Factor bit5 and bit4"]
pub type PLLMULT5_4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    pub fn sclksel(&self) -> SCLKSEL_R {
        SCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock select Status"]
    #[inline(always)]
    pub fn sclksts(&self) -> SCLKSTS_R {
        SCLKSTS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    pub fn ahbdiv(&self) -> AHBDIV_R {
        AHBDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC division bit1 and bit0"]
    #[inline(always)]
    pub fn adcdiv1_0(&self) -> ADCDIV1_0_R {
        ADCDIV1_0_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL reference clock select"]
    #[inline(always)]
    pub fn pllrcs(&self) -> PLLRCS_R {
        PLLRCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HEXT division selection for PLL entry clock"]
    #[inline(always)]
    pub fn pllhextdiv(&self) -> PLLHEXTDIV_R {
        PLLHEXTDIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor bit3 to bit0"]
    #[inline(always)]
    pub fn pllmult3_0(&self) -> PLLMULT3_0_R {
        PLLMULT3_0_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Clock output selection bit2 to bit0"]
    #[inline(always)]
    pub fn clkout_sel(&self) -> CLKOUT_SEL_R {
        CLKOUT_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - ADC division bit2"]
    #[inline(always)]
    pub fn adcdiv2(&self) -> ADCDIV2_R {
        ADCDIV2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - PLL Multiplication Factor bit5 and bit4"]
    #[inline(always)]
    pub fn pllmult5_4(&self) -> PLLMULT5_4_R {
        PLLMULT5_4_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("sclksel", &format_args!("{}", self.sclksel().bits()))
            .field("sclksts", &format_args!("{}", self.sclksts().bits()))
            .field("ahbdiv", &format_args!("{}", self.ahbdiv().bits()))
            .field("apb1div", &format_args!("{}", self.apb1div().bits()))
            .field("apb2div", &format_args!("{}", self.apb2div().bits()))
            .field("adcdiv1_0", &format_args!("{}", self.adcdiv1_0().bits()))
            .field("pllrcs", &format_args!("{}", self.pllrcs().bit()))
            .field("pllhextdiv", &format_args!("{}", self.pllhextdiv().bit()))
            .field("pllmult3_0", &format_args!("{}", self.pllmult3_0().bits()))
            .field("clkout_sel", &format_args!("{}", self.clkout_sel().bits()))
            .field("adcdiv2", &format_args!("{}", self.adcdiv2().bit()))
            .field("pllmult5_4", &format_args!("{}", self.pllmult5_4().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    #[must_use]
    pub fn sclksel(&mut self) -> SCLKSEL_W<CFG_SPEC> {
        SCLKSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdiv(&mut self) -> AHBDIV_W<CFG_SPEC> {
        AHBDIV_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - APB1 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> APB1DIV_W<CFG_SPEC> {
        APB1DIV_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - APB2 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> APB2DIV_W<CFG_SPEC> {
        APB2DIV_W::new(self, 11)
    }
    #[doc = "Bits 14:15 - ADC division bit1 and bit0"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv1_0(&mut self) -> ADCDIV1_0_W<CFG_SPEC> {
        ADCDIV1_0_W::new(self, 14)
    }
    #[doc = "Bit 16 - PLL reference clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllrcs(&mut self) -> PLLRCS_W<CFG_SPEC> {
        PLLRCS_W::new(self, 16)
    }
    #[doc = "Bit 17 - HEXT division selection for PLL entry clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllhextdiv(&mut self) -> PLLHEXTDIV_W<CFG_SPEC> {
        PLLHEXTDIV_W::new(self, 17)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor bit3 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn pllmult3_0(&mut self) -> PLLMULT3_0_W<CFG_SPEC> {
        PLLMULT3_0_W::new(self, 18)
    }
    #[doc = "Bits 24:26 - Clock output selection bit2 to bit0"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_sel(&mut self) -> CLKOUT_SEL_W<CFG_SPEC> {
        CLKOUT_SEL_W::new(self, 24)
    }
    #[doc = "Bit 28 - ADC division bit2"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv2(&mut self) -> ADCDIV2_W<CFG_SPEC> {
        ADCDIV2_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - PLL Multiplication Factor bit5 and bit4"]
    #[inline(always)]
    #[must_use]
    pub fn pllmult5_4(&mut self) -> PLLMULT5_4_W<CFG_SPEC> {
        PLLMULT5_4_W::new(self, 29)
    }
}
#[doc = "Clock configuration register (CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
