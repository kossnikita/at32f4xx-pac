#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `IDL` reader - bit 3-0 for usart identification"]
pub type IDL_R = crate::FieldReader;
#[doc = "Field `IDL` writer - bit 3-0 for usart identification"]
pub type IDL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDBN` reader - Identification bit num"]
pub type IDBN_R = crate::BitReader;
#[doc = "Field `IDBN` writer - Identification bit num"]
pub type IDBN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `BFBN` reader - Break frame bit num"]
pub type BFBN_R = crate::BitReader<BFBN_A>;
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
#[doc = "Break frame interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfienr {
    #[doc = "0: Break frame interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Break frame interrupt is enabled"]
    Enabled = 1,
}
impl From<Bfienr> for bool {
    #[inline(always)]
    fn from(variant: Bfienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFIEN` reader - Break frame interrupt enable"]
pub type BFIEN_R = crate::BitReader<Bfienr>;
impl BFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfienr {
        match self.bits {
            false => Bfienr::Disabled,
            true => Bfienr::Enabled,
        }
    }
    #[doc = "Break frame interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bfienr::Disabled
    }
    #[doc = "Break frame interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bfienr::Enabled
    }
}
#[doc = "Break frame interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BfienwWO {
    #[doc = "0: Break frame interrupt disable"]
    Disable = 0,
    #[doc = "1: Break frame interrupt enable"]
    Enable = 1,
}
impl From<BfienwWO> for bool {
    #[inline(always)]
    fn from(variant: BfienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFIEN` writer - Break frame interrupt enable"]
pub type BFIEN_W<'a, REG> = crate::BitWriter<'a, REG, BfienwWO>;
impl<'a, REG> BFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break frame interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BfienwWO::Disable)
    }
    #[doc = "Break frame interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BfienwWO::Enable)
    }
}
#[doc = "Last bit clock pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbcpr {
    #[doc = "0: The clock pulse of the last data bit is no output on the clock pin"]
    Disabled = 0,
    #[doc = "1: The clock pulse of the last data bit is output on the clock pin"]
    Enabled = 1,
}
impl From<Lbcpr> for bool {
    #[inline(always)]
    fn from(variant: Lbcpr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCP` reader - Last bit clock pulse"]
pub type LBCP_R = crate::BitReader<Lbcpr>;
impl LBCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbcpr {
        match self.bits {
            false => Lbcpr::Disabled,
            true => Lbcpr::Enabled,
        }
    }
    #[doc = "The clock pulse of the last data bit is no output on the clock pin"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lbcpr::Disabled
    }
    #[doc = "The clock pulse of the last data bit is output on the clock pin"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lbcpr::Enabled
    }
}
#[doc = "Last bit clock pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LbcpwWO {
    #[doc = "0: The clock pulse of the last data bit is no output on the clock pin"]
    Disable = 0,
    #[doc = "1: The clock pulse of the last data bit is output on the clock pin"]
    Enable = 1,
}
impl From<LbcpwWO> for bool {
    #[inline(always)]
    fn from(variant: LbcpwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCP` writer - Last bit clock pulse"]
pub type LBCP_W<'a, REG> = crate::BitWriter<'a, REG, LbcpwWO>;
impl<'a, REG> LBCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is no output on the clock pin"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LbcpwWO::Disable)
    }
    #[doc = "The clock pulse of the last data bit is output on the clock pin"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LbcpwWO::Enable)
    }
}
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
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type CLKPHA_R = crate::BitReader<CLKPHA_A>;
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
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type CLKPOL_R = crate::BitReader<CLKPOL_A>;
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
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkenr {
    #[doc = "0: Clock is disabled"]
    Disabled = 0,
    #[doc = "1: Clock is enabled"]
    Enabled = 1,
}
impl From<Clkenr> for bool {
    #[inline(always)]
    fn from(variant: Clkenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader<Clkenr>;
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkenr {
        match self.bits {
            false => Clkenr::Disabled,
            true => Clkenr::Enabled,
        }
    }
    #[doc = "Clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkenr::Disabled
    }
    #[doc = "Clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Clkenr::Enabled
    }
}
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkenwWO {
    #[doc = "0: Clock disable"]
    Disable = 0,
    #[doc = "1: Clock enable"]
    Enable = 1,
}
impl From<ClkenwWO> for bool {
    #[inline(always)]
    fn from(variant: ClkenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, ClkenwWO>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkenwWO::Disable)
    }
    #[doc = "Clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkenwWO::Enable)
    }
}
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
impl crate::IsEnum for STOPBN_A {}
#[doc = "Field `STOPBN` reader - STOP bit num"]
pub type STOPBN_R = crate::FieldReader<STOPBN_A>;
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
pub type STOPBN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOPBN_A, crate::Safe>;
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
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linenr {
    #[doc = "0: LIN mode is disabled"]
    Disabled = 0,
    #[doc = "1: LIN mode is enabled"]
    Enabled = 1,
}
impl From<Linenr> for bool {
    #[inline(always)]
    fn from(variant: Linenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader<Linenr>;
impl LINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linenr {
        match self.bits {
            false => Linenr::Disabled,
            true => Linenr::Enabled,
        }
    }
    #[doc = "LIN mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Linenr::Disabled
    }
    #[doc = "LIN mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Linenr::Enabled
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinenwWO {
    #[doc = "0: LIN mode disable"]
    Disable = 0,
    #[doc = "1: LIN mode enable"]
    Enable = 1,
}
impl From<LinenwWO> for bool {
    #[inline(always)]
    fn from(variant: LinenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG, LinenwWO>;
impl<'a, REG> LINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LinenwWO::Disable)
    }
    #[doc = "LIN mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LinenwWO::Enable)
    }
}
#[doc = "Field `TRPSWAP` reader - Transmit receive pin swap"]
pub type TRPSWAP_R = crate::BitReader;
#[doc = "Field `TRPSWAP` writer - Transmit receive pin swap"]
pub type TRPSWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXREV` reader - RX polarity reverse"]
pub type RXREV_R = crate::BitReader;
#[doc = "Field `RXREV` writer - RX polarity reverse"]
pub type RXREV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXREV` reader - TX polarity reverse"]
pub type TXREV_R = crate::BitReader;
#[doc = "Field `TXREV` writer - TX polarity reverse"]
pub type TXREV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTREV` reader - DT register polarity reverse"]
pub type DTREV_R = crate::BitReader;
#[doc = "Field `DTREV` writer - DT register polarity reverse"]
pub type DTREV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTF` reader - MSB transmit first"]
pub type MTF_R = crate::BitReader;
#[doc = "Field `MTF` writer - MSB transmit first"]
pub type MTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDH` reader - bit 7-4 for usart identification"]
pub type IDH_R = crate::FieldReader;
#[doc = "Field `IDH` writer - bit 7-4 for usart identification"]
pub type IDH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - bit 3-0 for usart identification"]
    #[inline(always)]
    pub fn idl(&self) -> IDL_R {
        IDL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Identification bit num"]
    #[inline(always)]
    pub fn idbn(&self) -> IDBN_R {
        IDBN_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 15 - Transmit receive pin swap"]
    #[inline(always)]
    pub fn trpswap(&self) -> TRPSWAP_R {
        TRPSWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX polarity reverse"]
    #[inline(always)]
    pub fn rxrev(&self) -> RXREV_R {
        RXREV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX polarity reverse"]
    #[inline(always)]
    pub fn txrev(&self) -> TXREV_R {
        TXREV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DT register polarity reverse"]
    #[inline(always)]
    pub fn dtrev(&self) -> DTREV_R {
        DTREV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MSB transmit first"]
    #[inline(always)]
    pub fn mtf(&self) -> MTF_R {
        MTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 28:31 - bit 7-4 for usart identification"]
    #[inline(always)]
    pub fn idh(&self) -> IDH_R {
        IDH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("idh", &self.idh())
            .field("mtf", &self.mtf())
            .field("dtrev", &self.dtrev())
            .field("txrev", &self.txrev())
            .field("rxrev", &self.rxrev())
            .field("trpswap", &self.trpswap())
            .field("linen", &self.linen())
            .field("stopbn", &self.stopbn())
            .field("clken", &self.clken())
            .field("clkpol", &self.clkpol())
            .field("clkpha", &self.clkpha())
            .field("lbcp", &self.lbcp())
            .field("bfien", &self.bfien())
            .field("bfbn", &self.bfbn())
            .field("idbn", &self.idbn())
            .field("idl", &self.idl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - bit 3-0 for usart identification"]
    #[inline(always)]
    pub fn idl(&mut self) -> IDL_W<'_, CTRL2_SPEC> {
        IDL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Identification bit num"]
    #[inline(always)]
    pub fn idbn(&mut self) -> IDBN_W<'_, CTRL2_SPEC> {
        IDBN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    pub fn bfbn(&mut self) -> BFBN_W<'_, CTRL2_SPEC> {
        BFBN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    pub fn bfien(&mut self) -> BFIEN_W<'_, CTRL2_SPEC> {
        BFIEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcp(&mut self) -> LBCP_W<'_, CTRL2_SPEC> {
        LBCP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&mut self) -> CLKPHA_W<'_, CTRL2_SPEC> {
        CLKPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W<'_, CTRL2_SPEC> {
        CLKPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<'_, CTRL2_SPEC> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    pub fn stopbn(&mut self) -> STOPBN_W<'_, CTRL2_SPEC> {
        STOPBN_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<'_, CTRL2_SPEC> {
        LINEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit receive pin swap"]
    #[inline(always)]
    pub fn trpswap(&mut self) -> TRPSWAP_W<'_, CTRL2_SPEC> {
        TRPSWAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - RX polarity reverse"]
    #[inline(always)]
    pub fn rxrev(&mut self) -> RXREV_W<'_, CTRL2_SPEC> {
        RXREV_W::new(self, 16)
    }
    #[doc = "Bit 17 - TX polarity reverse"]
    #[inline(always)]
    pub fn txrev(&mut self) -> TXREV_W<'_, CTRL2_SPEC> {
        TXREV_W::new(self, 17)
    }
    #[doc = "Bit 18 - DT register polarity reverse"]
    #[inline(always)]
    pub fn dtrev(&mut self) -> DTREV_W<'_, CTRL2_SPEC> {
        DTREV_W::new(self, 18)
    }
    #[doc = "Bit 19 - MSB transmit first"]
    #[inline(always)]
    pub fn mtf(&mut self) -> MTF_W<'_, CTRL2_SPEC> {
        MTF_W::new(self, 19)
    }
    #[doc = "Bits 28:31 - bit 7-4 for usart identification"]
    #[inline(always)]
    pub fn idh(&mut self) -> IDH_W<'_, CTRL2_SPEC> {
        IDH_W::new(self, 28)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {}
