#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cenr {
    #[doc = "0: I2C peripheral is disabled"]
    Disabled = 0,
    #[doc = "1: I2C peripheral is enabled"]
    Enabled = 1,
}
impl From<I2cenr> for bool {
    #[inline(always)]
    fn from(variant: I2cenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` reader - Peripheral enable"]
pub type I2CEN_R = crate::BitReader<I2cenr>;
impl I2CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cenr {
        match self.bits {
            false => I2cenr::Disabled,
            true => I2cenr::Enabled,
        }
    }
    #[doc = "I2C peripheral is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2cenr::Disabled
    }
    #[doc = "I2C peripheral is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2cenr::Enabled
    }
}
#[doc = "Peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cenwWO {
    #[doc = "0: Disable I2C peripheral"]
    Disable = 0,
    #[doc = "1: Enable I2C peripheral"]
    Enable = 1,
}
impl From<I2cenwWO> for bool {
    #[inline(always)]
    fn from(variant: I2cenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CEN` writer - Peripheral enable"]
pub type I2CEN_W<'a, REG> = crate::BitWriter<'a, REG, I2cenwWO>;
impl<'a, REG> I2CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable I2C peripheral"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2cenwWO::Disable)
    }
    #[doc = "Enable I2C peripheral"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2cenwWO::Enable)
    }
}
#[doc = "I2C peripheral mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERMODE_A {
    #[doc = "0: I2C mode"]
    I2c = 0,
    #[doc = "1: SMBus mode"]
    Smbus = 1,
}
impl From<PERMODE_A> for bool {
    #[inline(always)]
    fn from(variant: PERMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERMODE` reader - I2C peripheral mode"]
pub type PERMODE_R = crate::BitReader<PERMODE_A>;
impl PERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PERMODE_A {
        match self.bits {
            false => PERMODE_A::I2c,
            true => PERMODE_A::Smbus,
        }
    }
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == PERMODE_A::I2c
    }
    #[doc = "SMBus mode"]
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == PERMODE_A::Smbus
    }
}
#[doc = "Field `PERMODE` writer - I2C peripheral mode"]
pub type PERMODE_W<'a, REG> = crate::BitWriter<'a, REG, PERMODE_A>;
impl<'a, REG> PERMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C mode"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(PERMODE_A::I2c)
    }
    #[doc = "SMBus mode"]
    #[inline(always)]
    pub fn smbus(self) -> &'a mut crate::W<REG> {
        self.variant(PERMODE_A::Smbus)
    }
}
#[doc = "SMBus device mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBMODE_A {
    #[doc = "0: SMBus slave"]
    Slave = 0,
    #[doc = "1: SMBus host"]
    Host = 1,
}
impl From<SMBMODE_A> for bool {
    #[inline(always)]
    fn from(variant: SMBMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBMODE` reader - SMBus device mode"]
pub type SMBMODE_R = crate::BitReader<SMBMODE_A>;
impl SMBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBMODE_A {
        match self.bits {
            false => SMBMODE_A::Slave,
            true => SMBMODE_A::Host,
        }
    }
    #[doc = "SMBus slave"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == SMBMODE_A::Slave
    }
    #[doc = "SMBus host"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == SMBMODE_A::Host
    }
}
#[doc = "Field `SMBMODE` writer - SMBus device mode"]
pub type SMBMODE_W<'a, REG> = crate::BitWriter<'a, REG, SMBMODE_A>;
impl<'a, REG> SMBMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(SMBMODE_A::Slave)
    }
    #[doc = "SMBus host"]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(SMBMODE_A::Host)
    }
}
#[doc = "SMBus address resolution protocol enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arpenr {
    #[doc = "0: SMBus address resolution protocol is disabled"]
    Disabled = 0,
    #[doc = "1: SMBus address resolution protocol is enabled"]
    Enabled = 1,
}
impl From<Arpenr> for bool {
    #[inline(always)]
    fn from(variant: Arpenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARPEN` reader - SMBus address resolution protocol enable"]
pub type ARPEN_R = crate::BitReader<Arpenr>;
impl ARPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arpenr {
        match self.bits {
            false => Arpenr::Disabled,
            true => Arpenr::Enabled,
        }
    }
    #[doc = "SMBus address resolution protocol is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Arpenr::Disabled
    }
    #[doc = "SMBus address resolution protocol is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Arpenr::Enabled
    }
}
#[doc = "SMBus address resolution protocol enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArpenwWO {
    #[doc = "0: Disable SMBus address resolution protocol"]
    Disable = 0,
    #[doc = "1: Enable SMBus address resolution protocol"]
    Enable = 1,
}
impl From<ArpenwWO> for bool {
    #[inline(always)]
    fn from(variant: ArpenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARPEN` writer - SMBus address resolution protocol enable"]
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG, ArpenwWO>;
impl<'a, REG> ARPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable SMBus address resolution protocol"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ArpenwWO::Disable)
    }
    #[doc = "Enable SMBus address resolution protocol"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ArpenwWO::Enable)
    }
}
#[doc = "PEC calculation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pecenr {
    #[doc = "0: PEC calculation is disabled"]
    Disabled = 0,
    #[doc = "1: PEC calculation is enabled"]
    Enabled = 1,
}
impl From<Pecenr> for bool {
    #[inline(always)]
    fn from(variant: Pecenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC calculation enable"]
pub type PECEN_R = crate::BitReader<Pecenr>;
impl PECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pecenr {
        match self.bits {
            false => Pecenr::Disabled,
            true => Pecenr::Enabled,
        }
    }
    #[doc = "PEC calculation is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pecenr::Disabled
    }
    #[doc = "PEC calculation is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pecenr::Enabled
    }
}
#[doc = "PEC calculation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PecenwWO {
    #[doc = "0: Disable PEC calculation"]
    Disable = 0,
    #[doc = "1: Enable PEC calculation"]
    Enable = 1,
}
impl From<PecenwWO> for bool {
    #[inline(always)]
    fn from(variant: PecenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` writer - PEC calculation enable"]
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG, PecenwWO>;
impl<'a, REG> PECEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PEC calculation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PecenwWO::Disable)
    }
    #[doc = "Enable PEC calculation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PecenwWO::Enable)
    }
}
#[doc = "General call address enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcaenr {
    #[doc = "0: General call address is disabled"]
    Disabled = 0,
    #[doc = "1: General call address is enabled"]
    Enabled = 1,
}
impl From<Gcaenr> for bool {
    #[inline(always)]
    fn from(variant: Gcaenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCAEN` reader - General call address enable"]
pub type GCAEN_R = crate::BitReader<Gcaenr>;
impl GCAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcaenr {
        match self.bits {
            false => Gcaenr::Disabled,
            true => Gcaenr::Enabled,
        }
    }
    #[doc = "General call address is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gcaenr::Disabled
    }
    #[doc = "General call address is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Gcaenr::Enabled
    }
}
#[doc = "General call address enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcaenwWO {
    #[doc = "0: Disable general call address"]
    Disable = 0,
    #[doc = "1: Enable general call address"]
    Enable = 1,
}
impl From<GcaenwWO> for bool {
    #[inline(always)]
    fn from(variant: GcaenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCAEN` writer - General call address enable"]
pub type GCAEN_W<'a, REG> = crate::BitWriter<'a, REG, GcaenwWO>;
impl<'a, REG> GCAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable general call address"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GcaenwWO::Disable)
    }
    #[doc = "Enable general call address"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GcaenwWO::Enable)
    }
}
#[doc = "Clock stretching mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stretchr {
    #[doc = "0: Clock stretching mode is disabled"]
    Disabled = 0,
    #[doc = "1: Clock stretching mode is enabled"]
    Enabled = 1,
}
impl From<Stretchr> for bool {
    #[inline(always)]
    fn from(variant: Stretchr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRETCH` reader - Clock stretching mode"]
pub type STRETCH_R = crate::BitReader<Stretchr>;
impl STRETCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stretchr {
        match self.bits {
            false => Stretchr::Disabled,
            true => Stretchr::Enabled,
        }
    }
    #[doc = "Clock stretching mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stretchr::Disabled
    }
    #[doc = "Clock stretching mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stretchr::Enabled
    }
}
#[doc = "Clock stretching mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StretchwWO {
    #[doc = "0: Disable clock stretching mode"]
    Disable = 0,
    #[doc = "1: Enable clock stretching mode"]
    Enable = 1,
}
impl From<StretchwWO> for bool {
    #[inline(always)]
    fn from(variant: StretchwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRETCH` writer - Clock stretching mode"]
pub type STRETCH_W<'a, REG> = crate::BitWriter<'a, REG, StretchwWO>;
impl<'a, REG> STRETCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock stretching mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(StretchwWO::Disable)
    }
    #[doc = "Enable clock stretching mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(StretchwWO::Enable)
    }
}
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENSTART_A {
    #[doc = "0: No Start generation"]
    NoStart = 0,
    #[doc = "1: In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    Start = 1,
}
impl From<GENSTART_A> for bool {
    #[inline(always)]
    fn from(variant: GENSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENSTART` reader - Start generation"]
pub type GENSTART_R = crate::BitReader<GENSTART_A>;
impl GENSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GENSTART_A {
        match self.bits {
            false => GENSTART_A::NoStart,
            true => GENSTART_A::Start,
        }
    }
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == GENSTART_A::NoStart
    }
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == GENSTART_A::Start
    }
}
#[doc = "Field `GENSTART` writer - Start generation"]
pub type GENSTART_W<'a, REG> = crate::BitWriter<'a, REG, GENSTART_A>;
impl<'a, REG> GENSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(GENSTART_A::NoStart)
    }
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(GENSTART_A::Start)
    }
}
#[doc = "Stop generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENSTOP_A {
    #[doc = "0: No Stop generation"]
    NoStop = 0,
    #[doc = "1: In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    Stop = 1,
}
impl From<GENSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: GENSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENSTOP` reader - Stop generation"]
pub type GENSTOP_R = crate::BitReader<GENSTOP_A>;
impl GENSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GENSTOP_A {
        match self.bits {
            false => GENSTOP_A::NoStop,
            true => GENSTOP_A::Stop,
        }
    }
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == GENSTOP_A::NoStop
    }
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == GENSTOP_A::Stop
    }
}
#[doc = "Field `GENSTOP` writer - Stop generation"]
pub type GENSTOP_W<'a, REG> = crate::BitWriter<'a, REG, GENSTOP_A>;
impl<'a, REG> GENSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut crate::W<REG> {
        self.variant(GENSTOP_A::NoStop)
    }
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(GENSTOP_A::Stop)
    }
}
#[doc = "Acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackenr {
    #[doc = "0: No acknowledge returned"]
    Disabled = 0,
    #[doc = "1: Acknowledge returned after a byte is received (matched address or data)"]
    Enabled = 1,
}
impl From<Ackenr> for bool {
    #[inline(always)]
    fn from(variant: Ackenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKEN` reader - Acknowledge enable"]
