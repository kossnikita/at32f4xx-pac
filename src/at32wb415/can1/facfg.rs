#[doc = "Register `FACFG` reader"]
pub type R = crate::R<FACFG_SPEC>;
#[doc = "Register `FACFG` writer"]
pub type W = crate::W<FACFG_SPEC>;
#[doc = "Field `EN[0-13]` reader - Filter activate enable"]
pub type EN_R = crate::BitReader<EN0R_A>;
#[doc = "Filter activate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN0R_A {
    #[doc = "0: Filter is disabled"]
    Disabled = 0,
    #[doc = "1: Filter is enabled"]
    Enabled = 1,
}
impl From<EN0R_A> for bool {
    #[inline(always)]
    fn from(variant: EN0R_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN0R_A {
        match self.bits {
            false => EN0R_A::Disabled,
            true => EN0R_A::Enabled,
        }
    }
    #[doc = "Filter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN0R_A::Disabled
    }
    #[doc = "Filter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN0R_A::Enabled
    }
}
#[doc = "Filter activate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN0W_AW {
    #[doc = "0: Filter disable"]
    Disable = 0,
    #[doc = "1: Filter enable"]
    Enable = 1,
}
impl From<EN0W_AW> for bool {
    #[inline(always)]
    fn from(variant: EN0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN[0-13]` writer - Filter activate enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN0W_AW>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EN0W_AW::Disable)
    }
    #[doc = "Filter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EN0W_AW::Enable)
    }
}
impl R {
    #[doc = "Filter activate enable\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn en(&self, n: u8) -> EN_R {
        assert!(n < 14);
        EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    pub fn en0(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN_R {
        EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    pub fn en3(&self) -> EN_R {
        EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    pub fn en4(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    pub fn en5(&self) -> EN_R {
        EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    pub fn en6(&self) -> EN_R {
        EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    pub fn en7(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    pub fn en8(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    pub fn en9(&self) -> EN_R {
        EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    pub fn en10(&self) -> EN_R {
        EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    pub fn en11(&self) -> EN_R {
        EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    pub fn en12(&self) -> EN_R {
        EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    pub fn en13(&self) -> EN_R {
        EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FACFG")
            .field("en0", &format_args!("{}", self.en0().bit()))
            .field("en1", &format_args!("{}", self.en1().bit()))
            .field("en2", &format_args!("{}", self.en2().bit()))
            .field("en3", &format_args!("{}", self.en3().bit()))
            .field("en4", &format_args!("{}", self.en4().bit()))
            .field("en5", &format_args!("{}", self.en5().bit()))
            .field("en6", &format_args!("{}", self.en6().bit()))
            .field("en7", &format_args!("{}", self.en7().bit()))
            .field("en8", &format_args!("{}", self.en8().bit()))
            .field("en9", &format_args!("{}", self.en9().bit()))
            .field("en10", &format_args!("{}", self.en10().bit()))
            .field("en11", &format_args!("{}", self.en11().bit()))
            .field("en12", &format_args!("{}", self.en12().bit()))
            .field("en13", &format_args!("{}", self.en13().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FACFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self, n: u8) -> EN_W<FACFG_SPEC> {
        assert!(n < 14);
        EN_W::new(self, n)
    }
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en5(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en6(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en7(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en8(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en9(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en10(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en11(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en12(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en13(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 13)
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
#[doc = "Filter activate configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`facfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`facfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FACFG_SPEC;
impl crate::RegisterSpec for FACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`facfg::R`](R) reader structure"]
impl crate::Readable for FACFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`facfg::W`](W) writer structure"]
impl crate::Writable for FACFG_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FACFG to value 0"]
impl crate::Resettable for FACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
