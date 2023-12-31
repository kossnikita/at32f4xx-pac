#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `ID` reader - USART identification"]
pub type ID_R = crate::FieldReader;
#[doc = "Field `ID` writer - USART identification"]
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BFBN` reader - Break frame bit num"]
pub type BFBN_R = crate::BitReader<BFBN_A>;
#[doc = "Break frame bit num\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFBN_A {
    #[doc = "0: 10-bit break frame"]
    Bit10 = 0,
    #[doc = "1: 11-bit break frame"]
    Bit11 = 1,
}
impl From<BFBN_A> for bool {
    #[inline(always)]
    fn from(variant: BFBN_A) -> Self {
        variant as u8 != 0
    }
}
impl BFBN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFBN_A {
        match self.bits {
            false => BFBN_A::Bit10,
            true => BFBN_A::Bit11,
        }
    }
    #[doc = "10-bit break frame"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == BFBN_A::Bit10
    }
    #[doc = "11-bit break frame"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == BFBN_A::Bit11
    }
}
#[doc = "Field `BFBN` writer - Break frame bit num"]
pub type BFBN_W<'a, REG> = crate::BitWriter<'a, REG, BFBN_A>;
impl<'a, REG> BFBN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break frame"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(BFBN_A::Bit10)
    }
    #[doc = "11-bit break frame"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(BFBN_A::Bit11)
    }
}
#[doc = "Field `BFIEN` reader - Break frame interrupt enable"]
pub type BFIEN_R = crate::BitReader<BFIENR_A>;
#[doc = "Break frame interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFIENR_A {
    #[doc = "0: Break frame interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Break frame interrupt is enabled"]
    Enabled = 1,
}
impl From<BFIENR_A> for bool {
    #[inline(always)]
    fn from(variant: BFIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl BFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BFIENR_A {
        match self.bits {
            false => BFIENR_A::Disabled,
            true => BFIENR_A::Enabled,
        }
    }
    #[doc = "Break frame interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFIENR_A::Disabled
    }
    #[doc = "Break frame interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFIENR_A::Enabled
    }
}
#[doc = "Break frame interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFIENW_AW {
    #[doc = "0: Break frame interrupt disable"]
    Disable = 0,
    #[doc = "1: Break frame interrupt enable"]
    Enable = 1,
}
impl From<BFIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: BFIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFIEN` writer - Break frame interrupt enable"]
pub type BFIEN_W<'a, REG> = crate::BitWriter<'a, REG, BFIENW_AW>;
impl<'a, REG> BFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break frame interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BFIENW_AW::Disable)
    }
    #[doc = "Break frame interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BFIENW_AW::Enable)
    }
}
#[doc = "Field `LBCP` reader - Last bit clock pulse"]
pub type LBCP_R = crate::BitReader<LBCPR_A>;
#[doc = "Last bit clock pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCPR_A {
    #[doc = "0: The clock pulse of the last data bit is no output on the clock pin"]
    Disabled = 0,
    #[doc = "1: The clock pulse of the last data bit is output on the clock pin"]
    Enabled = 1,
}
impl From<LBCPR_A> for bool {
    #[inline(always)]
    fn from(variant: LBCPR_A) -> Self {
        variant as u8 != 0
    }
}
impl LBCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBCPR_A {
        match self.bits {
            false => LBCPR_A::Disabled,
            true => LBCPR_A::Enabled,
        }
    }
    #[doc = "The clock pulse of the last data bit is no output on the clock pin"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBCPR_A::Disabled
    }
    #[doc = "The clock pulse of the last data bit is output on the clock pin"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBCPR_A::Enabled
    }
}
#[doc = "Last bit clock pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCPW_AW {
    #[doc = "0: The clock pulse of the last data bit is no output on the clock pin"]
    Disable = 0,
    #[doc = "1: The clock pulse of the last data bit is output on the clock pin"]
    Enable = 1,
}
impl From<LBCPW_AW> for bool {
    #[inline(always)]
    fn from(variant: LBCPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCP` writer - Last bit clock pulse"]
pub type LBCP_W<'a, REG> = crate::BitWriter<'a, REG, LBCPW_AW>;
impl<'a, REG> LBCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is no output on the clock pin"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LBCPW_AW::Disable)
    }
    #[doc = "The clock pulse of the last data bit is output on the clock pin"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LBCPW_AW::Enable)
    }
}
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type CLKPHA_R = crate::BitReader<CLKPHA_A>;
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPHA_A {
    #[doc = "0: Data capture is done on the first clock edge"]
    First = 0,
    #[doc = "1: Data capture is done on the second clock edge"]
    Second = 1,
}
impl From<CLKPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKPHA_A {
        match self.bits {
            false => CLKPHA_A::First,
            true => CLKPHA_A::Second,
        }
    }
    #[doc = "Data capture is done on the first clock edge"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CLKPHA_A::First
    }
    #[doc = "Data capture is done on the second clock edge"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CLKPHA_A::Second
    }
}
#[doc = "Field `CLKPHA` writer - Clock phase"]
pub type CLKPHA_W<'a, REG> = crate::BitWriter<'a, REG, CLKPHA_A>;
impl<'a, REG> CLKPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data capture is done on the first clock edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPHA_A::First)
    }
    #[doc = "Data capture is done on the second clock edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPHA_A::Second)
    }
}
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type CLKPOL_R = crate::BitReader<CLKPOL_A>;
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPOL_A {
    #[doc = "0: Clock output low"]
    Low = 0,
    #[doc = "1: Clock output high"]
    High = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::Low,
            true => CLKPOL_A::High,
        }
    }
    #[doc = "Clock output low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CLKPOL_A::Low
    }
    #[doc = "Clock output high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CLKPOL_A::High
    }
}
#[doc = "Field `CLKPOL` writer - Clock polarity"]
pub type CLKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CLKPOL_A>;
impl<'a, REG> CLKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::Low)
    }
    #[doc = "Clock output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::High)
    }
}
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader<CLKENR_A>;
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKENR_A {
    #[doc = "0: Clock is disabled"]
    Disabled = 0,
    #[doc = "1: Clock is enabled"]
    Enabled = 1,
}
impl From<CLKENR_A> for bool {
    #[inline(always)]
    fn from(variant: CLKENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKENR_A {
        match self.bits {
            false => CLKENR_A::Disabled,
            true => CLKENR_A::Enabled,
        }
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKENR_A::Disabled
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKENR_A::Enabled
    }
}
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKENW_AW {
    #[doc = "0: Clock disable"]
    Disable = 0,
    #[doc = "1: Clock enable"]
    Enable = 1,
}
impl From<CLKENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CLKENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKENW_AW>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CLKENW_AW::Disable)
    }
    #[doc = "Clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CLKENW_AW::Enable)
    }
}
#[doc = "Field `STOPBN` reader - STOP bit num"]
pub type STOPBN_R = crate::FieldReader<STOPBN_A>;
#[doc = "STOP bit num\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOPBN_A {
    #[doc = "0: 1 stop bit"]
    Bit1 = 0,
    #[doc = "1: 0.5 stop bit"]
    Bit05 = 1,
    #[doc = "2: 2 stop bits"]
    Bit2 = 2,
    #[doc = "3: 1.5 stop bit"]
    Bit15 = 3,
}
impl From<STOPBN_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPBN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOPBN_A {
    type Ux = u8;
}
impl STOPBN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPBN_A {
        match self.bits {
            0 => STOPBN_A::Bit1,
            1 => STOPBN_A::Bit05,
            2 => STOPBN_A::Bit2,
            3 => STOPBN_A::Bit15,
            _ => unreachable!(),
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == STOPBN_A::Bit1
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn is_bit05(&self) -> bool {
        *self == STOPBN_A::Bit05
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn is_bit2(&self) -> bool {
        *self == STOPBN_A::Bit2
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn is_bit15(&self) -> bool {
        *self == STOPBN_A::Bit15
    }
}
#[doc = "Field `STOPBN` writer - STOP bit num"]
pub type STOPBN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STOPBN_A>;
impl<'a, REG> STOPBN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn bit1(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBN_A::Bit1)
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn bit05(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBN_A::Bit05)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn bit2(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBN_A::Bit2)
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn bit15(self) -> &'a mut crate::W<REG> {
        self.variant(STOPBN_A::Bit15)
    }
}
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader<LINENR_A>;
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINENR_A {
    #[doc = "0: LIN mode is disabled"]
    Disabled = 0,
    #[doc = "1: LIN mode is enabled"]
    Enabled = 1,
}
impl From<LINENR_A> for bool {
    #[inline(always)]
    fn from(variant: LINENR_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINENR_A {
        match self.bits {
            false => LINENR_A::Disabled,
            true => LINENR_A::Enabled,
        }
    }
    #[doc = "LIN mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINENR_A::Disabled
    }
    #[doc = "LIN mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINENR_A::Enabled
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINENW_AW {
    #[doc = "0: LIN mode disable"]
    Disable = 0,
    #[doc = "1: LIN mode enable"]
    Enable = 1,
}
impl From<LINENW_AW> for bool {
    #[inline(always)]
    fn from(variant: LINENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG, LINENW_AW>;
impl<'a, REG> LINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LINENW_AW::Disable)
    }
    #[doc = "LIN mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LINENW_AW::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - USART identification"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    pub fn bfbn(&self) -> BFBN_R {
        BFBN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    pub fn bfien(&self) -> BFIEN_R {
        BFIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcp(&self) -> LBCP_R {
        LBCP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    pub fn stopbn(&self) -> STOPBN_R {
        STOPBN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("linen", &format_args!("{}", self.linen().bit()))
            .field("stopbn", &format_args!("{}", self.stopbn().bits()))
            .field("clken", &format_args!("{}", self.clken().bit()))
            .field("clkpol", &format_args!("{}", self.clkpol().bit()))
            .field("clkpha", &format_args!("{}", self.clkpha().bit()))
            .field("lbcp", &format_args!("{}", self.lbcp().bit()))
            .field("bfien", &format_args!("{}", self.bfien().bit()))
            .field("bfbn", &format_args!("{}", self.bfbn().bit()))
            .field("id", &format_args!("{}", self.id().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART identification"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<CTRL2_SPEC> {
        ID_W::new(self, 0)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn bfbn(&mut self) -> BFBN_W<CTRL2_SPEC> {
        BFBN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfien(&mut self) -> BFIEN_W<CTRL2_SPEC> {
        BFIEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcp(&mut self) -> LBCP_W<CTRL2_SPEC> {
        LBCP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRL2_SPEC> {
        CLKPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL2_SPEC> {
        CLKPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CTRL2_SPEC> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    #[must_use]
    pub fn stopbn(&mut self) -> STOPBN_W<CTRL2_SPEC> {
        STOPBN_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<CTRL2_SPEC> {
        LINEN_W::new(self, 14)
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
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
