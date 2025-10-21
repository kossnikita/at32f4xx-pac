#[doc = "Register `RF%s` reader"]
pub type R = crate::R<RF_SPEC>;
#[doc = "Register `RF%s` writer"]
pub type W = crate::W<RF_SPEC>;
#[doc = "Field `MN` reader - Receive FIFO message num"]
pub type MN_R = crate::FieldReader;
#[doc = "Receive FIFO full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffr {
    #[doc = "0: Receive FIFO is not full"]
    NotFull = 0,
    #[doc = "1: Receive FIFO is full"]
    Full = 1,
}
impl From<Ffr> for bool {
    #[inline(always)]
    fn from(variant: Ffr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FF` reader - Receive FIFO full flag"]
pub type FF_R = crate::BitReader<Ffr>;
impl FF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffr {
        match self.bits {
            false => Ffr::NotFull,
            true => Ffr::Full,
        }
    }
    #[doc = "Receive FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Ffr::NotFull
    }
    #[doc = "Receive FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Ffr::Full
    }
}
#[doc = "Receive FIFO full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FfwWO {
    #[doc = "1: Clear full flag"]
    Clear = 1,
}
impl From<FfwWO> for bool {
    #[inline(always)]
    fn from(variant: FfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FF` writer - Receive FIFO full flag"]
pub type FF_W<'a, REG> = crate::BitWriter1C<'a, REG, FfwWO>;
impl<'a, REG> FF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear full flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FfwWO::Clear)
    }
}
#[doc = "Receive FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ofr {
    #[doc = "0: No overflow"]
    NoOverflow = 0,
    #[doc = "1: Receive FIFO overflow"]
    Overflow = 1,
}
impl From<Ofr> for bool {
    #[inline(always)]
    fn from(variant: Ofr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OF` reader - Receive FIFO overflow flag"]
pub type OF_R = crate::BitReader<Ofr>;
impl OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ofr {
        match self.bits {
            false => Ofr::NoOverflow,
            true => Ofr::Overflow,
        }
    }
    #[doc = "No overflow"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Ofr::NoOverflow
    }
    #[doc = "Receive FIFO overflow"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Ofr::Overflow
    }
}
#[doc = "Receive FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OfwWO {
    #[doc = "1: Clear overflow flag"]
    Clear = 1,
}
impl From<OfwWO> for bool {
    #[inline(always)]
    fn from(variant: OfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OF` writer - Receive FIFO overflow flag"]
pub type OF_W<'a, REG> = crate::BitWriter1C<'a, REG, OfwWO>;
impl<'a, REG> OF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear overflow flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OfwWO::Clear)
    }
}
#[doc = "Receive FIFO release\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rr {
    #[doc = "0: FIFO is released"]
    Released = 0,
    #[doc = "1: FIFO release in progress"]
    Releasing = 1,
}
impl From<Rr> for bool {
    #[inline(always)]
    fn from(variant: Rr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R` reader - Receive FIFO release"]
pub type R_R = crate::BitReader<Rr>;
impl R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rr {
        match self.bits {
            false => Rr::Released,
            true => Rr::Releasing,
        }
    }
    #[doc = "FIFO is released"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Rr::Released
    }
    #[doc = "FIFO release in progress"]
    #[inline(always)]
    pub fn is_releasing(&self) -> bool {
        *self == Rr::Releasing
    }
}
#[doc = "Receive FIFO release\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RwWO {
    #[doc = "1: Release FIFO"]
    Release = 1,
}
impl From<RwWO> for bool {
    #[inline(always)]
    fn from(variant: RwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R` writer - Receive FIFO release"]
pub type R_W<'a, REG> = crate::BitWriter1S<'a, REG, RwWO>;
impl<'a, REG> R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Release FIFO"]
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(RwWO::Release)
    }
}
impl R {
    #[doc = "Bits 0:1 - Receive FIFO message num"]
    #[inline(always)]
    pub fn mn(&self) -> MN_R {
        MN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO full flag"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overflow flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO release"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF")
            .field("r", &self.r())
            .field("of", &self.of())
            .field("ff", &self.ff())
            .field("mn", &self.mn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO full flag"]
    #[inline(always)]
    pub fn ff(&mut self) -> FF_W<'_, RF_SPEC> {
        FF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO overflow flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W<'_, RF_SPEC> {
        OF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO release"]
    #[inline(always)]
    pub fn r(&mut self) -> R_W<'_, RF_SPEC> {
        R_W::new(self, 5)
    }
}
#[doc = "Receive FIFO %s register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF_SPEC;
impl crate::RegisterSpec for RF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf::R`](R) reader structure"]
impl crate::Readable for RF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf::W`](W) writer structure"]
impl crate::Writable for RF_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x38;
}
#[doc = "`reset()` method sets RF%s to value 0"]
impl crate::Resettable for RF_SPEC {}
