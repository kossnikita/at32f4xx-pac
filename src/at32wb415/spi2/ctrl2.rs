#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "DMA receive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmarenr {
    #[doc = "0: DMA receive is disabled"]
    Disabled = 0,
    #[doc = "1: DMA receive is enabled"]
    Enabled = 1,
}
impl From<Dmarenr> for bool {
    #[inline(always)]
    fn from(variant: Dmarenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` reader - DMA receive enable"]
pub type DMAREN_R = crate::BitReader<Dmarenr>;
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmarenr {
        match self.bits {
            false => Dmarenr::Disabled,
            true => Dmarenr::Enabled,
        }
    }
    #[doc = "DMA receive is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmarenr::Disabled
    }
    #[doc = "DMA receive is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmarenr::Enabled
    }
}
#[doc = "DMA receive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmarenwWO {
    #[doc = "0: DMA receive disable"]
    Disable = 0,
    #[doc = "1: DMA receive enable"]
    Enable = 1,
}
impl From<DmarenwWO> for bool {
    #[inline(always)]
    fn from(variant: DmarenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` writer - DMA receive enable"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG, DmarenwWO>;
impl<'a, REG> DMAREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA receive disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmarenwWO::Disable)
    }
    #[doc = "DMA receive enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmarenwWO::Enable)
    }
}
#[doc = "DMA transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmatenr {
    #[doc = "0: DMA transmit is disabled"]
    Disabled = 0,
    #[doc = "1: DMA transmit is enabled"]
    Enabled = 1,
}
impl From<Dmatenr> for bool {
    #[inline(always)]
    fn from(variant: Dmatenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` reader - DMA transmit enable"]
pub type DMATEN_R = crate::BitReader<Dmatenr>;
impl DMATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmatenr {
        match self.bits {
            false => Dmatenr::Disabled,
            true => Dmatenr::Enabled,
        }
    }
    #[doc = "DMA transmit is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmatenr::Disabled
    }
    #[doc = "DMA transmit is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmatenr::Enabled
    }
}
#[doc = "DMA transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmatenwWO {
    #[doc = "0: DMA transmit disable"]
    Disable = 0,
    #[doc = "1: DMA transmit enable"]
    Enable = 1,
}
impl From<DmatenwWO> for bool {
    #[inline(always)]
    fn from(variant: DmatenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` writer - DMA transmit enable"]
pub type DMATEN_W<'a, REG> = crate::BitWriter<'a, REG, DmatenwWO>;
impl<'a, REG> DMATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transmit disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmatenwWO::Disable)
    }
    #[doc = "DMA transmit enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmatenwWO::Enable)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errier {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt is enabled"]
    Enabled = 1,
}
impl From<Errier> for bool {
    #[inline(always)]
    fn from(variant: Errier) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<Errier>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errier {
        match self.bits {
            false => Errier::Disabled,
            true => Errier::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errier::Disabled
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errier::Enabled
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErriewWO {
    #[doc = "0: Error interrupt disable"]
    Disable = 0,
    #[doc = "1: Error interrupt enable"]
    Enable = 1,
}
impl From<ErriewWO> for bool {
    #[inline(always)]
    fn from(variant: ErriewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ErriewWO>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ErriewWO::Disable)
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ErriewWO::Enable)
    }
}
#[doc = "Receive data buffer full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdbfier {
    #[doc = "0: Receive data buffer full interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive data buffer full interrupt is enabled"]
    Enabled = 1,
}
impl From<Rdbfier> for bool {
    #[inline(always)]
    fn from(variant: Rdbfier) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBFIE` reader - Receive data buffer full interrupt enable"]
pub type RDBFIE_R = crate::BitReader<Rdbfier>;
impl RDBFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdbfier {
        match self.bits {
            false => Rdbfier::Disabled,
            true => Rdbfier::Enabled,
        }
    }
    #[doc = "Receive data buffer full interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rdbfier::Disabled
    }
    #[doc = "Receive data buffer full interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rdbfier::Enabled
    }
}
#[doc = "Receive data buffer full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdbfiewWO {
    #[doc = "0: Receive data buffer full interrupt disable"]
    Disable = 0,
    #[doc = "1: Receive data buffer full interrupt enable"]
    Enable = 1,
}
impl From<RdbfiewWO> for bool {
    #[inline(always)]
    fn from(variant: RdbfiewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBFIE` writer - Receive data buffer full interrupt enable"]
pub type RDBFIE_W<'a, REG> = crate::BitWriter<'a, REG, RdbfiewWO>;
impl<'a, REG> RDBFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data buffer full interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RdbfiewWO::Disable)
    }
    #[doc = "Receive data buffer full interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RdbfiewWO::Enable)
    }
}
#[doc = "Transmit data buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdbeier {
    #[doc = "0: Transmit data buffer empty interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Transmit data buffer empty interrupt is enabled"]
    Enabled = 1,
}
impl From<Tdbeier> for bool {
    #[inline(always)]
    fn from(variant: Tdbeier) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBEIE` reader - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_R = crate::BitReader<Tdbeier>;
impl TDBEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdbeier {
        match self.bits {
            false => Tdbeier::Disabled,
            true => Tdbeier::Enabled,
        }
    }
    #[doc = "Transmit data buffer empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tdbeier::Disabled
    }
    #[doc = "Transmit data buffer empty interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tdbeier::Enabled
    }
}
#[doc = "Transmit data buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdbeiewWO {
    #[doc = "0: Transmit data buffer empty interrupt disable"]
    Disable = 0,
    #[doc = "1: Transmit data buffer empty interrupt enable"]
    Enable = 1,
}
impl From<TdbeiewWO> for bool {
    #[inline(always)]
    fn from(variant: TdbeiewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBEIE` writer - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_W<'a, REG> = crate::BitWriter<'a, REG, TdbeiewWO>;
impl<'a, REG> TDBEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data buffer empty interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TdbeiewWO::Disable)
    }
    #[doc = "Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TdbeiewWO::Enable)
    }
}
#[doc = "Field `MDIV3` reader - Master clock frequency division bit3"]
pub type MDIV3_R = crate::BitReader;
#[doc = "Field `MDIV3` writer - Master clock frequency division bit3"]
pub type MDIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    pub fn rdbfie(&self) -> RDBFIE_R {
        RDBFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tdbeie(&self) -> TDBEIE_R {
        TDBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master clock frequency division bit3"]
    #[inline(always)]
    pub fn mdiv3(&self) -> MDIV3_R {
        MDIV3_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("mdiv3", &format_args!("{}", self.mdiv3().bit()))
            .field("tdbeie", &format_args!("{}", self.tdbeie().bit()))
            .field("rdbfie", &format_args!("{}", self.rdbfie().bit()))
            .field("errie", &format_args!("{}", self.errie().bit()))
            .field("dmaten", &format_args!("{}", self.dmaten().bit()))
            .field("dmaren", &format_args!("{}", self.dmaren().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTRL2_SPEC> {
        DMAREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTRL2_SPEC> {
        DMATEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTRL2_SPEC> {
        ERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfie(&mut self) -> RDBFIE_W<CTRL2_SPEC> {
        RDBFIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeie(&mut self) -> TDBEIE_W<CTRL2_SPEC> {
        TDBEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Master clock frequency division bit3"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv3(&mut self) -> MDIV3_W<CTRL2_SPEC> {
        MDIV3_W::new(self, 8)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
