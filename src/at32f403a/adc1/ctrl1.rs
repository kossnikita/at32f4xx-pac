#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `VMCSEL` reader - Voltage monitoring channel select"]
pub type VMCSEL_R = crate::FieldReader;
#[doc = "Field `VMCSEL` writer - Voltage monitoring channel select"]
pub type VMCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CCEIEN` reader - Channel conversion end interrupt enable"]
pub type CCEIEN_R = crate::BitReader<CCEIENR_A>;
#[doc = "Channel conversion end interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEIENR_A {
    #[doc = "0: Channel conversion end interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Channel conversion end interrupt is enabled"]
    Enabled = 1,
}
impl From<CCEIENR_A> for bool {
    #[inline(always)]
    fn from(variant: CCEIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CCEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCEIENR_A {
        match self.bits {
            false => CCEIENR_A::Disabled,
            true => CCEIENR_A::Enabled,
        }
    }
    #[doc = "Channel conversion end interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCEIENR_A::Disabled
    }
    #[doc = "Channel conversion end interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCEIENR_A::Enabled
    }
}
#[doc = "Channel conversion end interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEIENW_AW {
    #[doc = "0: Channel conversion end interrupt disable"]
    Disable = 0,
    #[doc = "1: Channel conversion end interrupt enable"]
    Enable = 1,
}
impl From<CCEIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CCEIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEIEN` writer - Channel conversion end interrupt enable"]
pub type CCEIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCEIENW_AW>;
impl<'a, REG, const O: u8> CCEIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel conversion end interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CCEIENW_AW::Disable)
    }
    #[doc = "Channel conversion end interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CCEIENW_AW::Enable)
    }
}
#[doc = "Field `VMORIEN` reader - Voltage monitoring out of range interrupt enable"]
pub type VMORIEN_R = crate::BitReader<VMORIENR_A>;
#[doc = "Voltage monitoring out of range interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMORIENR_A {
    #[doc = "0: Voltage monitoring out of range interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Voltage monitoring out of range interrupt is enabled"]
    Enabled = 1,
}
impl From<VMORIENR_A> for bool {
    #[inline(always)]
    fn from(variant: VMORIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl VMORIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMORIENR_A {
        match self.bits {
            false => VMORIENR_A::Disabled,
            true => VMORIENR_A::Enabled,
        }
    }
    #[doc = "Voltage monitoring out of range interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VMORIENR_A::Disabled
    }
    #[doc = "Voltage monitoring out of range interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VMORIENR_A::Enabled
    }
}
#[doc = "Voltage monitoring out of range interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMORIENW_AW {
    #[doc = "0: Voltage monitoring out of range interrupt disable"]
    Disable = 0,
    #[doc = "1: Voltage monitoring out of range interrupt enable"]
    Enable = 1,
}
impl From<VMORIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: VMORIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMORIEN` writer - Voltage monitoring out of range interrupt enable"]
pub type VMORIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VMORIENW_AW>;
impl<'a, REG, const O: u8> VMORIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage monitoring out of range interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VMORIENW_AW::Disable)
    }
    #[doc = "Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VMORIENW_AW::Enable)
    }
}
#[doc = "Field `PCCEIEN` reader - Conversion end interrupt enable for preempted channels"]
pub use CCEIEN_R as PCCEIEN_R;
#[doc = "Field `PCCEIEN` writer - Conversion end interrupt enable for preempted channels"]
pub use CCEIEN_W as PCCEIEN_W;
#[doc = "Field `SQEN` reader - Sequence mode enable"]
pub type SQEN_R = crate::BitReader<SQENR_A>;
#[doc = "Sequence mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQENR_A {
    #[doc = "0: Sequence mode is disabled"]
    Disabled = 0,
    #[doc = "1: Sequence mode is enabled"]
    Enabled = 1,
}
impl From<SQENR_A> for bool {
    #[inline(always)]
    fn from(variant: SQENR_A) -> Self {
        variant as u8 != 0
    }
}
impl SQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQENR_A {
        match self.bits {
            false => SQENR_A::Disabled,
            true => SQENR_A::Enabled,
        }
    }
    #[doc = "Sequence mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SQENR_A::Disabled
    }
    #[doc = "Sequence mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SQENR_A::Enabled
    }
}
#[doc = "Sequence mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SQENW_AW {
    #[doc = "0: Sequence mode disable"]
    Disable = 0,
    #[doc = "1: Sequence mode enable"]
    Enable = 1,
}
impl From<SQENW_AW> for bool {
    #[inline(always)]
    fn from(variant: SQENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQEN` writer - Sequence mode enable"]
