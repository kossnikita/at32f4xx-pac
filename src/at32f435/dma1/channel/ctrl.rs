#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type CHEN_R = crate::BitReader<CHENR_A>;
#[doc = "Channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHENR_A {
    #[doc = "0: Channel is disabled"]
    Disabled = 0,
    #[doc = "1: Channel is enabled"]
    Enabled = 1,
}
impl From<CHENR_A> for bool {
    #[inline(always)]
    fn from(variant: CHENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHENR_A {
        match self.bits {
            false => CHENR_A::Disabled,
            true => CHENR_A::Enabled,
        }
    }
    #[doc = "Channel is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHENR_A::Disabled
    }
    #[doc = "Channel is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHENR_A::Enabled
    }
}
#[doc = "Channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHENW_AW {
    #[doc = "0: Channel disable"]
    Disable = 0,
    #[doc = "1: Channel enable"]
    Enable = 1,
}
impl From<CHENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CHENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN` writer - Channel enable"]
pub type CHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CHENW_AW>;
impl<'a, REG, const O: u8> CHEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CHENW_AW::Disable)
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CHENW_AW::Enable)
    }
}
#[doc = "Field `FDTIEN` reader - Transfer complete interrupt enable"]
pub type FDTIEN_R = crate::BitReader<FDTIENR_A>;
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDTIENR_A {
    #[doc = "0: Transfer complete interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Transfer complete interrupt is enabled"]
    Enabled = 1,
}
impl From<FDTIENR_A> for bool {
    #[inline(always)]
    fn from(variant: FDTIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl FDTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDTIENR_A {
        match self.bits {
            false => FDTIENR_A::Disabled,
            true => FDTIENR_A::Enabled,
        }
    }
    #[doc = "Transfer complete interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FDTIENR_A::Disabled
    }
    #[doc = "Transfer complete interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FDTIENR_A::Enabled
    }
}
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDTIENW_AW {
    #[doc = "0: Transfer complete interrupt disable"]
    Disable = 0,
    #[doc = "1: Transfer complete interrupt enable"]
    Enable = 1,
}
impl From<FDTIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: FDTIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDTIEN` writer - Transfer complete interrupt enable"]
pub type FDTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FDTIENW_AW>;
impl<'a, REG, const O: u8> FDTIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer complete interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FDTIENW_AW::Disable)
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FDTIENW_AW::Enable)
    }
}
#[doc = "Field `HDTIEN` reader - Half transfer interrupt enable"]
pub type HDTIEN_R = crate::BitReader<HDTIENR_A>;
#[doc = "Half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDTIENR_A {
    #[doc = "0: Half-transfer interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Half-transfer interrupt is enabled"]
    Enabled = 1,
}
impl From<HDTIENR_A> for bool {
    #[inline(always)]
    fn from(variant: HDTIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl HDTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDTIENR_A {
        match self.bits {
            false => HDTIENR_A::Disabled,
            true => HDTIENR_A::Enabled,
        }
    }
    #[doc = "Half-transfer interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HDTIENR_A::Disabled
    }
    #[doc = "Half-transfer interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HDTIENR_A::Enabled
    }
}
#[doc = "Half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDTIENW_AW {
    #[doc = "0: Half-transfer interrupt disable"]
    Disable = 0,
    #[doc = "1: Half-transfer interrupt enable"]
    Enable = 1,
}
impl From<HDTIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: HDTIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDTIEN` writer - Half transfer interrupt enable"]
pub type HDTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HDTIENW_AW>;
impl<'a, REG, const O: u8> HDTIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-transfer interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HDTIENW_AW::Disable)
    }
    #[doc = "Half-transfer interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HDTIENW_AW::Enable)
    }
}
#[doc = "Field `DTERRIEN` reader - Transfer error interrupt enable"]
pub type DTERRIEN_R = crate::BitReader<DTERRIENR_A>;
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTERRIENR_A {
    #[doc = "0: Data transfer error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Data transfer error interrupt is enabled"]
    Enabled = 1,
}
impl From<DTERRIENR_A> for bool {
    #[inline(always)]
    fn from(variant: DTERRIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl DTERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTERRIENR_A {
        match self.bits {
            false => DTERRIENR_A::Disabled,
            true => DTERRIENR_A::Enabled,
        }
    }
    #[doc = "Data transfer error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTERRIENR_A::Disabled
    }
    #[doc = "Data transfer error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTERRIENR_A::Enabled
    }
}
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTERRIENW_AW {
    #[doc = "0: Data transfer error interrupt disable"]
    Disable = 0,
    #[doc = "1: Data transfer error interrupt enable"]
    Enable = 1,
}
impl From<DTERRIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: DTERRIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTERRIEN` writer - Transfer error interrupt enable"]
pub type DTERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTERRIENW_AW>;
impl<'a, REG, const O: u8> DTERRIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transfer error interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DTERRIENW_AW::Disable)
    }
    #[doc = "Data transfer error interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DTERRIENW_AW::Enable)
    }
}
#[doc = "Field `DTD` reader - Data transfer direction"]
pub type DTD_R = crate::BitReader<DTD_A>;
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTD_A {
    #[doc = "0: Read from peripherals"]
    Peripherals = 0,
    #[doc = "1: Read from memory"]
    Memory = 1,
}
impl From<DTD_A> for bool {
    #[inline(always)]
    fn from(variant: DTD_A) -> Self {
        variant as u8 != 0
    }
}
impl DTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTD_A {
        match self.bits {
            false => DTD_A::Peripherals,
            true => DTD_A::Memory,
        }
    }
    #[doc = "Read from peripherals"]
    #[inline(always)]
    pub fn is_peripherals(&self) -> bool {
        *self == DTD_A::Peripherals
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == DTD_A::Memory
    }
}
#[doc = "Field `DTD` writer - Data transfer direction"]
pub type DTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DTD_A>;
impl<'a, REG, const O: u8> DTD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from peripherals"]
    #[inline(always)]
    pub fn peripherals(self) -> &'a mut crate::W<REG> {
        self.variant(DTD_A::Peripherals)
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(DTD_A::Memory)
    }
}
#[doc = "Field `LM` reader - Loop mode"]
pub type LM_R = crate::BitReader<LMR_A>;
#[doc = "Loop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LMR_A {
    #[doc = "0: Circular mode is disabled"]
    Disabled = 0,
    #[doc = "1: Circular mode is enabled"]
    Enabled = 1,
}
impl From<LMR_A> for bool {
    #[inline(always)]
    fn from(variant: LMR_A) -> Self {
        variant as u8 != 0
    }
}
impl LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMR_A {
        match self.bits {
            false => LMR_A::Disabled,
            true => LMR_A::Enabled,
        }
    }
    #[doc = "Circular mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LMR_A::Disabled
    }
    #[doc = "Circular mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LMR_A::Enabled
    }
}
#[doc = "Loop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LMW_AW {
    #[doc = "0: Circular mode disable"]
    Disable = 0,
    #[doc = "1: Circular mode enable"]
    Enable = 1,
}
impl From<LMW_AW> for bool {
    #[inline(always)]
    fn from(variant: LMW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LM` writer - Loop mode"]
pub type LM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LMW_AW>;
impl<'a, REG, const O: u8> LM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Circular mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LMW_AW::Disable)
    }
    #[doc = "Circular mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LMW_AW::Enable)
    }
}
#[doc = "Field `PINCM` reader - Peripheral increment mode"]
pub type PINCM_R = crate::BitReader<PINCMR_A>;
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINCMR_A {
    #[doc = "0: Peripheral address increment is disabled"]
    Disabled = 0,
    #[doc = "1: Peripheral address increment is enabled"]
    Enabled = 1,
}
impl From<PINCMR_A> for bool {
    #[inline(always)]
    fn from(variant: PINCMR_A) -> Self {
        variant as u8 != 0
    }
}
impl PINCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCMR_A {
        match self.bits {
            false => PINCMR_A::Disabled,
            true => PINCMR_A::Enabled,
        }
    }
    #[doc = "Peripheral address increment is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINCMR_A::Disabled
    }
    #[doc = "Peripheral address increment is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINCMR_A::Enabled
    }
}
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINCMW_AW {
    #[doc = "0: Peripheral address increment disable"]
    Disable = 0,
    #[doc = "1: Peripheral address increment enable"]
    Enable = 1,
}
impl From<PINCMW_AW> for bool {
    #[inline(always)]
    fn from(variant: PINCMW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM` writer - Peripheral increment mode"]
pub type PINCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PINCMW_AW>;
impl<'a, REG, const O: u8> PINCM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral address increment disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PINCMW_AW::Disable)
    }
    #[doc = "Peripheral address increment enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PINCMW_AW::Enable)
    }
}
#[doc = "Field `MINCM` reader - Memory increment mode"]
pub type MINCM_R = crate::BitReader<MINCMR_A>;
#[doc = "Memory increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINCMR_A {
    #[doc = "0: Memory address increment is disabled"]
    Disabled = 0,
    #[doc = "1: Memory address increment is enabled"]
    Enabled = 1,
}
impl From<MINCMR_A> for bool {
    #[inline(always)]
    fn from(variant: MINCMR_A) -> Self {
        variant as u8 != 0
    }
}
impl MINCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINCMR_A {
        match self.bits {
            false => MINCMR_A::Disabled,
            true => MINCMR_A::Enabled,
        }
    }
    #[doc = "Memory address increment is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MINCMR_A::Disabled
    }
    #[doc = "Memory address increment is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MINCMR_A::Enabled
    }
}
#[doc = "Memory increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINCMW_AW {
    #[doc = "0: Memory address increment disable"]
    Disable = 0,
    #[doc = "1: Memory address increment enable"]
    Enable = 1,
}
impl From<MINCMW_AW> for bool {
    #[inline(always)]
    fn from(variant: MINCMW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MINCM` writer - Memory increment mode"]
pub type MINCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MINCMW_AW>;
impl<'a, REG, const O: u8> MINCM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory address increment disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MINCMW_AW::Disable)
    }
    #[doc = "Memory address increment enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MINCMW_AW::Enable)
    }
}
#[doc = "Field `PWIDTH` reader - Peripheral data bit width"]
pub type PWIDTH_R = crate::FieldReader<PWIDTH_A>;
#[doc = "Peripheral data bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWIDTH_A {
    #[doc = "0: 8 bits"]
    Bit8 = 0,
    #[doc = "1: 16 bits"]
    Bit16 = 1,
    #[doc = "2: 32 bits"]
    Bit32 = 2,
}
impl From<PWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: PWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWIDTH_A {
    type Ux = u8;
}
impl PWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWIDTH_A> {
        match self.bits {
            0 => Some(PWIDTH_A::Bit8),
            1 => Some(PWIDTH_A::Bit16),
            2 => Some(PWIDTH_A::Bit32),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == PWIDTH_A::Bit8
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == PWIDTH_A::Bit16
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == PWIDTH_A::Bit32
    }
}
#[doc = "Field `PWIDTH` writer - Peripheral data bit width"]
pub type PWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, PWIDTH_A>;
impl<'a, REG, const O: u8> PWIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(PWIDTH_A::Bit8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(PWIDTH_A::Bit16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(PWIDTH_A::Bit32)
    }
}
#[doc = "Field `MWIDTH` reader - Memory data bit width"]
pub type MWIDTH_R = crate::FieldReader<MWIDTH_A>;
#[doc = "Memory data bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MWIDTH_A {
    #[doc = "0: 8 bits"]
    Bit8 = 0,
    #[doc = "1: 16 bits"]
    Bit16 = 1,
    #[doc = "2: 32 bits"]
    Bit32 = 2,
}
impl From<MWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: MWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MWIDTH_A {
    type Ux = u8;
}
impl MWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MWIDTH_A> {
        match self.bits {
            0 => Some(MWIDTH_A::Bit8),
            1 => Some(MWIDTH_A::Bit16),
            2 => Some(MWIDTH_A::Bit32),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == MWIDTH_A::Bit8
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == MWIDTH_A::Bit16
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == MWIDTH_A::Bit32
    }
}
#[doc = "Field `MWIDTH` writer - Memory data bit width"]
pub type MWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, MWIDTH_A>;
impl<'a, REG, const O: u8> MWIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(MWIDTH_A::Bit8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(MWIDTH_A::Bit16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(MWIDTH_A::Bit32)
    }
}
#[doc = "Field `CHPL` reader - Channel Priority level"]
pub type CHPL_R = crate::FieldReader<CHPL_A>;
#[doc = "Channel Priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHPL_A {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: Medium"]
    Medium = 1,
    #[doc = "2: High"]
    High = 2,
    #[doc = "3: VeryHigh"]
    VeryHigh = 3,
}
impl From<CHPL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHPL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHPL_A {
    type Ux = u8;
}
impl CHPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHPL_A {
        match self.bits {
            0 => CHPL_A::Low,
            1 => CHPL_A::Medium,
            2 => CHPL_A::High,
            3 => CHPL_A::VeryHigh,
            _ => unreachable!(),
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CHPL_A::Low
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == CHPL_A::Medium
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CHPL_A::High
    }
    #[doc = "VeryHigh"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == CHPL_A::VeryHigh
    }
}
#[doc = "Field `CHPL` writer - Channel Priority level"]
pub type CHPL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CHPL_A>;
impl<'a, REG, const O: u8> CHPL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CHPL_A::Low)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(CHPL_A::Medium)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CHPL_A::High)
    }
    #[doc = "VeryHigh"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut crate::W<REG> {
        self.variant(CHPL_A::VeryHigh)
    }
}
#[doc = "Field `M2M` reader - Memory to memory mode"]
pub type M2M_R = crate::BitReader<M2MR_A>;
#[doc = "Memory to memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M2MR_A {
    #[doc = "0: Memory to memory mode is disabled"]
    Disabled = 0,
    #[doc = "1: Memory to memory mode is enabled"]
    Enabled = 1,
}
impl From<M2MR_A> for bool {
    #[inline(always)]
    fn from(variant: M2MR_A) -> Self {
        variant as u8 != 0
    }
}
impl M2M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2MR_A {
        match self.bits {
            false => M2MR_A::Disabled,
            true => M2MR_A::Enabled,
        }
    }
    #[doc = "Memory to memory mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M2MR_A::Disabled
    }
    #[doc = "Memory to memory mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M2MR_A::Enabled
    }
}
#[doc = "Memory to memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M2MW_AW {
    #[doc = "0: Memory to memory mode disable"]
    Disable = 0,
    #[doc = "1: Memory to memory mode enable"]
    Enable = 1,
}
impl From<M2MW_AW> for bool {
    #[inline(always)]
    fn from(variant: M2MW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M2M` writer - Memory to memory mode"]
pub type M2M_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, M2MW_AW>;
impl<'a, REG, const O: u8> M2M_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory to memory mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(M2MW_AW::Disable)
    }
    #[doc = "Memory to memory mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(M2MW_AW::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn fdtien(&self) -> FDTIEN_R {
        FDTIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn hdtien(&self) -> HDTIEN_R {
        HDTIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn dterrien(&self) -> DTERRIEN_R {
        DTERRIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dtd(&self) -> DTD_R {
        DTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Loop mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pincm(&self) -> PINCM_R {
        PINCM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    pub fn mincm(&self) -> MINCM_R {
        MINCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral data bit width"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory data bit width"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    pub fn chpl(&self) -> CHPL_R {
        CHPL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2M_R {
        M2M_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("chen", &format_args!("{}", self.chen().bit()))
            .field("fdtien", &format_args!("{}", self.fdtien().bit()))
            .field("hdtien", &format_args!("{}", self.hdtien().bit()))
            .field("dterrien", &format_args!("{}", self.dterrien().bit()))
            .field("dtd", &format_args!("{}", self.dtd().bit()))
            .field("lm", &format_args!("{}", self.lm().bit()))
            .field("pincm", &format_args!("{}", self.pincm().bit()))
            .field("mincm", &format_args!("{}", self.mincm().bit()))
            .field("pwidth", &format_args!("{}", self.pwidth().bits()))
            .field("mwidth", &format_args!("{}", self.mwidth().bits()))
            .field("chpl", &format_args!("{}", self.chpl().bits()))
            .field("m2m", &format_args!("{}", self.m2m().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CTRL_SPEC, 0> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdtien(&mut self) -> FDTIEN_W<CTRL_SPEC, 1> {
        FDTIEN_W::new(self)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdtien(&mut self) -> HDTIEN_W<CTRL_SPEC, 2> {
        HDTIEN_W::new(self)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterrien(&mut self) -> DTERRIEN_W<CTRL_SPEC, 3> {
        DTERRIEN_W::new(self)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DTD_W<CTRL_SPEC, 4> {
        DTD_W::new(self)
    }
    #[doc = "Bit 5 - Loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<CTRL_SPEC, 5> {
        LM_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pincm(&mut self) -> PINCM_W<CTRL_SPEC, 6> {
        PINCM_W::new(self)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn mincm(&mut self) -> MINCM_W<CTRL_SPEC, 7> {
        MINCM_W::new(self)
    }
    #[doc = "Bits 8:9 - Peripheral data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<CTRL_SPEC, 8> {
        PWIDTH_W::new(self)
    }
    #[doc = "Bits 10:11 - Memory data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<CTRL_SPEC, 10> {
        MWIDTH_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn chpl(&mut self) -> CHPL_W<CTRL_SPEC, 12> {
        CHPL_W::new(self)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2M_W<CTRL_SPEC, 14> {
        M2M_W::new(self)
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
#[doc = "DMA channel %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
