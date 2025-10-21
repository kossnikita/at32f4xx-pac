#[doc = "Register `OADDR2` reader"]
pub type R = crate::R<OADDR2_SPEC>;
#[doc = "Register `OADDR2` writer"]
pub type W = crate::W<OADDR2_SPEC>;
#[doc = "Own address 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR2EN_A {
    #[doc = "0: Single addressing mode"]
    Single = 0,
    #[doc = "1: Dual addressing mode"]
    Dual = 1,
}
impl From<ADDR2EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR2EN` reader - Own address 2 enable"]
pub type ADDR2EN_R = crate::BitReader<ADDR2EN_A>;
impl ADDR2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR2EN_A {
        match self.bits {
            false => ADDR2EN_A::Single,
            true => ADDR2EN_A::Dual,
        }
    }
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ADDR2EN_A::Single
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == ADDR2EN_A::Dual
    }
}
#[doc = "Field `ADDR2EN` writer - Own address 2 enable"]
pub type ADDR2EN_W<'a, REG> = crate::BitWriter<'a, REG, ADDR2EN_A>;
impl<'a, REG> ADDR2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single addressing mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR2EN_A::Single)
    }
    #[doc = "Dual addressing mode"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR2EN_A::Dual)
    }
}
#[doc = "Field `ADDR2` reader - Own address 2"]
pub type ADDR2_R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - Own address 2"]
pub type ADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - Own address 2 enable"]
    #[inline(always)]
    pub fn addr2en(&self) -> ADDR2EN_R {
        ADDR2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Own address 2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR2")
            .field("addr2", &self.addr2())
            .field("addr2en", &self.addr2en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Own address 2 enable"]
    #[inline(always)]
    pub fn addr2en(&mut self) -> ADDR2EN_W<'_, OADDR2_SPEC> {
        ADDR2EN_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Own address 2"]
    #[inline(always)]
    pub fn addr2(&mut self) -> ADDR2_W<'_, OADDR2_SPEC> {
        ADDR2_W::new(self, 1)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oaddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oaddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OADDR2_SPEC;
impl crate::RegisterSpec for OADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr2::R`](R) reader structure"]
impl crate::Readable for OADDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oaddr2::W`](W) writer structure"]
impl crate::Writable for OADDR2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OADDR2 to value 0"]
impl crate::Resettable for OADDR2_SPEC {}
