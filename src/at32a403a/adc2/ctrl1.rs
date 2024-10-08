#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `VMCSEL` reader - Voltage monitoring channel select"]
pub type VMCSEL_R = crate::FieldReader;
#[doc = "Field `VMCSEL` writer - Voltage monitoring channel select"]
pub type VMCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Channel conversion end interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cceienr {
    #[doc = "0: Channel conversion end interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Channel conversion end interrupt is enabled"]
    Enabled = 1,
}
impl From<Cceienr> for bool {
    #[inline(always)]
    fn from(variant: Cceienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEIEN` reader - Channel conversion end interrupt enable"]
pub type CCEIEN_R = crate::BitReader<Cceienr>;
impl CCEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cceienr {
        match self.bits {
            false => Cceienr::Disabled,
            true => Cceienr::Enabled,
        }
    }
    #[doc = "Channel conversion end interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cceienr::Disabled
    }
    #[doc = "Channel conversion end interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cceienr::Enabled
    }
}
#[doc = "Channel conversion end interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CceienwWO {
    #[doc = "0: Channel conversion end interrupt disable"]
    Disable = 0,
    #[doc = "1: Channel conversion end interrupt enable"]
    Enable = 1,
}
impl From<CceienwWO> for bool {
    #[inline(always)]
    fn from(variant: CceienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEIEN` writer - Channel conversion end interrupt enable"]
pub type CCEIEN_W<'a, REG> = crate::BitWriter<'a, REG, CceienwWO>;
impl<'a, REG> CCEIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel conversion end interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CceienwWO::Disable)
    }
    #[doc = "Channel conversion end interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CceienwWO::Enable)
    }
}
#[doc = "Voltage monitoring out of range interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vmorienr {
    #[doc = "0: Voltage monitoring out of range interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Voltage monitoring out of range interrupt is enabled"]
    Enabled = 1,
}
impl From<Vmorienr> for bool {
    #[inline(always)]
    fn from(variant: Vmorienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMORIEN` reader - Voltage monitoring out of range interrupt enable"]
pub type VMORIEN_R = crate::BitReader<Vmorienr>;
impl VMORIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vmorienr {
        match self.bits {
            false => Vmorienr::Disabled,
            true => Vmorienr::Enabled,
        }
    }
    #[doc = "Voltage monitoring out of range interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Vmorienr::Disabled
    }
    #[doc = "Voltage monitoring out of range interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Vmorienr::Enabled
    }
}
#[doc = "Voltage monitoring out of range interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmorienwWO {
    #[doc = "0: Voltage monitoring out of range interrupt disable"]
    Disable = 0,
    #[doc = "1: Voltage monitoring out of range interrupt enable"]
    Enable = 1,
}
impl From<VmorienwWO> for bool {
    #[inline(always)]
    fn from(variant: VmorienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMORIEN` writer - Voltage monitoring out of range interrupt enable"]
pub type VMORIEN_W<'a, REG> = crate::BitWriter<'a, REG, VmorienwWO>;
impl<'a, REG> VMORIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage monitoring out of range interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(VmorienwWO::Disable)
    }
    #[doc = "Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(VmorienwWO::Enable)
    }
}
#[doc = "Field `PCCEIEN` reader - Conversion end interrupt enable for preempted channels"]
pub use CCEIEN_R as PCCEIEN_R;
#[doc = "Field `PCCEIEN` writer - Conversion end interrupt enable for preempted channels"]
pub use CCEIEN_W as PCCEIEN_W;
#[doc = "Sequence mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqenr {
    #[doc = "0: Sequence mode is disabled"]
    Disabled = 0,
    #[doc = "1: Sequence mode is enabled"]
    Enabled = 1,
}
impl From<Sqenr> for bool {
    #[inline(always)]
    fn from(variant: Sqenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQEN` reader - Sequence mode enable"]
pub type SQEN_R = crate::BitReader<Sqenr>;
impl SQEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqenr {
        match self.bits {
            false => Sqenr::Disabled,
            true => Sqenr::Enabled,
        }
    }
    #[doc = "Sequence mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sqenr::Disabled
    }
    #[doc = "Sequence mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sqenr::Enabled
    }
}
#[doc = "Sequence mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SqenwWO {
    #[doc = "0: Sequence mode disable"]
    Disable = 0,
    #[doc = "1: Sequence mode enable"]
    Enable = 1,
}
impl From<SqenwWO> for bool {
    #[inline(always)]
    fn from(variant: SqenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQEN` writer - Sequence mode enable"]
pub type SQEN_W<'a, REG> = crate::BitWriter<'a, REG, SqenwWO>;
impl<'a, REG> SQEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sequence mode disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SqenwWO::Disable)
    }
    #[doc = "Sequence mode enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SqenwWO::Enable)
    }
}
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
#[doc = "Field `VMSGEN` reader - Voltage monitoring enable on a single channel"]
pub type VMSGEN_R = crate::BitReader<VMSGEN_A>;
impl VMSGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VMSGEN_A {
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
pub type VMSGEN_W<'a, REG> = crate::BitWriter<'a, REG, VMSGEN_A>;
impl<'a, REG> VMSGEN_W<'a, REG>
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
#[doc = "Preempted group automatic conversion enable after ordinary group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcautoenr {
    #[doc = "0: Preempted group automatic conversion is disabled"]
    Disabled = 0,
    #[doc = "1: Preempted group automatic conversion is enabled"]
    Enabled = 1,
}
impl From<Pcautoenr> for bool {
    #[inline(always)]
    fn from(variant: Pcautoenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCAUTOEN` reader - Preempted group automatic conversion enable after ordinary group"]
pub type PCAUTOEN_R = crate::BitReader<Pcautoenr>;
impl PCAUTOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcautoenr {
        match self.bits {
            false => Pcautoenr::Disabled,
            true => Pcautoenr::Enabled,
        }
    }
    #[doc = "Preempted group automatic conversion is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pcautoenr::Disabled
    }
    #[doc = "Preempted group automatic conversion is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pcautoenr::Enabled
    }
}
#[doc = "Preempted group automatic conversion enable after ordinary group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcautoenwWO {
    #[doc = "0: Preempted group automatic conversion disable"]
    Disable = 0,
    #[doc = "1: Preempted group automatic conversion enable"]
    Enable = 1,
}
impl From<PcautoenwWO> for bool {
    #[inline(always)]
    fn from(variant: PcautoenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCAUTOEN` writer - Preempted group automatic conversion enable after ordinary group"]
pub type PCAUTOEN_W<'a, REG> = crate::BitWriter<'a, REG, PcautoenwWO>;
impl<'a, REG> PCAUTOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preempted group automatic conversion disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PcautoenwWO::Disable)
    }
    #[doc = "Preempted group automatic conversion enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PcautoenwWO::Enable)
    }
}
#[doc = "Partitioned mode enable on ordinary channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocpenr {
    #[doc = "0: Partitioned mode on channels is disabled"]
    Disabled = 0,
    #[doc = "1: Partitioned mode on channels is enabled"]
    Enabled = 1,
}
impl From<Ocpenr> for bool {
    #[inline(always)]
    fn from(variant: Ocpenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCPEN` reader - Partitioned mode enable on ordinary channels"]
pub type OCPEN_R = crate::BitReader<Ocpenr>;
impl OCPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocpenr {
        match self.bits {
            false => Ocpenr::Disabled,
            true => Ocpenr::Enabled,
        }
    }
    #[doc = "Partitioned mode on channels is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ocpenr::Disabled
    }
    #[doc = "Partitioned mode on channels is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ocpenr::Enabled
    }
}
#[doc = "Partitioned mode enable on ordinary channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OcpenwWO {
    #[doc = "0: Partitioned mode on channels disable"]
    Disable = 0,
    #[doc = "1: Partitioned mode on channels enable"]
    Enable = 1,
}
impl From<OcpenwWO> for bool {
    #[inline(always)]
    fn from(variant: OcpenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCPEN` writer - Partitioned mode enable on ordinary channels"]
pub type OCPEN_W<'a, REG> = crate::BitWriter<'a, REG, OcpenwWO>;
impl<'a, REG> OCPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Partitioned mode on channels disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OcpenwWO::Disable)
    }
    #[doc = "Partitioned mode on channels enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OcpenwWO::Enable)
    }
}
#[doc = "Field `PCPEN` reader - Partitioned mode enable on preempted channels"]
pub use OCPEN_R as PCPEN_R;
#[doc = "Field `PCPEN` writer - Partitioned mode enable on preempted channels"]
pub use OCPEN_W as PCPEN_W;
#[doc = "Field `OCPCNT` reader - Partitioned mode conversion count of ordinary channels"]
pub type OCPCNT_R = crate::FieldReader;
#[doc = "Field `OCPCNT` writer - Partitioned mode conversion count of ordinary channels"]
pub type OCPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Voltage monitoring enable on preempted channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcvmenr {
    #[doc = "0: Voltage monitoring is disabled"]
    Disabled = 0,
    #[doc = "1: Voltage monitoring is enabled"]
    Enabled = 1,
}
impl From<Pcvmenr> for bool {
    #[inline(always)]
    fn from(variant: Pcvmenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCVMEN` reader - Voltage monitoring enable on preempted channels"]
pub type PCVMEN_R = crate::BitReader<Pcvmenr>;
impl PCVMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcvmenr {
        match self.bits {
            false => Pcvmenr::Disabled,
            true => Pcvmenr::Enabled,
        }
    }
    #[doc = "Voltage monitoring is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pcvmenr::Disabled
    }
    #[doc = "Voltage monitoring is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pcvmenr::Enabled
    }
}
#[doc = "Voltage monitoring enable on preempted channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcvmenwWO {
    #[doc = "0: Voltage monitoring disable"]
    Disable = 0,
    #[doc = "1: Voltage monitoring enable"]
    Enable = 1,
}
impl From<PcvmenwWO> for bool {
    #[inline(always)]
    fn from(variant: PcvmenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCVMEN` writer - Voltage monitoring enable on preempted channels"]
pub type PCVMEN_W<'a, REG> = crate::BitWriter<'a, REG, PcvmenwWO>;
impl<'a, REG> PCVMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage monitoring disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PcvmenwWO::Disable)
    }
    #[doc = "Voltage monitoring enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PcvmenwWO::Enable)
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
            .field("pcvmen", &self.pcvmen())
            .field("ocvmen", &self.ocvmen())
            .field("ocpcnt", &self.ocpcnt())
            .field("ocpen", &self.ocpen())
            .field("pcpen", &self.pcpen())
            .field("pcautoen", &self.pcautoen())
            .field("vmsgen", &self.vmsgen())
            .field("sqen", &self.sqen())
            .field("cceien", &self.cceien())
            .field("pcceien", &self.pcceien())
            .field("vmorien", &self.vmorien())
            .field("vmcsel", &self.vmcsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    #[must_use]
    pub fn vmcsel(&mut self) -> VMCSEL_W<CTRL1_SPEC> {
        VMCSEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cceien(&mut self) -> CCEIEN_W<CTRL1_SPEC> {
        CCEIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmorien(&mut self) -> VMORIEN_W<CTRL1_SPEC> {
        VMORIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcceien(&mut self) -> PCCEIEN_W<CTRL1_SPEC> {
        PCCEIEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sqen(&mut self) -> SQEN_W<CTRL1_SPEC> {
        SQEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn vmsgen(&mut self) -> VMSGEN_W<CTRL1_SPEC> {
        VMSGEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    #[must_use]
    pub fn pcautoen(&mut self) -> PCAUTOEN_W<CTRL1_SPEC> {
        PCAUTOEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpen(&mut self) -> OCPEN_W<CTRL1_SPEC> {
        OCPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcpen(&mut self) -> PCPEN_W<CTRL1_SPEC> {
        PCPEN_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpcnt(&mut self) -> OCPCNT_W<CTRL1_SPEC> {
        OCPCNT_W::new(self, 13)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcvmen(&mut self) -> PCVMEN_W<CTRL1_SPEC> {
        PCVMEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocvmen(&mut self) -> OCVMEN_W<CTRL1_SPEC> {
        OCVMEN_W::new(self, 23)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
