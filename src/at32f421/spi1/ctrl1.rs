#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type CLKPHA_R = crate::BitReader<CLKPHA_A>;
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPHA_A {
    #[doc = "0: Data capture starts from the first clock edge"]
    First = 0,
    #[doc = "1: Data capture starts from the second clock edge"]
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
    #[doc = "Data capture starts from the first clock edge"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CLKPHA_A::First
    }
    #[doc = "Data capture starts from the second clock edge"]
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
    #[doc = "Data capture starts from the first clock edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPHA_A::First)
    }
    #[doc = "Data capture starts from the second clock edge"]
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
    #[doc = "0: Low level"]
    Low = 0,
    #[doc = "1: High level"]
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
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CLKPOL_A::Low
    }
    #[doc = "High level"]
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
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::High)
    }
}
#[doc = "Field `MSTEN` reader - Master enable"]
pub type MSTEN_R = crate::BitReader<MSTEN_A>;
#[doc = "Master enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTEN_A {
    #[doc = "0: Master is disabled (Slave)\""]
    Slave = 0,
    #[doc = "1: Master is enabled\""]
    Master = 1,
}
impl From<MSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTEN_A {
        match self.bits {
            false => MSTEN_A::Slave,
            true => MSTEN_A::Master,
        }
    }
    #[doc = "Master is disabled (Slave)\""]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTEN_A::Slave
    }
    #[doc = "Master is enabled\""]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTEN_A::Master
    }
}
#[doc = "Field `MSTEN` writer - Master enable"]
pub type MSTEN_W<'a, REG> = crate::BitWriter<'a, REG, MSTEN_A>;
impl<'a, REG> MSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master is disabled (Slave)\""]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MSTEN_A::Slave)
    }
    #[doc = "Master is enabled\""]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MSTEN_A::Master)
    }
}
#[doc = "Field `MDIV2_0` reader - Master clock frequency division bit2-0"]
pub type MDIV2_0_R = crate::FieldReader;
#[doc = "Field `MDIV2_0` writer - Master clock frequency division bit2-0"]
pub type MDIV2_0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader<SPIENR_A>;
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIENR_A {
    #[doc = "0: SPI is disabled"]
    Disabled = 0,
    #[doc = "1: SPI is enabled"]
    Enabled = 1,
}
impl From<SPIENR_A> for bool {
    #[inline(always)]
    fn from(variant: SPIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPIENR_A {
        match self.bits {
            false => SPIENR_A::Disabled,
            true => SPIENR_A::Enabled,
        }
    }
    #[doc = "SPI is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPIENR_A::Disabled
    }
    #[doc = "SPI is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPIENR_A::Enabled
    }
}
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIENW_AW {
    #[doc = "0: SPI disable"]
    Disable = 0,
    #[doc = "1: SPI enable"]
    Enable = 1,
}
impl From<SPIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG, SPIENW_AW>;
impl<'a, REG> SPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SPIENW_AW::Disable)
    }
    #[doc = "SPI enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SPIENW_AW::Enable)
    }
}
#[doc = "Field `LTF` reader - LSB transmit first"]
pub type LTF_R = crate::BitReader<LTF_A>;
#[doc = "LSB transmit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTF_A {
    #[doc = "0: Most significant bit first"]
    Msb = 0,
    #[doc = "1: Least significant bit first"]
    Lsb = 1,
}
impl From<LTF_A> for bool {
    #[inline(always)]
    fn from(variant: LTF_A) -> Self {
        variant as u8 != 0
    }
}
impl LTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LTF_A {
        match self.bits {
            false => LTF_A::Msb,
            true => LTF_A::Lsb,
        }
    }
    #[doc = "Most significant bit first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == LTF_A::Msb
    }
    #[doc = "Least significant bit first"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == LTF_A::Lsb
    }
}
#[doc = "Field `LTF` writer - LSB transmit first"]
pub type LTF_W<'a, REG> = crate::BitWriter<'a, REG, LTF_A>;
impl<'a, REG> LTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Most significant bit first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(LTF_A::Msb)
    }
    #[doc = "Least significant bit first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(LTF_A::Lsb)
    }
}
#[doc = "Field `SWCSIL` reader - Software CS internal level"]
pub type SWCSIL_R = crate::BitReader<SWCSIL_A>;
#[doc = "Software CS internal level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWCSIL_A {
    #[doc = "0: Low level"]
    Low = 0,
    #[doc = "1: High level"]
    High = 1,
}
impl From<SWCSIL_A> for bool {
    #[inline(always)]
    fn from(variant: SWCSIL_A) -> Self {
        variant as u8 != 0
    }
}
impl SWCSIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWCSIL_A {
        match self.bits {
            false => SWCSIL_A::Low,
            true => SWCSIL_A::High,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SWCSIL_A::Low
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SWCSIL_A::High
    }
}
#[doc = "Field `SWCSIL` writer - Software CS internal level"]
pub type SWCSIL_W<'a, REG> = crate::BitWriter<'a, REG, SWCSIL_A>;
impl<'a, REG> SWCSIL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SWCSIL_A::Low)
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SWCSIL_A::High)
    }
}
#[doc = "Field `SWCSEN` reader - Software CS enable"]
pub type SWCSEN_R = crate::BitReader<SWCSENR_A>;
#[doc = "Software CS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWCSENR_A {
    #[doc = "0: Software CS is disabled"]
    Disabled = 0,
    #[doc = "1: Software CS is enabled"]
    Enabled = 1,
}
impl From<SWCSENR_A> for bool {
    #[inline(always)]
    fn from(variant: SWCSENR_A) -> Self {
        variant as u8 != 0
    }
}
impl SWCSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWCSENR_A {
        match self.bits {
            false => SWCSENR_A::Disabled,
            true => SWCSENR_A::Enabled,
        }
    }
    #[doc = "Software CS is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWCSENR_A::Disabled
    }
    #[doc = "Software CS is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWCSENR_A::Enabled
    }
}
#[doc = "Software CS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWCSENW_AW {
    #[doc = "0: Software CS disable"]
    Disable = 0,
    #[doc = "1: Software CS enable"]
    Enable = 1,
}
impl From<SWCSENW_AW> for bool {
    #[inline(always)]
    fn from(variant: SWCSENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWCSEN` writer - Software CS enable"]
pub type SWCSEN_W<'a, REG> = crate::BitWriter<'a, REG, SWCSENW_AW>;
impl<'a, REG> SWCSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software CS disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SWCSENW_AW::Disable)
    }
    #[doc = "Software CS enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SWCSENW_AW::Enable)
    }
}
#[doc = "Field `ORA` reader - Only receive active"]
pub type ORA_R = crate::BitReader<ORA_A>;
#[doc = "Only receive active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORA_A {
    #[doc = "0: Transmission and reception"]
    RxTx = 0,
    #[doc = "1: Receive-only mode"]
    ReceiveOnly = 1,
}
impl From<ORA_A> for bool {
    #[inline(always)]
    fn from(variant: ORA_A) -> Self {
        variant as u8 != 0
    }
}
impl ORA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ORA_A {
        match self.bits {
            false => ORA_A::RxTx,
            true => ORA_A::ReceiveOnly,
        }
    }
    #[doc = "Transmission and reception"]
    #[inline(always)]
    pub fn is_rx_tx(&self) -> bool {
        *self == ORA_A::RxTx
    }
    #[doc = "Receive-only mode"]
    #[inline(always)]
    pub fn is_receive_only(&self) -> bool {
        *self == ORA_A::ReceiveOnly
    }
}
#[doc = "Field `ORA` writer - Only receive active"]
pub type ORA_W<'a, REG> = crate::BitWriter<'a, REG, ORA_A>;
impl<'a, REG> ORA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission and reception"]
    #[inline(always)]
    pub fn rx_tx(self) -> &'a mut crate::W<REG> {
        self.variant(ORA_A::RxTx)
    }
    #[doc = "Receive-only mode"]
    #[inline(always)]
    pub fn receive_only(self) -> &'a mut crate::W<REG> {
        self.variant(ORA_A::ReceiveOnly)
    }
}
#[doc = "Field `FBN` reader - frame bit num"]
pub type FBN_R = crate::BitReader<FBN_A>;
#[doc = "frame bit num\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBN_A {
    #[doc = "0: 8-bit data frame"]
    Bit8 = 0,
    #[doc = "1: 16-bit data frame"]
    Bit16 = 1,
}
impl From<FBN_A> for bool {
    #[inline(always)]
    fn from(variant: FBN_A) -> Self {
        variant as u8 != 0
    }
}
impl FBN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FBN_A {
        match self.bits {
            false => FBN_A::Bit8,
            true => FBN_A::Bit16,
        }
    }
    #[doc = "8-bit data frame"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == FBN_A::Bit8
    }
    #[doc = "16-bit data frame"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == FBN_A::Bit16
    }
}
#[doc = "Field `FBN` writer - frame bit num"]
pub type FBN_W<'a, REG> = crate::BitWriter<'a, REG, FBN_A>;
impl<'a, REG> FBN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit data frame"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(FBN_A::Bit8)
    }
    #[doc = "16-bit data frame"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(FBN_A::Bit16)
    }
}
#[doc = "Field `NTC` reader - Next transmission CRC"]
pub type NTC_R = crate::BitReader<NTC_A>;
#[doc = "Next transmission CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NTC_A {
    #[doc = "0: Next transmitted data is the normal value"]
    Normal = 0,
    #[doc = "1: Next transmitted data is CRC value"]
    Crc = 1,
}
impl From<NTC_A> for bool {
    #[inline(always)]
    fn from(variant: NTC_A) -> Self {
        variant as u8 != 0
    }
}
impl NTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NTC_A {
        match self.bits {
            false => NTC_A::Normal,
            true => NTC_A::Crc,
        }
    }
    #[doc = "Next transmitted data is the normal value"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == NTC_A::Normal
    }
    #[doc = "Next transmitted data is CRC value"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == NTC_A::Crc
    }
}
#[doc = "Field `NTC` writer - Next transmission CRC"]
pub type NTC_W<'a, REG> = crate::BitWriter<'a, REG, NTC_A>;
impl<'a, REG> NTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next transmitted data is the normal value"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(NTC_A::Normal)
    }
    #[doc = "Next transmitted data is CRC value"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(NTC_A::Crc)
    }
}
#[doc = "Field `CCEN` reader - CRC calculation enable"]
pub type CCEN_R = crate::BitReader<CCENR_A>;
#[doc = "CRC calculation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCENR_A {
    #[doc = "0: RC calculation is disabled"]
    Disabled = 0,
    #[doc = "1: RC calculation is enabled"]
    Enabled = 1,
}
impl From<CCENR_A> for bool {
    #[inline(always)]
    fn from(variant: CCENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCENR_A {
        match self.bits {
            false => CCENR_A::Disabled,
            true => CCENR_A::Enabled,
        }
    }
    #[doc = "RC calculation is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCENR_A::Disabled
    }
    #[doc = "RC calculation is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCENR_A::Enabled
    }
}
#[doc = "CRC calculation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCENW_AW {
    #[doc = "0: RC calculation disable"]
    Disable = 0,
    #[doc = "1: RC calculation enable"]
    Enable = 1,
}
impl From<CCENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CCENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEN` writer - CRC calculation enable"]
pub type CCEN_W<'a, REG> = crate::BitWriter<'a, REG, CCENW_AW>;
impl<'a, REG> CCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RC calculation disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CCENW_AW::Disable)
    }
    #[doc = "RC calculation enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CCENW_AW::Enable)
    }
}
#[doc = "Field `SLBTD` reader - Single line bidirectional half-duplex transmission direction"]
pub type SLBTD_R = crate::BitReader<SLBTD_A>;
#[doc = "Single line bidirectional half-duplex transmission direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLBTD_A {
    #[doc = "0: Receive-only mode"]
    Receive = 0,
    #[doc = "1: Transmit-only mode"]
    Transmit = 1,
}
impl From<SLBTD_A> for bool {
    #[inline(always)]
    fn from(variant: SLBTD_A) -> Self {
        variant as u8 != 0
    }
}
impl SLBTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLBTD_A {
        match self.bits {
            false => SLBTD_A::Receive,
            true => SLBTD_A::Transmit,
        }
    }
    #[doc = "Receive-only mode"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == SLBTD_A::Receive
    }
    #[doc = "Transmit-only mode"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == SLBTD_A::Transmit
    }
}
#[doc = "Field `SLBTD` writer - Single line bidirectional half-duplex transmission direction"]
pub type SLBTD_W<'a, REG> = crate::BitWriter<'a, REG, SLBTD_A>;
impl<'a, REG> SLBTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive-only mode"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(SLBTD_A::Receive)
    }
    #[doc = "Transmit-only mode"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(SLBTD_A::Transmit)
    }
}
#[doc = "Field `SLBEN` reader - Single line bidirectional half-duplex enable"]
pub type SLBEN_R = crate::BitReader<SLBENR_A>;
#[doc = "Single line bidirectional half-duplex enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLBENR_A {
    #[doc = "0: Single line bidirectional half-duplex is disabled"]
    Disabled = 0,
    #[doc = "1: Single line bidirectional half-duplex is enabled"]
    Enabled = 1,
}
impl From<SLBENR_A> for bool {
    #[inline(always)]
    fn from(variant: SLBENR_A) -> Self {
        variant as u8 != 0
    }
}
impl SLBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLBENR_A {
        match self.bits {
            false => SLBENR_A::Disabled,
            true => SLBENR_A::Enabled,
        }
    }
    #[doc = "Single line bidirectional half-duplex is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLBENR_A::Disabled
    }
    #[doc = "Single line bidirectional half-duplex is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLBENR_A::Enabled
    }
}
#[doc = "Single line bidirectional half-duplex enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLBENW_AW {
    #[doc = "0: Single line bidirectional half-duplex disable"]
    Disable = 0,
    #[doc = "1: Single line bidirectional half-duplex enable"]
    Enable = 1,
}
impl From<SLBENW_AW> for bool {
    #[inline(always)]
    fn from(variant: SLBENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLBEN` writer - Single line bidirectional half-duplex enable"]
pub type SLBEN_W<'a, REG> = crate::BitWriter<'a, REG, SLBENW_AW>;
impl<'a, REG> SLBEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single line bidirectional half-duplex disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SLBENW_AW::Disable)
    }
    #[doc = "Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SLBENW_AW::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master enable"]
    #[inline(always)]
    pub fn msten(&self) -> MSTEN_R {
        MSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master clock frequency division bit2-0"]
    #[inline(always)]
    pub fn mdiv2_0(&self) -> MDIV2_0_R {
        MDIV2_0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB transmit first"]
    #[inline(always)]
    pub fn ltf(&self) -> LTF_R {
        LTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software CS internal level"]
    #[inline(always)]
    pub fn swcsil(&self) -> SWCSIL_R {
        SWCSIL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software CS enable"]
    #[inline(always)]
    pub fn swcsen(&self) -> SWCSEN_R {
        SWCSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Only receive active"]
    #[inline(always)]
    pub fn ora(&self) -> ORA_R {
        ORA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - frame bit num"]
    #[inline(always)]
    pub fn fbn(&self) -> FBN_R {
        FBN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Next transmission CRC"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC calculation enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CCEN_R {
        CCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Single line bidirectional half-duplex transmission direction"]
    #[inline(always)]
    pub fn slbtd(&self) -> SLBTD_R {
        SLBTD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    pub fn slben(&self) -> SLBEN_R {
        SLBEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("slben", &format_args!("{}", self.slben().bit()))
            .field("slbtd", &format_args!("{}", self.slbtd().bit()))
            .field("ccen", &format_args!("{}", self.ccen().bit()))
            .field("ntc", &format_args!("{}", self.ntc().bit()))
            .field("fbn", &format_args!("{}", self.fbn().bit()))
            .field("ora", &format_args!("{}", self.ora().bit()))
            .field("swcsen", &format_args!("{}", self.swcsen().bit()))
            .field("swcsil", &format_args!("{}", self.swcsil().bit()))
            .field("ltf", &format_args!("{}", self.ltf().bit()))
            .field("spien", &format_args!("{}", self.spien().bit()))
            .field("mdiv2_0", &format_args!("{}", self.mdiv2_0().bits()))
            .field("msten", &format_args!("{}", self.msten().bit()))
            .field("clkpol", &format_args!("{}", self.clkpol().bit()))
            .field("clkpha", &format_args!("{}", self.clkpha().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRL1_SPEC> {
        CLKPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL1_SPEC> {
        CLKPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master enable"]
    #[inline(always)]
    #[must_use]
    pub fn msten(&mut self) -> MSTEN_W<CTRL1_SPEC> {
        MSTEN_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Master clock frequency division bit2-0"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv2_0(&mut self) -> MDIV2_0_W<CTRL1_SPEC> {
        MDIV2_0_W::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<CTRL1_SPEC> {
        SPIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - LSB transmit first"]
    #[inline(always)]
    #[must_use]
    pub fn ltf(&mut self) -> LTF_W<CTRL1_SPEC> {
        LTF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software CS internal level"]
    #[inline(always)]
    #[must_use]
    pub fn swcsil(&mut self) -> SWCSIL_W<CTRL1_SPEC> {
        SWCSIL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software CS enable"]
    #[inline(always)]
    #[must_use]
    pub fn swcsen(&mut self) -> SWCSEN_W<CTRL1_SPEC> {
        SWCSEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Only receive active"]
    #[inline(always)]
    #[must_use]
    pub fn ora(&mut self) -> ORA_W<CTRL1_SPEC> {
        ORA_W::new(self, 10)
    }
    #[doc = "Bit 11 - frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn fbn(&mut self) -> FBN_W<CTRL1_SPEC> {
        FBN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Next transmission CRC"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<CTRL1_SPEC> {
        NTC_W::new(self, 12)
    }
    #[doc = "Bit 13 - CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccen(&mut self) -> CCEN_W<CTRL1_SPEC> {
        CCEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Single line bidirectional half-duplex transmission direction"]
    #[inline(always)]
    #[must_use]
    pub fn slbtd(&mut self) -> SLBTD_W<CTRL1_SPEC> {
        SLBTD_W::new(self, 14)
    }
    #[doc = "Bit 15 - Single line bidirectional half-duplex enable"]
    #[inline(always)]
    #[must_use]
    pub fn slben(&mut self) -> SLBEN_W<CTRL1_SPEC> {
        SLBEN_W::new(self, 15)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
