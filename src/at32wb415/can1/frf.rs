#[doc = "Register `FRF` reader"]
pub type R = crate::R<FRF_SPEC>;
#[doc = "Register `FRF` writer"]
pub type W = crate::W<FRF_SPEC>;
#[doc = "Field `SEL[0-13]` reader - Filter relation FIFO select"]
pub type SEL_R = crate::BitReader<SEL0_A>;
#[doc = "Filter relation FIFO select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL0_A {
    #[doc = "0: Associated with FIFO0"]
    Dual = 0,
    #[doc = "1: Associated with FIFO1"]
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
    pub fn variant(&self) -> SEL0_A {
        match self.bits {
            false => SEL0_A::Dual,
            true => SEL0_A::Single,
        }
    }
    #[doc = "Associated with FIFO0"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == SEL0_A::Dual
    }
    #[doc = "Associated with FIFO1"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == SEL0_A::Single
    }
}
#[doc = "Field `SEL[0-13]` writer - Filter relation FIFO select"]
pub type SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SEL0_A>;
impl<'a, REG, const O: u8> SEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Associated with FIFO0"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::Dual)
    }
    #[doc = "Associated with FIFO1"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(SEL0_A::Single)
    }
}
impl R {
    #[doc = "Filter relation FIFO select"]
    #[inline(always)]
    pub unsafe fn sel(&self, n: u8) -> SEL_R {
        SEL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRF")
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
impl core::fmt::Debug for crate::generic::Reg<FRF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn sel<const O: u8>(&mut self) -> SEL_W<FRF_SPEC, O> {
        SEL_W::new(self)
    }
    #[doc = "Bit 0 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL_W<FRF_SPEC, 0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 1 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL_W<FRF_SPEC, 1> {
        SEL_W::new(self)
    }
    #[doc = "Bit 2 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL_W<FRF_SPEC, 2> {
        SEL_W::new(self)
    }
    #[doc = "Bit 3 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL_W<FRF_SPEC, 3> {
        SEL_W::new(self)
    }
    #[doc = "Bit 4 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL_W<FRF_SPEC, 4> {
        SEL_W::new(self)
    }
    #[doc = "Bit 5 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> SEL_W<FRF_SPEC, 5> {
        SEL_W::new(self)
    }
    #[doc = "Bit 6 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL_W<FRF_SPEC, 6> {
        SEL_W::new(self)
    }
    #[doc = "Bit 7 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> SEL_W<FRF_SPEC, 7> {
        SEL_W::new(self)
    }
    #[doc = "Bit 8 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL_W<FRF_SPEC, 8> {
        SEL_W::new(self)
    }
    #[doc = "Bit 9 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel9(&mut self) -> SEL_W<FRF_SPEC, 9> {
        SEL_W::new(self)
    }
    #[doc = "Bit 10 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> SEL_W<FRF_SPEC, 10> {
        SEL_W::new(self)
    }
    #[doc = "Bit 11 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> SEL_W<FRF_SPEC, 11> {
        SEL_W::new(self)
    }
    #[doc = "Bit 12 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL_W<FRF_SPEC, 12> {
        SEL_W::new(self)
    }
    #[doc = "Bit 13 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn sel13(&mut self) -> SEL_W<FRF_SPEC, 13> {
        SEL_W::new(self)
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
#[doc = "Filter related FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRF_SPEC;
impl crate::RegisterSpec for FRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frf::R`](R) reader structure"]
impl crate::Readable for FRF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frf::W`](W) writer structure"]
impl crate::Writable for FRF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRF to value 0"]
impl crate::Resettable for FRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
