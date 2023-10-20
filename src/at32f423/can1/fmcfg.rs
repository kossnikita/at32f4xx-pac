#[doc = "Register `FMCFG` reader"]
pub type R = crate::R<FMCFG_SPEC>;
#[doc = "Register `FMCFG` writer"]
pub type W = crate::W<FMCFG_SPEC>;
#[doc = "Field `SEL[0-13]` reader - Filter mode select"]
pub type SEL_R = crate::BitReader<SEL0_A>;
#[doc = "Filter mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL0_A {
    #[doc = "0: Identifier mask mode"]
    Mask = 0,
    #[doc = "1: Identifier list mode"]
    List = 1,
}
impl From<SEL0_A> for bool {
    #[inline(always)]
    fn from(variant: SEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEL0_A {
        match self.bits {
            false => SEL0_A::Mask,
            true => SEL0_A::List,
        }
    }
    #[doc = "Identifier mask mode"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == SEL0_A::Mask
    }
    #[doc = "Identifier list mode"]
    #[inline(always)]
    pub fn is_list(&self) -> bool {
        *self == SEL0_A::List
    }
}
#[doc = "Field `SEL[0-13]` writer - Filter mode select"]
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG, SEL0_A>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Identifier mask mode"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::Mask)
    }
    #[doc = "Identifier list mode"]
    #[inline(always)]
    pub fn list(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::List)
    }
}
impl R {
    #[doc = "Filter mode select\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn sel(&self, n: u8) -> SEL_R {
        assert!(n < 14);
        SEL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Filter mode select"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode select"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode select"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode select"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode select"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode select"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode select"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode select"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode select"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode select"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode select"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode select"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode select"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode select"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMCFG")
            .field("sel0", &format_args!("{}", self.sel0().bit()))
            .field("sel1", &format_args!("{}", self.sel1().bit()))
            .field("sel2", &format_args!("{}", self.sel2().bit()))
            .field("sel3", &format_args!("{}", self.sel3().bit()))
            .field("sel4", &format_args!("{}", self.sel4().bit()))
            .field("sel5", &format_args!("{}", self.sel5().bit()))
            .field("sel6", &format_args!("{}", self.sel6().bit()))
            .field("sel7", &format_args!("{}", self.sel7().bit()))
            .field("sel8", &format_args!("{}", self.sel8().bit()))
            .field("sel9", &format_args!("{}", self.sel9().bit()))
            .field("sel10", &format_args!("{}", self.sel10().bit()))
            .field("sel11", &format_args!("{}", self.sel11().bit()))
            .field("sel12", &format_args!("{}", self.sel12().bit()))
            .field("sel13", &format_args!("{}", self.sel13().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FMCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self, n: u8) -> SEL_W<FMCFG_SPEC> {
        assert!(n < 14);
        SEL_W::new(self, n)
    }
    #[doc = "Bit 0 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel9(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sel13(&mut self) -> SEL_W<FMCFG_SPEC> {
        SEL_W::new(self, 13)
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
#[doc = "Filter mode config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMCFG_SPEC;
impl crate::RegisterSpec for FMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcfg::R`](R) reader structure"]
impl crate::Readable for FMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmcfg::W`](W) writer structure"]
impl crate::Writable for FMCFG_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMCFG to value 0"]
impl crate::Resettable for FMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
