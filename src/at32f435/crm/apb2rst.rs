#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `TMR1RST` reader - Timer1 reset"]
pub type TMR1RST_R = crate::BitReader;
#[doc = "Field `TMR1RST` writer - Timer1 reset"]
pub type TMR1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR8RST` reader - Timer8 reset"]
pub type TMR8RST_R = crate::BitReader;
#[doc = "Field `TMR8RST` writer - Timer8 reset"]
pub type TMR8RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6RST` reader - USART6 reset"]
pub type USART6RST_R = crate::BitReader;
#[doc = "Field `USART6RST` writer - USART6 reset"]
pub type USART6RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type ADCRST_R = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type ADCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4RST` reader - SPI4 reset"]
pub type SPI4RST_R = crate::BitReader;
#[doc = "Field `SPI4RST` writer - SPI4 reset"]
pub type SPI4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCFGRST` reader - SCFG reset"]
pub type SCFGRST_R = crate::BitReader;
#[doc = "Field `SCFGRST` writer - SCFG reset"]
pub type SCFGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR9RST` reader - Timer9 reset"]
pub type TMR9RST_R = crate::BitReader;
#[doc = "Field `TMR9RST` writer - Timer9 reset"]
pub type TMR9RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR10RST` reader - Timer10 reset"]
pub type TMR10RST_R = crate::BitReader;
#[doc = "Field `TMR10RST` writer - Timer10 reset"]
pub type TMR10RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR11RST` reader - Timer 11 reset"]
pub type TMR11RST_R = crate::BitReader;
#[doc = "Field `TMR11RST` writer - Timer 11 reset"]
pub type TMR11RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR20RST` reader - Timer20 reset"]
pub type TMR20RST_R = crate::BitReader;
#[doc = "Field `TMR20RST` writer - Timer20 reset"]
pub type TMR20RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCRST` reader - ACC reset"]
pub type ACCRST_R = crate::BitReader;
#[doc = "Field `ACCRST` writer - ACC reset"]
pub type ACCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> TMR1RST_R {
        TMR1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer8 reset"]
    #[inline(always)]
    pub fn tmr8rst(&self) -> TMR8RST_R {
        TMR8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    pub fn scfgrst(&self) -> SCFGRST_R {
        SCFGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9rst(&self) -> TMR9RST_R {
        TMR9RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10rst(&self) -> TMR10RST_R {
        TMR10RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    pub fn tmr11rst(&self) -> TMR11RST_R {
        TMR11RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer20 reset"]
    #[inline(always)]
    pub fn tmr20rst(&self) -> TMR20RST_R {
        TMR20RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    pub fn accrst(&self) -> ACCRST_R {
        ACCRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1rst(&mut self) -> TMR1RST_W<APB2RST_SPEC, 0> {
        TMR1RST_W::new(self)
    }
    #[doc = "Bit 1 - Timer8 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr8rst(&mut self) -> TMR8RST_W<APB2RST_SPEC, 1> {
        TMR8RST_W::new(self)
    }
    #[doc = "Bit 4 - USART1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RST_SPEC, 4> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 5 - USART6 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<APB2RST_SPEC, 5> {
        USART6RST_W::new(self)
    }
    #[doc = "Bit 8 - ADC reset"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<APB2RST_SPEC, 8> {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RST_SPEC, 12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<APB2RST_SPEC, 13> {
        SPI4RST_W::new(self)
    }
    #[doc = "Bit 14 - SCFG reset"]
    #[inline(always)]
    #[must_use]
    pub fn scfgrst(&mut self) -> SCFGRST_W<APB2RST_SPEC, 14> {
        SCFGRST_W::new(self)
    }
    #[doc = "Bit 16 - Timer9 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9rst(&mut self) -> TMR9RST_W<APB2RST_SPEC, 16> {
        TMR9RST_W::new(self)
    }
    #[doc = "Bit 17 - Timer10 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10rst(&mut self) -> TMR10RST_W<APB2RST_SPEC, 17> {
        TMR10RST_W::new(self)
    }
    #[doc = "Bit 18 - Timer 11 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11rst(&mut self) -> TMR11RST_W<APB2RST_SPEC, 18> {
        TMR11RST_W::new(self)
    }
    #[doc = "Bit 20 - Timer20 reset"]
    #[inline(always)]
    #[must_use]
    pub fn tmr20rst(&mut self) -> TMR20RST_W<APB2RST_SPEC, 20> {
        TMR20RST_W::new(self)
    }
    #[doc = "Bit 29 - ACC reset"]
    #[inline(always)]
    #[must_use]
    pub fn accrst(&mut self) -> ACCRST_W<APB2RST_SPEC, 29> {
        ACCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
