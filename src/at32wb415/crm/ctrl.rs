#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `HICKEN` reader - High speed internal clock enable"]
pub type HICKEN_R = crate::BitReader<HICKENR_A>;
#[doc = "High speed internal clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HICKENR_A {
    #[doc = "0: High speed internal clock is disabled"]
    Disabled = 0,
    #[doc = "1: High speed internal clock is enabled"]
    Enabled = 1,
}
impl From<HICKENR_A> for bool {
    #[inline(always)]
    fn from(variant: HICKENR_A) -> Self {
        variant as u8 != 0
    }
}
impl HICKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HICKENR_A {
        match self.bits {
            false => HICKENR_A::Disabled,
            true => HICKENR_A::Enabled,
        }
    }
    #[doc = "High speed internal clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HICKENR_A::Disabled
    }
    #[doc = "High speed internal clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HICKENR_A::Enabled
    }
}
#[doc = "High speed internal clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HICKENW_AW {
    #[doc = "0: High speed internal clock disable"]
    Disable = 0,
    #[doc = "1: High speed internal clock enable"]
    Enable = 1,
}
impl From<HICKENW_AW> for bool {
    #[inline(always)]
    fn from(variant: HICKENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICKEN` writer - High speed internal clock enable"]
pub type HICKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HICKENW_AW>;
impl<'a, REG, const O: u8> HICKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed internal clock disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HICKENW_AW::Disable)
    }
    #[doc = "High speed internal clock enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HICKENW_AW::Enable)
    }
}
#[doc = "Field `HICKSTBL` reader - High speed internal clock ready flag"]
pub type HICKSTBL_R = crate::BitReader<HICKSTBLR_A>;
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
impl HICKSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HICKSTBLR_A {
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
pub type HICKTRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `HICKCAL` reader - High speed internal clock calibration"]
pub type HICKCAL_R = crate::FieldReader;
#[doc = "Field `HEXTEN` reader - High speed exernal crystal enable"]
pub type HEXTEN_R = crate::BitReader<HEXTENR_A>;
#[doc = "High speed exernal crystal enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEXTENR_A {
    #[doc = "0: High speed external crystal is disabled"]
    Disabled = 0,
    #[doc = "1: High speed external crystal is enabled"]
    Enabled = 1,
}
impl From<HEXTENR_A> for bool {
    #[inline(always)]
    fn from(variant: HEXTENR_A) -> Self {
        variant as u8 != 0
    }
}
impl HEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEXTENR_A {
        match self.bits {
            false => HEXTENR_A::Disabled,
            true => HEXTENR_A::Enabled,
        }
    }
    #[doc = "High speed external crystal is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HEXTENR_A::Disabled
    }
    #[doc = "High speed external crystal is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HEXTENR_A::Enabled
    }
}
#[doc = "High speed exernal crystal enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEXTENW_AW {
    #[doc = "0: High speed external crystal disable"]
    Disable = 0,
    #[doc = "1: High speed external crystal enable"]
    Enable = 1,
}
impl From<HEXTENW_AW> for bool {
    #[inline(always)]
    fn from(variant: HEXTENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTEN` writer - High speed exernal crystal enable"]
pub type HEXTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HEXTENW_AW>;
impl<'a, REG, const O: u8> HEXTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed external crystal disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HEXTENW_AW::Disable)
    }
    #[doc = "High speed external crystal enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HEXTENW_AW::Enable)
    }
}
#[doc = "Field `HEXTSTBL` reader - High speed exernal crystal ready flag"]
pub type HEXTSTBL_R = crate::BitReader<HEXTSTBLR_A>;
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
impl HEXTSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEXTSTBLR_A {
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
#[doc = "Field `HEXTBYPS` reader - High speed exernal crystal bypass"]
pub type HEXTBYPS_R = crate::BitReader<HEXTBYPSR_A>;
#[doc = "High speed exernal crystal bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEXTBYPSR_A {
    #[doc = "0: High speed external crystal bypass is disabled"]
    Disabled = 0,
    #[doc = "1: High speed external crystal bypass is enabled"]
    Enabled = 1,
}
impl From<HEXTBYPSR_A> for bool {
    #[inline(always)]
    fn from(variant: HEXTBYPSR_A) -> Self {
        variant as u8 != 0
    }
}
impl HEXTBYPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEXTBYPSR_A {
        match self.bits {
            false => HEXTBYPSR_A::Disabled,
            true => HEXTBYPSR_A::Enabled,
        }
    }
    #[doc = "High speed external crystal bypass is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HEXTBYPSR_A::Disabled
    }
    #[doc = "High speed external crystal bypass is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HEXTBYPSR_A::Enabled
    }
}
#[doc = "High speed exernal crystal bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEXTBYPSW_AW {
    #[doc = "0: High speed external crystal bypass disable"]
    Disable = 0,
    #[doc = "1: High speed external crystal bypass enable"]
    Enable = 1,
}
impl From<HEXTBYPSW_AW> for bool {
    #[inline(always)]
    fn from(variant: HEXTBYPSW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTBYPS` writer - High speed exernal crystal bypass"]
pub type HEXTBYPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HEXTBYPSW_AW>;
impl<'a, REG, const O: u8> HEXTBYPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High speed external crystal bypass disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HEXTBYPSW_AW::Disable)
    }
    #[doc = "High speed external crystal bypass enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HEXTBYPSW_AW::Enable)
    }
}
#[doc = "Field `CFDEN` reader - Clock failure detection enable"]
pub type CFDEN_R = crate::BitReader<CFDENR_A>;
#[doc = "Clock failure detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFDENR_A {
    #[doc = "0: Clock failure detector is disabled"]
    Disabled = 0,
    #[doc = "1: Clock failure detector is enabled"]
    Enabled = 1,
}
impl From<CFDENR_A> for bool {
    #[inline(always)]
    fn from(variant: CFDENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CFDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFDENR_A {
        match self.bits {
            false => CFDENR_A::Disabled,
            true => CFDENR_A::Enabled,
        }
    }
    #[doc = "Clock failure detector is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CFDENR_A::Disabled
    }
    #[doc = "Clock failure detector is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CFDENR_A::Enabled
    }
}
#[doc = "Clock failure detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFDENW_AW {
    #[doc = "0: Clock failure detector disable"]
    Disable = 0,
    #[doc = "1: Clock failure detector enable"]
    Enable = 1,
}
impl From<CFDENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CFDENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFDEN` writer - Clock failure detection enable"]
pub type CFDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CFDENW_AW>;
impl<'a, REG, const O: u8> CFDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock failure detector disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CFDENW_AW::Disable)
    }
    #[doc = "Clock failure detector enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CFDENW_AW::Enable)
    }
}
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PLLEN_R = crate::BitReader<PLLENR_A>;
#[doc = "PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLENR_A {
    #[doc = "0: PLL is disabled"]
    Disabled = 0,
    #[doc = "1: PLL is enabled"]
    Enabled = 1,
}
impl From<PLLENR_A> for bool {
    #[inline(always)]
    fn from(variant: PLLENR_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLENR_A {
        match self.bits {
            false => PLLENR_A::Disabled,
            true => PLLENR_A::Enabled,
        }
    }
    #[doc = "PLL is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLENR_A::Disabled
    }
    #[doc = "PLL is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLENR_A::Enabled
    }
}
#[doc = "PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLENW_AW {
    #[doc = "0: PLL disable"]
    Disable = 0,
    #[doc = "1: PLL enable"]
    Enable = 1,
}
impl From<PLLENW_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PLLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PLLENW_AW>;
impl<'a, REG, const O: u8> PLLEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PLLENW_AW::Disable)
    }
    #[doc = "PLL enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PLLENW_AW::Enable)
    }
}
#[doc = "Field `PLLSTBL` reader - PLL clock ready flag"]
pub type PLLSTBL_R = crate::BitReader<PLLSTBLR_A>;
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
impl PLLSTBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTBLR_A {
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - High speed internal clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hicken(&mut self) -> HICKEN_W<CTRL_SPEC, 0> {
        HICKEN_W::new(self)
    }
    #[doc = "Bits 2:7 - High speed internal clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hicktrim(&mut self) -> HICKTRIM_W<CTRL_SPEC, 2> {
        HICKTRIM_W::new(self)
    }
    #[doc = "Bit 16 - High speed exernal crystal enable"]
    #[inline(always)]
    #[must_use]
    pub fn hexten(&mut self) -> HEXTEN_W<CTRL_SPEC, 16> {
        HEXTEN_W::new(self)
    }
    #[doc = "Bit 18 - High speed exernal crystal bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hextbyps(&mut self) -> HEXTBYPS_W<CTRL_SPEC, 18> {
        HEXTBYPS_W::new(self)
    }
    #[doc = "Bit 19 - Clock failure detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<CTRL_SPEC, 19> {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<CTRL_SPEC, 24> {
        PLLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x83"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x83;
}
