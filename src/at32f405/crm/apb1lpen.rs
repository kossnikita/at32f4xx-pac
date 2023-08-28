#[doc = "Register `APB1LPEN` reader"]
pub type R = crate::R<APB1LPEN_SPEC>;
#[doc = "Register `APB1LPEN` writer"]
pub type W = crate::W<APB1LPEN_SPEC>;
#[doc = "Field `TMR2LPEN` reader - Timer2 clock enable during sleep mode"]
pub type TMR2LPEN_R = crate::BitReader;
#[doc = "Field `TMR2LPEN` writer - Timer2 clock enable during sleep mode"]
pub type TMR2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR3LPEN` reader - Timer3 clock enable during sleep mode"]
pub type TMR3LPEN_R = crate::BitReader;
#[doc = "Field `TMR3LPEN` writer - Timer3 clock enable during sleep mode"]
pub type TMR3LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR4LPEN` reader - Timer4 clock enable during sleep mode"]
pub type TMR4LPEN_R = crate::BitReader;
#[doc = "Field `TMR4LPEN` writer - Timer4 clock enable during sleep mode"]
pub type TMR4LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR6LPEN` reader - Timer6 clock enable during sleep mode"]
pub type TMR6LPEN_R = crate::BitReader;
#[doc = "Field `TMR6LPEN` writer - Timer6 clock enable during sleep mode"]
pub type TMR6LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR7LPEN` reader - Timer7 clock enable during sleep mode"]
pub type TMR7LPEN_R = crate::BitReader;
#[doc = "Field `TMR7LPEN` writer - Timer7 clock enable during sleep mode"]
pub type TMR7LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR13LPEN` reader - Timer13 clock enable during sleep mode"]
pub type TMR13LPEN_R = crate::BitReader;
#[doc = "Field `TMR13LPEN` writer - Timer13 clock enable during sleep mode"]
pub type TMR13LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR14LPEN` reader - Timer14 clock enable during sleep mode"]
pub type TMR14LPEN_R = crate::BitReader;
#[doc = "Field `TMR14LPEN` writer - Timer14 clock enable during sleep mode"]
pub type TMR14LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDTLPEN` reader - WWDT clock enable during sleep mode"]
pub type WWDTLPEN_R = crate::BitReader;
#[doc = "Field `WWDTLPEN` writer - WWDT clock enable during sleep mode"]
pub type WWDTLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during sleep mode"]
pub type SPI2LPEN_R = crate::BitReader;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during sleep mode"]
pub type SPI2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during sleep mode"]
pub type SPI3LPEN_R = crate::BitReader;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during sleep mode"]
pub type SPI3LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during sleep mode"]
pub type USART2LPEN_R = crate::BitReader;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during sleep mode"]
pub type USART2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3LPEN` reader - USART3 clock enable during sleep mode"]
pub type USART3LPEN_R = crate::BitReader;
#[doc = "Field `USART3LPEN` writer - USART3 clock enable during sleep mode"]
pub type USART3LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART4LPEN` reader - USART4 clock enable during sleep mode"]
pub type USART4LPEN_R = crate::BitReader;
#[doc = "Field `USART4LPEN` writer - USART4 clock enable during sleep mode"]
pub type USART4LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART5LPEN` reader - USART5 clock enable during sleep mode"]
pub type USART5LPEN_R = crate::BitReader;
#[doc = "Field `USART5LPEN` writer - USART5 clock enable during sleep mode"]
pub type USART5LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1CPEN` reader - I2C1 clock enable during sleep mode"]
pub type I2C1CPEN_R = crate::BitReader;
#[doc = "Field `I2C1CPEN` writer - I2C1 clock enable during sleep mode"]
pub type I2C1CPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2CPEN` reader - I2C2 clock enable during sleep mode"]
pub type I2C2CPEN_R = crate::BitReader;
#[doc = "Field `I2C2CPEN` writer - I2C2 clock enable during sleep mode"]
pub type I2C2CPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C3CPEN` reader - I2C3 clock enable during sleep mode"]
pub type I2C3CPEN_R = crate::BitReader;
#[doc = "Field `I2C3CPEN` writer - I2C3 clock enable during sleep mode"]
pub type I2C3CPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN1LPEN` reader - CAN1 clock enable during sleep mode"]
pub type CAN1LPEN_R = crate::BitReader;
#[doc = "Field `CAN1LPEN` writer - CAN1 clock enable during sleep mode"]
pub type CAN1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWCLPEN` reader - PWC clock enable during sleep mode"]
pub type PWCLPEN_R = crate::BitReader;
#[doc = "Field `PWCLPEN` writer - PWC clock enable during sleep mode"]
pub type PWCLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART7LPEN` reader - UART7 clock enable during sleep mode"]
pub type UART7LPEN_R = crate::BitReader;
#[doc = "Field `UART7LPEN` writer - UART7 clock enable during sleep mode"]
pub type UART7LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART8LPEN` reader - UART8 clock enable during sleep mode"]
pub type UART8LPEN_R = crate::BitReader;
#[doc = "Field `UART8LPEN` writer - UART8 clock enable during sleep mode"]
pub type UART8LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr2lpen(&self) -> TMR2LPEN_R {
        TMR2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr3lpen(&self) -> TMR3LPEN_R {
        TMR3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr4lpen(&self) -> TMR4LPEN_R {
        TMR4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer6 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr6lpen(&self) -> TMR6LPEN_R {
        TMR6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer7 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr7lpen(&self) -> TMR7LPEN_R {
        TMR7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer13 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr13lpen(&self) -> TMR13LPEN_R {
        TMR13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer14 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tmr14lpen(&self) -> TMR14LPEN_R {
        TMR14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDT clock enable during sleep mode"]
    #[inline(always)]
    pub fn wwdtlpen(&self) -> WWDTLPEN_R {
        WWDTLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart4lpen(&self) -> USART4LPEN_R {
        USART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable during sleep mode"]
    #[inline(always)]
    pub fn usart5lpen(&self) -> USART5LPEN_R {
        USART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c1cpen(&self) -> I2C1CPEN_R {
        I2C1CPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c2cpen(&self) -> I2C2CPEN_R {
        I2C2CPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during sleep mode"]
    #[inline(always)]
    pub fn i2c3cpen(&self) -> I2C3CPEN_R {
        I2C3CPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn can1lpen(&self) -> CAN1LPEN_R {
        CAN1LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - PWC clock enable during sleep mode"]
    #[inline(always)]
    pub fn pwclpen(&self) -> PWCLPEN_R {
        PWCLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable during sleep mode"]
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable during sleep mode"]
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2lpen(&mut self) -> TMR2LPEN_W<APB1LPEN_SPEC, 0> {
        TMR2LPEN_W::new(self)
    }
    #[doc = "Bit 1 - Timer3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3lpen(&mut self) -> TMR3LPEN_W<APB1LPEN_SPEC, 1> {
        TMR3LPEN_W::new(self)
    }
    #[doc = "Bit 2 - Timer4 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4lpen(&mut self) -> TMR4LPEN_W<APB1LPEN_SPEC, 2> {
        TMR4LPEN_W::new(self)
    }
    #[doc = "Bit 4 - Timer6 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr6lpen(&mut self) -> TMR6LPEN_W<APB1LPEN_SPEC, 4> {
        TMR6LPEN_W::new(self)
    }
    #[doc = "Bit 5 - Timer7 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr7lpen(&mut self) -> TMR7LPEN_W<APB1LPEN_SPEC, 5> {
        TMR7LPEN_W::new(self)
    }
    #[doc = "Bit 7 - Timer13 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr13lpen(&mut self) -> TMR13LPEN_W<APB1LPEN_SPEC, 7> {
        TMR13LPEN_W::new(self)
    }
    #[doc = "Bit 8 - Timer14 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmr14lpen(&mut self) -> TMR14LPEN_W<APB1LPEN_SPEC, 8> {
        TMR14LPEN_W::new(self)
    }
    #[doc = "Bit 11 - WWDT clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdtlpen(&mut self) -> WWDTLPEN_W<APB1LPEN_SPEC, 11> {
        WWDTLPEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<APB1LPEN_SPEC, 14> {
        SPI2LPEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<APB1LPEN_SPEC, 15> {
        SPI3LPEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<APB1LPEN_SPEC, 17> {
        USART2LPEN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<APB1LPEN_SPEC, 18> {
        USART3LPEN_W::new(self)
    }
    #[doc = "Bit 19 - USART4 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart4lpen(&mut self) -> USART4LPEN_W<APB1LPEN_SPEC, 19> {
        USART4LPEN_W::new(self)
    }
    #[doc = "Bit 20 - USART5 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart5lpen(&mut self) -> USART5LPEN_W<APB1LPEN_SPEC, 20> {
        USART5LPEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1cpen(&mut self) -> I2C1CPEN_W<APB1LPEN_SPEC, 21> {
        I2C1CPEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2cpen(&mut self) -> I2C2CPEN_W<APB1LPEN_SPEC, 22> {
        I2C2CPEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3cpen(&mut self) -> I2C3CPEN_W<APB1LPEN_SPEC, 23> {
        I2C3CPEN_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn can1lpen(&mut self) -> CAN1LPEN_W<APB1LPEN_SPEC, 25> {
        CAN1LPEN_W::new(self)
    }
    #[doc = "Bit 28 - PWC clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwclpen(&mut self) -> PWCLPEN_W<APB1LPEN_SPEC, 28> {
        PWCLPEN_W::new(self)
    }
    #[doc = "Bit 30 - UART7 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W<APB1LPEN_SPEC, 30> {
        UART7LPEN_W::new(self)
    }
    #[doc = "Bit 31 - UART8 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W<APB1LPEN_SPEC, 31> {
        UART8LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB1 peripheral Low-power clock enable register (CRM_APB1LPEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lpen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lpen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LPEN_SPEC;
impl crate::RegisterSpec for APB1LPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lpen::R`](R) reader structure"]
impl crate::Readable for APB1LPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1lpen::W`](W) writer structure"]
impl crate::Writable for APB1LPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LPEN to value 0xf6fe_e9ff"]
impl crate::Resettable for APB1LPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0xf6fe_e9ff;
}
