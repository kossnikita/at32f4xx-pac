#[doc = "Register `CFGLR` reader"]
pub type R = crate::R<CFGLR_SPEC>;
#[doc = "Register `CFGLR` writer"]
pub type W = crate::W<CFGLR_SPEC>;
#[doc = "Field `IOMC[0-7]` reader - Port n.%s mode configurate bits"]
pub type IOMC_R = crate::FieldReader<IOMC0_A>;
#[doc = "Port n.%s mode configurate bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOMC0_A {
    #[doc = "0: Input mode"]
    Input = 0,
    #[doc = "1: Output mode, large sourcing/sinking strength"]
    OutputLarge = 1,
    #[doc = "2: Output mode, normal sourcing/sinking strength"]
    Output = 2,
    #[doc = "3: Output mode, maximum sourcing/sinking strength"]
    OutputMaximum = 3,
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
    pub fn variant(&self) -> IOMC0_A {
        match self.bits {
            0 => IOMC0_A::Input,
            1 => IOMC0_A::OutputLarge,
            2 => IOMC0_A::Output,
            3 => IOMC0_A::OutputMaximum,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == IOMC0_A::Input
    }
    #[doc = "Output mode, large sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_output_large(&self) -> bool {
        *self == IOMC0_A::OutputLarge
    }
    #[doc = "Output mode, normal sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == IOMC0_A::Output
    }
    #[doc = "Output mode, maximum sourcing/sinking strength"]
    #[inline(always)]
    pub fn is_output_maximum(&self) -> bool {
        *self == IOMC0_A::OutputMaximum
    }
}
#[doc = "Field `IOMC[0-7]` writer - Port n.%s mode configurate bits"]
pub type IOMC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, IOMC0_A>;
impl<'a, REG, const O: u8> IOMC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::Input)
    }
    #[doc = "Output mode, large sourcing/sinking strength"]
    #[inline(always)]
    pub fn output_large(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::OutputLarge)
    }
    #[doc = "Output mode, normal sourcing/sinking strength"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::Output)
    }
    #[doc = "Output mode, maximum sourcing/sinking strength"]
    #[inline(always)]
    pub fn output_maximum(self) -> &'a mut crate::W<REG> {
        self.variant(IOMC0_A::OutputMaximum)
    }
}
#[doc = "Field `IOFC[0-7]` reader - Port n.%s function configurate bits"]
pub type IOFC_R = crate::FieldReader<IOFC0_A>;
#[doc = "Port n.%s function configurate bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOFC0_A {
    #[doc = "0: Analog input"]
    Analog = 0,
    #[doc = "1: Floating input"]
    Floating = 1,
    #[doc = "2: Pull-down or pull-up input"]
    PullDownPullUp = 2,
    #[doc = "3: Multiplexed open drain"]
    MuxOpenDrain = 3,
}
impl From<IOFC0_A> for u8 {
    #[inline(always)]
    fn from(variant: IOFC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IOFC0_A {
    type Ux = u8;
}
impl IOFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOFC0_A {
        match self.bits {
            0 => IOFC0_A::Analog,
            1 => IOFC0_A::Floating,
            2 => IOFC0_A::PullDownPullUp,
            3 => IOFC0_A::MuxOpenDrain,
            _ => unreachable!(),
        }
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == IOFC0_A::Analog
    }
    #[doc = "Floating input"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == IOFC0_A::Floating
    }
    #[doc = "Pull-down or pull-up input"]
    #[inline(always)]
    pub fn is_pull_down_pull_up(&self) -> bool {
        *self == IOFC0_A::PullDownPullUp
    }
    #[doc = "Multiplexed open drain"]
    #[inline(always)]
    pub fn is_mux_open_drain(&self) -> bool {
        *self == IOFC0_A::MuxOpenDrain
    }
}
#[doc = "Field `IOFC[0-7]` writer - Port n.%s function configurate bits"]
pub type IOFC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, IOFC0_A>;
impl<'a, REG, const O: u8> IOFC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC0_A::Analog)
    }
    #[doc = "Floating input"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC0_A::Floating)
    }
    #[doc = "Pull-down or pull-up input"]
    #[inline(always)]
    pub fn pull_down_pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC0_A::PullDownPullUp)
    }
    #[doc = "Multiplexed open drain"]
    #[inline(always)]
    pub fn mux_open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(IOFC0_A::MuxOpenDrain)
    }
}
impl R {
    #[doc = "Port n.[0-7]
mode configurate bits"]
    #[inline(always)]
    pub unsafe fn iomc(&self, n: u8) -> IOMC_R {
        IOMC_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Port n.0 mode configurate bits"]
    #[inline(always)]
    pub fn iomc0(&self) -> IOMC_R {
        IOMC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode configurate bits"]
    #[inline(always)]
    pub fn iomc1(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode configurate bits"]
    #[inline(always)]
    pub fn iomc2(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode configurate bits"]
    #[inline(always)]
    pub fn iomc3(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode configurate bits"]
    #[inline(always)]
    pub fn iomc4(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode configurate bits"]
    #[inline(always)]
    pub fn iomc5(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode configurate bits"]
    #[inline(always)]
    pub fn iomc6(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode configurate bits"]
    #[inline(always)]
    pub fn iomc7(&self) -> IOMC_R {
        IOMC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Port n.[0-7]
function configurate bits"]
    #[inline(always)]
    pub unsafe fn iofc(&self, n: u8) -> IOFC_R {
        IOFC_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.0 function configurate bits"]
    #[inline(always)]
    pub fn iofc0(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 function configurate bits"]
    #[inline(always)]
    pub fn iofc1(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 function configurate bits"]
    #[inline(always)]
    pub fn iofc2(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 function configurate bits"]
    #[inline(always)]
    pub fn iofc3(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 function configurate bits"]
    #[inline(always)]
    pub fn iofc4(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 function configurate bits"]
    #[inline(always)]
    pub fn iofc5(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 function configurate bits"]
    #[inline(always)]
    pub fn iofc6(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 function configurate bits"]
    #[inline(always)]
    pub fn iofc7(&self) -> IOFC_R {
        IOFC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGLR")
            .field("iomc0", &format_args!("{}", self.iomc0().bits()))
            .field("iomc1", &format_args!("{}", self.iomc1().bits()))
            .field("iomc2", &format_args!("{}", self.iomc2().bits()))
            .field("iomc3", &format_args!("{}", self.iomc3().bits()))
            .field("iomc4", &format_args!("{}", self.iomc4().bits()))
            .field("iomc5", &format_args!("{}", self.iomc5().bits()))
            .field("iomc6", &format_args!("{}", self.iomc6().bits()))
            .field("iomc7", &format_args!("{}", self.iomc7().bits()))
            .field("iofc0", &format_args!("{}", self.iofc0().bits()))
            .field("iofc1", &format_args!("{}", self.iofc1().bits()))
            .field("iofc2", &format_args!("{}", self.iofc2().bits()))
            .field("iofc3", &format_args!("{}", self.iofc3().bits()))
            .field("iofc4", &format_args!("{}", self.iofc4().bits()))
            .field("iofc5", &format_args!("{}", self.iofc5().bits()))
            .field("iofc6", &format_args!("{}", self.iofc6().bits()))
            .field("iofc7", &format_args!("{}", self.iofc7().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFGLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Port n.[0-7]
mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn iomc<const O: u8>(&mut self) -> IOMC_W<CFGLR_SPEC, O> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 0:1 - Port n.0 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc0(&mut self) -> IOMC_W<CFGLR_SPEC, 0> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 4:5 - Port n.1 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc1(&mut self) -> IOMC_W<CFGLR_SPEC, 4> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 8:9 - Port n.2 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc2(&mut self) -> IOMC_W<CFGLR_SPEC, 8> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 12:13 - Port n.3 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc3(&mut self) -> IOMC_W<CFGLR_SPEC, 12> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 16:17 - Port n.4 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc4(&mut self) -> IOMC_W<CFGLR_SPEC, 16> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 20:21 - Port n.5 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc5(&mut self) -> IOMC_W<CFGLR_SPEC, 20> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 24:25 - Port n.6 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc6(&mut self) -> IOMC_W<CFGLR_SPEC, 24> {
        IOMC_W::new(self)
    }
    #[doc = "Bits 28:29 - Port n.7 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc7(&mut self) -> IOMC_W<CFGLR_SPEC, 28> {
        IOMC_W::new(self)
    }
    #[doc = "Port n.[0-7]
function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn iofc<const O: u8>(&mut self) -> IOFC_W<CFGLR_SPEC, O> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 2:3 - Port n.0 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc0(&mut self) -> IOFC_W<CFGLR_SPEC, 2> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 6:7 - Port n.1 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc1(&mut self) -> IOFC_W<CFGLR_SPEC, 6> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 10:11 - Port n.2 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc2(&mut self) -> IOFC_W<CFGLR_SPEC, 10> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 14:15 - Port n.3 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc3(&mut self) -> IOFC_W<CFGLR_SPEC, 14> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 18:19 - Port n.4 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc4(&mut self) -> IOFC_W<CFGLR_SPEC, 18> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 22:23 - Port n.5 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc5(&mut self) -> IOFC_W<CFGLR_SPEC, 22> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 26:27 - Port n.6 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc6(&mut self) -> IOFC_W<CFGLR_SPEC, 26> {
        IOFC_W::new(self)
    }
    #[doc = "Bits 30:31 - Port n.7 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc7(&mut self) -> IOFC_W<CFGLR_SPEC, 30> {
        IOFC_W::new(self)
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
#[doc = "GPIO function configurate low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGLR_SPEC;
impl crate::RegisterSpec for CFGLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfglr::R`](R) reader structure"]
impl crate::Readable for CFGLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfglr::W`](W) writer structure"]
impl crate::Writable for CFGLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGLR to value 0x4444_4444"]
impl crate::Resettable for CFGLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
