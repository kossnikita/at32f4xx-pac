#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `DMAREN` reader - DMA receive enable"]
pub type DMAREN_R = crate::BitReader<DMARENR_A>;
#[doc = "DMA receive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARENR_A {
    #[doc = "0: DMA receive is disabled"]
    Disabled = 0,
    #[doc = "1: DMA receive is enabled"]
    Enabled = 1,
}
impl From<DMARENR_A> for bool {
    #[inline(always)]
    fn from(variant: DMARENR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMARENR_A {
        match self.bits {
            false => DMARENR_A::Disabled,
            true => DMARENR_A::Enabled,
        }
    }
    #[doc = "DMA receive is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMARENR_A::Disabled
    }
    #[doc = "DMA receive is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMARENR_A::Enabled
    }
}
#[doc = "DMA receive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARENW_AW {
    #[doc = "0: DMA receive disable"]
    Disable = 0,
    #[doc = "1: DMA receive enable"]
    Enable = 1,
}
impl From<DMARENW_AW> for bool {
    #[inline(always)]
    fn from(variant: DMARENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAREN` writer - DMA receive enable"]
pub type DMAREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMARENW_AW>;
impl<'a, REG, const O: u8> DMAREN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA receive disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DMARENW_AW::Disable)
    }
    #[doc = "DMA receive enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DMARENW_AW::Enable)
    }
}
#[doc = "Field `DMATEN` reader - DMA transmit enable"]
pub type DMATEN_R = crate::BitReader<DMATENR_A>;
#[doc = "DMA transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMATENR_A {
    #[doc = "0: DMA transmit is disabled"]
    Disabled = 0,
    #[doc = "1: DMA transmit is enabled"]
    Enabled = 1,
}
impl From<DMATENR_A> for bool {
    #[inline(always)]
    fn from(variant: DMATENR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMATENR_A {
        match self.bits {
            false => DMATENR_A::Disabled,
            true => DMATENR_A::Enabled,
        }
    }
    #[doc = "DMA transmit is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATENR_A::Disabled
    }
    #[doc = "DMA transmit is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATENR_A::Enabled
    }
}
#[doc = "DMA transmit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMATENW_AW {
    #[doc = "0: DMA transmit disable"]
    Disable = 0,
    #[doc = "1: DMA transmit enable"]
    Enable = 1,
}
impl From<DMATENW_AW> for bool {
    #[inline(always)]
    fn from(variant: DMATENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATEN` writer - DMA transmit enable"]
pub type DMATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMATENW_AW>;
impl<'a, REG, const O: u8> DMATEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transmit disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DMATENW_AW::Disable)
    }
    #[doc = "DMA transmit enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DMATENW_AW::Enable)
    }
}
#[doc = "Field `HWCSOE` reader - Hardware CS output enable"]
pub type HWCSOE_R = crate::BitReader<HWCSOER_A>;
#[doc = "Hardware CS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWCSOER_A {
    #[doc = "0: Hardware CS output is disabled"]
    Disabled = 0,
    #[doc = "1: Hardware CS output is enabled"]
    Enabled = 1,
}
impl From<HWCSOER_A> for bool {
    #[inline(always)]
    fn from(variant: HWCSOER_A) -> Self {
        variant as u8 != 0
    }
}
impl HWCSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWCSOER_A {
        match self.bits {
            false => HWCSOER_A::Disabled,
            true => HWCSOER_A::Enabled,
        }
    }
    #[doc = "Hardware CS output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWCSOER_A::Disabled
    }
    #[doc = "Hardware CS output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWCSOER_A::Enabled
    }
}
#[doc = "Hardware CS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWCSOEW_AW {
    #[doc = "0: Hardware CS output disable"]
    Disable = 0,
    #[doc = "1: Hardware CS output enable"]
    Enable = 1,
}
impl From<HWCSOEW_AW> for bool {
    #[inline(always)]
    fn from(variant: HWCSOEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWCSOE` writer - Hardware CS output enable"]
pub type HWCSOE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HWCSOEW_AW>;
impl<'a, REG, const O: u8> HWCSOE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware CS output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HWCSOEW_AW::Disable)
    }
    #[doc = "Hardware CS output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HWCSOEW_AW::Enable)
    }
}
#[doc = "Field `TIEN` reader - TI mode enable"]
pub type TIEN_R = crate::BitReader;
#[doc = "Field `TIEN` writer - TI mode enable"]
pub type TIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIER_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIER_A {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt is enabled"]
    Enabled = 1,
}
impl From<ERRIER_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIER_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIER_A {
        match self.bits {
            false => ERRIER_A::Disabled,
            true => ERRIER_A::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIER_A::Disabled
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIER_A::Enabled
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIEW_AW {
    #[doc = "0: Error interrupt disable"]
    Disable = 0,
    #[doc = "1: Error interrupt enable"]
    Enable = 1,
}
impl From<ERRIEW_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRIEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERRIEW_AW>;
impl<'a, REG, const O: u8> ERRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIEW_AW::Disable)
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIEW_AW::Enable)
    }
}
#[doc = "Field `RDBFIE` reader - Receive data buffer full interrupt enable"]
pub type RDBFIE_R = crate::BitReader<RDBFIER_A>;
#[doc = "Receive data buffer full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFIER_A {
    #[doc = "0: Receive data buffer full interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Receive data buffer full interrupt is enabled"]
    Enabled = 1,
}
impl From<RDBFIER_A> for bool {
    #[inline(always)]
    fn from(variant: RDBFIER_A) -> Self {
        variant as u8 != 0
    }
}
impl RDBFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDBFIER_A {
        match self.bits {
            false => RDBFIER_A::Disabled,
            true => RDBFIER_A::Enabled,
        }
    }
    #[doc = "Receive data buffer full interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDBFIER_A::Disabled
    }
    #[doc = "Receive data buffer full interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDBFIER_A::Enabled
    }
}
#[doc = "Receive data buffer full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDBFIEW_AW {
    #[doc = "0: Receive data buffer full interrupt disable"]
    Disable = 0,
    #[doc = "1: Receive data buffer full interrupt enable"]
    Enable = 1,
}
impl From<RDBFIEW_AW> for bool {
    #[inline(always)]
    fn from(variant: RDBFIEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDBFIE` writer - Receive data buffer full interrupt enable"]
pub type RDBFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RDBFIEW_AW>;
impl<'a, REG, const O: u8> RDBFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data buffer full interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RDBFIEW_AW::Disable)
    }
    #[doc = "Receive data buffer full interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RDBFIEW_AW::Enable)
    }
}
#[doc = "Field `TDBEIE` reader - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_R = crate::BitReader<TDBEIER_A>;
#[doc = "Transmit data buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEIER_A {
    #[doc = "0: Transmit data buffer empty interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Transmit data buffer empty interrupt is enabled"]
    Enabled = 1,
}
impl From<TDBEIER_A> for bool {
    #[inline(always)]
    fn from(variant: TDBEIER_A) -> Self {
        variant as u8 != 0
    }
}
impl TDBEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDBEIER_A {
        match self.bits {
            false => TDBEIER_A::Disabled,
            true => TDBEIER_A::Enabled,
        }
    }
    #[doc = "Transmit data buffer empty interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDBEIER_A::Disabled
    }
    #[doc = "Transmit data buffer empty interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDBEIER_A::Enabled
    }
}
#[doc = "Transmit data buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDBEIEW_AW {
    #[doc = "0: Transmit data buffer empty interrupt disable"]
    Disable = 0,
    #[doc = "1: Transmit data buffer empty interrupt enable"]
    Enable = 1,
}
impl From<TDBEIEW_AW> for bool {
    #[inline(always)]
    fn from(variant: TDBEIEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDBEIE` writer - Transmit data buffer empty interrupt enable"]
pub type TDBEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TDBEIEW_AW>;
impl<'a, REG, const O: u8> TDBEIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data buffer empty interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TDBEIEW_AW::Disable)
    }
    #[doc = "Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TDBEIEW_AW::Enable)
    }
}
#[doc = "Field `MDIV3` reader - Master clock frequency division bit3"]
pub type MDIV3_R = crate::BitReader;
#[doc = "Field `MDIV3` writer - Master clock frequency division bit3"]
pub type MDIV3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MDIV3EN` reader - Master clock frequency 3 division enable"]
pub type MDIV3EN_R = crate::BitReader;
#[doc = "Field `MDIV3EN` writer - Master clock frequency 3 division enable"]
pub type MDIV3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 2 - Hardware CS output enable"]
    #[inline(always)]
    pub fn hwcsoe(&self) -> HWCSOE_R {
        HWCSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TI mode enable"]
    #[inline(always)]
    pub fn tien(&self) -> TIEN_R {
        TIEN_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 9 - Master clock frequency 3 division enable"]
    #[inline(always)]
    pub fn mdiv3en(&self) -> MDIV3EN_R {
        MDIV3EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA receive enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTRL2_SPEC, 0> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 1 - DMA transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTRL2_SPEC, 1> {
        DMATEN_W::new(self)
    }
    #[doc = "Bit 2 - Hardware CS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwcsoe(&mut self) -> HWCSOE_W<CTRL2_SPEC, 2> {
        HWCSOE_W::new(self)
    }
    #[doc = "Bit 4 - TI mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TIEN_W<CTRL2_SPEC, 4> {
        TIEN_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTRL2_SPEC, 5> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive data buffer full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfie(&mut self) -> RDBFIE_W<CTRL2_SPEC, 6> {
        RDBFIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit data buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeie(&mut self) -> TDBEIE_W<CTRL2_SPEC, 7> {
        TDBEIE_W::new(self)
    }
    #[doc = "Bit 8 - Master clock frequency division bit3"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv3(&mut self) -> MDIV3_W<CTRL2_SPEC, 8> {
        MDIV3_W::new(self)
    }
    #[doc = "Bit 9 - Master clock frequency 3 division enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv3en(&mut self) -> MDIV3EN_W<CTRL2_SPEC, 9> {
        MDIV3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
