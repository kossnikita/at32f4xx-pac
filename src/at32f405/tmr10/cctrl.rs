#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CCTRL_SPEC>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CCTRL_SPEC>;
#[doc = "Channel %s enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1enr {
    #[doc = "0: Channel is disabled"]
    Disabled = 0,
    #[doc = "1: Channel is enabled"]
    Enabled = 1,
}
impl From<C1enr> for bool {
    #[inline(always)]
    fn from(variant: C1enr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN(1-1)` reader - Channel %s enable"]
pub type CEN_R = crate::BitReader<C1enr>;
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1enr {
        match self.bits {
            false => C1enr::Disabled,
            true => C1enr::Enabled,
        }
    }
    #[doc = "Channel is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == C1enr::Disabled
    }
    #[doc = "Channel is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == C1enr::Enabled
    }
}
#[doc = "Channel %s enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1enwWO {
    #[doc = "0: Channel disable"]
    Disable = 0,
    #[doc = "1: Channel enable"]
    Enable = 1,
}
impl From<C1enwWO> for bool {
    #[inline(always)]
    fn from(variant: C1enwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN(1-1)` writer - Channel %s enable"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG, C1enwWO>;
impl<'a, REG> CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(C1enwWO::Disable)
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(C1enwWO::Enable)
    }
}
#[doc = "Channel %s polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1P_A {
    #[doc = "0: Output active high or Input active rising edge"]
    High = 0,
    #[doc = "1: Output active low or Input active falling edge"]
    Low = 1,
}
impl From<C1P_A> for bool {
    #[inline(always)]
    fn from(variant: C1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CP(1-1)` reader - Channel %s polarity"]
pub type CP_R = crate::BitReader<C1P_A>;
impl CP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1P_A {
        match self.bits {
            false => C1P_A::High,
            true => C1P_A::Low,
        }
    }
    #[doc = "Output active high or Input active rising edge"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == C1P_A::High
    }
    #[doc = "Output active low or Input active falling edge"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == C1P_A::Low
    }
}
#[doc = "Field `CP(1-1)` writer - Channel %s polarity"]
pub type CP_W<'a, REG> = crate::BitWriter<'a, REG, C1P_A>;
impl<'a, REG> CP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output active high or Input active rising edge"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(C1P_A::High)
    }
    #[doc = "Output active low or Input active falling edge"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(C1P_A::Low)
    }
}
#[doc = "Field `C1CEN` reader - Channel 1 complementary enable"]
pub type C1CEN_R = crate::BitReader;
#[doc = "Field `C1CEN` writer - Channel 1 complementary enable"]
pub type C1CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCP(1-1)` reader - Channel %s complementary polarity"]
pub use CP_R as CCP_R;
#[doc = "Field `CCP(1-1)` writer - Channel %s complementary polarity"]
pub use CP_W as CCP_W;
impl R {
    #[doc = "Channel (1-1) enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1EN` field.</div>"]
    #[inline(always)]
    pub fn cen(&self, n: u8) -> CEN_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CEN_R::new(((self.bits >> (n * 0)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) enable"]
    #[inline(always)]
    pub fn cen_iter(&self) -> impl Iterator<Item = CEN_R> + '_ {
        (0..1).map(move |n| CEN_R::new(((self.bits >> (n * 0)) & 1) != 0))
    }
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel (1-1) polarity"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1P` field.</div>"]
    #[inline(always)]
    pub fn cp(&self, n: u8) -> CP_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) polarity"]
    #[inline(always)]
    pub fn cp_iter(&self) -> impl Iterator<Item = CP_R> + '_ {
        (0..1).map(move |n| CP_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Channel 1 polarity"]
    #[inline(always)]
    pub fn c1p(&self) -> CP_R {
        CP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    pub fn c1cen(&self) -> C1CEN_R {
        C1CEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Channel (1-1) complementary polarity"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1CP` field.</div>"]
    #[inline(always)]
    pub fn ccp(&self, n: u8) -> CCP_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCP_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) complementary polarity"]
    #[inline(always)]
    pub fn ccp_iter(&self) -> impl Iterator<Item = CCP_R> + '_ {
        (0..1).map(move |n| CCP_R::new(((self.bits >> (n * 0 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    pub fn c1cp(&self) -> CCP_R {
        CCP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("c1cp", &self.c1cp())
            .field("c1cen", &self.c1cen())
            .field("c1p", &self.c1p())
            .field("c1en", &self.c1en())
            .finish()
    }
}
impl W {
    #[doc = "Channel (1-1) enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1EN` field.</div>"]
    #[inline(always)]
    pub fn cen(&mut self, n: u8) -> CEN_W<'_, CCTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CEN_W::new(self, n * 0)
    }
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&mut self) -> CEN_W<'_, CCTRL_SPEC> {
        CEN_W::new(self, 0)
    }
    #[doc = "Channel (1-1) polarity"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1P` field.</div>"]
    #[inline(always)]
    pub fn cp(&mut self, n: u8) -> CP_W<'_, CCTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CP_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Channel 1 polarity"]
    #[inline(always)]
    pub fn c1p(&mut self) -> CP_W<'_, CCTRL_SPEC> {
        CP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    pub fn c1cen(&mut self) -> C1CEN_W<'_, CCTRL_SPEC> {
        C1CEN_W::new(self, 2)
    }
    #[doc = "Channel (1-1) complementary polarity"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1CP` field.</div>"]
    #[inline(always)]
    pub fn ccp(&mut self, n: u8) -> CCP_W<'_, CCTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCP_W::new(self, n * 0 + 3)
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    pub fn c1cp(&mut self) -> CCP_W<'_, CCTRL_SPEC> {
        CCP_W::new(self, 3)
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCTRL_SPEC;
impl crate::RegisterSpec for CCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CCTRL_SPEC {}
