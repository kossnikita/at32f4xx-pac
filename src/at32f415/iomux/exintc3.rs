#[doc = "Register `EXINTC3` reader"]
pub type R = crate::R<EXINTC3_SPEC>;
#[doc = "Register `EXINTC3` writer"]
pub type W = crate::W<EXINTC3_SPEC>;
#[doc = "Select the input source for EXINT%s external interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXINT8_A {
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
impl From<EXINT8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXINT8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXINT8_A {
    type Ux = u8;
}
impl crate::IsEnum for EXINT8_A {}
#[doc = "Field `EXINT(8-11)` reader - Select the input source for EXINT%s external interrupt"]
pub type EXINT_R = crate::FieldReader<EXINT8_A>;
impl EXINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXINT8_A> {
        match self.bits {
            0 => Some(EXINT8_A::Gpioa),
            1 => Some(EXINT8_A::Gpiob),
            2 => Some(EXINT8_A::Gpioc),
            3 => Some(EXINT8_A::Gpiod),
            4 => Some(EXINT8_A::Gpiof),
            _ => None,
        }
    }
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn is_gpioa(&self) -> bool {
        *self == EXINT8_A::Gpioa
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiob(&self) -> bool {
        *self == EXINT8_A::Gpiob
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpioc(&self) -> bool {
        *self == EXINT8_A::Gpioc
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiod(&self) -> bool {
        *self == EXINT8_A::Gpiod
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn is_gpiof(&self) -> bool {
        *self == EXINT8_A::Gpiof
    }
}
#[doc = "Field `EXINT(8-11)` writer - Select the input source for EXINT%s external interrupt"]
pub type EXINT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXINT8_A>;
impl<'a, REG> EXINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIOA pin"]
    #[inline(always)]
    pub fn gpioa(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT8_A::Gpioa)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiob(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT8_A::Gpiob)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpioc(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT8_A::Gpioc)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiod(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT8_A::Gpiod)
    }
    #[doc = "GPIOB pin"]
    #[inline(always)]
    pub fn gpiof(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT8_A::Gpiof)
    }
}
impl R {
    #[doc = "Select the input source for EXINT(8-11) external interrupt"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT8` field.</div>"]
    #[inline(always)]
    pub fn exint(&self, n: u8) -> EXINT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Select the input source for EXINT(8-11) external interrupt"]
    #[inline(always)]
    pub fn exint_iter(&self) -> impl Iterator<Item = EXINT_R> + '_ {
        (0..4).map(move |n| EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT8 external interrupt"]
    #[inline(always)]
    pub fn exint8(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT9 external interrupt"]
    #[inline(always)]
    pub fn exint9(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT10 external interrupt"]
    #[inline(always)]
    pub fn exint10(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT11 external interrupt"]
    #[inline(always)]
    pub fn exint11(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC3")
            .field("exint8", &self.exint8())
            .field("exint9", &self.exint9())
            .field("exint10", &self.exint10())
            .field("exint11", &self.exint11())
            .finish()
    }
}
impl W {
    #[doc = "Select the input source for EXINT(8-11) external interrupt"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT8` field.</div>"]
    #[inline(always)]
    pub fn exint(&mut self, n: u8) -> EXINT_W<'_, EXINTC3_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - Select the input source for EXINT8 external interrupt"]
    #[inline(always)]
    pub fn exint8(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select the input source for EXINT9 external interrupt"]
    #[inline(always)]
    pub fn exint9(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Select the input source for EXINT10 external interrupt"]
    #[inline(always)]
    pub fn exint10(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Select the input source for EXINT11 external interrupt"]
    #[inline(always)]
    pub fn exint11(&mut self) -> EXINT_W<'_, EXINTC3_SPEC> {
        EXINT_W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 3 (IOMUX_EXINTC3)\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC3_SPEC;
impl crate::RegisterSpec for EXINTC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc3::R`](R) reader structure"]
impl crate::Readable for EXINTC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc3::W`](W) writer structure"]
impl crate::Writable for EXINTC3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC3 to value 0"]
impl crate::Resettable for EXINTC3_SPEC {}
