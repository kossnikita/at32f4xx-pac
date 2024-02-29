#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chenr {
    #[doc = "0: Channel is disabled"]
    Disabled = 0,
    #[doc = "1: Channel is enabled"]
    Enabled = 1,
}
impl From<Chenr> for bool {
    #[inline(always)]
    fn from(variant: Chenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN` reader - Channel enable"]
pub type CHEN_R = crate::BitReader<Chenr>;
impl CHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chenr {
        match self.bits {
            false => Chenr::Disabled,
            true => Chenr::Enabled,
        }
    }
    #[doc = "Channel is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Chenr::Disabled
    }
    #[doc = "Channel is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Chenr::Enabled
    }
}
#[doc = "Channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChenwWO {
    #[doc = "0: Channel disable"]
    Disable = 0,
    #[doc = "1: Channel enable"]
    Enable = 1,
}
impl From<ChenwWO> for bool {
    #[inline(always)]
    fn from(variant: ChenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN` writer - Channel enable"]
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG, ChenwWO>;
impl<'a, REG> CHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ChenwWO::Disable)
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ChenwWO::Enable)
    }
}
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdtienr {
    #[doc = "0: Transfer complete interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Transfer complete interrupt is enabled"]
    Enabled = 1,
}
impl From<Fdtienr> for bool {
    #[inline(always)]
    fn from(variant: Fdtienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDTIEN` reader - Transfer complete interrupt enable"]
pub type FDTIEN_R = crate::BitReader<Fdtienr>;
impl FDTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdtienr {
        match self.bits {
            false => Fdtienr::Disabled,
            true => Fdtienr::Enabled,
        }
    }
    #[doc = "Transfer complete interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fdtienr::Disabled
    }
    #[doc = "Transfer complete interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fdtienr::Enabled
    }
}
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FdtienwWO {
    #[doc = "0: Transfer complete interrupt disable"]
    Disable = 0,
    #[doc = "1: Transfer complete interrupt enable"]
    Enable = 1,
}
impl From<FdtienwWO> for bool {
    #[inline(always)]
    fn from(variant: FdtienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDTIEN` writer - Transfer complete interrupt enable"]
pub type FDTIEN_W<'a, REG> = crate::BitWriter<'a, REG, FdtienwWO>;
impl<'a, REG> FDTIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer complete interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FdtienwWO::Disable)
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FdtienwWO::Enable)
    }
}
#[doc = "Half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdtienr {
    #[doc = "0: Half-transfer interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Half-transfer interrupt is enabled"]
    Enabled = 1,
}
impl From<Hdtienr> for bool {
    #[inline(always)]
    fn from(variant: Hdtienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDTIEN` reader - Half transfer interrupt enable"]
pub type HDTIEN_R = crate::BitReader<Hdtienr>;
impl HDTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdtienr {
        match self.bits {
            false => Hdtienr::Disabled,
            true => Hdtienr::Enabled,
        }
    }
    #[doc = "Half-transfer interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hdtienr::Disabled
    }
    #[doc = "Half-transfer interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hdtienr::Enabled
    }
}
#[doc = "Half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdtienwWO {
    #[doc = "0: Half-transfer interrupt disable"]
    Disable = 0,
    #[doc = "1: Half-transfer interrupt enable"]
    Enable = 1,
}
impl From<HdtienwWO> for bool {
    #[inline(always)]
    fn from(variant: HdtienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDTIEN` writer - Half transfer interrupt enable"]
pub type HDTIEN_W<'a, REG> = crate::BitWriter<'a, REG, HdtienwWO>;
impl<'a, REG> HDTIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half-transfer interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HdtienwWO::Disable)
    }
    #[doc = "Half-transfer interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HdtienwWO::Enable)
    }
}
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dterrienr {
    #[doc = "0: Data transfer error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Data transfer error interrupt is enabled"]
    Enabled = 1,
}
impl From<Dterrienr> for bool {
    #[inline(always)]
    fn from(variant: Dterrienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTERRIEN` reader - Transfer error interrupt enable"]
pub type DTERRIEN_R = crate::BitReader<Dterrienr>;
impl DTERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dterrienr {
        match self.bits {
            false => Dterrienr::Disabled,
            true => Dterrienr::Enabled,
        }
    }
    #[doc = "Data transfer error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dterrienr::Disabled
    }
    #[doc = "Data transfer error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dterrienr::Enabled
    }
}
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DterrienwWO {
    #[doc = "0: Data transfer error interrupt disable"]
    Disable = 0,
    #[doc = "1: Data transfer error interrupt enable"]
    Enable = 1,
}
impl From<DterrienwWO> for bool {
    #[inline(always)]
    fn from(variant: DterrienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTERRIEN` writer - Transfer error interrupt enable"]
pub type DTERRIEN_W<'a, REG> = crate::BitWriter<'a, REG, DterrienwWO>;
impl<'a, REG> DTERRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transfer error interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DterrienwWO::Disable)
    }
    #[doc = "Data transfer error interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DterrienwWO::Enable)
    }
}
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
#[doc = "Field `DTD` reader - Data transfer direction"]
pub type DTD_R = crate::BitReader<DTD_A>;
impl DTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTD_A {
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
pub type DTD_W<'a, REG> = crate::BitWriter<'a, REG, DTD_A>;
impl<'a, REG> DTD_W<'a, REG>
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
#[doc = "Loop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lmr {
    #[doc = "0: Circular mode is disabled"]
    Disabled = 0,
    #[doc = "1: Circular mode is enabled"]
    Enabled = 1,
}
impl From<Lmr> for bool {
    #[inline(always)]
    fn from(variant: Lmr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LM` reader - Loop mode"]
pub type LM_R = crate::BitReader<Lmr>;
impl LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lmr {
        match self.bits {
            false => Lmr::Disabled,
            true => Lmr::Enabled,
        }
    }
    #[doc = "Circular mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lmr::Disabled
    }
    #[doc = "Circular mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lmr::Enabled
    }
}
#[doc = "Loop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LmwWO {
    #[doc = "0: Circular mode disable"]
    Disable = 0,
    #[doc = "1: Circular mode enable"]
    Enable = 1,
}
impl From<LmwWO> for bool {
    #[inline(always)]
    fn from(variant: LmwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LM` writer - Loop mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG, LmwWO>;
impl<'a, REG> LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Circular mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LmwWO::Disable)
    }
    #[doc = "Circular mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LmwWO::Enable)
    }
}
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pincmr {
    #[doc = "0: Peripheral address increment is disabled"]
    Disabled = 0,
    #[doc = "1: Peripheral address increment is enabled"]
    Enabled = 1,
}
impl From<Pincmr> for bool {
    #[inline(always)]
    fn from(variant: Pincmr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM` reader - Peripheral increment mode"]
pub type PINCM_R = crate::BitReader<Pincmr>;
impl PINCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pincmr {
        match self.bits {
            false => Pincmr::Disabled,
            true => Pincmr::Enabled,
        }
    }
    #[doc = "Peripheral address increment is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pincmr::Disabled
    }
    #[doc = "Peripheral address increment is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pincmr::Enabled
    }
}
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PincmwWO {
    #[doc = "0: Peripheral address increment disable"]
    Disable = 0,
    #[doc = "1: Peripheral address increment enable"]
    Enable = 1,
}
impl From<PincmwWO> for bool {
    #[inline(always)]
    fn from(variant: PincmwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCM` writer - Peripheral increment mode"]
pub type PINCM_W<'a, REG> = crate::BitWriter<'a, REG, PincmwWO>;
impl<'a, REG> PINCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral address increment disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmwWO::Disable)
    }
    #[doc = "Peripheral address increment enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PincmwWO::Enable)
    }
}
#[doc = "Memory increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mincmr {
    #[doc = "0: Memory address increment is disabled"]
    Disabled = 0,
    #[doc = "1: Memory address increment is enabled"]
    Enabled = 1,
}
impl From<Mincmr> for bool {
    #[inline(always)]
    fn from(variant: Mincmr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MINCM` reader - Memory increment mode"]
pub type MINCM_R = crate::BitReader<Mincmr>;
impl MINCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mincmr {
        match self.bits {
            false => Mincmr::Disabled,
            true => Mincmr::Enabled,
        }
    }
    #[doc = "Memory address increment is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mincmr::Disabled
    }
    #[doc = "Memory address increment is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mincmr::Enabled
    }
}
#[doc = "Memory increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MincmwWO {
    #[doc = "0: Memory address increment disable"]
    Disable = 0,
    #[doc = "1: Memory address increment enable"]
    Enable = 1,
}
impl From<MincmwWO> for bool {
    #[inline(always)]
    fn from(variant: MincmwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MINCM` writer - Memory increment mode"]
pub type MINCM_W<'a, REG> = crate::BitWriter<'a, REG, MincmwWO>;
impl<'a, REG> MINCM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory address increment disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MincmwWO::Disable)
    }
    #[doc = "Memory address increment enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MincmwWO::Enable)
    }
}
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
#[doc = "Field `PWIDTH` reader - Peripheral data bit width"]
pub type PWIDTH_R = crate::FieldReader<PWIDTH_A>;
impl PWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWIDTH_A> {
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
pub type PWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWIDTH_A>;
impl<'a, REG> PWIDTH_W<'a, REG>
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
#[doc = "Field `MWIDTH` reader - Memory data bit width"]
pub type MWIDTH_R = crate::FieldReader<MWIDTH_A>;
impl MWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MWIDTH_A> {
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
pub type MWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MWIDTH_A>;
impl<'a, REG> MWIDTH_W<'a, REG>
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
#[doc = "Field `CHPL` reader - Channel Priority level"]
pub type CHPL_R = crate::FieldReader<CHPL_A>;
impl CHPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHPL_A {
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
pub type CHPL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CHPL_A>;
impl<'a, REG> CHPL_W<'a, REG>
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
#[doc = "Memory to memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M2mr {
    #[doc = "0: Memory to memory mode is disabled"]
    Disabled = 0,
    #[doc = "1: Memory to memory mode is enabled"]
    Enabled = 1,
}
impl From<M2mr> for bool {
    #[inline(always)]
    fn from(variant: M2mr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M2M` reader - Memory to memory mode"]
pub type M2M_R = crate::BitReader<M2mr>;
impl M2M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M2mr {
        match self.bits {
            false => M2mr::Disabled,
            true => M2mr::Enabled,
        }
    }
    #[doc = "Memory to memory mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M2mr::Disabled
    }
    #[doc = "Memory to memory mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M2mr::Enabled
    }
}
#[doc = "Memory to memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M2mwWO {
    #[doc = "0: Memory to memory mode disable"]
    Disable = 0,
    #[doc = "1: Memory to memory mode enable"]
    Enable = 1,
}
impl From<M2mwWO> for bool {
    #[inline(always)]
    fn from(variant: M2mwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M2M` writer - Memory to memory mode"]
pub type M2M_W<'a, REG> = crate::BitWriter<'a, REG, M2mwWO>;
impl<'a, REG> M2M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory to memory mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(M2mwWO::Disable)
    }
    #[doc = "Memory to memory mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(M2mwWO::Enable)
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CTRL_SPEC> {
        CHEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdtien(&mut self) -> FDTIEN_W<CTRL_SPEC> {
        FDTIEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdtien(&mut self) -> HDTIEN_W<CTRL_SPEC> {
        HDTIEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterrien(&mut self) -> DTERRIEN_W<CTRL_SPEC> {
        DTERRIEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DTD_W<CTRL_SPEC> {
        DTD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<CTRL_SPEC> {
        LM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pincm(&mut self) -> PINCM_W<CTRL_SPEC> {
        PINCM_W::new(self, 6)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn mincm(&mut self) -> MINCM_W<CTRL_SPEC> {
        MINCM_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Peripheral data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<CTRL_SPEC> {
        PWIDTH_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Memory data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<CTRL_SPEC> {
        MWIDTH_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn chpl(&mut self) -> CHPL_W<CTRL_SPEC> {
        CHPL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2M_W<CTRL_SPEC> {
        M2M_W::new(self, 14)
    }
}
#[doc = "DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
