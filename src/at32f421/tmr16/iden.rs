#[doc = "Register `IDEN` reader"]
pub type R = crate::R<IDEN_SPEC>;
#[doc = "Register `IDEN` writer"]
pub type W = crate::W<IDEN_SPEC>;
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OVFIEN_R = crate::BitReader<OVFIENR_A>;
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFIENR_A {
    #[doc = "0: Overflow interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Overflow interrupt is enabled"]
    Enabled = 1,
}
impl From<OVFIENR_A> for bool {
    #[inline(always)]
    fn from(variant: OVFIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVFIENR_A {
        match self.bits {
            false => OVFIENR_A::Disabled,
            true => OVFIENR_A::Enabled,
        }
    }
    #[doc = "Overflow interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVFIENR_A::Disabled
    }
    #[doc = "Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVFIENR_A::Enabled
    }
}
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFIENW_AW {
    #[doc = "0: Overflow interrupt disable"]
    Disable = 0,
    #[doc = "1: Overflow interrupt enable"]
    Enable = 1,
}
impl From<OVFIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OVFIEN_W<'a, REG> = crate::BitWriter<'a, REG, OVFIENW_AW>;
impl<'a, REG> OVFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OVFIENW_AW::Disable)
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OVFIENW_AW::Enable)
    }
}
#[doc = "Field `CIEN[1-1]` reader - Channel %s interrupt enable"]
pub type CIEN_R = crate::BitReader<C1IENR_A>;
#[doc = "Channel %s interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1IENR_A {
    #[doc = "0: Channel interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Channel interrupt is enabled"]
    Enabled = 1,
}
impl From<C1IENR_A> for bool {
    #[inline(always)]
    fn from(variant: C1IENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1IENR_A {
        match self.bits {
            false => C1IENR_A::Disabled,
            true => C1IENR_A::Enabled,
        }
    }
    #[doc = "Channel interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1IENR_A::Disabled
    }
    #[doc = "Channel interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1IENR_A::Enabled
    }
}
#[doc = "Channel %s interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1IENW_AW {
    #[doc = "0: Channel interrupt disable"]
    Disable = 0,
    #[doc = "1: Channel interrupt enable"]
    Enable = 1,
}
impl From<C1IENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1IENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIEN[1-1]` writer - Channel %s interrupt enable"]
pub type CIEN_W<'a, REG> = crate::BitWriter<'a, REG, C1IENW_AW>;
impl<'a, REG> CIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1IENW_AW::Disable)
    }
    #[doc = "Channel interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1IENW_AW::Enable)
    }
}
#[doc = "Field `HALLIEN` reader - HALL interrupt enable"]
pub type HALLIEN_R = crate::BitReader<HALLIENR_A>;
#[doc = "HALL interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALLIENR_A {
    #[doc = "0: HALL interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: HALL interrupt is enabled"]
    Enabled = 1,
}
impl From<HALLIENR_A> for bool {
    #[inline(always)]
    fn from(variant: HALLIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl HALLIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HALLIENR_A {
        match self.bits {
            false => HALLIENR_A::Disabled,
            true => HALLIENR_A::Enabled,
        }
    }
    #[doc = "HALL interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALLIENR_A::Disabled
    }
    #[doc = "HALL interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALLIENR_A::Enabled
    }
}
#[doc = "HALL interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALLIENW_AW {
    #[doc = "0: HALL interrupt disable"]
    Disable = 0,
    #[doc = "1: HALL interrupt enable"]
    Enable = 1,
}
impl From<HALLIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: HALLIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALLIEN` writer - HALL interrupt enable"]
pub type HALLIEN_W<'a, REG> = crate::BitWriter<'a, REG, HALLIENW_AW>;
impl<'a, REG> HALLIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HALL interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HALLIENW_AW::Disable)
    }
    #[doc = "HALL interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HALLIENW_AW::Enable)
    }
}
#[doc = "Field `BRKIE` reader - Brake interrupt enable"]
pub type BRKIE_R = crate::BitReader<BRKIER_A>;
#[doc = "Brake interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKIER_A {
    #[doc = "0: Break interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Break interrupt is enabled"]
    Enabled = 1,
}
impl From<BRKIER_A> for bool {
    #[inline(always)]
    fn from(variant: BRKIER_A) -> Self {
        variant as u8 != 0
    }
}
impl BRKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRKIER_A {
        match self.bits {
            false => BRKIER_A::Disabled,
            true => BRKIER_A::Enabled,
        }
    }
    #[doc = "Break interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRKIER_A::Disabled
    }
    #[doc = "Break interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRKIER_A::Enabled
    }
}
#[doc = "Brake interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKIEW_AW {
    #[doc = "0: Break interrupt disable"]
    Disable = 0,
    #[doc = "1: Break interrupt enable"]
    Enable = 1,
}
impl From<BRKIEW_AW> for bool {
    #[inline(always)]
    fn from(variant: BRKIEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKIE` writer - Brake interrupt enable"]
pub type BRKIE_W<'a, REG> = crate::BitWriter<'a, REG, BRKIEW_AW>;
impl<'a, REG> BRKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BRKIEW_AW::Disable)
    }
    #[doc = "Break interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BRKIEW_AW::Enable)
    }
}
#[doc = "Field `OVFDEN` reader - Overflow DMA request enable"]
pub type OVFDEN_R = crate::BitReader<OVFDENR_A>;
#[doc = "Overflow DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFDENR_A {
    #[doc = "0: Overflow event DMA request is disabled"]
    Disabled = 0,
    #[doc = "1: Overflow event DMA request is enabled"]
    Enabled = 1,
}
impl From<OVFDENR_A> for bool {
    #[inline(always)]
    fn from(variant: OVFDENR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVFDENR_A {
        match self.bits {
            false => OVFDENR_A::Disabled,
            true => OVFDENR_A::Enabled,
        }
    }
    #[doc = "Overflow event DMA request is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVFDENR_A::Disabled
    }
    #[doc = "Overflow event DMA request is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVFDENR_A::Enabled
    }
}
#[doc = "Overflow DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFDENW_AW {
    #[doc = "0: Overflow event DMA request disable"]
    Disable = 0,
    #[doc = "1: Overflow event DMA request enable"]
    Enable = 1,
}
impl From<OVFDENW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFDENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFDEN` writer - Overflow DMA request enable"]
pub type OVFDEN_W<'a, REG> = crate::BitWriter<'a, REG, OVFDENW_AW>;
impl<'a, REG> OVFDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow event DMA request disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OVFDENW_AW::Disable)
    }
    #[doc = "Overflow event DMA request enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OVFDENW_AW::Enable)
    }
}
#[doc = "Field `CDEN[1-1]` reader - Channel %s DMA request enable"]
pub type CDEN_R = crate::BitReader<C1DENR_A>;
#[doc = "Channel %s DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1DENR_A {
    #[doc = "0: Channel DMA request is disabled"]
    Disabled = 0,
    #[doc = "1: Channel DMA request is enabled"]
    Enabled = 1,
}
impl From<C1DENR_A> for bool {
    #[inline(always)]
    fn from(variant: C1DENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1DENR_A {
        match self.bits {
            false => C1DENR_A::Disabled,
            true => C1DENR_A::Enabled,
        }
    }
    #[doc = "Channel DMA request is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1DENR_A::Disabled
    }
    #[doc = "Channel DMA request is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1DENR_A::Enabled
    }
}
#[doc = "Channel %s DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1DENW_AW {
    #[doc = "0: Channel DMA request disable"]
    Disable = 0,
    #[doc = "1: Channel DMA request enable"]
    Enable = 1,
}
impl From<C1DENW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1DENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN[1-1]` writer - Channel %s DMA request enable"]
pub type CDEN_W<'a, REG> = crate::BitWriter<'a, REG, C1DENW_AW>;
impl<'a, REG> CDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel DMA request disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1DENW_AW::Disable)
    }
    #[doc = "Channel DMA request enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1DENW_AW::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel [1-1]
interrupt enable\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cien(&self, n: u8) -> CIEN_R {
        assert!(n < 1);
        CIEN_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
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
    #[doc = "Channel [1-1]
DMA request enable\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cden(&self, n: u8) -> CDEN_R {
        assert!(n < 1);
        CDEN_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
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
            .field("c1den", &format_args!("{}", self.c1den().bit()))
            .field("ovfden", &format_args!("{}", self.ovfden().bit()))
            .field("brkie", &format_args!("{}", self.brkie().bit()))
            .field("hallien", &format_args!("{}", self.hallien().bit()))
            .field("c1ien", &format_args!("{}", self.c1ien().bit()))
            .field("ovfien", &format_args!("{}", self.ovfien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IDEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfien(&mut self) -> OVFIEN_W<IDEN_SPEC> {
        OVFIEN_W::new(self, 0)
    }
    #[doc = "Channel [1-1]
interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cien(&mut self, n: u8) -> CIEN_W<IDEN_SPEC> {
        assert!(n < 1);
        CIEN_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1ien(&mut self) -> CIEN_W<IDEN_SPEC> {
        CIEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hallien(&mut self) -> HALLIEN_W<IDEN_SPEC> {
        HALLIEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Brake interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BRKIE_W<IDEN_SPEC> {
        BRKIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfden(&mut self) -> OVFDEN_W<IDEN_SPEC> {
        OVFDEN_W::new(self, 8)
    }
    #[doc = "Channel [1-1]
DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cden(&mut self, n: u8) -> CDEN_W<IDEN_SPEC> {
        assert!(n < 1);
        CDEN_W::new(self, n * 0 + 9)
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1den(&mut self) -> CDEN_W<IDEN_SPEC> {
        CDEN_W::new(self, 9)
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
#[doc = "Interrupt/DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iden::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iden::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDEN_SPEC;
impl crate::RegisterSpec for IDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iden::R`](R) reader structure"]
impl crate::Readable for IDEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iden::W`](W) writer structure"]
impl crate::Writable for IDEN_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IDEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
