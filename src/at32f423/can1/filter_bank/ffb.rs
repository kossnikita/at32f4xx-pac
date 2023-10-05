#[doc = "Register `FFB%s` reader"]
pub type R = crate::R<FFB_SPEC>;
#[doc = "Register `FFB%s` writer"]
pub type W = crate::W<FFB_SPEC>;
#[doc = "Field `FFDB[0-31]` reader - Filter data bit"]
pub type FFDB_R = crate::BitReader;
#[doc = "Field `FFDB[0-31]` writer - Filter data bit"]
pub type FFDB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Filter data bit"]
    #[inline(always)]
    pub unsafe fn ffdb(&self, n: u8) -> FFDB_R {
        FFDB_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb0(&self) -> FFDB_R {
        FFDB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb1(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb2(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb3(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb4(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb5(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb6(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb7(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb8(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb9(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb10(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb11(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb12(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb13(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb14(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb15(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb16(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb17(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb18(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb19(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb20(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb21(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb22(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb23(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb24(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb25(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb26(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb27(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb28(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb29(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb30(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Filter data bit"]
    #[inline(always)]
    pub fn ffdb31(&self) -> FFDB_R {
        FFDB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ffdb<const O: u8>(&mut self) -> FFDB_W<FFB_SPEC, O> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 0 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb0(&mut self) -> FFDB_W<FFB_SPEC, 0> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 1 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb1(&mut self) -> FFDB_W<FFB_SPEC, 1> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 2 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb2(&mut self) -> FFDB_W<FFB_SPEC, 2> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 3 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb3(&mut self) -> FFDB_W<FFB_SPEC, 3> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 4 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb4(&mut self) -> FFDB_W<FFB_SPEC, 4> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 5 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb5(&mut self) -> FFDB_W<FFB_SPEC, 5> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 6 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb6(&mut self) -> FFDB_W<FFB_SPEC, 6> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 7 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb7(&mut self) -> FFDB_W<FFB_SPEC, 7> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 8 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb8(&mut self) -> FFDB_W<FFB_SPEC, 8> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 9 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb9(&mut self) -> FFDB_W<FFB_SPEC, 9> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 10 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb10(&mut self) -> FFDB_W<FFB_SPEC, 10> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 11 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb11(&mut self) -> FFDB_W<FFB_SPEC, 11> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 12 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb12(&mut self) -> FFDB_W<FFB_SPEC, 12> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 13 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb13(&mut self) -> FFDB_W<FFB_SPEC, 13> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 14 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb14(&mut self) -> FFDB_W<FFB_SPEC, 14> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 15 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb15(&mut self) -> FFDB_W<FFB_SPEC, 15> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 16 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb16(&mut self) -> FFDB_W<FFB_SPEC, 16> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 17 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb17(&mut self) -> FFDB_W<FFB_SPEC, 17> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 18 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb18(&mut self) -> FFDB_W<FFB_SPEC, 18> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 19 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb19(&mut self) -> FFDB_W<FFB_SPEC, 19> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 20 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb20(&mut self) -> FFDB_W<FFB_SPEC, 20> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 21 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb21(&mut self) -> FFDB_W<FFB_SPEC, 21> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 22 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb22(&mut self) -> FFDB_W<FFB_SPEC, 22> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 23 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb23(&mut self) -> FFDB_W<FFB_SPEC, 23> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 24 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb24(&mut self) -> FFDB_W<FFB_SPEC, 24> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 25 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb25(&mut self) -> FFDB_W<FFB_SPEC, 25> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 26 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb26(&mut self) -> FFDB_W<FFB_SPEC, 26> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 27 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb27(&mut self) -> FFDB_W<FFB_SPEC, 27> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 28 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb28(&mut self) -> FFDB_W<FFB_SPEC, 28> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 29 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb29(&mut self) -> FFDB_W<FFB_SPEC, 29> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 30 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb30(&mut self) -> FFDB_W<FFB_SPEC, 30> {
        FFDB_W::new(self)
    }
    #[doc = "Bit 31 - Filter data bit"]
    #[inline(always)]
    #[must_use]
    pub fn ffdb31(&mut self) -> FFDB_W<FFB_SPEC, 31> {
        FFDB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CAN filter bank filter bit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FFB_SPEC;
impl crate::RegisterSpec for FFB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffb::R`](R) reader structure"]
impl crate::Readable for FFB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ffb::W`](W) writer structure"]
impl crate::Writable for FFB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFB%s to value 0"]
impl crate::Resettable for FFB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