pub type SQEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SQENW_AW>;
impl<'a, REG, const O: u8> SQEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sequence mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SQENW_AW::Disable)
    }
    #[doc = "Sequence mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SQENW_AW::Enable)
    }
}
#[doc = "Field `VMSGEN` reader - Voltage monitoring enable on a single channel"]
pub type VMSGEN_R = crate::BitReader<VMSGEN_A>;
#[doc = "Voltage monitoring enable on a single channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMSGEN_A {
    #[doc = "0: Voltage monitoring enabled on all channels"]
    All = 0,
    #[doc = "1: Voltage monitoring enabled a single channel"]
    Single = 1,
}
impl From<VMSGEN_A> for bool {
    #[inline(always)]
    fn from(variant: VMSGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VMSGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMSGEN_A {
        match self.bits {
            false => VMSGEN_A::All,
            true => VMSGEN_A::Single,
        }
    }
    #[doc = "Voltage monitoring enabled on all channels"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == VMSGEN_A::All
    }
    #[doc = "Voltage monitoring enabled a single channel"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == VMSGEN_A::Single
    }
}
#[doc = "Field `VMSGEN` writer - Voltage monitoring enable on a single channel"]
pub type VMSGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, VMSGEN_A>;
impl<'a, REG, const O: u8> VMSGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage monitoring enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(VMSGEN_A::All)
    }
    #[doc = "Voltage monitoring enabled a single channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(VMSGEN_A::Single)
    }
}
#[doc = "Field `PCAUTOEN` reader - Preempted group automatic conversion enable after ordinary group"]
pub type PCAUTOEN_R = crate::BitReader<PCAUTOENR_A>;
#[doc = "Preempted group automatic conversion enable after ordinary group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCAUTOENR_A {
    #[doc = "0: Preempted group automatic conversion is disabled"]
    Disabled = 0,
    #[doc = "1: Preempted group automatic conversion is enabled"]
    Enabled = 1,
}
impl From<PCAUTOENR_A> for bool {
    #[inline(always)]
    fn from(variant: PCAUTOENR_A) -> Self {
        variant as u8 != 0
    }
}
impl PCAUTOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCAUTOENR_A {
        match self.bits {
            false => PCAUTOENR_A::Disabled,
            true => PCAUTOENR_A::Enabled,
        }
    }
    #[doc = "Preempted group automatic conversion is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCAUTOENR_A::Disabled
    }
    #[doc = "Preempted group automatic conversion is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCAUTOENR_A::Enabled
    }
}
#[doc = "Preempted group automatic conversion enable after ordinary group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCAUTOENW_AW {
    #[doc = "0: Preempted group automatic conversion disable"]
    Disable = 0,
    #[doc = "1: Preempted group automatic conversion enable"]
    Enable = 1,
}
impl From<PCAUTOENW_AW> for bool {
    #[inline(always)]
    fn from(variant: PCAUTOENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCAUTOEN` writer - Preempted group automatic conversion enable after ordinary group"]
pub type PCAUTOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCAUTOENW_AW>;
impl<'a, REG, const O: u8> PCAUTOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preempted group automatic conversion disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PCAUTOENW_AW::Disable)
    }
    #[doc = "Preempted group automatic conversion enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PCAUTOENW_AW::Enable)
    }
}
#[doc = "Field `OCPEN` reader - Partitioned mode enable on ordinary channels"]
pub type OCPEN_R = crate::BitReader<OCPENR_A>;
#[doc = "Partitioned mode enable on ordinary channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCPENR_A {
    #[doc = "0: Partitioned mode on channels is disabled"]
    Disabled = 0,
    #[doc = "1: Partitioned mode on channels is enabled"]
    Enabled = 1,
}
impl From<OCPENR_A> for bool {
    #[inline(always)]
    fn from(variant: OCPENR_A) -> Self {
        variant as u8 != 0
    }
}
impl OCPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCPENR_A {
        match self.bits {
            false => OCPENR_A::Disabled,
            true => OCPENR_A::Enabled,
        }
    }
    #[doc = "Partitioned mode on channels is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OCPENR_A::Disabled
    }
    #[doc = "Partitioned mode on channels is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OCPENR_A::Enabled
    }
}
#[doc = "Partitioned mode enable on ordinary channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCPENW_AW {
    #[doc = "0: Partitioned mode on channels disable"]
    Disable = 0,
    #[doc = "1: Partitioned mode on channels enable"]
    Enable = 1,
}
impl From<OCPENW_AW> for bool {
    #[inline(always)]
    fn from(variant: OCPENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCPEN` writer - Partitioned mode enable on ordinary channels"]
pub type OCPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OCPENW_AW>;
impl<'a, REG, const O: u8> OCPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Partitioned mode on channels disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OCPENW_AW::Disable)
    }
    #[doc = "Partitioned mode on channels enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OCPENW_AW::Enable)
    }
}
#[doc = "Field `PCPEN` reader - Partitioned mode enable on preempted channels"]
pub use OCPEN_R as PCPEN_R;
#[doc = "Field `PCPEN` writer - Partitioned mode enable on preempted channels"]
pub use OCPEN_W as PCPEN_W;
#[doc = "Field `OCPCNT` reader - Partitioned mode conversion count of ordinary channels"]
pub type OCPCNT_R = crate::FieldReader;
#[doc = "Field `OCPCNT` writer - Partitioned mode conversion count of ordinary channels"]
pub type OCPCNT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
#[doc = "Field `MSSEL` reader - Master slave mode select"]
pub type MSSEL_R = crate::FieldReader;
#[doc = "Field `MSSEL` writer - Master slave mode select"]
pub type MSSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PCVMEN` reader - Voltage monitoring enable on preempted channels"]
pub type PCVMEN_R = crate::BitReader<PCVMENR_A>;
#[doc = "Voltage monitoring enable on preempted channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCVMENR_A {
    #[doc = "0: Voltage monitoring is disabled"]
    Disabled = 0,
    #[doc = "1: Voltage monitoring is enabled"]
    Enabled = 1,
}
impl From<PCVMENR_A> for bool {
    #[inline(always)]
    fn from(variant: PCVMENR_A) -> Self {
        variant as u8 != 0
    }
}
impl PCVMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCVMENR_A {
        match self.bits {
            false => PCVMENR_A::Disabled,
            true => PCVMENR_A::Enabled,
        }
    }
    #[doc = "Voltage monitoring is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCVMENR_A::Disabled
    }
    #[doc = "Voltage monitoring is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCVMENR_A::Enabled
    }
}
#[doc = "Voltage monitoring enable on preempted channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCVMENW_AW {
    #[doc = "0: Voltage monitoring disable"]
    Disable = 0,
    #[doc = "1: Voltage monitoring enable"]
    Enable = 1,
}
impl From<PCVMENW_AW> for bool {
    #[inline(always)]
    fn from(variant: PCVMENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCVMEN` writer - Voltage monitoring enable on preempted channels"]
