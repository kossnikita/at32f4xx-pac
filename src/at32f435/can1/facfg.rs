#[doc = "Register `FACFG` reader"]
pub type R = crate::R<FACFG_SPEC>;
#[doc = "Register `FACFG` writer"]
pub type W = crate::W<FACFG_SPEC>;
#[doc = "Filter activate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En0r {
    #[doc = "0: Filter is disabled"]
    Disabled = 0,
    #[doc = "1: Filter is enabled"]
    Enabled = 1,
}
impl From<En0r> for bool {
    #[inline(always)]
    fn from(variant: En0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN(0-27)` reader - Filter activate enable"]
pub type EN_R = crate::BitReader<En0r>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En0r {
        match self.bits {
            false => En0r::Disabled,
            true => En0r::Enabled,
        }
    }
    #[doc = "Filter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En0r::Disabled
    }
    #[doc = "Filter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En0r::Enabled
    }
}
#[doc = "Filter activate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En0wWO {
    #[doc = "0: Filter disable"]
    Disable = 0,
    #[doc = "1: Filter enable"]
    Enable = 1,
}
impl From<En0wWO> for bool {
    #[inline(always)]
    fn from(variant: En0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN(0-27)` writer - Filter activate enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, En0wWO>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En0wWO::Disable)
    }
    #[doc = "Filter enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En0wWO::Enable)
    }
}
impl R {
    #[doc = "Filter activate enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `EN0` field"]
    #[inline(always)]
    pub fn en(&self, n: u8) -> EN_R {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
        EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Filter activate enable"]
    #[inline(always)]
    pub fn en_iter(&self) -> impl Iterator<Item = EN_R> + '_ {
        (0..28).map(move |n| EN_R::new(((self.bits >> n) & 1) != 0))
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
    #[doc = "Bit 14 - Filter activate enable"]
    #[inline(always)]
    pub fn en14(&self) -> EN_R {
        EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter activate enable"]
    #[inline(always)]
    pub fn en15(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter activate enable"]
    #[inline(always)]
    pub fn en16(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter activate enable"]
    #[inline(always)]
    pub fn en17(&self) -> EN_R {
        EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter activate enable"]
    #[inline(always)]
    pub fn en18(&self) -> EN_R {
        EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter activate enable"]
    #[inline(always)]
    pub fn en19(&self) -> EN_R {
        EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter activate enable"]
    #[inline(always)]
    pub fn en20(&self) -> EN_R {
        EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter activate enable"]
    #[inline(always)]
    pub fn en21(&self) -> EN_R {
        EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter activate enable"]
    #[inline(always)]
    pub fn en22(&self) -> EN_R {
        EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter activate enable"]
    #[inline(always)]
    pub fn en23(&self) -> EN_R {
        EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter activate enable"]
    #[inline(always)]
    pub fn en24(&self) -> EN_R {
        EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter activate enable"]
    #[inline(always)]
    pub fn en25(&self) -> EN_R {
        EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter activate enable"]
    #[inline(always)]
    pub fn en26(&self) -> EN_R {
        EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter activate enable"]
    #[inline(always)]
    pub fn en27(&self) -> EN_R {
        EN_R::new(((self.bits >> 27) & 1) != 0)
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
            .field("en14", &format_args!("{}", self.en14().bit()))
            .field("en15", &format_args!("{}", self.en15().bit()))
            .field("en16", &format_args!("{}", self.en16().bit()))
            .field("en17", &format_args!("{}", self.en17().bit()))
            .field("en18", &format_args!("{}", self.en18().bit()))
            .field("en19", &format_args!("{}", self.en19().bit()))
            .field("en20", &format_args!("{}", self.en20().bit()))
            .field("en21", &format_args!("{}", self.en21().bit()))
            .field("en22", &format_args!("{}", self.en22().bit()))
            .field("en23", &format_args!("{}", self.en23().bit()))
            .field("en24", &format_args!("{}", self.en24().bit()))
            .field("en25", &format_args!("{}", self.en25().bit()))
            .field("en26", &format_args!("{}", self.en26().bit()))
            .field("en27", &format_args!("{}", self.en27().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FACFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Filter activate enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `EN0` field"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self, n: u8) -> EN_W<FACFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 28][n as usize];
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
    #[doc = "Bit 14 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en14(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en15(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en16(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en17(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en18(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en19(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en20(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en21(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en22(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en23(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en24(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en25(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en26(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn en27(&mut self) -> EN_W<FACFG_SPEC> {
        EN_W::new(self, 27)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FACFG to value 0"]
impl crate::Resettable for FACFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
