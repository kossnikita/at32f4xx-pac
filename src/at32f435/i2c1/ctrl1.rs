#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "I2C peripheral enable\n\nValue on reset: 0"]
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
#[doc = "Field `I2CEN` reader - I2C peripheral enable"]
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
#[doc = "I2C peripheral enable\n\nValue on reset: 0"]
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
#[doc = "Field `I2CEN` writer - I2C peripheral enable"]
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
#[doc = "Transmit data interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdienr {
    #[doc = "0: Data transmit data interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Data transmit data interrupt is enabled"]
    Enabled = 1,
}
impl From<Tdienr> for bool {
    #[inline(always)]
    fn from(variant: Tdienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIEN` reader - Transmit data interrupt enable"]
pub type TDIEN_R = crate::BitReader<Tdienr>;
impl TDIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdienr {
        match self.bits {
            false => Tdienr::Disabled,
            true => Tdienr::Enabled,
        }
    }
    #[doc = "Data transmit data interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tdienr::Disabled
    }
    #[doc = "Data transmit data interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tdienr::Enabled
    }
}
#[doc = "Transmit data interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdienwWO {
    #[doc = "0: Disable Data transmit data interrupt"]
    Disable = 0,
    #[doc = "1: Enable Data transmit data interrupt"]
    Enable = 1,
}
impl From<TdienwWO> for bool {
    #[inline(always)]
    fn from(variant: TdienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIEN` writer - Transmit data interrupt enable"]
pub type TDIEN_W<'a, REG> = crate::BitWriter<'a, REG, TdienwWO>;
impl<'a, REG> TDIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Data transmit data interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TdienwWO::Disable)
    }
    #[doc = "Enable Data transmit data interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TdienwWO::Enable)
    }
}
#[doc = "Field `RDIEN` reader - Receive data interrupt enable"]
pub type RDIEN_R = crate::BitReader;
#[doc = "Field `RDIEN` writer - Receive data interrupt enable"]
pub type RDIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRIEN` reader - Address match interrupt enable"]
pub type ADDRIEN_R = crate::BitReader;
#[doc = "Field `ADDRIEN` writer - Address match interrupt enable"]
pub type ADDRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILIEN` reader - Acknowledge fail interrupt enable"]
pub type ACKFAILIEN_R = crate::BitReader;
#[doc = "Field `ACKFAILIEN` writer - Acknowledge fail interrupt enable"]
pub type ACKFAILIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIEN` reader - Stop generation complete interrupt enable"]
pub type STOPIEN_R = crate::BitReader;
#[doc = "Field `STOPIEN` writer - Stop generation complete interrupt enable"]
pub type STOPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCIEN` reader - Transfer data complete interrupt enable"]
pub type TDCIEN_R = crate::BitReader;
#[doc = "Field `TDCIEN` writer - Transfer data complete interrupt enable"]
pub type TDCIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIEN` reader - Error interrupts enable"]
pub type ERRIEN_R = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupts enable"]
pub type ERRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFLT` reader - Digital filter value"]
pub type DFLT_R = crate::FieldReader;
#[doc = "Field `DFLT` writer - Digital filter value"]
pub type DFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMATEN` reader - DMA Transmit data request enable"]
pub type DMATEN_R = crate::BitReader;
#[doc = "Field `DMATEN` writer - DMA Transmit data request enable"]
pub type DMATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAREN` reader - DMA receive data request enable"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - DMA receive data request enable"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCTRL` reader - Slave receiving data control"]
pub type SCTRL_R = crate::BitReader;
#[doc = "Field `SCTRL` writer - Slave receiving data control"]
pub type SCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRETCH` reader - Clock stretching mode"]
pub type STRETCH_R = crate::BitReader;
#[doc = "Field `STRETCH` writer - Clock stretching mode"]
pub type STRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCAEN` reader - General call address enable"]
pub type GCAEN_R = crate::BitReader;
#[doc = "Field `GCAEN` writer - General call address enable"]
pub type GCAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HADDREN` reader - SMBus host address enable"]
pub type HADDREN_R = crate::BitReader;
#[doc = "Field `HADDREN` writer - SMBus host address enable"]
pub type HADDREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVADDREN` reader - SMBus device default address enable"]
pub type DEVADDREN_R = crate::BitReader;
#[doc = "Field `DEVADDREN` writer - SMBus device default address enable"]
pub type DEVADDREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALERT` reader - SMBus alert enable / pin set"]
pub type SMBALERT_R = crate::BitReader;
#[doc = "Field `SMBALERT` writer - SMBus alert enable / pin set"]
pub type SMBALERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` reader - PEC calculation enable"]
pub type PECEN_R = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC calculation enable"]
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit data interrupt enable"]
    #[inline(always)]
    pub fn tdien(&self) -> TDIEN_R {
        TDIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data interrupt enable"]
    #[inline(always)]
    pub fn rdien(&self) -> RDIEN_R {
        RDIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable"]
    #[inline(always)]
    pub fn addrien(&self) -> ADDRIEN_R {
        ADDRIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Acknowledge fail interrupt enable"]
    #[inline(always)]
    pub fn ackfailien(&self) -> ACKFAILIEN_R {
        ACKFAILIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop generation complete interrupt enable"]
    #[inline(always)]
    pub fn stopien(&self) -> STOPIEN_R {
        STOPIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer data complete interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&self) -> TDCIEN_R {
        TDCIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital filter value"]
    #[inline(always)]
    pub fn dflt(&self) -> DFLT_R {
        DFLT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - DMA Transmit data request enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA receive data request enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave receiving data control"]
    #[inline(always)]
    pub fn sctrl(&self) -> SCTRL_R {
        SCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GCAEN_R {
        GCAEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus host address enable"]
    #[inline(always)]
    pub fn haddren(&self) -> HADDREN_R {
        HADDREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    pub fn devaddren(&self) -> DEVADDREN_R {
        DEVADDREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable / pin set"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("i2cen", &self.i2cen())
            .field("tdien", &self.tdien())
            .field("rdien", &self.rdien())
            .field("addrien", &self.addrien())
            .field("ackfailien", &self.ackfailien())
            .field("stopien", &self.stopien())
            .field("tdcien", &self.tdcien())
            .field("errien", &self.errien())
            .field("dflt", &self.dflt())
            .field("dmaten", &self.dmaten())
            .field("dmaren", &self.dmaren())
            .field("sctrl", &self.sctrl())
            .field("stretch", &self.stretch())
            .field("gcaen", &self.gcaen())
            .field("haddren", &self.haddren())
            .field("devaddren", &self.devaddren())
            .field("smbalert", &self.smbalert())
            .field("pecen", &self.pecen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<CTRL1_SPEC> {
        I2CEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit data interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdien(&mut self) -> TDIEN_W<CTRL1_SPEC> {
        TDIEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive data interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdien(&mut self) -> RDIEN_W<CTRL1_SPEC> {
        RDIEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Address match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn addrien(&mut self) -> ADDRIEN_W<CTRL1_SPEC> {
        ADDRIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Acknowledge fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailien(&mut self) -> ACKFAILIEN_W<CTRL1_SPEC> {
        ACKFAILIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stop generation complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopien(&mut self) -> STOPIEN_W<CTRL1_SPEC> {
        STOPIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer data complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcien(&mut self) -> TDCIEN_W<CTRL1_SPEC> {
        TDCIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ERRIEN_W<CTRL1_SPEC> {
        ERRIEN_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital filter value"]
    #[inline(always)]
    #[must_use]
    pub fn dflt(&mut self) -> DFLT_W<CTRL1_SPEC> {
        DFLT_W::new(self, 8)
    }
    #[doc = "Bit 14 - DMA Transmit data request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTRL1_SPEC> {
        DMATEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA receive data request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTRL1_SPEC> {
        DMAREN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave receiving data control"]
    #[inline(always)]
    #[must_use]
    pub fn sctrl(&mut self) -> SCTRL_W<CTRL1_SPEC> {
        SCTRL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching mode"]
    #[inline(always)]
    #[must_use]
    pub fn stretch(&mut self) -> STRETCH_W<CTRL1_SPEC> {
        STRETCH_W::new(self, 17)
    }
    #[doc = "Bit 19 - General call address enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcaen(&mut self) -> GCAEN_W<CTRL1_SPEC> {
        GCAEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus host address enable"]
    #[inline(always)]
    #[must_use]
    pub fn haddren(&mut self) -> HADDREN_W<CTRL1_SPEC> {
        HADDREN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus device default address enable"]
    #[inline(always)]
    #[must_use]
    pub fn devaddren(&mut self) -> DEVADDREN_W<CTRL1_SPEC> {
        DEVADDREN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus alert enable / pin set"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SMBALERT_W<CTRL1_SPEC> {
        SMBALERT_W::new(self, 22)
    }
    #[doc = "Bit 23 - PEC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CTRL1_SPEC> {
        PECEN_W::new(self, 23)
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
