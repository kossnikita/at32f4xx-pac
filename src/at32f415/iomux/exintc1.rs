#[doc = "Register `EXINTC1` reader"]
pub type R = crate::R<EXINTC1_SPEC>;
#[doc = "Register `EXINTC1` writer"]
pub type W = crate::W<EXINTC1_SPEC>;
#[doc = "Field `EXINT[0-3]` reader - Select the input source for EXINT%s external interrupt"]
pub type EXINT_R = crate::FieldReader<EXINT0_A>;
#[doc = "Select the input source for EXINT%s external interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXINT0_A {
    #[doc = "0: GPIOA pin"]
    Gpioa = 0,
    #[doc = "1: GPIOB pin"]
    Gpiob = 1,
    #[doc = "2: GPIOB pin"]
    Gpioc = 2,
    #[doc = "3: GPIOB pin"]
    Gpiod = 3,
    #[doc = "4: GPIOB pin"]
    Gpiof = 4,
}
impl From<EXINT0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXINT0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXINT0_A {
    type Ux = u8;
}
impl EXINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXINT0_A> {
        match self.bits {
            0 => Some(EXINT0_A::Gpioa),
            1 => Some(EXINT0_A::Gpiob),
            2 => Some(EXINT0_A::Gpioc),
            3 => Some(EXINT0_A::Gpiod),
            4 => Some(EXINT0_A::Gpiof),
            _ => None,
        }
    }
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == EXINT0_A::Gpioa
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == EXINT0_A::Gpiob
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpioc(&self) -> bool {
        *self == EXINT0_A::Gpioc
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiod(&self) -> bool {
        *self == EXINT0_A::Gpiod
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiof(&self) -> bool {
        *self == EXINT0_A::Gpiof
    }
}
#[doc = "Field `EXINT[0-3]` writer - Select the input source for EXINT%s external interrupt"]
pub type EXINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXINT0_A>;
impl<'a, REG, const O: u8> EXINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT0_A::Gpioa)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT0_A::Gpiob)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpioc(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT0_A::Gpioc)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiod(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT0_A::Gpiod)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiof(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT0_A::Gpiof)
    }
}
impl R {
    #[doc = "Select the input source for EXINT[0-3]
external interrupt"]
    #[inline(always)]
    pub unsafe fn exint(&self, n: u8) -> EXINT_R {
        EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT0 external interrupt"]
    #[inline(always)]
    pub fn exint0(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT1 external interrupt"]
    #[inline(always)]
    pub fn exint1(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT2 external interrupt"]
    #[inline(always)]
    pub fn exint2(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT3 external interrupt"]
    #[inline(always)]
    pub fn exint3(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Select the input source for EXINT[0-3]
external interrupt"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn exint<const O: u8>(&mut self) -> EXINT_W<EXINTC1_SPEC, O> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT0 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint0(&mut self) -> EXINT_W<EXINTC1_SPEC, 0> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT1 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint1(&mut self) -> EXINT_W<EXINTC1_SPEC, 4> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT2 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint2(&mut self) -> EXINT_W<EXINTC1_SPEC, 8> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT3 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint3(&mut self) -> EXINT_W<EXINTC1_SPEC, 12> {
        EXINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External interrupt configuration register 1 (IOMUX_EXINTC1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC1_SPEC;
impl crate::RegisterSpec for EXINTC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc1::R`](R) reader structure"]
impl crate::Readable for EXINTC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc1::W`](W) writer structure"]
impl crate::Writable for EXINTC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC1 to value 0"]
impl crate::Resettable for EXINTC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
