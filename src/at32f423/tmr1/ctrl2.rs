#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CBCTRL` reader - Channel buffer control"]
pub type CBCTRL_R = crate::BitReader<CBCTRLR_A>;
#[doc = "Channel buffer control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBCTRLR_A {
    #[doc = "0: CxEN, CxCEN and CxOCTRL bits are not buffered"]
    Disabled = 0,
    #[doc = "1: CxEN, CxCEN and CxOCTRL bits are buffered"]
    Enabled = 1,
}
impl From<CBCTRLR_A> for bool {
    #[inline(always)]
    fn from(variant: CBCTRLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CBCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CBCTRLR_A {
        match self.bits {
            false => CBCTRLR_A::Disabled,
            true => CBCTRLR_A::Enabled,
        }
    }
    #[doc = "CxEN, CxCEN and CxOCTRL bits are not buffered"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CBCTRLR_A::Disabled
    }
    #[doc = "CxEN, CxCEN and CxOCTRL bits are buffered"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CBCTRLR_A::Enabled
    }
}
#[doc = "Channel buffer control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBCTRLW_AW {
    #[doc = "0: CxEN, CxCEN and CxOCTRL bits buffer disable"]
    Disable = 0,
    #[doc = "1: CxEN, CxCEN and CxOCTRL bits buffer enable"]
    Enable = 1,
}
impl From<CBCTRLW_AW> for bool {
    #[inline(always)]
    fn from(variant: CBCTRLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBCTRL` writer - Channel buffer control"]
pub type CBCTRL_W<'a, REG> = crate::BitWriter<'a, REG, CBCTRLW_AW>;
impl<'a, REG> CBCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxEN, CxCEN and CxOCTRL bits buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CBCTRLW_AW::Disable)
    }
    #[doc = "CxEN, CxCEN and CxOCTRL bits buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CBCTRLW_AW::Enable)
    }
}
#[doc = "Field `CCFS` reader - Channel control bit flash select"]
pub type CCFS_R = crate::BitReader<CCFS_A>;
#[doc = "Channel control bit flash select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFS_A {
    #[doc = "0: Control bits are updated by setting the HALL bit"]
    Hall = 0,
    #[doc = "1: Control bits are updated by setting the HALL bit or a rising edge on TRGIN"]
    Trgin = 1,
}
impl From<CCFS_A> for bool {
    #[inline(always)]
    fn from(variant: CCFS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCFS_A {
        match self.bits {
            false => CCFS_A::Hall,
            true => CCFS_A::Trgin,
        }
    }
    #[doc = "Control bits are updated by setting the HALL bit"]
    #[inline(always)]
    pub fn is_hall(&self) -> bool {
        *self == CCFS_A::Hall
    }
    #[doc = "Control bits are updated by setting the HALL bit or a rising edge on TRGIN"]
    #[inline(always)]
    pub fn is_trgin(&self) -> bool {
        *self == CCFS_A::Trgin
    }
}
#[doc = "Field `CCFS` writer - Channel control bit flash select"]
pub type CCFS_W<'a, REG> = crate::BitWriter<'a, REG, CCFS_A>;
impl<'a, REG> CCFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control bits are updated by setting the HALL bit"]
    #[inline(always)]
    pub fn hall(self) -> &'a mut crate::W<REG> {
        self.variant(CCFS_A::Hall)
    }
    #[doc = "Control bits are updated by setting the HALL bit or a rising edge on TRGIN"]
    #[inline(always)]
    pub fn trgin(self) -> &'a mut crate::W<REG> {
        self.variant(CCFS_A::Trgin)
    }
}
#[doc = "Field `DRS` reader - DMA request source"]
pub type DRS_R = crate::BitReader<DRS_A>;
#[doc = "DMA request source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRS_A {
    #[doc = "0: Capture/compare event"]
    CaptureCompare = 0,
    #[doc = "1: Overflow event"]
    Overflow = 1,
}
impl From<DRS_A> for bool {
    #[inline(always)]
    fn from(variant: DRS_A) -> Self {
        variant as u8 != 0
    }
}
impl DRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRS_A {
        match self.bits {
            false => DRS_A::CaptureCompare,
            true => DRS_A::Overflow,
        }
    }
    #[doc = "Capture/compare event"]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == DRS_A::CaptureCompare
    }
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == DRS_A::Overflow
    }
}
#[doc = "Field `DRS` writer - DMA request source"]
pub type DRS_W<'a, REG> = crate::BitWriter<'a, REG, DRS_A>;
impl<'a, REG> DRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(DRS_A::CaptureCompare)
    }
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(DRS_A::Overflow)
    }
}
#[doc = "Field `PTOS` reader - Primary TMR output selection"]
pub type PTOS_R = crate::FieldReader<PTOS_A>;
#[doc = "Primary TMR output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTOS_A {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "2: Update"]
    Update = 2,
    #[doc = "3: Compare pulse"]
    ComparePulse = 3,
    #[doc = "4: C1ORAW signal"]
    C1oraw = 4,
    #[doc = "5: C2ORAW signal"]
    C2oraw = 5,
    #[doc = "6: C3ORAW signal"]
    C3oraw = 6,
    #[doc = "7: C4ORAW signal"]
    C4oraw = 7,
}
impl From<PTOS_A> for u8 {
    #[inline(always)]
    fn from(variant: PTOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTOS_A {
    type Ux = u8;
}
impl PTOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTOS_A {
        match self.bits {
            0 => PTOS_A::Reset,
            1 => PTOS_A::Enable,
            2 => PTOS_A::Update,
            3 => PTOS_A::ComparePulse,
            4 => PTOS_A::C1oraw,
            5 => PTOS_A::C2oraw,
            6 => PTOS_A::C3oraw,
            7 => PTOS_A::C4oraw,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PTOS_A::Reset
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PTOS_A::Enable
    }
    #[doc = "Update"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == PTOS_A::Update
    }
    #[doc = "Compare pulse"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == PTOS_A::ComparePulse
    }
    #[doc = "C1ORAW signal"]
    #[inline(always)]
    pub fn is_c1oraw(&self) -> bool {
        *self == PTOS_A::C1oraw
    }
    #[doc = "C2ORAW signal"]
    #[inline(always)]
    pub fn is_c2oraw(&self) -> bool {
        *self == PTOS_A::C2oraw
    }
    #[doc = "C3ORAW signal"]
    #[inline(always)]
    pub fn is_c3oraw(&self) -> bool {
        *self == PTOS_A::C3oraw
    }
    #[doc = "C4ORAW signal"]
    #[inline(always)]
    pub fn is_c4oraw(&self) -> bool {
        *self == PTOS_A::C4oraw
    }
}
#[doc = "Field `PTOS` writer - Primary TMR output selection"]
pub type PTOS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PTOS_A>;
impl<'a, REG> PTOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::Reset)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::Enable)
    }
    #[doc = "Update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::Update)
    }
    #[doc = "Compare pulse"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::ComparePulse)
    }
    #[doc = "C1ORAW signal"]
    #[inline(always)]
    pub fn c1oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C1oraw)
    }
    #[doc = "C2ORAW signal"]
    #[inline(always)]
    pub fn c2oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C2oraw)
    }
    #[doc = "C3ORAW signal"]
    #[inline(always)]
    pub fn c3oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C3oraw)
    }
    #[doc = "C4ORAW signal"]
    #[inline(always)]
    pub fn c4oraw(self) -> &'a mut crate::W<REG> {
        self.variant(PTOS_A::C4oraw)
    }
}
#[doc = "Field `C1INSEL` reader - C1IN selection"]
pub type C1INSEL_R = crate::BitReader<C1INSEL_A>;
#[doc = "C1IN selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1INSEL_A {
    #[doc = "0: CH1 pin is connected to C1IRAW input"]
    Ch1 = 0,
    #[doc = "1: The XOR result of CH1, CH2 and CH3 pins is connected to C1IRAW input"]
    Xor = 1,
}
impl From<C1INSEL_A> for bool {
    #[inline(always)]
    fn from(variant: C1INSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl C1INSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1INSEL_A {
        match self.bits {
            false => C1INSEL_A::Ch1,
            true => C1INSEL_A::Xor,
        }
    }
    #[doc = "CH1 pin is connected to C1IRAW input"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == C1INSEL_A::Ch1
    }
    #[doc = "The XOR result of CH1, CH2 and CH3 pins is connected to C1IRAW input"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == C1INSEL_A::Xor
    }
}
#[doc = "Field `C1INSEL` writer - C1IN selection"]
pub type C1INSEL_W<'a, REG> = crate::BitWriter<'a, REG, C1INSEL_A>;
impl<'a, REG> C1INSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH1 pin is connected to C1IRAW input"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(C1INSEL_A::Ch1)
    }
    #[doc = "The XOR result of CH1, CH2 and CH3 pins is connected to C1IRAW input"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut crate::W<REG> {
        self.variant(C1INSEL_A::Xor)
    }
}
#[doc = "Field `CIOS[1-4]` reader - Channel %s idle output state"]
pub type CIOS_R = crate::BitReader<C1IOS_A>;
#[doc = "Channel %s idle output state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1IOS_A {
    #[doc = "0: After dead-time state is low"]
    Low = 0,
    #[doc = "1: After dead-time state is high"]
    High = 1,
}
impl From<C1IOS_A> for bool {
    #[inline(always)]
    fn from(variant: C1IOS_A) -> Self {
        variant as u8 != 0
    }
}
impl CIOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1IOS_A {
        match self.bits {
            false => C1IOS_A::Low,
            true => C1IOS_A::High,
        }
    }
    #[doc = "After dead-time state is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == C1IOS_A::Low
    }
    #[doc = "After dead-time state is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == C1IOS_A::High
    }
}
#[doc = "Field `CIOS[1-4]` writer - Channel %s idle output state"]
pub type CIOS_W<'a, REG> = crate::BitWriter<'a, REG, C1IOS_A>;
impl<'a, REG> CIOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "After dead-time state is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(C1IOS_A::Low)
    }
    #[doc = "After dead-time state is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(C1IOS_A::High)
    }
}
#[doc = "Field `CCIOS[1-3]` reader - Channel %s complementary idle output state"]
pub type CCIOS_R = crate::BitReader<C1CIOS_A>;
#[doc = "Channel %s complementary idle output state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1CIOS_A {
    #[doc = "0: After dead-time state is low"]
    Low = 0,
    #[doc = "1: After dead-time state is high"]
    High = 1,
}
impl From<C1CIOS_A> for bool {
    #[inline(always)]
    fn from(variant: C1CIOS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1CIOS_A {
        match self.bits {
            false => C1CIOS_A::Low,
            true => C1CIOS_A::High,
        }
    }
    #[doc = "After dead-time state is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == C1CIOS_A::Low
    }
    #[doc = "After dead-time state is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == C1CIOS_A::High
    }
}
#[doc = "Field `CCIOS[1-3]` writer - Channel %s complementary idle output state"]
pub type CCIOS_W<'a, REG> = crate::BitWriter<'a, REG, C1CIOS_A>;
impl<'a, REG> CCIOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "After dead-time state is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(C1CIOS_A::Low)
    }
    #[doc = "After dead-time state is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(C1CIOS_A::High)
    }
}
#[doc = "Field `TRGOUT2EN` reader - TRGOUT2 enable"]
pub type TRGOUT2EN_R = crate::BitReader;
#[doc = "Field `TRGOUT2EN` writer - TRGOUT2 enable"]
pub type TRGOUT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    pub fn cbctrl(&self) -> CBCTRL_R {
        CBCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel control bit flash select"]
    #[inline(always)]
    pub fn ccfs(&self) -> CCFS_R {
        CCFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&self) -> DRS_R {
        DRS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    pub fn ptos(&self) -> PTOS_R {
        PTOS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    pub fn c1insel(&self) -> C1INSEL_R {
        C1INSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Channel [1-4]
idle output state\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cios(&self, n: u8) -> CIOS_R {
        assert!(n < 4);
        CIOS_R::new(((self.bits >> (n * 2 + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    pub fn c1ios(&self) -> CIOS_R {
        CIOS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    pub fn c2ios(&self) -> CIOS_R {
        CIOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 idle output state"]
    #[inline(always)]
    pub fn c3ios(&self) -> CIOS_R {
        CIOS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 idle output state"]
    #[inline(always)]
    pub fn c4ios(&self) -> CIOS_R {
        CIOS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Channel [1-3]
complementary idle output state\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn ccios(&self, n: u8) -> CCIOS_R {
        assert!(n < 3);
        CCIOS_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    pub fn c1cios(&self) -> CCIOS_R {
        CCIOS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    pub fn c2cios(&self) -> CCIOS_R {
        CCIOS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 complementary idle output state"]
    #[inline(always)]
    pub fn c3cios(&self) -> CCIOS_R {
        CCIOS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - TRGOUT2 enable"]
    #[inline(always)]
    pub fn trgout2en(&self) -> TRGOUT2EN_R {
        TRGOUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("trgout2en", &format_args!("{}", self.trgout2en().bit()))
            .field("c1ios", &format_args!("{}", self.c1ios().bit()))
            .field("c2ios", &format_args!("{}", self.c2ios().bit()))
            .field("c3ios", &format_args!("{}", self.c3ios().bit()))
            .field("c4ios", &format_args!("{}", self.c4ios().bit()))
            .field("c1cios", &format_args!("{}", self.c1cios().bit()))
            .field("c2cios", &format_args!("{}", self.c2cios().bit()))
            .field("c3cios", &format_args!("{}", self.c3cios().bit()))
            .field("c1insel", &format_args!("{}", self.c1insel().bit()))
            .field("ptos", &format_args!("{}", self.ptos().bits()))
            .field("drs", &format_args!("{}", self.drs().bit()))
            .field("ccfs", &format_args!("{}", self.ccfs().bit()))
            .field("cbctrl", &format_args!("{}", self.cbctrl().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    #[must_use]
    pub fn cbctrl(&mut self) -> CBCTRL_W<CTRL2_SPEC> {
        CBCTRL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Channel control bit flash select"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CCFS_W<CTRL2_SPEC> {
        CCFS_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    #[must_use]
    pub fn drs(&mut self) -> DRS_W<CTRL2_SPEC> {
        DRS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptos(&mut self) -> PTOS_W<CTRL2_SPEC> {
        PTOS_W::new(self, 4)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1insel(&mut self) -> C1INSEL_W<CTRL2_SPEC> {
        C1INSEL_W::new(self, 7)
    }
    #[doc = "Channel [1-4]
idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn cios(&mut self, n: u8) -> CIOS_W<CTRL2_SPEC> {
        assert!(n < 4);
        CIOS_W::new(self, n * 2 + 8)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1ios(&mut self) -> CIOS_W<CTRL2_SPEC> {
        CIOS_W::new(self, 8)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2ios(&mut self) -> CIOS_W<CTRL2_SPEC> {
        CIOS_W::new(self, 10)
    }
    #[doc = "Bit 12 - Channel 3 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c3ios(&mut self) -> CIOS_W<CTRL2_SPEC> {
        CIOS_W::new(self, 12)
    }
    #[doc = "Bit 14 - Channel 4 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c4ios(&mut self) -> CIOS_W<CTRL2_SPEC> {
        CIOS_W::new(self, 14)
    }
    #[doc = "Channel [1-3]
complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn ccios(&mut self, n: u8) -> CCIOS_W<CTRL2_SPEC> {
        assert!(n < 3);
        CCIOS_W::new(self, n * 2 + 9)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1cios(&mut self) -> CCIOS_W<CTRL2_SPEC> {
        CCIOS_W::new(self, 9)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2cios(&mut self) -> CCIOS_W<CTRL2_SPEC> {
        CCIOS_W::new(self, 11)
    }
    #[doc = "Bit 13 - Channel 3 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c3cios(&mut self) -> CCIOS_W<CTRL2_SPEC> {
        CCIOS_W::new(self, 13)
    }
    #[doc = "Bit 31 - TRGOUT2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgout2en(&mut self) -> TRGOUT2EN_W<CTRL2_SPEC> {
        TRGOUT2EN_W::new(self, 31)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
