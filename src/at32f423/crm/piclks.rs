#[doc = "Register `PICLKS` reader"]
pub type R = crate::R<PICLKS_SPEC>;
#[doc = "Register `PICLKS` writer"]
pub type W = crate::W<PICLKS_SPEC>;
#[doc = "Field `USART1SEL` reader - USART1 clock select"]
pub type USART1SEL_R = crate::FieldReader;
#[doc = "Field `USART1SEL` writer - USART1 clock select"]
pub type USART1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USART2SEL` reader - USART2 clock select"]
pub type USART2SEL_R = crate::FieldReader;
#[doc = "Field `USART2SEL` writer - USART2 clock select"]
pub type USART2SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USART3SEL` reader - USART3 clock select"]
pub type USART3SEL_R = crate::FieldReader;
#[doc = "Field `USART3SEL` writer - USART3 clock select"]
pub type USART3SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `I2C1SEL` reader - I2C1 clock select"]
pub type I2C1SEL_R = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 clock select"]
pub type I2C1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock select"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock select"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock select"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock select"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PICLKS")
            .field("usart1sel", &format_args!("{}", self.usart1sel().bits()))
            .field("usart2sel", &format_args!("{}", self.usart2sel().bits()))
            .field("usart3sel", &format_args!("{}", self.usart3sel().bits()))
            .field("i2c1sel", &format_args!("{}", self.i2c1sel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PICLKS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock select"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<PICLKS_SPEC, 0> {
        USART1SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - USART2 clock select"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<PICLKS_SPEC, 2> {
        USART2SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 clock select"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<PICLKS_SPEC, 4> {
        USART3SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - I2C1 clock select"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<PICLKS_SPEC, 12> {
        I2C1SEL_W::new(self)
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
#[doc = "Peripheral independent clock register (CRM_PICLKS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`piclks::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`piclks::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PICLKS_SPEC;
impl crate::RegisterSpec for PICLKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`piclks::R`](R) reader structure"]
impl crate::Readable for PICLKS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`piclks::W`](W) writer structure"]
impl crate::Writable for PICLKS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PICLKS to value 0"]
impl crate::Resettable for PICLKS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
