#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "High speed internal clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hickenr {
    #[doc = "0: High speed internal clock is disabled"]
    Disabled = 0,
    #[doc = "1: High speed internal clock is enabled"]
    Enabled = 1,
}
impl From<Hickenr> for bool {
    #[inline(always)]
    fn from(variant: Hickenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICKEN` reader - High speed internal clock enable"]
pub type HICKEN_R = crate::BitReader<Hickenr>;
impl HICKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hickenr {
        match self.bits {
            false => Hickenr::Disabled,
            true => Hickenr::Enabled,
        }
    }
    #[doc = "High speed internal clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hickenr::Disabled
    }
    #[doc = "High speed internal clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hickenr::Enabled
    }
}
#[doc = "High speed internal clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HickenwWO {
    #[doc = "0: High speed internal clock disable"]
    Disable = 0,
    #[doc = "1: High speed internal clock enable"]
    Enable = 1,
}
impl From<HickenwWO> for bool {
    #[inline(always)]
    fn from(variant: HickenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICKEN` writer - High speed internal clock enable"]
pub type HICKEN_W<'a, REG> = crate::BitWriter<'a, REG, HickenwWO>;
impl<'a, REG> HICKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed internal clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HickenwWO::Disable)
    }
    #[doc = "High speed internal clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HickenwWO::Enable)
    }
}
#[doc = "High speed internal clock ready flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HICKSTBLR_A {
    #[doc = "0: Not ready"]
    NotReady = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<HICKSTBLR_A> for bool {
    #[inline(always)]
    fn from(variant: HICKSTBLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICKSTBL` reader - High speed internal clock ready flag"]
pub type HICKSTBL_R = crate::BitReader<HICKSTBLR_A>;
impl HICKSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HICKSTBLR_A {
        match self.bits {
            false => HICKSTBLR_A::NotReady,
            true => HICKSTBLR_A::Ready,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HICKSTBLR_A::NotReady
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HICKSTBLR_A::Ready
    }
}
#[doc = "Field `HICKTRIM` reader - High speed internal clock trimming"]
pub type HICKTRIM_R = crate::FieldReader;
#[doc = "Field `HICKTRIM` writer - High speed internal clock trimming"]
pub type HICKTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HICKCAL` reader - High speed internal clock calibration"]
pub type HICKCAL_R = crate::FieldReader;
#[doc = "High speed exernal crystal enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hextenr {
    #[doc = "0: High speed external crystal is disabled"]
    Disabled = 0,
    #[doc = "1: High speed external crystal is enabled"]
    Enabled = 1,
}
impl From<Hextenr> for bool {
    #[inline(always)]
    fn from(variant: Hextenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTEN` reader - High speed exernal crystal enable"]
pub type HEXTEN_R = crate::BitReader<Hextenr>;
impl HEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hextenr {
        match self.bits {
            false => Hextenr::Disabled,
            true => Hextenr::Enabled,
        }
    }
    #[doc = "High speed external crystal is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hextenr::Disabled
    }
    #[doc = "High speed external crystal is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hextenr::Enabled
    }
}
#[doc = "High speed exernal crystal enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HextenwWO {
    #[doc = "0: High speed external crystal disable"]
    Disable = 0,
    #[doc = "1: High speed external crystal enable"]
    Enable = 1,
}
impl From<HextenwWO> for bool {
    #[inline(always)]
    fn from(variant: HextenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTEN` writer - High speed exernal crystal enable"]
pub type HEXTEN_W<'a, REG> = crate::BitWriter<'a, REG, HextenwWO>;
impl<'a, REG> HEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed external crystal disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HextenwWO::Disable)
    }
    #[doc = "High speed external crystal enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HextenwWO::Enable)
    }
}
#[doc = "High speed exernal crystal ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEXTSTBLR_A {
    #[doc = "0: Not ready"]
    NotReady = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<HEXTSTBLR_A> for bool {
    #[inline(always)]
    fn from(variant: HEXTSTBLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTSTBL` reader - High speed exernal crystal ready flag"]
pub type HEXTSTBL_R = crate::BitReader<HEXTSTBLR_A>;
impl HEXTSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HEXTSTBLR_A {
        match self.bits {
            false => HEXTSTBLR_A::NotReady,
            true => HEXTSTBLR_A::Ready,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HEXTSTBLR_A::NotReady
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HEXTSTBLR_A::Ready
    }
}
#[doc = "High speed exernal crystal bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hextbypsr {
    #[doc = "0: High speed external crystal bypass is disabled"]
    Disabled = 0,
    #[doc = "1: High speed external crystal bypass is enabled"]
    Enabled = 1,
}
impl From<Hextbypsr> for bool {
    #[inline(always)]
    fn from(variant: Hextbypsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTBYPS` reader - High speed exernal crystal bypass"]
pub type HEXTBYPS_R = crate::BitReader<Hextbypsr>;
impl HEXTBYPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hextbypsr {
        match self.bits {
            false => Hextbypsr::Disabled,
            true => Hextbypsr::Enabled,
        }
    }
    #[doc = "High speed external crystal bypass is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hextbypsr::Disabled
    }
    #[doc = "High speed external crystal bypass is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hextbypsr::Enabled
    }
}
#[doc = "High speed exernal crystal bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HextbypswWO {
    #[doc = "0: High speed external crystal bypass disable"]
    Disable = 0,
    #[doc = "1: High speed external crystal bypass enable"]
    Enable = 1,
}
impl From<HextbypswWO> for bool {
    #[inline(always)]
    fn from(variant: HextbypswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTBYPS` writer - High speed exernal crystal bypass"]
pub type HEXTBYPS_W<'a, REG> = crate::BitWriter<'a, REG, HextbypswWO>;
impl<'a, REG> HEXTBYPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed external crystal bypass disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HextbypswWO::Disable)
    }
    #[doc = "High speed external crystal bypass enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HextbypswWO::Enable)
    }
}
#[doc = "Clock failure detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfdenr {
    #[doc = "0: Clock failure detector is disabled"]
    Disabled = 0,
    #[doc = "1: Clock failure detector is enabled"]
    Enabled = 1,
}
impl From<Cfdenr> for bool {
    #[inline(always)]
    fn from(variant: Cfdenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFDEN` reader - Clock failure detection enable"]
pub type CFDEN_R = crate::BitReader<Cfdenr>;
impl CFDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfdenr {
        match self.bits {
            false => Cfdenr::Disabled,
            true => Cfdenr::Enabled,
        }
    }
    #[doc = "Clock failure detector is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cfdenr::Disabled
    }
    #[doc = "Clock failure detector is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cfdenr::Enabled
    }
}
#[doc = "Clock failure detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CfdenwWO {
    #[doc = "0: Clock failure detector disable"]
    Disable = 0,
    #[doc = "1: Clock failure detector enable"]
    Enable = 1,
}
impl From<CfdenwWO> for bool {
    #[inline(always)]
    fn from(variant: CfdenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFDEN` writer - Clock failure detection enable"]
pub type CFDEN_W<'a, REG> = crate::BitWriter<'a, REG, CfdenwWO>;
impl<'a, REG> CFDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock failure detector disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CfdenwWO::Disable)
    }
    #[doc = "Clock failure detector enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CfdenwWO::Enable)
    }
}
#[doc = "PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllenr {
    #[doc = "0: PLL is disabled"]
    Disabled = 0,
    #[doc = "1: PLL is enabled"]
    Enabled = 1,
}
impl From<Pllenr> for bool {
    #[inline(always)]
    fn from(variant: Pllenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PLLEN_R = crate::BitReader<Pllenr>;
impl PLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllenr {
        match self.bits {
            false => Pllenr::Disabled,
            true => Pllenr::Enabled,
        }
    }
    #[doc = "PLL is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pllenr::Disabled
    }
    #[doc = "PLL is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pllenr::Enabled
    }
}
#[doc = "PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PllenwWO {
    #[doc = "0: PLL disable"]
    Disable = 0,
    #[doc = "1: PLL enable"]
    Enable = 1,
}
impl From<PllenwWO> for bool {
    #[inline(always)]
    fn from(variant: PllenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG, PllenwWO>;
impl<'a, REG> PLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PllenwWO::Disable)
    }
    #[doc = "PLL enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PllenwWO::Enable)
    }
}
#[doc = "PLL clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTBLR_A {
    #[doc = "0: Not ready"]
    NotReady = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<PLLSTBLR_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBL` reader - PLL clock ready flag"]
pub type PLLSTBL_R = crate::BitReader<PLLSTBLR_A>;
impl PLLSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSTBLR_A {
        match self.bits {
            false => PLLSTBLR_A::NotReady,
            true => PLLSTBLR_A::Ready,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PLLSTBLR_A::NotReady
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PLLSTBLR_A::Ready
    }
}
impl R {
    #[doc = "Bit 0 - High speed internal clock enable"]
    #[inline(always)]
    pub fn hicken(&self) -> HICKEN_R {
        HICKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High speed internal clock ready flag"]
    #[inline(always)]
    pub fn hickstbl(&self) -> HICKSTBL_R {
        HICKSTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hicktrim(&self) -> HICKTRIM_R {
        HICKTRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - High speed internal clock calibration"]
    #[inline(always)]
    pub fn hickcal(&self) -> HICKCAL_R {
        HICKCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - High speed exernal crystal enable"]
    #[inline(always)]
    pub fn hexten(&self) -> HEXTEN_R {
        HEXTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - High speed exernal crystal ready flag"]
    #[inline(always)]
    pub fn hextstbl(&self) -> HEXTSTBL_R {
        HEXTSTBL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - High speed exernal crystal bypass"]
    #[inline(always)]
    pub fn hextbyps(&self) -> HEXTBYPS_R {
        HEXTBYPS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock failure detection enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllstbl(&self) -> PLLSTBL_R {
        PLLSTBL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("hicken", &format_args!("{}", self.hicken().bit()))
            .field("hickstbl", &format_args!("{}", self.hickstbl().bit()))
            .field("hicktrim", &format_args!("{}", self.hicktrim().bits()))
            .field("hickcal", &format_args!("{}", self.hickcal().bits()))
            .field("hexten", &format_args!("{}", self.hexten().bit()))
            .field("hextstbl", &format_args!("{}", self.hextstbl().bit()))
            .field("hextbyps", &format_args!("{}", self.hextbyps().bit()))
            .field("cfden", &format_args!("{}", self.cfden().bit()))
            .field("pllen", &format_args!("{}", self.pllen().bit()))
            .field("pllstbl", &format_args!("{}", self.pllstbl().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - High speed internal clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hicken(&mut self) -> HICKEN_W<CTRL_SPEC> {
        HICKEN_W::new(self, 0)
    }
    #[doc = "Bits 2:7 - High speed internal clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hicktrim(&mut self) -> HICKTRIM_W<CTRL_SPEC> {
        HICKTRIM_W::new(self, 2)
    }
    #[doc = "Bit 16 - High speed exernal crystal enable"]
    #[inline(always)]
    #[must_use]
    pub fn hexten(&mut self) -> HEXTEN_W<CTRL_SPEC> {
        HEXTEN_W::new(self, 16)
    }
    #[doc = "Bit 18 - High speed exernal crystal bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hextbyps(&mut self) -> HEXTBYPS_W<CTRL_SPEC> {
        HEXTBYPS_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clock failure detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<CTRL_SPEC> {
        CFDEN_W::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<CTRL_SPEC> {
        PLLEN_W::new(self, 24)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x83"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x83;
}