pub type PCVMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCVMENW_AW>;
impl<'a, REG, const O: u8> PCVMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage monitoring disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PCVMENW_AW::Disable)
    }
    #[doc = "Voltage monitoring enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PCVMENW_AW::Enable)
    }
}
#[doc = "Field `OCVMEN` reader - Voltage monitoring enable on ordinary channels"]
pub use PCVMEN_R as OCVMEN_R;
#[doc = "Field `OCVMEN` writer - Voltage monitoring enable on ordinary channels"]
pub use PCVMEN_W as OCVMEN_W;
impl R {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    pub fn vmcsel(&self) -> VMCSEL_R {
        VMCSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CCEIEN_R {
        CCEIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    pub fn vmorien(&self) -> VMORIEN_R {
        VMORIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    pub fn pcceien(&self) -> PCCEIEN_R {
        PCCEIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    pub fn sqen(&self) -> SQEN_R {
        SQEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    pub fn vmsgen(&self) -> VMSGEN_R {
        VMSGEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    pub fn pcautoen(&self) -> PCAUTOEN_R {
        PCAUTOEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    pub fn ocpen(&self) -> OCPEN_R {
        OCPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    pub fn pcpen(&self) -> PCPEN_R {
        PCPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    pub fn ocpcnt(&self) -> OCPCNT_R {
        OCPCNT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Master slave mode select"]
    #[inline(always)]
    pub fn mssel(&self) -> MSSEL_R {
        MSSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    pub fn pcvmen(&self) -> PCVMEN_R {
        PCVMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    pub fn ocvmen(&self) -> OCVMEN_R {
        OCVMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("pcvmen", &format_args!("{}", self.pcvmen().bit()))
            .field("ocvmen", &format_args!("{}", self.ocvmen().bit()))
            .field("mssel", &format_args!("{}", self.mssel().bits()))
            .field("ocpcnt", &format_args!("{}", self.ocpcnt().bits()))
            .field("ocpen", &format_args!("{}", self.ocpen().bit()))
            .field("pcpen", &format_args!("{}", self.pcpen().bit()))
            .field("pcautoen", &format_args!("{}", self.pcautoen().bit()))
            .field("vmsgen", &format_args!("{}", self.vmsgen().bit()))
            .field("sqen", &format_args!("{}", self.sqen().bit()))
            .field("cceien", &format_args!("{}", self.cceien().bit()))
            .field("pcceien", &format_args!("{}", self.pcceien().bit()))
            .field("vmorien", &format_args!("{}", self.vmorien().bit()))
            .field("vmcsel", &format_args!("{}", self.vmcsel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    #[must_use]
    pub fn vmcsel(&mut self) -> VMCSEL_W<CTRL1_SPEC, 0> {
        VMCSEL_W::new(self)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cceien(&mut self) -> CCEIEN_W<CTRL1_SPEC, 5> {
        CCEIEN_W::new(self)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmorien(&mut self) -> VMORIEN_W<CTRL1_SPEC, 6> {
        VMORIEN_W::new(self)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcceien(&mut self) -> PCCEIEN_W<CTRL1_SPEC, 7> {
        PCCEIEN_W::new(self)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sqen(&mut self) -> SQEN_W<CTRL1_SPEC, 8> {
        SQEN_W::new(self)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn vmsgen(&mut self) -> VMSGEN_W<CTRL1_SPEC, 9> {
        VMSGEN_W::new(self)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    #[must_use]
    pub fn pcautoen(&mut self) -> PCAUTOEN_W<CTRL1_SPEC, 10> {
        PCAUTOEN_W::new(self)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpen(&mut self) -> OCPEN_W<CTRL1_SPEC, 11> {
        OCPEN_W::new(self)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcpen(&mut self) -> PCPEN_W<CTRL1_SPEC, 12> {
        PCPEN_W::new(self)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpcnt(&mut self) -> OCPCNT_W<CTRL1_SPEC, 13> {
        OCPCNT_W::new(self)
    }
    #[doc = "Bits 16:19 - Master slave mode select"]
    #[inline(always)]
    #[must_use]
    pub fn mssel(&mut self) -> MSSEL_W<CTRL1_SPEC, 16> {
        MSSEL_W::new(self)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcvmen(&mut self) -> PCVMEN_W<CTRL1_SPEC, 22> {
        PCVMEN_W::new(self)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocvmen(&mut self) -> OCVMEN_W<CTRL1_SPEC, 23> {
        OCVMEN_W::new(self)
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
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
