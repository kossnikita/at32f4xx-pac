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
    pub fn variant(&self) -> OVFIENR_A {
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
pub type OVFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OVFIENW_AW>;
impl<'a, REG, const O: u8> OVFIEN_W<'a, REG, O>
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
#[doc = "Field `CIEN[1-2]` reader - Channel %s interrupt enable"]
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
    pub fn variant(&self) -> C1IENR_A {
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
#[doc = "Field `CIEN[1-2]` writer - Channel %s interrupt enable"]
pub type CIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, C1IENW_AW>;
impl<'a, REG, const O: u8> CIEN_W<'a, REG, O>
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
pub type HALLIEN_R = crate::BitReader;
#[doc = "Field `HALLIEN` writer - HALL interrupt enable"]
pub type HALLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIEN` reader - Trigger interrupt enable"]
pub type TIEN_R = crate::BitReader;
#[doc = "Field `TIEN` writer - Trigger interrupt enable"]
pub type TIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRKIE` reader - Brake interrupt enable"]
pub type BRKIE_R = crate::BitReader;
#[doc = "Field `BRKIE` writer - Brake interrupt enable"]
pub type BRKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVFDEN` reader - Overflow DMA request enable"]
pub type OVFDEN_R = crate::BitReader;
#[doc = "Field `OVFDEN` writer - Overflow DMA request enable"]
pub type OVFDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1DEN` reader - Channel 1 DMA request enable"]
pub type C1DEN_R = crate::BitReader;
#[doc = "Field `C1DEN` writer - Channel 1 DMA request enable"]
pub type C1DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2DEN` reader - Channel 2 DMA request enable"]
pub type C2DEN_R = crate::BitReader;
#[doc = "Field `C2DEN` writer - Channel 2 DMA request enable"]
pub type C2DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HALLDE` reader - HALL DMA request enable"]
pub type HALLDE_R = crate::BitReader;
#[doc = "Field `HALLDE` writer - HALL DMA request enable"]
pub type HALLDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDEN` reader - Trigger DMA request enable"]
pub type TDEN_R = crate::BitReader;
#[doc = "Field `TDEN` writer - Trigger DMA request enable"]
pub type TDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel [1-2]
interrupt enable"]
    #[inline(always)]
    pub unsafe fn cien(&self, n: u8) -> CIEN_R {
        CIEN_R::new(((self.bits >> (n - 1 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&self) -> CIEN_R {
        CIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    pub fn c2ien(&self) -> CIEN_R {
        CIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    pub fn hallien(&self) -> HALLIEN_R {
        HALLIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tien(&self) -> TIEN_R {
        TIEN_R::new(((self.bits >> 6) & 1) != 0)
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
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    pub fn c1den(&self) -> C1DEN_R {
        C1DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 DMA request enable"]
    #[inline(always)]
    pub fn c2den(&self) -> C2DEN_R {
        C2DEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - HALL DMA request enable"]
    #[inline(always)]
    pub fn hallde(&self) -> HALLDE_R {
        HALLDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tden(&self) -> TDEN_R {
        TDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEN")
            .field("tden", &format_args!("{}", self.tden().bit()))
            .field("hallde", &format_args!("{}", self.hallde().bit()))
            .field("c2den", &format_args!("{}", self.c2den().bit()))
            .field("c1den", &format_args!("{}", self.c1den().bit()))
            .field("ovfden", &format_args!("{}", self.ovfden().bit()))
            .field("brkie", &format_args!("{}", self.brkie().bit()))
            .field("tien", &format_args!("{}", self.tien().bit()))
            .field("hallien", &format_args!("{}", self.hallien().bit()))
            .field("c1ien", &format_args!("{}", self.c1ien().bit()))
            .field("c2ien", &format_args!("{}", self.c2ien().bit()))
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
    pub fn ovfien(&mut self) -> OVFIEN_W<IDEN_SPEC, 0> {
        OVFIEN_W::new(self)
    }
    #[doc = "Channel [1-2]
interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cien<const O: u8>(&mut self) -> CIEN_W<IDEN_SPEC, O> {
        CIEN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1ien(&mut self) -> CIEN_W<IDEN_SPEC, 1> {
        CIEN_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2ien(&mut self) -> CIEN_W<IDEN_SPEC, 2> {
        CIEN_W::new(self)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hallien(&mut self) -> HALLIEN_W<IDEN_SPEC, 5> {
        HALLIEN_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TIEN_W<IDEN_SPEC, 6> {
        TIEN_W::new(self)
    }
    #[doc = "Bit 7 - Brake interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BRKIE_W<IDEN_SPEC, 7> {
        BRKIE_W::new(self)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfden(&mut self) -> OVFDEN_W<IDEN_SPEC, 8> {
        OVFDEN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1den(&mut self) -> C1DEN_W<IDEN_SPEC, 9> {
        C1DEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2den(&mut self) -> C2DEN_W<IDEN_SPEC, 10> {
        C2DEN_W::new(self)
    }
    #[doc = "Bit 13 - HALL DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn hallde(&mut self) -> HALLDE_W<IDEN_SPEC, 13> {
        HALLDE_W::new(self)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tden(&mut self) -> TDEN_W<IDEN_SPEC, 14> {
        TDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IDEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
