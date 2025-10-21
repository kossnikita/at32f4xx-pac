#[doc = "Register `IDEN` reader"]
pub type R = crate::R<IDEN_SPEC>;
#[doc = "Register `IDEN` writer"]
pub type W = crate::W<IDEN_SPEC>;
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfienr {
    #[doc = "0: Overflow interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Overflow interrupt is enabled"]
    Enabled = 1,
}
impl From<Ovfienr> for bool {
    #[inline(always)]
    fn from(variant: Ovfienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OVFIEN_R = crate::BitReader<Ovfienr>;
impl OVFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfienr {
        match self.bits {
            false => Ovfienr::Disabled,
            true => Ovfienr::Enabled,
        }
    }
    #[doc = "Overflow interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovfienr::Disabled
    }
    #[doc = "Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovfienr::Enabled
    }
}
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvfienwWO {
    #[doc = "0: Overflow interrupt disable"]
    Disable = 0,
    #[doc = "1: Overflow interrupt enable"]
    Enable = 1,
}
impl From<OvfienwWO> for bool {
    #[inline(always)]
    fn from(variant: OvfienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OVFIEN_W<'a, REG> = crate::BitWriter<'a, REG, OvfienwWO>;
impl<'a, REG> OVFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OvfienwWO::Disable)
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OvfienwWO::Enable)
    }
}
#[doc = "Channel %s interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ienr {
    #[doc = "0: Channel interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Channel interrupt is enabled"]
    Enabled = 1,
}
impl From<C1ienr> for bool {
    #[inline(always)]
    fn from(variant: C1ienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIEN(1-1)` reader - Channel %s interrupt enable"]
pub type CIEN_R = crate::BitReader<C1ienr>;
impl CIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1ienr {
        match self.bits {
            false => C1ienr::Disabled,
            true => C1ienr::Enabled,
        }
    }
    #[doc = "Channel interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1ienr::Disabled
    }
    #[doc = "Channel interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1ienr::Enabled
    }
}
#[doc = "Channel %s interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ienwWO {
    #[doc = "0: Channel interrupt disable"]
    Disable = 0,
    #[doc = "1: Channel interrupt enable"]
    Enable = 1,
}
impl From<C1ienwWO> for bool {
    #[inline(always)]
    fn from(variant: C1ienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIEN(1-1)` writer - Channel %s interrupt enable"]
pub type CIEN_W<'a, REG> = crate::BitWriter<'a, REG, C1ienwWO>;
impl<'a, REG> CIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1ienwWO::Disable)
    }
    #[doc = "Channel interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1ienwWO::Enable)
    }
}
#[doc = "HALL interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hallienr {
    #[doc = "0: HALL interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: HALL interrupt is enabled"]
    Enabled = 1,
}
impl From<Hallienr> for bool {
    #[inline(always)]
    fn from(variant: Hallienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALLIEN` reader - HALL interrupt enable"]
pub type HALLIEN_R = crate::BitReader<Hallienr>;
impl HALLIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hallienr {
        match self.bits {
            false => Hallienr::Disabled,
            true => Hallienr::Enabled,
        }
    }
    #[doc = "HALL interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hallienr::Disabled
    }
    #[doc = "HALL interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hallienr::Enabled
    }
}
#[doc = "HALL interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HallienwWO {
    #[doc = "0: HALL interrupt disable"]
    Disable = 0,
    #[doc = "1: HALL interrupt enable"]
    Enable = 1,
}
impl From<HallienwWO> for bool {
    #[inline(always)]
    fn from(variant: HallienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALLIEN` writer - HALL interrupt enable"]
pub type HALLIEN_W<'a, REG> = crate::BitWriter<'a, REG, HallienwWO>;
impl<'a, REG> HALLIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HALL interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HallienwWO::Disable)
    }
    #[doc = "HALL interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HallienwWO::Enable)
    }
}
#[doc = "Brake interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brkier {
    #[doc = "0: Break interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Break interrupt is enabled"]
    Enabled = 1,
}
impl From<Brkier> for bool {
    #[inline(always)]
    fn from(variant: Brkier) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKIE` reader - Brake interrupt enable"]
pub type BRKIE_R = crate::BitReader<Brkier>;
impl BRKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brkier {
        match self.bits {
            false => Brkier::Disabled,
            true => Brkier::Enabled,
        }
    }
    #[doc = "Break interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Brkier::Disabled
    }
    #[doc = "Break interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Brkier::Enabled
    }
}
#[doc = "Brake interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrkiewWO {
    #[doc = "0: Break interrupt disable"]
    Disable = 0,
    #[doc = "1: Break interrupt enable"]
    Enable = 1,
}
impl From<BrkiewWO> for bool {
    #[inline(always)]
    fn from(variant: BrkiewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKIE` writer - Brake interrupt enable"]
pub type BRKIE_W<'a, REG> = crate::BitWriter<'a, REG, BrkiewWO>;
impl<'a, REG> BRKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BrkiewWO::Disable)
    }
    #[doc = "Break interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BrkiewWO::Enable)
    }
}
#[doc = "Overflow DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfdenr {
    #[doc = "0: Overflow event DMA request is disabled"]
    Disabled = 0,
    #[doc = "1: Overflow event DMA request is enabled"]
    Enabled = 1,
}
impl From<Ovfdenr> for bool {
    #[inline(always)]
    fn from(variant: Ovfdenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFDEN` reader - Overflow DMA request enable"]
pub type OVFDEN_R = crate::BitReader<Ovfdenr>;
impl OVFDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfdenr {
        match self.bits {
            false => Ovfdenr::Disabled,
            true => Ovfdenr::Enabled,
        }
    }
    #[doc = "Overflow event DMA request is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovfdenr::Disabled
    }
    #[doc = "Overflow event DMA request is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovfdenr::Enabled
    }
}
#[doc = "Overflow DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvfdenwWO {
    #[doc = "0: Overflow event DMA request disable"]
    Disable = 0,
    #[doc = "1: Overflow event DMA request enable"]
    Enable = 1,
}
impl From<OvfdenwWO> for bool {
    #[inline(always)]
    fn from(variant: OvfdenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFDEN` writer - Overflow DMA request enable"]
pub type OVFDEN_W<'a, REG> = crate::BitWriter<'a, REG, OvfdenwWO>;
impl<'a, REG> OVFDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow event DMA request disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OvfdenwWO::Disable)
    }
    #[doc = "Overflow event DMA request enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OvfdenwWO::Enable)
    }
}
#[doc = "Channel %s DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1denr {
    #[doc = "0: Channel DMA request is disabled"]
    Disabled = 0,
    #[doc = "1: Channel DMA request is enabled"]
    Enabled = 1,
}
impl From<C1denr> for bool {
    #[inline(always)]
    fn from(variant: C1denr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN(1-1)` reader - Channel %s DMA request enable"]
pub type CDEN_R = crate::BitReader<C1denr>;
impl CDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1denr {
        match self.bits {
            false => C1denr::Disabled,
            true => C1denr::Enabled,
        }
    }
    #[doc = "Channel DMA request is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1denr::Disabled
    }
    #[doc = "Channel DMA request is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1denr::Enabled
    }
}
#[doc = "Channel %s DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1denwWO {
    #[doc = "0: Channel DMA request disable"]
    Disable = 0,
    #[doc = "1: Channel DMA request enable"]
    Enable = 1,
}
impl From<C1denwWO> for bool {
    #[inline(always)]
    fn from(variant: C1denwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN(1-1)` writer - Channel %s DMA request enable"]
pub type CDEN_W<'a, REG> = crate::BitWriter<'a, REG, C1denwWO>;
impl<'a, REG> CDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel DMA request disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1denwWO::Disable)
    }
    #[doc = "Channel DMA request enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1denwWO::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel (1-1) interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IEN` field.</div>"]
    #[inline(always)]
    pub fn cien(&self, n: u8) -> CIEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CIEN_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) interrupt enable"]
    #[inline(always)]
    pub fn cien_iter(&self) -> impl Iterator<Item = CIEN_R> + '_ {
        (0..1).map(move |n| CIEN_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&self) -> CIEN_R {
        CIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    pub fn hallien(&self) -> HALLIEN_R {
        HALLIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Brake interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    pub fn ovfden(&self) -> OVFDEN_R {
        OVFDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Channel (1-1) DMA request enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1DEN` field.</div>"]
    #[inline(always)]
    pub fn cden(&self, n: u8) -> CDEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CDEN_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) DMA request enable"]
    #[inline(always)]
    pub fn cden_iter(&self) -> impl Iterator<Item = CDEN_R> + '_ {
        (0..1).map(move |n| CDEN_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    pub fn c1den(&self) -> CDEN_R {
        CDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEN")
            .field("c1den", &self.c1den())
            .field("ovfden", &self.ovfden())
            .field("brkie", &self.brkie())
            .field("hallien", &self.hallien())
            .field("c1ien", &self.c1ien())
            .field("ovfien", &self.ovfien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&mut self) -> OVFIEN_W<'_, IDEN_SPEC> {
        OVFIEN_W::new(self, 0)
    }
    #[doc = "Channel (1-1) interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IEN` field.</div>"]
    #[inline(always)]
    pub fn cien(&mut self, n: u8) -> CIEN_W<'_, IDEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CIEN_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&mut self) -> CIEN_W<'_, IDEN_SPEC> {
        CIEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    pub fn hallien(&mut self) -> HALLIEN_W<'_, IDEN_SPEC> {
        HALLIEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Brake interrupt enable"]
    #[inline(always)]
    pub fn brkie(&mut self) -> BRKIE_W<'_, IDEN_SPEC> {
        BRKIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    pub fn ovfden(&mut self) -> OVFDEN_W<'_, IDEN_SPEC> {
        OVFDEN_W::new(self, 8)
    }
    #[doc = "Channel (1-1) DMA request enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1DEN` field.</div>"]
    #[inline(always)]
    pub fn cden(&mut self, n: u8) -> CDEN_W<'_, IDEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CDEN_W::new(self, n * 0 + 9)
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    pub fn c1den(&mut self) -> CDEN_W<'_, IDEN_SPEC> {
        CDEN_W::new(self, 9)
    }
}
#[doc = "Interrupt/DMA enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iden::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iden::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDEN_SPEC;
impl crate::RegisterSpec for IDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iden::R`](R) reader structure"]
impl crate::Readable for IDEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iden::W`](W) writer structure"]
impl crate::Writable for IDEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IDEN_SPEC {}
