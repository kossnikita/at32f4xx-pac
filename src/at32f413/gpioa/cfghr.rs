#[doc = "Register `CFGHR` reader"]
pub type R = crate::R<CFGHR_SPEC>;
#[doc = "Register `CFGHR` writer"]
pub type W = crate::W<CFGHR_SPEC>;
#[doc = "Field `IOMC[8-15]` reader - Port n.8 mode configurate bits"]
pub type IOMC_R = crate::FieldReader<IOMC8_A>;
#[doc = "Port n.8 mode configurate bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOMC8_A {
    #[doc = "0: Input mode"]
    Input = 0,
    #[doc = "1: Output mode, large sourcing/sinking strength"]
    OutputLarge = 1,
    #[doc = "2: Output mode, normal sourcing/sinking strength"]
    Output = 2,
    #[doc = "3: Output mode, maximum sourcing/sinking strength"]
    OutputMaximum = 3,
}
impl From<IOMC8_A> for u8 {
    #[inline(always)]
    fn from(variant: IOMC8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IOMC8_A {
    type Ux = u8;
}
impl IOMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOMC8_A {
        match self.bits {
            0 => IOMC8_A::Input,
            1 => IOMC8_A::OutputLarge,
            2 => IOMC8_A::Output,
            3 => IOMC8_A::OutputMaximum,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == IOMC8_A::Input
    }
    #[doc = "Output mode, large sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_output_large(&self) -> bool {
        *self == IOMC8_A::OutputLarge
    }
    #[doc = "Output mode, normal sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == IOMC8_A::Output
    }
    #[doc = "Output mode, maximum sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_output_maximum(&self) -> bool {
        *self == IOMC8_A::OutputMaximum
    }
}
#[doc = "Field `IOMC[8-15]` writer - Port n.8 mode configurate bits"]
pub type IOMC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, IOMC8_A>;
impl<'a, REG> IOMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC8_A::Input)
    }
    #[doc = "Output mode, large sourcing/sinking strength"]
    #[inline(always)]
    pub fn output_large(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC8_A::OutputLarge)
    }
    #[doc = "Output mode, normal sourcing/sinking strength"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC8_A::Output)
    }
    #[doc = "Output mode, maximum sourcing/sinking strength"]
    #[inline(always)]
    pub fn output_maximum(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC8_A::OutputMaximum)
    }
}
#[doc = "Field `IOFC[8-15]` reader - Port n.8 function configurate bits"]
pub type IOFC_R = crate::FieldReader<IOFC8_A>;
#[doc = "Port n.8 function configurate bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOFC8_A {
    #[doc = "0: Analog input"]
    Analog = 0,
    #[doc = "1: Floating input"]
    Floating = 1,
    #[doc = "2: Pull-down or pull-up input"]
    PullDownPullUp = 2,
    #[doc = "3: Multiplexed open drain"]
    MuxOpenDrain = 3,
}
impl From<IOFC8_A> for u8 {
    #[inline(always)]
    fn from(variant: IOFC8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IOFC8_A {
    type Ux = u8;
}
impl IOFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOFC8_A {
        match self.bits {
            0 => IOFC8_A::Analog,
            1 => IOFC8_A::Floating,
            2 => IOFC8_A::PullDownPullUp,
            3 => IOFC8_A::MuxOpenDrain,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == IOFC8_A::Analog
    }
    #[doc = "Floating input"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == IOFC8_A::Floating
    }
    #[doc = "Pull-down or pull-up input"]
    #[inline(always)]
    pub fn is_pull_down_pull_up(&self) -> bool {
        *self == IOFC8_A::PullDownPullUp
    }
    #[doc = "Multiplexed open drain"]
    #[inline(always)]
    pub fn is_mux_open_drain(&self) -> bool {
        *self == IOFC8_A::MuxOpenDrain
    }
}
#[doc = "Field `IOFC[8-15]` writer - Port n.8 function configurate bits"]
pub type IOFC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, IOFC8_A>;
impl<'a, REG> IOFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC8_A::Analog)
    }
    #[doc = "Floating input"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC8_A::Floating)
    }
    #[doc = "Pull-down or pull-up input"]
    #[inline(always)]
    pub fn pull_down_pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC8_A::PullDownPullUp)
    }
    #[doc = "Multiplexed open drain"]
    #[inline(always)]
    pub fn mux_open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC8_A::MuxOpenDrain)
    }
}
impl R {
    #[doc = "Port n.8 mode configurate bits\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn iomc(&self, n: u8) -> IOMC_R {
        assert!(n < 8);
        IOMC_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc8(&self) -> IOMC_R {
        IOMC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc9(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc10(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc11(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc12(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc13(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc14(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc15(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Port n.8 function configurate bits\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn iofc(&self, n: u8) -> IOFC_R {
        assert!(n < 8);
        IOFC_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc8(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc9(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc10(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc11(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc12(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc13(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc14(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc15(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGHR")
            .field("iomc8", &format_args!("{}", self.iomc8().bits()))
            .field("iomc9", &format_args!("{}", self.iomc9().bits()))
            .field("iomc10", &format_args!("{}", self.iomc10().bits()))
            .field("iomc11", &format_args!("{}", self.iomc11().bits()))
            .field("iomc12", &format_args!("{}", self.iomc12().bits()))
            .field("iomc13", &format_args!("{}", self.iomc13().bits()))
            .field("iomc14", &format_args!("{}", self.iomc14().bits()))
            .field("iomc15", &format_args!("{}", self.iomc15().bits()))
            .field("iofc8", &format_args!("{}", self.iofc8().bits()))
            .field("iofc9", &format_args!("{}", self.iofc9().bits()))
            .field("iofc10", &format_args!("{}", self.iofc10().bits()))
            .field("iofc11", &format_args!("{}", self.iofc11().bits()))
            .field("iofc12", &format_args!("{}", self.iofc12().bits()))
            .field("iofc13", &format_args!("{}", self.iofc13().bits()))
            .field("iofc14", &format_args!("{}", self.iofc14().bits()))
            .field("iofc15", &format_args!("{}", self.iofc15().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFGHR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc(&mut self, n: u8) -> IOMC_W<CFGHR_SPEC> {
        assert!(n < 8);
        IOMC_W::new(self, n * 4)
    }
    #[doc = "Bits 0:1 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc8(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc9(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc10(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc11(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc12(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc13(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc14(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc15(&mut self) -> IOMC_W<CFGHR_SPEC> {
        IOMC_W::new(self, 28)
    }
    #[doc = "Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc(&mut self, n: u8) -> IOFC_W<CFGHR_SPEC> {
        assert!(n < 8);
        IOFC_W::new(self, n * 4 + 2)
    }
    #[doc = "Bits 2:3 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc8(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc9(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc10(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc11(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 14)
    }
    #[doc = "Bits 18:19 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc12(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 18)
    }
    #[doc = "Bits 22:23 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc13(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc14(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 26)
    }
    #[doc = "Bits 30:31 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc15(&mut self) -> IOFC_W<CFGHR_SPEC> {
        IOFC_W::new(self, 30)
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
#[doc = "GPIO function configurate high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfghr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfghr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGHR_SPEC;
impl crate::RegisterSpec for CFGHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfghr::R`](R) reader structure"]
impl crate::Readable for CFGHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfghr::W`](W) writer structure"]
impl crate::Writable for CFGHR_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGHR to value 0x4444_4444"]
impl crate::Resettable for CFGHR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
