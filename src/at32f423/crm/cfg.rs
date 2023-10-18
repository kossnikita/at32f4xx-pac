#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `SCLKSEL` reader - System clock select"]
pub type SCLKSEL_R = crate::FieldReader<SCLKSEL_A>;
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
pub type SCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SCLKSEL_A>;
impl<'a, REG, const O: u8> SCLKSEL_W<'a, REG, O>
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
#[doc = "Field `SCLKSTS` reader - System Clock select Status"]
pub type SCLKSTS_R = crate::FieldReader<SCLKSTS_A>;
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
#[doc = "Field `AHBDIV` reader - AHB division"]
pub type AHBDIV_R = crate::FieldReader<AHBDIV_A>;
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
pub type AHBDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, AHBDIV_A>;
impl<'a, REG, const O: u8> AHBDIV_W<'a, REG, O>
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
#[doc = "Field `APB1DIV` reader - APB1 division"]
pub type APB1DIV_R = crate::FieldReader<APB1DIV_A>;
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
pub type APB1DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, APB1DIV_A>;
impl<'a, REG, const O: u8> APB1DIV_W<'a, REG, O>
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
#[doc = "Field `ERTCDIV` reader - HEXT division for ERTC clock"]
pub type ERTCDIV_R = crate::FieldReader;
#[doc = "Field `ERTCDIV` writer - HEXT division for ERTC clock"]
pub type ERTCDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CLKOUTDIV1` reader - Clock output division1"]
pub type CLKOUTDIV1_R = crate::FieldReader;
#[doc = "Field `CLKOUTDIV1` writer - Clock output division1"]
pub type CLKOUTDIV1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CLKOUT_SEL1` reader - Clock output selection1"]
pub type CLKOUT_SEL1_R = crate::FieldReader;
#[doc = "Field `CLKOUT_SEL1` writer - Clock output selection1"]
pub type CLKOUT_SEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock"]
    #[inline(always)]
    pub fn ertcdiv(&self) -> ERTCDIV_R {
        ERTCDIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 27:29 - Clock output division1"]
    #[inline(always)]
    pub fn clkoutdiv1(&self) -> CLKOUTDIV1_R {
        CLKOUTDIV1_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Clock output selection1"]
    #[inline(always)]
    pub fn clkout_sel1(&self) -> CLKOUT_SEL1_R {
        CLKOUT_SEL1_R::new(((self.bits >> 30) & 3) as u8)
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
            .field("ertcdiv", &format_args!("{}", self.ertcdiv().bits()))
            .field("clkoutdiv1", &format_args!("{}", self.clkoutdiv1().bits()))
            .field(
                "clkout_sel1",
                &format_args!("{}", self.clkout_sel1().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    #[must_use]
    pub fn sclksel(&mut self) -> SCLKSEL_W<CFG_SPEC, 0> {
        SCLKSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    #[must_use]
    pub fn ahbdiv(&mut self) -> AHBDIV_W<CFG_SPEC, 4> {
        AHBDIV_W::new(self)
    }
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> APB1DIV_W<CFG_SPEC, 10> {
        APB1DIV_W::new(self)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> APB2DIV_W<CFG_SPEC, 13> {
        APB2DIV_W::new(self)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn ertcdiv(&mut self) -> ERTCDIV_W<CFG_SPEC, 16> {
        ERTCDIV_W::new(self)
    }
    #[doc = "Bits 27:29 - Clock output division1"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutdiv1(&mut self) -> CLKOUTDIV1_W<CFG_SPEC, 27> {
        CLKOUTDIV1_W::new(self)
    }
    #[doc = "Bits 30:31 - Clock output selection1"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_sel1(&mut self) -> CLKOUT_SEL1_W<CFG_SPEC, 30> {
        CLKOUT_SEL1_W::new(self)
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
#[doc = "Clock configuration register(CRM_CFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
