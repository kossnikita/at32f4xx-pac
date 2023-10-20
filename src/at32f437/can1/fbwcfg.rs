#[doc = "Register `FBWCFG` reader"]
pub type R = crate::R<FBWCFG_SPEC>;
#[doc = "Register `FBWCFG` writer"]
pub type W = crate::W<FBWCFG_SPEC>;
#[doc = "Field `SEL[0-27]` reader - Filter bit width select"]
pub type SEL_R = crate::BitReader<SEL0_A>;
#[doc = "Filter bit width select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL0_A {
    #[doc = "0: Dual 16-bit"]
    Dual = 0,
    #[doc = "1: Single 32-bit"]
    Single = 1,
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
            false => SEL0_A::Dual,
            true => SEL0_A::Single,
        }
    }
    #[doc = "Dual 16-bit"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == SEL0_A::Dual
    }
    #[doc = "Single 32-bit"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == SEL0_A::Single
    }
}
#[doc = "Field `SEL[0-27]` writer - Filter bit width select"]
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG, SEL0_A>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dual 16-bit"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::Dual)
    }
    #[doc = "Single 32-bit"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::Single)
    }
}
impl R {
    #[doc = "Filter bit width select\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn sel(&self, n: u8) -> SEL_R {
        assert!(n < 28);
        SEL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Filter bit width select"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter bit width select"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter bit width select"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter bit width select"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter bit width select"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter bit width select"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter bit width select"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter bit width select"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter bit width select"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter bit width select"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter bit width select"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter bit width select"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter bit width select"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter bit width select"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter bit width select"]
    #[inline(always)]
    pub fn sel14(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter bit width select"]
    #[inline(always)]
    pub fn sel15(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter bit width select"]
    #[inline(always)]
    pub fn sel16(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter bit width select"]
    #[inline(always)]
    pub fn sel17(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter bit width select"]
    #[inline(always)]
    pub fn sel18(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter bit width select"]
    #[inline(always)]
    pub fn sel19(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter bit width select"]
    #[inline(always)]
    pub fn sel20(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter bit width select"]
    #[inline(always)]
    pub fn sel21(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter bit width select"]
    #[inline(always)]
    pub fn sel22(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter bit width select"]
    #[inline(always)]
    pub fn sel23(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter bit width select"]
    #[inline(always)]
    pub fn sel24(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter bit width select"]
    #[inline(always)]
    pub fn sel25(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter bit width select"]
    #[inline(always)]
    pub fn sel26(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter bit width select"]
    #[inline(always)]
    pub fn sel27(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBWCFG")
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
            .field("sel14", &format_args!("{}", self.sel14().bit()))
            .field("sel15", &format_args!("{}", self.sel15().bit()))
            .field("sel16", &format_args!("{}", self.sel16().bit()))
            .field("sel17", &format_args!("{}", self.sel17().bit()))
            .field("sel18", &format_args!("{}", self.sel18().bit()))
            .field("sel19", &format_args!("{}", self.sel19().bit()))
            .field("sel20", &format_args!("{}", self.sel20().bit()))
            .field("sel21", &format_args!("{}", self.sel21().bit()))
            .field("sel22", &format_args!("{}", self.sel22().bit()))
            .field("sel23", &format_args!("{}", self.sel23().bit()))
            .field("sel24", &format_args!("{}", self.sel24().bit()))
            .field("sel25", &format_args!("{}", self.sel25().bit()))
            .field("sel26", &format_args!("{}", self.sel26().bit()))
            .field("sel27", &format_args!("{}", self.sel27().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FBWCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self, n: u8) -> SEL_W<FBWCFG_SPEC> {
        assert!(n < 28);
        SEL_W::new(self, n)
    }
    #[doc = "Bit 0 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel9(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel13(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 13)
    }
    #[doc = "Bit 14 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel14(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 14)
    }
    #[doc = "Bit 15 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel15(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 15)
    }
    #[doc = "Bit 16 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel16(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel17(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel18(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 18)
    }
    #[doc = "Bit 19 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel19(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel20(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel21(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 21)
    }
    #[doc = "Bit 22 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel22(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 22)
    }
    #[doc = "Bit 23 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel23(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel24(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel25(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel26(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter bit width select"]
    #[inline(always)]
    #[must_use]
    pub fn sel27(&mut self) -> SEL_W<FBWCFG_SPEC> {
        SEL_W::new(self, 27)
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
#[doc = "Filter bit width config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbwcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbwcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FBWCFG_SPEC;
impl crate::RegisterSpec for FBWCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbwcfg::R`](R) reader structure"]
impl crate::Readable for FBWCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fbwcfg::W`](W) writer structure"]
impl crate::Writable for FBWCFG_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBWCFG to value 0"]
impl crate::Resettable for FBWCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
