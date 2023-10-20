#[doc = "Register `ODT` reader"]
pub type R = crate::R<ODT_SPEC>;
#[doc = "Register `ODT` writer"]
pub type W = crate::W<ODT_SPEC>;
#[doc = "Field `ODT[0-15]` reader - Port output data"]
pub type ODT_R = crate::BitReader<ODT0R_A>;
#[doc = "Port output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODT0R_A {
    #[doc = "0: Pull-down"]
    PullDown = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
}
impl From<ODT0R_A> for bool {
    #[inline(always)]
    fn from(variant: ODT0R_A) -> Self {
        variant as u8 != 0
    }
}
impl ODT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODT0R_A {
        match self.bits {
            false => ODT0R_A::PullDown,
            true => ODT0R_A::PullUp,
        }
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == ODT0R_A::PullDown
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == ODT0R_A::PullUp
    }
}
#[doc = "Port output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODT0W_AW {
    #[doc = "0: Set output to logic low"]
    Low = 0,
    #[doc = "1: Set output to logic high"]
    High = 1,
}
impl From<ODT0W_AW> for bool {
    #[inline(always)]
    fn from(variant: ODT0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODT[0-15]` writer - Port output data"]
pub type ODT_W<'a, REG> = crate::BitWriter<'a, REG, ODT0W_AW>;
impl<'a, REG> ODT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set output to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ODT0W_AW::Low)
    }
    #[doc = "Set output to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ODT0W_AW::High)
    }
}
impl R {
    #[doc = "Port output data\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn odt(&self, n: u8) -> ODT_R {
        assert!(n < 16);
        ODT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    pub fn odt0(&self) -> ODT_R {
        ODT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    pub fn odt1(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    pub fn odt2(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    pub fn odt3(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    pub fn odt4(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    pub fn odt5(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    pub fn odt6(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    pub fn odt7(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    pub fn odt8(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    pub fn odt9(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    pub fn odt10(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    pub fn odt11(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    pub fn odt12(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    pub fn odt13(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    pub fn odt14(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    pub fn odt15(&self) -> ODT_R {
        ODT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODT")
            .field("odt0", &format_args!("{}", self.odt0().bit()))
            .field("odt1", &format_args!("{}", self.odt1().bit()))
            .field("odt2", &format_args!("{}", self.odt2().bit()))
            .field("odt3", &format_args!("{}", self.odt3().bit()))
            .field("odt4", &format_args!("{}", self.odt4().bit()))
            .field("odt5", &format_args!("{}", self.odt5().bit()))
            .field("odt6", &format_args!("{}", self.odt6().bit()))
            .field("odt7", &format_args!("{}", self.odt7().bit()))
            .field("odt8", &format_args!("{}", self.odt8().bit()))
            .field("odt9", &format_args!("{}", self.odt9().bit()))
            .field("odt10", &format_args!("{}", self.odt10().bit()))
            .field("odt11", &format_args!("{}", self.odt11().bit()))
            .field("odt12", &format_args!("{}", self.odt12().bit()))
            .field("odt13", &format_args!("{}", self.odt13().bit()))
            .field("odt14", &format_args!("{}", self.odt14().bit()))
            .field("odt15", &format_args!("{}", self.odt15().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ODT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt(&mut self, n: u8) -> ODT_W<ODT_SPEC> {
        assert!(n < 16);
        ODT_W::new(self, n)
    }
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt0(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt1(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt2(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt3(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt4(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt5(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt6(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt7(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt8(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt9(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt10(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt11(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt12(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt13(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt14(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt15(&mut self) -> ODT_W<ODT_SPEC> {
        ODT_W::new(self, 15)
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
#[doc = "Port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODT_SPEC;
impl crate::RegisterSpec for ODT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for ODT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`odt::W`](W) writer structure"]
impl crate::Writable for ODT_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for ODT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
