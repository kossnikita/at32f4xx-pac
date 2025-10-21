#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Reload counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rldfr {
    #[doc = "0: Downcounter doesn't reached 0x40"]
    NotReached = 0,
    #[doc = "1: Downcounter reached 0x40"]
    Reached = 1,
}
impl From<Rldfr> for bool {
    #[inline(always)]
    fn from(variant: Rldfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDF` reader - Reload counter interrupt flag"]
pub type RLDF_R = crate::BitReader<Rldfr>;
impl RLDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rldfr {
        match self.bits {
            false => Rldfr::NotReached,
            true => Rldfr::Reached,
        }
    }
    #[doc = "Downcounter doesn't reached 0x40"]
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == Rldfr::NotReached
    }
    #[doc = "Downcounter reached 0x40"]
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == Rldfr::Reached
    }
}
#[doc = "Reload counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RldfwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<RldfwWO> for bool {
    #[inline(always)]
    fn from(variant: RldfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDF` writer - Reload counter interrupt flag"]
pub type RLDF_W<'a, REG> = crate::BitWriter0C<'a, REG, RldfwWO>;
impl<'a, REG> RLDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RldfwWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Reload counter interrupt flag"]
    #[inline(always)]
    pub fn rldf(&self) -> RLDF_R {
        RLDF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS").field("rldf", &self.rldf()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reload counter interrupt flag"]
    #[inline(always)]
    pub fn rldf(&mut self) -> RLDF_W<'_, STS_SPEC> {
        RLDF_W::new(self, 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {}
