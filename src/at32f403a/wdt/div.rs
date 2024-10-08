#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Division divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "0: LICK divided by 4"]
    Div4 = 0,
    #[doc = "1: LICK divided by 8"]
    Div8 = 1,
    #[doc = "2: LICK divided by 16"]
    Div16 = 2,
    #[doc = "3: LICK divided by 32"]
    Div32 = 3,
    #[doc = "4: LICK divided by 64"]
    Div64 = 4,
    #[doc = "5: LICK divided by 128"]
    Div128 = 5,
    #[doc = "6: LICK divided by 256"]
    Div256 = 6,
    #[doc = "7: LICK divided by 256"]
    Div256_2 = 7,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIV_A {
    type Ux = u8;
}
impl crate::IsEnum for DIV_A {}
#[doc = "Field `DIV` reader - Division divider"]
pub type DIV_R = crate::FieldReader<DIV_A>;
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIV_A {
        match self.bits {
            0 => DIV_A::Div4,
            1 => DIV_A::Div8,
            2 => DIV_A::Div16,
            3 => DIV_A::Div32,
            4 => DIV_A::Div64,
            5 => DIV_A::Div128,
            6 => DIV_A::Div256,
            7 => DIV_A::Div256_2,
            _ => unreachable!(),
        }
    }
    #[doc = "LICK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIV_A::Div4
    }
    #[doc = "LICK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIV_A::Div8
    }
    #[doc = "LICK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIV_A::Div16
    }
    #[doc = "LICK divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIV_A::Div32
    }
    #[doc = "LICK divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DIV_A::Div64
    }
    #[doc = "LICK divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DIV_A::Div128
    }
    #[doc = "LICK divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == DIV_A::Div256
    }
    #[doc = "LICK divided by 256"]
    #[inline(always)]
    pub fn is_div256_2(&self) -> bool {
        *self == DIV_A::Div256_2
    }
}
#[doc = "Field `DIV` writer - Division divider"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DIV_A, crate::Safe>;
impl<'a, REG> DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LICK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div4)
    }
    #[doc = "LICK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div8)
    }
    #[doc = "LICK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div16)
    }
    #[doc = "LICK divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div32)
    }
    #[doc = "LICK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div64)
    }
    #[doc = "LICK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div128)
    }
    #[doc = "LICK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div256)
    }
    #[doc = "LICK divided by 256"]
    #[inline(always)]
    pub fn div256_2(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div256_2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Division divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV").field("div", &self.div()).finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Division divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<DIV_SPEC> {
        DIV_W::new(self, 0)
    }
}
#[doc = "Division register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: u32 = 0;
}
