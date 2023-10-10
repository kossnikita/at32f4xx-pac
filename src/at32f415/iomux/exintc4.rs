#[doc = "Register `EXINTC4` reader"]
pub type R = crate::R<EXINTC4_SPEC>;
#[doc = "Register `EXINTC4` writer"]
pub type W = crate::W<EXINTC4_SPEC>;
#[doc = "Field `EXINT[12-15]` reader - Select the input source for EXINT%s external interrupt"]
pub type EXINT_R = crate::FieldReader<EXINT12_A>;
#[doc = "Select the input source for EXINT%s external interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXINT12_A {
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
impl From<EXINT12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXINT12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXINT12_A {
    type Ux = u8;
}
impl EXINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXINT12_A> {
        match self.bits {
            0 => Some(EXINT12_A::Gpioa),
            1 => Some(EXINT12_A::Gpiob),
            2 => Some(EXINT12_A::Gpioc),
            3 => Some(EXINT12_A::Gpiod),
            4 => Some(EXINT12_A::Gpiof),
            _ => None,
        }
    }
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == EXINT12_A::Gpioa
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == EXINT12_A::Gpiob
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpioc(&self) -> bool {
        *self == EXINT12_A::Gpioc
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiod(&self) -> bool {
        *self == EXINT12_A::Gpiod
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiof(&self) -> bool {
        *self == EXINT12_A::Gpiof
    }
}
#[doc = "Field `EXINT[12-15]` writer - Select the input source for EXINT%s external interrupt"]
pub type EXINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, EXINT12_A>;
impl<'a, REG, const O: u8> EXINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT12_A::Gpioa)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT12_A::Gpiob)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpioc(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT12_A::Gpioc)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiod(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT12_A::Gpiod)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiof(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT12_A::Gpiof)
    }
}
impl R {
    #[doc = "Select the input source for EXINT[12-15]
external interrupt"]
    #[inline(always)]
    pub unsafe fn exint(&self, n: u8) -> EXINT_R {
        EXINT_R::new(((self.bits >> ((n - 12) * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT12 external interrupt"]
    #[inline(always)]
    pub fn exint12(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT13 external interrupt"]
    #[inline(always)]
    pub fn exint13(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT14 external interrupt"]
    #[inline(always)]
    pub fn exint14(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT15 external interrupt"]
    #[inline(always)]
    pub fn exint15(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC4")
            .field("exint12", &format_args!("{}", self.exint12().bits()))
            .field("exint13", &format_args!("{}", self.exint13().bits()))
            .field("exint14", &format_args!("{}", self.exint14().bits()))
            .field("exint15", &format_args!("{}", self.exint15().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXINTC4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Select the input source for EXINT[12-15]
external interrupt"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn exint<const O: u8>(&mut self) -> EXINT_W<EXINTC4_SPEC, O> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT12 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint12(&mut self) -> EXINT_W<EXINTC4_SPEC, 0> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT13 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint13(&mut self) -> EXINT_W<EXINTC4_SPEC, 4> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT14 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint14(&mut self) -> EXINT_W<EXINTC4_SPEC, 8> {
        EXINT_W::new(self)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT15 external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn exint15(&mut self) -> EXINT_W<EXINTC4_SPEC, 12> {
        EXINT_W::new(self)
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
#[doc = "External interrupt configuration register 4 (IOMUX_EXINTC4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exintc4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exintc4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC4_SPEC;
impl crate::RegisterSpec for EXINTC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc4::R`](R) reader structure"]
impl crate::Readable for EXINTC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc4::W`](W) writer structure"]
impl crate::Writable for EXINTC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXINTC4 to value 0"]
impl crate::Resettable for EXINTC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
