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
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    pub fn ovfden(&self) -> OVFDEN_R {
        OVFDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEN")
            .field("ovfden", &self.ovfden())
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
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfden(&mut self) -> OVFDEN_W<IDEN_SPEC> {
        OVFDEN_W::new(self, 8)
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