pub type ACKEN_R = crate::BitReader<Ackenr>;
impl ACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackenr {
        match self.bits {
            false => Ackenr::Disabled,
            true => Ackenr::Enabled,
        }
    }
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ackenr::Disabled
    }
    #[doc = "Acknowledge returned after a byte is received (matched address or data)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ackenr::Enabled
    }
}
#[doc = "Acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckenwWO {
    #[doc = "0: Disable acknowledge"]
    Disable = 0,
    #[doc = "1: Enable acknowledge"]
    Enable = 1,
}
impl From<AckenwWO> for bool {
    #[inline(always)]
    fn from(variant: AckenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKEN` writer - Acknowledge enable"]
pub type ACKEN_W<'a, REG> = crate::BitWriter<'a, REG, AckenwWO>;
impl<'a, REG> ACKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable acknowledge"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AckenwWO::Disable)
    }
    #[doc = "Enable acknowledge"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AckenwWO::Enable)
    }
}
#[doc = "Master receiving mode acknowledge control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MACKCTRL_A {
    #[doc = "0: ACKEN bit controls ACK of the current byte being transferred"]
    Current = 0,
    #[doc = "1: ACKEN bit controls ACK of the next byte to be transferred"]
    Next = 1,
}
impl From<MACKCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: MACKCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MACKCTRL` reader - Master receiving mode acknowledge control"]
pub type MACKCTRL_R = crate::BitReader<MACKCTRL_A>;
impl MACKCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MACKCTRL_A {
        match self.bits {
            false => MACKCTRL_A::Current,
            true => MACKCTRL_A::Next,
        }
    }
    #[doc = "ACKEN bit controls ACK of the current byte being transferred"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == MACKCTRL_A::Current
    }
    #[doc = "ACKEN bit controls ACK of the next byte to be transferred"]
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == MACKCTRL_A::Next
    }
}
#[doc = "Field `MACKCTRL` writer - Master receiving mode acknowledge control"]
pub type MACKCTRL_W<'a, REG> = crate::BitWriter<'a, REG, MACKCTRL_A>;
impl<'a, REG> MACKCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACKEN bit controls ACK of the current byte being transferred"]
    #[inline(always)]
    pub fn current(self) -> &'a mut crate::W<REG> {
        self.variant(MACKCTRL_A::Current)
    }
    #[doc = "ACKEN bit controls ACK of the next byte to be transferred"]
    #[inline(always)]
    pub fn next(self) -> &'a mut crate::W<REG> {
        self.variant(MACKCTRL_A::Next)
    }
}
#[doc = "Request PEC transmission enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pectenr {
    #[doc = "0: PEC transfer is disabled"]
    Disabled = 0,
    #[doc = "1: PEC transfer is enabled"]
    Enabled = 1,
}
impl From<Pectenr> for bool {
    #[inline(always)]
    fn from(variant: Pectenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTEN` reader - Request PEC transmission enable"]
pub type PECTEN_R = crate::BitReader<Pectenr>;
impl PECTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pectenr {
        match self.bits {
            false => Pectenr::Disabled,
            true => Pectenr::Enabled,
        }
    }
    #[doc = "PEC transfer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pectenr::Disabled
    }
    #[doc = "PEC transfer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pectenr::Enabled
    }
}
#[doc = "Request PEC transmission enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PectenwWO {
    #[doc = "0: Disable PEC transfer"]
    Disable = 0,
    #[doc = "1: Enable PEC transfer"]
    Enable = 1,
}
impl From<PectenwWO> for bool {
    #[inline(always)]
    fn from(variant: PectenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTEN` writer - Request PEC transmission enable"]
pub type PECTEN_W<'a, REG> = crate::BitWriter<'a, REG, PectenwWO>;
impl<'a, REG> PECTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PEC transfer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PectenwWO::Disable)
    }
    #[doc = "Enable PEC transfer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PectenwWO::Enable)
    }
}
#[doc = "SMBus alert pin set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBALERT_A {
    #[doc = "0: SMBus alert pin high"]
    High = 0,
    #[doc = "1: SMBus alert pin low"]
    Low = 1,
}
impl From<SMBALERT_A> for bool {
    #[inline(always)]
    fn from(variant: SMBALERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBALERT` reader - SMBus alert pin set"]
pub type SMBALERT_R = crate::BitReader<SMBALERT_A>;
impl SMBALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBALERT_A {
        match self.bits {
            false => SMBALERT_A::High,
            true => SMBALERT_A::Low,
        }
    }
    #[doc = "SMBus alert pin high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SMBALERT_A::High
    }
    #[doc = "SMBus alert pin low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SMBALERT_A::Low
    }
}
#[doc = "Field `SMBALERT` writer - SMBus alert pin set"]
pub type SMBALERT_W<'a, REG> = crate::BitWriter<'a, REG, SMBALERT_A>;
impl<'a, REG> SMBALERT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus alert pin high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SMBALERT_A::High)
    }
    #[doc = "SMBus alert pin low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SMBALERT_A::Low)
    }
}
#[doc = "I2C peripheral reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: I2C peripheral not under reset"]
    NotReset = 0,
    #[doc = "1: I2C peripheral under reset"]
    Reset = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - I2C peripheral reset"]
pub type RESET_R = crate::BitReader<RESET_A>;
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::NotReset,
            true => RESET_A::Reset,
        }
    }
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == RESET_A::NotReset
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESET_A::Reset
    }
}
#[doc = "Field `RESET` writer - I2C peripheral reset"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESET_A>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::NotReset)
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C peripheral mode"]
    #[inline(always)]
    pub fn permode(&self) -> PERMODE_R {
        PERMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus device mode"]
    #[inline(always)]
    pub fn smbmode(&self) -> SMBMODE_R {
        SMBMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus address resolution protocol enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GCAEN_R {
        GCAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn genstart(&self) -> GENSTART_R {
        GENSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn genstop(&self) -> GENSTOP_R {
        GENSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&self) -> ACKEN_R {
        ACKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master receiving mode acknowledge control"]
    #[inline(always)]
    pub fn mackctrl(&self) -> MACKCTRL_R {
        MACKCTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&self) -> PECTEN_R {
        PECTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert pin set"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C peripheral reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("reset", &self.reset())
            .field("smbalert", &self.smbalert())
            .field("pecten", &self.pecten())
            .field("mackctrl", &self.mackctrl())
            .field("acken", &self.acken())
            .field("genstop", &self.genstop())
            .field("genstart", &self.genstart())
            .field("stretch", &self.stretch())
            .field("gcaen", &self.gcaen())
            .field("pecen", &self.pecen())
            .field("arpen", &self.arpen())
            .field("smbmode", &self.smbmode())
            .field("permode", &self.permode())
            .field("i2cen", &self.i2cen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<CTRL1_SPEC> {
        I2CEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C peripheral mode"]
    #[inline(always)]
    #[must_use]
    pub fn permode(&mut self) -> PERMODE_W<CTRL1_SPEC> {
        PERMODE_W::new(self, 1)
    }
    #[doc = "Bit 3 - SMBus device mode"]
    #[inline(always)]
    #[must_use]
    pub fn smbmode(&mut self) -> SMBMODE_W<CTRL1_SPEC> {
        SMBMODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SMBus address resolution protocol enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<CTRL1_SPEC> {
        ARPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - PEC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CTRL1_SPEC> {
        PECEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - General call address enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcaen(&mut self) -> GCAEN_W<CTRL1_SPEC> {
        GCAEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clock stretching mode"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> STRETCH_W<CTRL1_SPEC> {
        STRETCH_W::new(self, 7)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn genstart(&mut self) -> GENSTART_W<CTRL1_SPEC> {
        GENSTART_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    #[must_use]
    pub fn genstop(&mut self) -> GENSTOP_W<CTRL1_SPEC> {
        GENSTOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn acken(&mut self) -> ACKEN_W<CTRL1_SPEC> {
        ACKEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Master receiving mode acknowledge control"]
    #[inline(always)]
    #[must_use]
    pub fn mackctrl(&mut self) -> MACKCTRL_W<CTRL1_SPEC> {
        MACKCTRL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Request PEC transmission enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecten(&mut self) -> PECTEN_W<CTRL1_SPEC> {
        PECTEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - SMBus alert pin set"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SMBALERT_W<CTRL1_SPEC> {
        SMBALERT_W::new(self, 13)
    }
    #[doc = "Bit 15 - I2C peripheral reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CTRL1_SPEC> {
        RESET_W::new(self, 15)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
