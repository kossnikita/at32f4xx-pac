#[doc = "Register `I2SCTRL` reader"]
pub type R = crate::R<I2SCTRL_SPEC>;
#[doc = "Register `I2SCTRL` writer"]
pub type W = crate::W<I2SCTRL_SPEC>;
#[doc = "I2S channel bit num\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBN_A {
    #[doc = "0: 16-bit wide"]
    Bit16 = 0,
    #[doc = "1: 32-bit wide"]
    Bit32 = 1,
}
impl From<CBN_A> for bool {
    #[inline(always)]
    fn from(variant: CBN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBN` reader - I2S channel bit num"]
pub type CBN_R = crate::BitReader<CBN_A>;
impl CBN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CBN_A {
        match self.bits {
            false => CBN_A::Bit16,
            true => CBN_A::Bit32,
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == CBN_A::Bit16
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == CBN_A::Bit32
    }
}
#[doc = "Field `CBN` writer - I2S channel bit num"]
pub type CBN_W<'a, REG> = crate::BitWriter<'a, REG, CBN_A>;
impl<'a, REG> CBN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(CBN_A::Bit16)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(CBN_A::Bit32)
    }
}
#[doc = "I2S data bit num\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBN_A {
    #[doc = "0: 16-bit wide"]
    Bit16 = 0,
    #[doc = "1: 24-bit wide"]
    Bit24 = 1,
    #[doc = "3: 32-bit wide"]
    Bit32 = 3,
}
impl From<DBN_A> for u8 {
    #[inline(always)]
    fn from(variant: DBN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBN_A {
    type Ux = u8;
}
impl crate::IsEnum for DBN_A {}
#[doc = "Field `DBN` reader - I2S data bit num"]
pub type DBN_R = crate::FieldReader<DBN_A>;
impl DBN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBN_A> {
        match self.bits {
            0 => Some(DBN_A::Bit16),
            1 => Some(DBN_A::Bit24),
            3 => Some(DBN_A::Bit32),
            _ => None,
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == DBN_A::Bit16
    }
    #[doc = "24-bit wide"]
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == DBN_A::Bit24
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == DBN_A::Bit32
    }
}
#[doc = "Field `DBN` writer - I2S data bit num"]
pub type DBN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DBN_A>;
impl<'a, REG> DBN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(DBN_A::Bit16)
    }
    #[doc = "24-bit wide"]
    #[inline(always)]
    pub fn bit24(self) -> &'a mut crate::W<REG> {
        self.variant(DBN_A::Bit24)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(DBN_A::Bit32)
    }
}
#[doc = "I2S clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPOL_A {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: High"]
    High = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPOL` reader - I2S clock polarity"]
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
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CLKPOL_A::Low
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CLKPOL_A::High
    }
}
#[doc = "Field `CLKPOL` writer - I2S clock polarity"]
pub type CLKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CLKPOL_A>;
impl<'a, REG> CLKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::Low)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::High)
    }
}
#[doc = "I2S standard select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STDSEL_A {
    #[doc = "0: Philips standard"]
    Philips = 0,
    #[doc = "1: MSB-aligned standard (left-aligned)"]
    Msb = 1,
    #[doc = "2: LSB-aligned standard (right-aligned)"]
    Lsb = 2,
    #[doc = "3: PCM standard"]
    Pcm = 3,
}
impl From<STDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STDSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STDSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for STDSEL_A {}
#[doc = "Field `STDSEL` reader - I2S standard select"]
pub type STDSEL_R = crate::FieldReader<STDSEL_A>;
impl STDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STDSEL_A {
        match self.bits {
            0 => STDSEL_A::Philips,
            1 => STDSEL_A::Msb,
            2 => STDSEL_A::Lsb,
            3 => STDSEL_A::Pcm,
            _ => unreachable!(),
        }
    }
    #[doc = "Philips standard"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == STDSEL_A::Philips
    }
    #[doc = "MSB-aligned standard (left-aligned)"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == STDSEL_A::Msb
    }
    #[doc = "LSB-aligned standard (right-aligned)"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == STDSEL_A::Lsb
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == STDSEL_A::Pcm
    }
}
#[doc = "Field `STDSEL` writer - I2S standard select"]
pub type STDSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STDSEL_A, crate::Safe>;
impl<'a, REG> STDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Philips standard"]
    #[inline(always)]
    pub fn philips(self) -> &'a mut crate::W<REG> {
        self.variant(STDSEL_A::Philips)
    }
    #[doc = "MSB-aligned standard (left-aligned)"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(STDSEL_A::Msb)
    }
    #[doc = "LSB-aligned standard (right-aligned)"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(STDSEL_A::Lsb)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut crate::W<REG> {
        self.variant(STDSEL_A::Pcm)
    }
}
#[doc = "PCM frame synchronization select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMFSSEL_A {
    #[doc = "0: Short frame synchronization"]
    Short = 0,
    #[doc = "1: Long frame synchronization"]
    Long = 1,
}
impl From<PCMFSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PCMFSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMFSSEL` reader - PCM frame synchronization select"]
pub type PCMFSSEL_R = crate::BitReader<PCMFSSEL_A>;
impl PCMFSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCMFSSEL_A {
        match self.bits {
            false => PCMFSSEL_A::Short,
            true => PCMFSSEL_A::Long,
        }
    }
    #[doc = "Short frame synchronization"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMFSSEL_A::Short
    }
    #[doc = "Long frame synchronization"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMFSSEL_A::Long
    }
}
#[doc = "Field `PCMFSSEL` writer - PCM frame synchronization select"]
pub type PCMFSSEL_W<'a, REG> = crate::BitWriter<'a, REG, PCMFSSEL_A>;
impl<'a, REG> PCMFSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short frame synchronization"]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(PCMFSSEL_A::Short)
    }
    #[doc = "Long frame synchronization"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(PCMFSSEL_A::Long)
    }
}
#[doc = "I2S operation select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPERSEL_A {
    #[doc = "0: Slave transmission"]
    SlaveTrasmission = 0,
    #[doc = "1: Slave reception"]
    SlaveReception = 1,
    #[doc = "2: Master transmission"]
    MasterTrasmission = 2,
    #[doc = "3: Master reception"]
    MasterReception = 3,
}
impl From<OPERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OPERSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPERSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for OPERSEL_A {}
#[doc = "Field `OPERSEL` reader - I2S operation select"]
pub type OPERSEL_R = crate::FieldReader<OPERSEL_A>;
impl OPERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPERSEL_A {
        match self.bits {
            0 => OPERSEL_A::SlaveTrasmission,
            1 => OPERSEL_A::SlaveReception,
            2 => OPERSEL_A::MasterTrasmission,
            3 => OPERSEL_A::MasterReception,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave transmission"]
    #[inline(always)]
    pub fn is_slave_trasmission(&self) -> bool {
        *self == OPERSEL_A::SlaveTrasmission
    }
    #[doc = "Slave reception"]
    #[inline(always)]
    pub fn is_slave_reception(&self) -> bool {
        *self == OPERSEL_A::SlaveReception
    }
    #[doc = "Master transmission"]
    #[inline(always)]
    pub fn is_master_trasmission(&self) -> bool {
        *self == OPERSEL_A::MasterTrasmission
    }
    #[doc = "Master reception"]
    #[inline(always)]
    pub fn is_master_reception(&self) -> bool {
        *self == OPERSEL_A::MasterReception
    }
}
#[doc = "Field `OPERSEL` writer - I2S operation select"]
pub type OPERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OPERSEL_A, crate::Safe>;
impl<'a, REG> OPERSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave transmission"]
    #[inline(always)]
    pub fn slave_trasmission(self) -> &'a mut crate::W<REG> {
        self.variant(OPERSEL_A::SlaveTrasmission)
    }
    #[doc = "Slave reception"]
    #[inline(always)]
    pub fn slave_reception(self) -> &'a mut crate::W<REG> {
        self.variant(OPERSEL_A::SlaveReception)
    }
    #[doc = "Master transmission"]
    #[inline(always)]
    pub fn master_trasmission(self) -> &'a mut crate::W<REG> {
        self.variant(OPERSEL_A::MasterTrasmission)
    }
    #[doc = "Master reception"]
    #[inline(always)]
    pub fn master_reception(self) -> &'a mut crate::W<REG> {
        self.variant(OPERSEL_A::MasterReception)
    }
}
#[doc = "I2S Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enr {
    #[doc = "0: I2S is disabled"]
    Disabled = 0,
    #[doc = "1: I2S is enabled"]
    Enabled = 1,
}
impl From<Enr> for bool {
    #[inline(always)]
    fn from(variant: Enr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - I2S Enable"]
pub type EN_R = crate::BitReader<Enr>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enr {
        match self.bits {
            false => Enr::Disabled,
            true => Enr::Enabled,
        }
    }
    #[doc = "I2S is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enr::Disabled
    }
    #[doc = "I2S is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enr::Enabled
    }
}
#[doc = "I2S Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnwWO {
    #[doc = "0: I2S disable"]
    Disable = 0,
    #[doc = "1: I2S enable"]
    Enable = 1,
}
impl From<EnwWO> for bool {
    #[inline(always)]
    fn from(variant: EnwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` writer - I2S Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EnwWO>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnwWO::Disable)
    }
    #[doc = "I2S enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnwWO::Enable)
    }
}
#[doc = "I2S mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSEL_A {
    #[doc = "0: SPI mode"]
    Spi = 0,
    #[doc = "1: I2S mode"]
    I2s = 1,
}
impl From<MSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSEL` reader - I2S mode select"]
pub type MSEL_R = crate::BitReader<MSEL_A>;
impl MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSEL_A {
        match self.bits {
            false => MSEL_A::Spi,
            true => MSEL_A::I2s,
        }
    }
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == MSEL_A::Spi
    }
    #[doc = "I2S mode"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == MSEL_A::I2s
    }
}
#[doc = "Field `MSEL` writer - I2S mode select"]
pub type MSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSEL_A>;
impl<'a, REG> MSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(MSEL_A::Spi)
    }
    #[doc = "I2S mode"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(MSEL_A::I2s)
    }
}
impl R {
    #[doc = "Bit 0 - I2S channel bit num"]
    #[inline(always)]
    pub fn cbn(&self) -> CBN_R {
        CBN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - I2S data bit num"]
    #[inline(always)]
    pub fn dbn(&self) -> DBN_R {
        DBN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - I2S clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard select"]
    #[inline(always)]
    pub fn stdsel(&self) -> STDSEL_R {
        STDSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization select"]
    #[inline(always)]
    pub fn pcmfssel(&self) -> PCMFSSEL_R {
        PCMFSSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S operation select"]
    #[inline(always)]
    pub fn opersel(&self) -> OPERSEL_R {
        OPERSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode select"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCTRL")
            .field("msel", &self.msel())
            .field("en", &self.en())
            .field("opersel", &self.opersel())
            .field("pcmfssel", &self.pcmfssel())
            .field("stdsel", &self.stdsel())
            .field("clkpol", &self.clkpol())
            .field("dbn", &self.dbn())
            .field("cbn", &self.cbn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I2S channel bit num"]
    #[inline(always)]
    pub fn cbn(&mut self) -> CBN_W<'_, I2SCTRL_SPEC> {
        CBN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - I2S data bit num"]
    #[inline(always)]
    pub fn dbn(&mut self) -> DBN_W<'_, I2SCTRL_SPEC> {
        DBN_W::new(self, 1)
    }
    #[doc = "Bit 3 - I2S clock polarity"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W<'_, I2SCTRL_SPEC> {
        CLKPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard select"]
    #[inline(always)]
    pub fn stdsel(&mut self) -> STDSEL_W<'_, I2SCTRL_SPEC> {
        STDSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization select"]
    #[inline(always)]
    pub fn pcmfssel(&mut self) -> PCMFSSEL_W<'_, I2SCTRL_SPEC> {
        PCMFSSEL_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S operation select"]
    #[inline(always)]
    pub fn opersel(&mut self) -> OPERSEL_W<'_, I2SCTRL_SPEC> {
        OPERSEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, I2SCTRL_SPEC> {
        EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode select"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W<'_, I2SCTRL_SPEC> {
        MSEL_W::new(self, 11)
    }
}
#[doc = "I2S control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCTRL_SPEC;
impl crate::RegisterSpec for I2SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sctrl::R`](R) reader structure"]
impl crate::Readable for I2SCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sctrl::W`](W) writer structure"]
impl crate::Writable for I2SCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SCTRL to value 0"]
impl crate::Resettable for I2SCTRL_SPEC {}
