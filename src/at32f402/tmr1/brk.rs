#[doc = "Register `BRK` reader"]
pub type R = crate::R<BRK_SPEC>;
#[doc = "Register `BRK` writer"]
pub type W = crate::W<BRK_SPEC>;
#[doc = "Field `DTC` reader - Dead-time configuration"]
pub type DTC_R = crate::FieldReader;
#[doc = "Field `DTC` writer - Dead-time configuration"]
pub type DTC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `WPC` reader - Write protected configuration"]
pub type WPC_R = crate::FieldReader<WPC_A>;
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
#[doc = "Field `FCSODIS` reader - Frozen channel status when holistic output disable"]
pub type FCSODIS_R = crate::BitReader<FCSODISR_A>;
#[doc = "Frozen channel status when holistic output disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCSODISR_A {
    #[doc = "0: CxOUT/CxCOUT outputs are disabled"]
    Disabled = 0,
    #[doc = "1: CxOUT/CxCOUT outputs are enabled"]
    Enabled = 1,
}
impl From<FCSODISR_A> for bool {
    #[inline(always)]
    fn from(variant: FCSODISR_A) -> Self {
        variant as u8 != 0
    }
}
impl FCSODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCSODISR_A {
        match self.bits {
            false => FCSODISR_A::Disabled,
            true => FCSODISR_A::Enabled,
        }
    }
    #[doc = "CxOUT/CxCOUT outputs are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FCSODISR_A::Disabled
    }
    #[doc = "CxOUT/CxCOUT outputs are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FCSODISR_A::Enabled
    }
}
#[doc = "Frozen channel status when holistic output disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCSODISW_AW {
    #[doc = "0: CxOUT/CxCOUT outputs disable"]
    Disable = 0,
    #[doc = "1: CxOUT/CxCOUT outputs enable"]
    Enable = 1,
}
impl From<FCSODISW_AW> for bool {
    #[inline(always)]
    fn from(variant: FCSODISW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCSODIS` writer - Frozen channel status when holistic output disable"]
pub type FCSODIS_W<'a, REG> = crate::BitWriter<'a, REG, FCSODISW_AW>;
impl<'a, REG> FCSODIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxOUT/CxCOUT outputs disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FCSODISW_AW::Disable)
    }
    #[doc = "CxOUT/CxCOUT outputs enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FCSODISW_AW::Enable)
    }
}
#[doc = "Field `FCSOEN` reader - Frozen channel status when holistic output enable"]
pub type FCSOEN_R = crate::BitReader<FCSOENR_A>;
#[doc = "Frozen channel status when holistic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCSOENR_A {
    #[doc = "0: CxOUT/CxCOUT outputs are disabled"]
    Disabled = 0,
    #[doc = "1: CxOUT/CxCOUT outputs are enabled"]
    Enabled = 1,
}
impl From<FCSOENR_A> for bool {
    #[inline(always)]
    fn from(variant: FCSOENR_A) -> Self {
        variant as u8 != 0
    }
}
impl FCSOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCSOENR_A {
        match self.bits {
            false => FCSOENR_A::Disabled,
            true => FCSOENR_A::Enabled,
        }
    }
    #[doc = "CxOUT/CxCOUT outputs are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FCSOENR_A::Disabled
    }
    #[doc = "CxOUT/CxCOUT outputs are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FCSOENR_A::Enabled
    }
}
#[doc = "Frozen channel status when holistic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCSOENW_AW {
    #[doc = "0: CxOUT/CxCOUT outputs disable"]
    Disable = 0,
    #[doc = "1: CxOUT/CxCOUT outputs enable"]
    Enable = 1,
}
impl From<FCSOENW_AW> for bool {
    #[inline(always)]
    fn from(variant: FCSOENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCSOEN` writer - Frozen channel status when holistic output enable"]
pub type FCSOEN_W<'a, REG> = crate::BitWriter<'a, REG, FCSOENW_AW>;
impl<'a, REG> FCSOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxOUT/CxCOUT outputs disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FCSOENW_AW::Disable)
    }
    #[doc = "CxOUT/CxCOUT outputs enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FCSOENW_AW::Enable)
    }
}
#[doc = "Field `BRKEN` reader - Brake enable"]
pub type BRKEN_R = crate::BitReader<BRKENR_A>;
#[doc = "Brake enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKENR_A {
    #[doc = "0: Break input is disabled"]
    Disabled = 0,
    #[doc = "1: Break input is enabled"]
    Enabled = 1,
}
impl From<BRKENR_A> for bool {
    #[inline(always)]
    fn from(variant: BRKENR_A) -> Self {
        variant as u8 != 0
    }
}
impl BRKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRKENR_A {
        match self.bits {
            false => BRKENR_A::Disabled,
            true => BRKENR_A::Enabled,
        }
    }
    #[doc = "Break input is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRKENR_A::Disabled
    }
    #[doc = "Break input is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRKENR_A::Enabled
    }
}
#[doc = "Brake enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKENW_AW {
    #[doc = "0: Break input disable"]
    Disable = 0,
    #[doc = "1: Break input enable"]
    Enable = 1,
}
impl From<BRKENW_AW> for bool {
    #[inline(always)]
    fn from(variant: BRKENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKEN` writer - Brake enable"]
pub type BRKEN_W<'a, REG> = crate::BitWriter<'a, REG, BRKENW_AW>;
impl<'a, REG> BRKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BRKENW_AW::Disable)
    }
    #[doc = "Break input enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BRKENW_AW::Enable)
    }
}
#[doc = "Field `BRKV` reader - Brake input validity"]
pub type BRKV_R = crate::BitReader<BRKV_A>;
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
#[doc = "Field `AOEN` reader - Automatic output enable"]
pub type AOEN_R = crate::BitReader<AOENR_A>;
#[doc = "Automatic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOENR_A {
    #[doc = "0: Automatic output is disabled"]
    Disabled = 0,
    #[doc = "1: Automatic output is enabled"]
    Enabled = 1,
}
impl From<AOENR_A> for bool {
    #[inline(always)]
    fn from(variant: AOENR_A) -> Self {
        variant as u8 != 0
    }
}
impl AOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AOENR_A {
        match self.bits {
            false => AOENR_A::Disabled,
            true => AOENR_A::Enabled,
        }
    }
    #[doc = "Automatic output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AOENR_A::Disabled
    }
    #[doc = "Automatic output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AOENR_A::Enabled
    }
}
#[doc = "Automatic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOENW_AW {
    #[doc = "0: Automatic output disable"]
    Disable = 0,
    #[doc = "1: Automatic output enable"]
    Enable = 1,
}
impl From<AOENW_AW> for bool {
    #[inline(always)]
    fn from(variant: AOENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOEN` writer - Automatic output enable"]
pub type AOEN_W<'a, REG> = crate::BitWriter<'a, REG, AOENW_AW>;
impl<'a, REG> AOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AOENW_AW::Disable)
    }
    #[doc = "Automatic output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AOENW_AW::Enable)
    }
}
#[doc = "Field `OEN` reader - Output enable"]
pub type OEN_R = crate::BitReader<OENR_A>;
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OENR_A {
    #[doc = "0: Output is disabled"]
    Disabled = 0,
    #[doc = "1: Output is enabled"]
    Enabled = 1,
}
impl From<OENR_A> for bool {
    #[inline(always)]
    fn from(variant: OENR_A) -> Self {
        variant as u8 != 0
    }
}
impl OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OENR_A {
        match self.bits {
            false => OENR_A::Disabled,
            true => OENR_A::Enabled,
        }
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OENR_A::Disabled
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OENR_A::Enabled
    }
}
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OENW_AW {
    #[doc = "0: Output disable"]
    Disable = 0,
    #[doc = "1: Output enable"]
    Enable = 1,
}
impl From<OENW_AW> for bool {
    #[inline(always)]
    fn from(variant: OENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEN` writer - Output enable"]
pub type OEN_W<'a, REG> = crate::BitWriter<'a, REG, OENW_AW>;
impl<'a, REG> OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OENW_AW::Disable)
    }
    #[doc = "Output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OENW_AW::Enable)
    }
}
#[doc = "Field `BKF` reader - Brake input filter"]
pub type BKF_R = crate::FieldReader;
#[doc = "Field `BKF` writer - Brake input filter"]
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 16:19 - Brake input filter"]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRK")
            .field("bkf", &format_args!("{}", self.bkf().bits()))
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
        self.read().fmt(f)
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
    #[doc = "Bits 16:19 - Brake input filter"]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<BRK_SPEC> {
        BKF_W::new(self, 16)
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
#[doc = "Brake register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRK_SPEC;
impl crate::RegisterSpec for BRK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brk::R`](R) reader structure"]
impl crate::Readable for BRK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brk::W`](W) writer structure"]
impl crate::Writable for BRK_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRK to value 0"]
impl crate::Resettable for BRK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
