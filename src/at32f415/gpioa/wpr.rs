#[doc = "Register `WPR` reader"]
pub type R = crate::R<WPR_SPEC>;
#[doc = "Register `WPR` writer"]
pub type W = crate::W<WPR_SPEC>;
#[doc = "Field `WPEN[0-15]` reader - Write protect enable %s"]
pub type WPEN_R = crate::BitReader<WPEN0R_A>;
#[doc = "Write protect enable %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPEN0R_A {
    #[doc = "0: Port is not write protected"]
    NotProtected = 0,
    #[doc = "1: Port is write protected"]
    Protected = 1,
}
impl From<WPEN0R_A> for bool {
    #[inline(always)]
    fn from(variant: WPEN0R_A) -> Self {
        variant as u8 != 0
    }
}
impl WPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPEN0R_A {
        match self.bits {
            false => WPEN0R_A::NotProtected,
            true => WPEN0R_A::Protected,
        }
    }
    #[doc = "Port is not write protected"]
    #[inline(always)]
    pub fn is_not_protected(&self) -> bool {
        *self == WPEN0R_A::NotProtected
    }
    #[doc = "Port is write protected"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == WPEN0R_A::Protected
    }
}
#[doc = "Write protect enable %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPEN0W_AW {
    #[doc = "0: No effect"]
    NoProtect = 0,
    #[doc = "1: Write protect"]
    Protect = 1,
}
impl From<WPEN0W_AW> for bool {
    #[inline(always)]
    fn from(variant: WPEN0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPEN[0-15]` writer - Write protect enable %s"]
pub type WPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WPEN0W_AW>;
impl<'a, REG, const O: u8> WPEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_protect(self) -> &'a mut crate::W<REG> {
        self.variant(WPEN0W_AW::NoProtect)
    }
    #[doc = "Write protect"]
    #[inline(always)]
    pub fn protect(self) -> &'a mut crate::W<REG> {
        self.variant(WPEN0W_AW::Protect)
    }
}
#[doc = "Field `WPSEQ` reader - Write protect sequence"]
pub type WPSEQ_R = crate::BitReader<WPSEQ_A>;
#[doc = "Write protect sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPSEQ_A {
    #[doc = "0: Clear"]
    Clear = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<WPSEQ_A> for bool {
    #[inline(always)]
    fn from(variant: WPSEQ_A) -> Self {
        variant as u8 != 0
    }
}
impl WPSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSEQ_A {
        match self.bits {
            false => WPSEQ_A::Clear,
            true => WPSEQ_A::Set,
        }
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WPSEQ_A::Clear
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WPSEQ_A::Set
    }
}
#[doc = "Field `WPSEQ` writer - Write protect sequence"]
pub type WPSEQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WPSEQ_A>;
impl<'a, REG, const O: u8> WPSEQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(WPSEQ_A::Clear)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(WPSEQ_A::Set)
    }
}
impl R {
    #[doc = "Write protect enable [0-15]"]
    #[inline(always)]
    pub unsafe fn wpen(&self, n: u8) -> WPEN_R {
        WPEN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Write protect enable 0"]
    #[inline(always)]
    pub fn wpen0(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write protect enable 1"]
    #[inline(always)]
    pub fn wpen1(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write protect enable 2"]
    #[inline(always)]
    pub fn wpen2(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write protect enable 3"]
    #[inline(always)]
    pub fn wpen3(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protect enable 4"]
    #[inline(always)]
    pub fn wpen4(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write protect enable 5"]
    #[inline(always)]
    pub fn wpen5(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write protect enable 6"]
    #[inline(always)]
    pub fn wpen6(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write protect enable 7"]
    #[inline(always)]
    pub fn wpen7(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write protect enable 8"]
    #[inline(always)]
    pub fn wpen8(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write protect enable 9"]
    #[inline(always)]
    pub fn wpen9(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write protect enable 10"]
    #[inline(always)]
    pub fn wpen10(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write protect enable 11"]
    #[inline(always)]
    pub fn wpen11(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write protect enable 12"]
    #[inline(always)]
    pub fn wpen12(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write protect enable 13"]
    #[inline(always)]
    pub fn wpen13(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write protect enable 14"]
    #[inline(always)]
    pub fn wpen14(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write protect enable 15"]
    #[inline(always)]
    pub fn wpen15(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write protect sequence"]
    #[inline(always)]
    pub fn wpseq(&self) -> WPSEQ_R {
        WPSEQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPR")
            .field("wpen0", &format_args!("{}", self.wpen0().bit()))
            .field("wpen1", &format_args!("{}", self.wpen1().bit()))
            .field("wpen2", &format_args!("{}", self.wpen2().bit()))
            .field("wpen3", &format_args!("{}", self.wpen3().bit()))
            .field("wpen4", &format_args!("{}", self.wpen4().bit()))
            .field("wpen5", &format_args!("{}", self.wpen5().bit()))
            .field("wpen6", &format_args!("{}", self.wpen6().bit()))
            .field("wpen7", &format_args!("{}", self.wpen7().bit()))
            .field("wpen8", &format_args!("{}", self.wpen8().bit()))
            .field("wpen9", &format_args!("{}", self.wpen9().bit()))
            .field("wpen10", &format_args!("{}", self.wpen10().bit()))
            .field("wpen11", &format_args!("{}", self.wpen11().bit()))
            .field("wpen12", &format_args!("{}", self.wpen12().bit()))
            .field("wpen13", &format_args!("{}", self.wpen13().bit()))
            .field("wpen14", &format_args!("{}", self.wpen14().bit()))
            .field("wpen15", &format_args!("{}", self.wpen15().bit()))
            .field("wpseq", &format_args!("{}", self.wpseq().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<WPR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Write protect enable [0-15]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn wpen<const O: u8>(&mut self) -> WPEN_W<WPR_SPEC, O> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 0 - Write protect enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn wpen0(&mut self) -> WPEN_W<WPR_SPEC, 0> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 1 - Write protect enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn wpen1(&mut self) -> WPEN_W<WPR_SPEC, 1> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 2 - Write protect enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn wpen2(&mut self) -> WPEN_W<WPR_SPEC, 2> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 3 - Write protect enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn wpen3(&mut self) -> WPEN_W<WPR_SPEC, 3> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 4 - Write protect enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn wpen4(&mut self) -> WPEN_W<WPR_SPEC, 4> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 5 - Write protect enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn wpen5(&mut self) -> WPEN_W<WPR_SPEC, 5> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 6 - Write protect enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn wpen6(&mut self) -> WPEN_W<WPR_SPEC, 6> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 7 - Write protect enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn wpen7(&mut self) -> WPEN_W<WPR_SPEC, 7> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 8 - Write protect enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn wpen8(&mut self) -> WPEN_W<WPR_SPEC, 8> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 9 - Write protect enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn wpen9(&mut self) -> WPEN_W<WPR_SPEC, 9> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 10 - Write protect enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn wpen10(&mut self) -> WPEN_W<WPR_SPEC, 10> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 11 - Write protect enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn wpen11(&mut self) -> WPEN_W<WPR_SPEC, 11> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 12 - Write protect enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn wpen12(&mut self) -> WPEN_W<WPR_SPEC, 12> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 13 - Write protect enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn wpen13(&mut self) -> WPEN_W<WPR_SPEC, 13> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 14 - Write protect enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn wpen14(&mut self) -> WPEN_W<WPR_SPEC, 14> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 15 - Write protect enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn wpen15(&mut self) -> WPEN_W<WPR_SPEC, 15> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 16 - Write protect sequence"]
    #[inline(always)]
    #[must_use]
    pub fn wpseq(&mut self) -> WPSEQ_W<WPR_SPEC, 16> {
        WPSEQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpr::R`](R) reader structure"]
impl crate::Readable for WPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
