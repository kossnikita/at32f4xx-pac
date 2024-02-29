#[doc = "Register `BRK` reader"]
pub type R = crate::R<BRK_SPEC>;
#[doc = "Register `BRK` writer"]
pub type W = crate::W<BRK_SPEC>;
#[doc = "Field `DTC` reader - Dead-time configuration"]
pub type DTC_R = crate::FieldReader;
#[doc = "Field `DTC` writer - Dead-time configuration"]
pub type DTC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Write protected configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WPC_A {
    #[doc = "0: Write protection is OFF"]
    NoProtect = 0,
    #[doc = "1: Write protection level 3, and the following bits are write protected"]
    Level3 = 1,
    #[doc = "2: Write protection level 2. The following bits and all bits in level 3 are write protected"]
    Level2 = 2,
    #[doc = "3: Write protection level 1. The following bits and all bits in level 2 are write protected"]
    Level1 = 3,
}
impl From<WPC_A> for u8 {
    #[inline(always)]
    fn from(variant: WPC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPC_A {
    type Ux = u8;
}
#[doc = "Field `WPC` reader - Write protected configuration"]
pub type WPC_R = crate::FieldReader<WPC_A>;
impl WPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WPC_A {
        match self.bits {
            0 => WPC_A::NoProtect,
            1 => WPC_A::Level3,
            2 => WPC_A::Level2,
            3 => WPC_A::Level1,
            _ => unreachable!(),
        }
    }
    #[doc = "Write protection is OFF"]
    #[inline(always)]
    pub fn is_no_protect(&self) -> bool {
        *self == WPC_A::NoProtect
    }
    #[doc = "Write protection level 3, and the following bits are write protected"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == WPC_A::Level3
    }
    #[doc = "Write protection level 2. The following bits and all bits in level 3 are write protected"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == WPC_A::Level2
    }
    #[doc = "Write protection level 1. The following bits and all bits in level 2 are write protected"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == WPC_A::Level1
    }
}
#[doc = "Field `WPC` writer - Write protected configuration"]
pub type WPC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WPC_A>;
impl<'a, REG> WPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write protection is OFF"]
    #[inline(always)]
    pub fn no_protect(self) -> &'a mut crate::W<REG> {
        self.variant(WPC_A::NoProtect)
    }
    #[doc = "Write protection level 3, and the following bits are write protected"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(WPC_A::Level3)
    }
    #[doc = "Write protection level 2. The following bits and all bits in level 3 are write protected"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(WPC_A::Level2)
    }
    #[doc = "Write protection level 1. The following bits and all bits in level 2 are write protected"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(WPC_A::Level1)
    }
}
#[doc = "Frozen channel status when holistic output disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcsodisr {
    #[doc = "0: CxOUT/CxCOUT outputs are disabled"]
    Disabled = 0,
    #[doc = "1: CxOUT/CxCOUT outputs are enabled"]
    Enabled = 1,
}
impl From<Fcsodisr> for bool {
    #[inline(always)]
    fn from(variant: Fcsodisr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCSODIS` reader - Frozen channel status when holistic output disable"]
pub type FCSODIS_R = crate::BitReader<Fcsodisr>;
impl FCSODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcsodisr {
        match self.bits {
            false => Fcsodisr::Disabled,
            true => Fcsodisr::Enabled,
        }
    }
    #[doc = "CxOUT/CxCOUT outputs are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fcsodisr::Disabled
    }
    #[doc = "CxOUT/CxCOUT outputs are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fcsodisr::Enabled
    }
}
#[doc = "Frozen channel status when holistic output disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FcsodiswWO {
    #[doc = "0: CxOUT/CxCOUT outputs disable"]
    Disable = 0,
    #[doc = "1: CxOUT/CxCOUT outputs enable"]
    Enable = 1,
}
impl From<FcsodiswWO> for bool {
    #[inline(always)]
    fn from(variant: FcsodiswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCSODIS` writer - Frozen channel status when holistic output disable"]
pub type FCSODIS_W<'a, REG> = crate::BitWriter<'a, REG, FcsodiswWO>;
impl<'a, REG> FCSODIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxOUT/CxCOUT outputs disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FcsodiswWO::Disable)
    }
    #[doc = "CxOUT/CxCOUT outputs enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FcsodiswWO::Enable)
    }
}
#[doc = "Frozen channel status when holistic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcsoenr {
    #[doc = "0: CxOUT/CxCOUT outputs are disabled"]
    Disabled = 0,
    #[doc = "1: CxOUT/CxCOUT outputs are enabled"]
    Enabled = 1,
}
impl From<Fcsoenr> for bool {
    #[inline(always)]
    fn from(variant: Fcsoenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCSOEN` reader - Frozen channel status when holistic output enable"]
pub type FCSOEN_R = crate::BitReader<Fcsoenr>;
impl FCSOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcsoenr {
        match self.bits {
            false => Fcsoenr::Disabled,
            true => Fcsoenr::Enabled,
        }
    }
    #[doc = "CxOUT/CxCOUT outputs are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fcsoenr::Disabled
    }
    #[doc = "CxOUT/CxCOUT outputs are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fcsoenr::Enabled
    }
}
#[doc = "Frozen channel status when holistic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FcsoenwWO {
    #[doc = "0: CxOUT/CxCOUT outputs disable"]
    Disable = 0,
    #[doc = "1: CxOUT/CxCOUT outputs enable"]
    Enable = 1,
}
impl From<FcsoenwWO> for bool {
    #[inline(always)]
    fn from(variant: FcsoenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCSOEN` writer - Frozen channel status when holistic output enable"]
pub type FCSOEN_W<'a, REG> = crate::BitWriter<'a, REG, FcsoenwWO>;
impl<'a, REG> FCSOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxOUT/CxCOUT outputs disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FcsoenwWO::Disable)
    }
    #[doc = "CxOUT/CxCOUT outputs enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FcsoenwWO::Enable)
    }
}
#[doc = "Brake enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkenr {
    #[doc = "0: Break input is disabled"]
    Disabled = 0,
    #[doc = "1: Break input is enabled"]
    Enabled = 1,
}
impl From<Brkenr> for bool {
    #[inline(always)]
    fn from(variant: Brkenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEN` reader - Brake enable"]
pub type BRKEN_R = crate::BitReader<Brkenr>;
impl BRKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brkenr {
        match self.bits {
            false => Brkenr::Disabled,
            true => Brkenr::Enabled,
        }
    }
    #[doc = "Break input is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Brkenr::Disabled
    }
    #[doc = "Break input is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Brkenr::Enabled
    }
}
#[doc = "Brake enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrkenwWO {
    #[doc = "0: Break input disable"]
    Disable = 0,
    #[doc = "1: Break input enable"]
    Enable = 1,
}
impl From<BrkenwWO> for bool {
    #[inline(always)]
    fn from(variant: BrkenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEN` writer - Brake enable"]
pub type BRKEN_W<'a, REG> = crate::BitWriter<'a, REG, BrkenwWO>;
impl<'a, REG> BRKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BrkenwWO::Disable)
    }
    #[doc = "Break input enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BrkenwWO::Enable)
    }
}
#[doc = "Brake input validity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKV_A {
    #[doc = "0: Break input is active low"]
    Low = 0,
    #[doc = "1: Break input is active high"]
    High = 1,
}
impl From<BRKV_A> for bool {
    #[inline(always)]
    fn from(variant: BRKV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKV` reader - Brake input validity"]
pub type BRKV_R = crate::BitReader<BRKV_A>;
impl BRKV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRKV_A {
        match self.bits {
            false => BRKV_A::Low,
            true => BRKV_A::High,
        }
    }
    #[doc = "Break input is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == BRKV_A::Low
    }
    #[doc = "Break input is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == BRKV_A::High
    }
}
#[doc = "Field `BRKV` writer - Brake input validity"]
pub type BRKV_W<'a, REG> = crate::BitWriter<'a, REG, BRKV_A>;
impl<'a, REG> BRKV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(BRKV_A::Low)
    }
    #[doc = "Break input is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(BRKV_A::High)
    }
}
#[doc = "Automatic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aoenr {
    #[doc = "0: Automatic output is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic output is enabled"]
    Enabled = 1,
}
impl From<Aoenr> for bool {
    #[inline(always)]
    fn from(variant: Aoenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOEN` reader - Automatic output enable"]
pub type AOEN_R = crate::BitReader<Aoenr>;
impl AOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aoenr {
        match self.bits {
            false => Aoenr::Disabled,
            true => Aoenr::Enabled,
        }
    }
    #[doc = "Automatic output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Aoenr::Disabled
    }
    #[doc = "Automatic output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Aoenr::Enabled
    }
}
#[doc = "Automatic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AoenwWO {
    #[doc = "0: Automatic output disable"]
    Disable = 0,
    #[doc = "1: Automatic output enable"]
    Enable = 1,
}
impl From<AoenwWO> for bool {
    #[inline(always)]
    fn from(variant: AoenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOEN` writer - Automatic output enable"]
pub type AOEN_W<'a, REG> = crate::BitWriter<'a, REG, AoenwWO>;
impl<'a, REG> AOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AoenwWO::Disable)
    }
    #[doc = "Automatic output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AoenwWO::Enable)
    }
}
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oenr {
    #[doc = "0: Output is disabled"]
    Disabled = 0,
    #[doc = "1: Output is enabled"]
    Enabled = 1,
}
impl From<Oenr> for bool {
    #[inline(always)]
    fn from(variant: Oenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEN` reader - Output enable"]
pub type OEN_R = crate::BitReader<Oenr>;
impl OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oenr {
        match self.bits {
            false => Oenr::Disabled,
            true => Oenr::Enabled,
        }
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Oenr::Disabled
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Oenr::Enabled
    }
}
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OenwWO {
    #[doc = "0: Output disable"]
    Disable = 0,
    #[doc = "1: Output enable"]
    Enable = 1,
}
impl From<OenwWO> for bool {
    #[inline(always)]
    fn from(variant: OenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEN` writer - Output enable"]
pub type OEN_W<'a, REG> = crate::BitWriter<'a, REG, OenwWO>;
impl<'a, REG> OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OenwWO::Disable)
    }
    #[doc = "Output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OenwWO::Enable)
    }
}
impl R {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    pub fn wpc(&self) -> WPC_R {
        WPC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    pub fn fcsodis(&self) -> FCSODIS_R {
        FCSODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    pub fn fcsoen(&self) -> FCSOEN_R {
        FCSOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    pub fn brken(&self) -> BRKEN_R {
        BRKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    pub fn brkv(&self) -> BRKV_R {
        BRKV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoen(&self) -> AOEN_R {
        AOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRK")
            .field("oen", &format_args!("{}", self.oen().bit()))
            .field("aoen", &format_args!("{}", self.aoen().bit()))
            .field("brkv", &format_args!("{}", self.brkv().bit()))
            .field("brken", &format_args!("{}", self.brken().bit()))
            .field("fcsoen", &format_args!("{}", self.fcsoen().bit()))
            .field("fcsodis", &format_args!("{}", self.fcsodis().bit()))
            .field("wpc", &format_args!("{}", self.wpc().bits()))
            .field("dtc", &format_args!("{}", self.dtc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BRK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dtc(&mut self) -> DTC_W<BRK_SPEC> {
        DTC_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wpc(&mut self) -> WPC_W<BRK_SPEC> {
        WPC_W::new(self, 8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    #[must_use]
    pub fn fcsodis(&mut self) -> FCSODIS_W<BRK_SPEC> {
        FCSODIS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcsoen(&mut self) -> FCSOEN_W<BRK_SPEC> {
        FCSOEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    #[must_use]
    pub fn brken(&mut self) -> BRKEN_W<BRK_SPEC> {
        BRKEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    #[must_use]
    pub fn brkv(&mut self) -> BRKV_W<BRK_SPEC> {
        BRKV_W::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn aoen(&mut self) -> AOEN_W<BRK_SPEC> {
        AOEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OEN_W<BRK_SPEC> {
        OEN_W::new(self, 15)
    }
}
#[doc = "Brake register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRK_SPEC;
impl crate::RegisterSpec for BRK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brk::R`](R) reader structure"]
impl crate::Readable for BRK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brk::W`](W) writer structure"]
impl crate::Writable for BRK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRK to value 0"]
impl crate::Resettable for BRK_SPEC {
    const RESET_VALUE: u32 = 0;
}
