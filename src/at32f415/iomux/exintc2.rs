#[doc = "Register `EXINTC2` reader"]
pub type R = crate::R<EXINTC2_SPEC>;
#[doc = "Register `EXINTC2` writer"]
pub type W = crate::W<EXINTC2_SPEC>;
#[doc = "Field `EXINT[4-7]` reader - Select the input source for EXINT%s external interrupt"]
pub type EXINT_R = crate::FieldReader<EXINT4_A>;
#[doc = "Select the input source for EXINT%s external interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXINT4_A {
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
impl From<EXINT4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXINT4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXINT4_A {
    type Ux = u8;
}
impl EXINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXINT4_A> {
        match self.bits {
            0 => Some(EXINT4_A::Gpioa),
            1 => Some(EXINT4_A::Gpiob),
            2 => Some(EXINT4_A::Gpioc),
            3 => Some(EXINT4_A::Gpiod),
            4 => Some(EXINT4_A::Gpiof),
            _ => None,
        }
    }
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == EXINT4_A::Gpioa
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == EXINT4_A::Gpiob
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpioc(&self) -> bool {
        *self == EXINT4_A::Gpioc
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiod(&self) -> bool {
        *self == EXINT4_A::Gpiod
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiof(&self) -> bool {
        *self == EXINT4_A::Gpiof
    }
}
#[doc = "Field `EXINT[4-7]` writer - Select the input source for EXINT%s external interrupt"]
pub type EXINT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXINT4_A>;
impl<'a, REG> EXINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT4_A::Gpioa)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT4_A::Gpiob)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpioc(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT4_A::Gpioc)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiod(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT4_A::Gpiod)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiof(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT4_A::Gpiof)
    }
}
impl R {
    #[doc = "Select the input source for EXINT[4-7]
external interrupt\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn exint(&self, n: u8) -> EXINT_R {
        assert!(n < 4);
        EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT4 external interrupt"]
    #[inline(always)]
    pub fn exint4(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT5 external interrupt"]
    #[inline(always)]
    pub fn exint5(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT6 external interrupt"]
    #[inline(always)]
    pub fn exint6(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT7 external interrupt"]
    #[inline(always)]
    pub fn exint7(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC2")
            .field("exint4", &format_args!("{}", self.exint4().bits()))
            .field("exint5", &format_args!("{}", self.exint5().bits()))
            .field("exint6", &format_args!("{}", self.exint6().bits()))
            .field("exint7", &format_args!("{}", self.exint7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXINTC2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Select the input source for EXINT[4-7]
external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint(&mut self, n: u8) -> EXINT_W<EXINTC2_SPEC> {
        assert!(n < 4);
        EXINT_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT4 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint4(&mut self) -> EXINT_W<EXINTC2_SPEC> {
        EXINT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT5 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint5(&mut self) -> EXINT_W<EXINTC2_SPEC> {
        EXINT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT6 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint6(&mut self) -> EXINT_W<EXINTC2_SPEC> {
        EXINT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT7 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint7(&mut self) -> EXINT_W<EXINTC2_SPEC> {
        EXINT_W::new(self, 12)
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
#[doc = "External interrupt configuration register 2 (IOMUX_EXINTC2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC2_SPEC;
impl crate::RegisterSpec for EXINTC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc2::R`](R) reader structure"]
impl crate::Readable for EXINTC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc2::W`](W) writer structure"]
impl crate::Writable for EXINTC2_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC2 to value 0"]
impl crate::Resettable for EXINTC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
