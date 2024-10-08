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
#[doc = "Field `CIEN(1-2)` reader - Channel %s interrupt enable"]
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
#[doc = "Field `CIEN(1-2)` writer - Channel %s interrupt enable"]
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
#[doc = "Field `HALLIEN` reader - HALL interrupt enable"]
pub type HALLIEN_R = crate::BitReader;
#[doc = "Field `HALLIEN` writer - HALL interrupt enable"]
pub type HALLIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIEN` reader - Trigger interrupt enable"]
pub type TIEN_R = crate::BitReader;
#[doc = "Field `TIEN` writer - Trigger interrupt enable"]
pub type TIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKIE` reader - Brake interrupt enable"]
pub type BRKIE_R = crate::BitReader;
#[doc = "Field `BRKIE` writer - Brake interrupt enable"]
pub type BRKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFDEN` reader - Overflow DMA request enable"]
pub type OVFDEN_R = crate::BitReader;
#[doc = "Field `OVFDEN` writer - Overflow DMA request enable"]
pub type OVFDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1DEN` reader - Channel 1 DMA request enable"]
pub type C1DEN_R = crate::BitReader;
#[doc = "Field `C1DEN` writer - Channel 1 DMA request enable"]
pub type C1DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2DEN` reader - Channel 2 DMA request enable"]
pub type C2DEN_R = crate::BitReader;
#[doc = "Field `C2DEN` writer - Channel 2 DMA request enable"]
pub type C2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALLDE` reader - HALL DMA request enable"]
pub type HALLDE_R = crate::BitReader;
#[doc = "Field `HALLDE` writer - HALL DMA request enable"]
pub type HALLDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDEN` reader - Trigger DMA request enable"]
pub type TDEN_R = crate::BitReader;
#[doc = "Field `TDEN` writer - Trigger DMA request enable"]
pub type TDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel (1-2) interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IEN` field.</div>"]
    #[inline(always)]
    pub fn cien(&self, n: u8) -> CIEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CIEN_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-2) interrupt enable"]
    #[inline(always)]
    pub fn cien_iter(&self) -> impl Iterator<Item = CIEN_R> + '_ {
        (0..2).map(move |n| CIEN_R::new(((self.bits >> (n + 1)) & 1) != 0))
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
            .field("tden", &self.tden())
            .field("hallde", &self.hallde())
            .field("c2den", &self.c2den())
            .field("c1den", &self.c1den())
            .field("ovfden", &self.ovfden())
            .field("brkie", &self.brkie())
            .field("tien", &self.tien())
            .field("hallien", &self.hallien())
            .field("c1ien", &self.c1ien())
            .field("c2ien", &self.c2ien())
            .field("ovfien", &self.ovfien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfien(&mut self) -> OVFIEN_W<IDEN_SPEC> {
        OVFIEN_W::new(self, 0)
    }
    #[doc = "Channel (1-2) interrupt enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IEN` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn cien(&mut self, n: u8) -> CIEN_W<IDEN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CIEN_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1ien(&mut self) -> CIEN_W<IDEN_SPEC> {
        CIEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2ien(&mut self) -> CIEN_W<IDEN_SPEC> {
        CIEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - HALL interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hallien(&mut self) -> HALLIEN_W<IDEN_SPEC> {
        HALLIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TIEN_W<IDEN_SPEC> {
        TIEN_W::new(self, 6)
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
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1den(&mut self) -> C1DEN_W<IDEN_SPEC> {
        C1DEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2den(&mut self) -> C2DEN_W<IDEN_SPEC> {
        C2DEN_W::new(self, 10)
    }
    #[doc = "Bit 13 - HALL DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn hallde(&mut self) -> HALLDE_W<IDEN_SPEC> {
        HALLDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tden(&mut self) -> TDEN_W<IDEN_SPEC> {
        TDEN_W::new(self, 14)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IDEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
