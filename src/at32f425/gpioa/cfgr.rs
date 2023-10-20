#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `IOMC[0-15]` reader - GPIOx pin %s mode configurate"]
pub type IOMC_R = crate::FieldReader<IOMC0_A>;
#[doc = "GPIOx pin %s mode configurate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOMC0_A {
    #[doc = "0: Input mode"]
    Input = 0,
    #[doc = "1: General-purpose output mode"]
    Output = 1,
    #[doc = "2: Multiplexed function mode"]
    Multiplexed = 2,
    #[doc = "3: Analog mode"]
    Analog = 3,
}
impl From<IOMC0_A> for u8 {
    #[inline(always)]
    fn from(variant: IOMC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IOMC0_A {
    type Ux = u8;
}
impl IOMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOMC0_A {
        match self.bits {
            0 => IOMC0_A::Input,
            1 => IOMC0_A::Output,
            2 => IOMC0_A::Multiplexed,
            3 => IOMC0_A::Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == IOMC0_A::Input
    }
    #[doc = "General-purpose output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == IOMC0_A::Output
    }
    #[doc = "Multiplexed function mode"]
    #[inline(always)]
    pub fn is_multiplexed(&self) -> bool {
        *self == IOMC0_A::Multiplexed
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == IOMC0_A::Analog
    }
}
#[doc = "Field `IOMC[0-15]` writer - GPIOx pin %s mode configurate"]
pub type IOMC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, IOMC0_A>;
impl<'a, REG> IOMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::Input)
    }
    #[doc = "General-purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::Output)
    }
    #[doc = "Multiplexed function mode"]
    #[inline(always)]
    pub fn multiplexed(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::Multiplexed)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::Analog)
    }
}
impl R {
    #[doc = "GPIOx pin [0-15]
mode configurate\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn iomc(&self, n: u8) -> IOMC_R {
        assert!(n < 16);
        IOMC_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - GPIOx pin 0 mode configurate"]
    #[inline(always)]
    pub fn iomc0(&self) -> IOMC_R {
        IOMC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 mode configurate"]
    #[inline(always)]
    pub fn iomc1(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 mode configurate"]
    #[inline(always)]
    pub fn iomc2(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 mode configurate"]
    #[inline(always)]
    pub fn iomc3(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 mode configurate"]
    #[inline(always)]
    pub fn iomc4(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 mode configurate"]
    #[inline(always)]
    pub fn iomc5(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 mode configurate"]
    #[inline(always)]
    pub fn iomc6(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 mode configurate"]
    #[inline(always)]
    pub fn iomc7(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 mode configurate"]
    #[inline(always)]
    pub fn iomc8(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 mode configurate"]
    #[inline(always)]
    pub fn iomc9(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 mode configurate"]
    #[inline(always)]
    pub fn iomc10(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 mode configurate"]
    #[inline(always)]
    pub fn iomc11(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 mode configurate"]
    #[inline(always)]
    pub fn iomc12(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 mode configurate"]
    #[inline(always)]
    pub fn iomc13(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 mode configurate"]
    #[inline(always)]
    pub fn iomc14(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 mode configurate"]
    #[inline(always)]
    pub fn iomc15(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("iomc0", &format_args!("{}", self.iomc0().bits()))
            .field("iomc1", &format_args!("{}", self.iomc1().bits()))
            .field("iomc2", &format_args!("{}", self.iomc2().bits()))
            .field("iomc3", &format_args!("{}", self.iomc3().bits()))
            .field("iomc4", &format_args!("{}", self.iomc4().bits()))
            .field("iomc5", &format_args!("{}", self.iomc5().bits()))
            .field("iomc6", &format_args!("{}", self.iomc6().bits()))
            .field("iomc7", &format_args!("{}", self.iomc7().bits()))
            .field("iomc8", &format_args!("{}", self.iomc8().bits()))
            .field("iomc9", &format_args!("{}", self.iomc9().bits()))
            .field("iomc10", &format_args!("{}", self.iomc10().bits()))
            .field("iomc11", &format_args!("{}", self.iomc11().bits()))
            .field("iomc12", &format_args!("{}", self.iomc12().bits()))
            .field("iomc13", &format_args!("{}", self.iomc13().bits()))
            .field("iomc14", &format_args!("{}", self.iomc14().bits()))
            .field("iomc15", &format_args!("{}", self.iomc15().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFGR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "GPIOx pin [0-15]
mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc(&mut self, n: u8) -> IOMC_W<CFGR_SPEC> {
        assert!(n < 16);
        IOMC_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - GPIOx pin 0 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc0(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc1(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc2(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc3(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc4(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc5(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc6(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc7(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc8(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc9(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc10(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc11(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc12(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc13(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc14(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc15(&mut self) -> IOMC_W<CFGR_SPEC> {
        IOMC_W::new(self, 30)
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
#[doc = "GPIO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
