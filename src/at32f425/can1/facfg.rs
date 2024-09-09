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
#[doc = "Field `EN(0-13)` reader - Filter activate enable"]
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
#[doc = "Field `EN(0-13)` writer - Filter activate enable"]
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
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EN0` field.</div>"]
    #[inline(always)]
    pub fn en(&self, n: u8) -> EN_R {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
        EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Filter activate enable"]
    #[inline(always)]
    pub fn en_iter(&self) -> impl Iterator<Item = EN_R> + '_ {
        (0..14).map(move |n| EN_R::new(((self.bits >> n) & 1) != 0))
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
            .field("en0", &self.en0())
            .field("en1", &self.en1())
            .field("en2", &self.en2())
            .field("en3", &self.en3())
            .field("en4", &self.en4())
            .field("en5", &self.en5())
            .field("en6", &self.en6())
            .field("en7", &self.en7())
            .field("en8", &self.en8())
            .field("en9", &self.en9())
            .field("en10", &self.en10())
            .field("en11", &self.en11())
            .field("en12", &self.en12())
            .field("en13", &self.en13())
            .finish()
    }
}
impl W {
    #[doc = "Filter activate enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EN0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self, n: u8) -> EN_W<FACFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 14][n as usize];
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
}
#[doc = "Filter activate configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`facfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`facfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
