#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<INTSTS_SPEC>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<INTSTS_SPEC>;
#[doc = "Field `LINE[0-20]` reader - Line %s status bit"]
pub type LINE_R = crate::BitReader<LINE0R_A>;
#[doc = "Line %s status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE0R_A {
    #[doc = "0: No interrupt occurred"]
    NoInterrupt = 0,
    #[doc = "1: Interrupt occurred"]
    Interrupt = 1,
}
impl From<LINE0R_A> for bool {
    #[inline(always)]
    fn from(variant: LINE0R_A) -> Self {
        variant as u8 != 0
    }
}
impl LINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE0R_A {
        match self.bits {
            false => LINE0R_A::NoInterrupt,
            true => LINE0R_A::Interrupt,
        }
    }
    #[doc = "No interrupt occurred"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == LINE0R_A::NoInterrupt
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == LINE0R_A::Interrupt
    }
}
#[doc = "Line %s status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE0W_AW {
    #[doc = "1: Clear status flag"]
    Clear = 1,
}
impl From<LINE0W_AW> for bool {
    #[inline(always)]
    fn from(variant: LINE0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE[0-20]` writer - Line %s status bit"]
pub type LINE_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, LINE0W_AW>;
impl<'a, REG, const O: u8> LINE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear status flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LINE0W_AW::Clear)
    }
}
impl R {
    #[doc = "Line [0-20]
status bit"]
    #[inline(always)]
    pub unsafe fn line(&self, n: u8) -> LINE_R {
        LINE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Line 0 status bit"]
    #[inline(always)]
    pub fn line0(&self) -> LINE_R {
        LINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 status bit"]
    #[inline(always)]
    pub fn line1(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 status bit"]
    #[inline(always)]
    pub fn line2(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 status bit"]
    #[inline(always)]
    pub fn line3(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 status bit"]
    #[inline(always)]
    pub fn line4(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 status bit"]
    #[inline(always)]
    pub fn line5(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 status bit"]
    #[inline(always)]
    pub fn line6(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 status bit"]
    #[inline(always)]
    pub fn line7(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Line 8 status bit"]
    #[inline(always)]
    pub fn line8(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Line 9 status bit"]
    #[inline(always)]
    pub fn line9(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Line 10 status bit"]
    #[inline(always)]
    pub fn line10(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Line 11 status bit"]
    #[inline(always)]
    pub fn line11(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Line 12 status bit"]
    #[inline(always)]
    pub fn line12(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Line 13 status bit"]
    #[inline(always)]
    pub fn line13(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Line 14 status bit"]
    #[inline(always)]
    pub fn line14(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Line 15 status bit"]
    #[inline(always)]
    pub fn line15(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Line 16 status bit"]
    #[inline(always)]
    pub fn line16(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Line 17 status bit"]
    #[inline(always)]
    pub fn line17(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Line 18 status bit"]
    #[inline(always)]
    pub fn line18(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Line 19 status bit"]
    #[inline(always)]
    pub fn line19(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Line 20 status bit"]
    #[inline(always)]
    pub fn line20(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Line [0-20]
status bit"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn line<const O: u8>(&mut self) -> LINE_W<INTSTS_SPEC, O> {
        LINE_W::new(self)
    }
    #[doc = "Bit 0 - Line 0 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line0(&mut self) -> LINE_W<INTSTS_SPEC, 0> {
        LINE_W::new(self)
    }
    #[doc = "Bit 1 - Line 1 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line1(&mut self) -> LINE_W<INTSTS_SPEC, 1> {
        LINE_W::new(self)
    }
    #[doc = "Bit 2 - Line 2 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line2(&mut self) -> LINE_W<INTSTS_SPEC, 2> {
        LINE_W::new(self)
    }
    #[doc = "Bit 3 - Line 3 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line3(&mut self) -> LINE_W<INTSTS_SPEC, 3> {
        LINE_W::new(self)
    }
    #[doc = "Bit 4 - Line 4 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line4(&mut self) -> LINE_W<INTSTS_SPEC, 4> {
        LINE_W::new(self)
    }
    #[doc = "Bit 5 - Line 5 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line5(&mut self) -> LINE_W<INTSTS_SPEC, 5> {
        LINE_W::new(self)
    }
    #[doc = "Bit 6 - Line 6 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line6(&mut self) -> LINE_W<INTSTS_SPEC, 6> {
        LINE_W::new(self)
    }
    #[doc = "Bit 7 - Line 7 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line7(&mut self) -> LINE_W<INTSTS_SPEC, 7> {
        LINE_W::new(self)
    }
    #[doc = "Bit 8 - Line 8 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line8(&mut self) -> LINE_W<INTSTS_SPEC, 8> {
        LINE_W::new(self)
    }
    #[doc = "Bit 9 - Line 9 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line9(&mut self) -> LINE_W<INTSTS_SPEC, 9> {
        LINE_W::new(self)
    }
    #[doc = "Bit 10 - Line 10 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line10(&mut self) -> LINE_W<INTSTS_SPEC, 10> {
        LINE_W::new(self)
    }
    #[doc = "Bit 11 - Line 11 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line11(&mut self) -> LINE_W<INTSTS_SPEC, 11> {
        LINE_W::new(self)
    }
    #[doc = "Bit 12 - Line 12 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line12(&mut self) -> LINE_W<INTSTS_SPEC, 12> {
        LINE_W::new(self)
    }
    #[doc = "Bit 13 - Line 13 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line13(&mut self) -> LINE_W<INTSTS_SPEC, 13> {
        LINE_W::new(self)
    }
    #[doc = "Bit 14 - Line 14 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line14(&mut self) -> LINE_W<INTSTS_SPEC, 14> {
        LINE_W::new(self)
    }
    #[doc = "Bit 15 - Line 15 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line15(&mut self) -> LINE_W<INTSTS_SPEC, 15> {
        LINE_W::new(self)
    }
    #[doc = "Bit 16 - Line 16 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line16(&mut self) -> LINE_W<INTSTS_SPEC, 16> {
        LINE_W::new(self)
    }
    #[doc = "Bit 17 - Line 17 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line17(&mut self) -> LINE_W<INTSTS_SPEC, 17> {
        LINE_W::new(self)
    }
    #[doc = "Bit 18 - Line 18 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line18(&mut self) -> LINE_W<INTSTS_SPEC, 18> {
        LINE_W::new(self)
    }
    #[doc = "Bit 19 - Line 19 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line19(&mut self) -> LINE_W<INTSTS_SPEC, 19> {
        LINE_W::new(self)
    }
    #[doc = "Bit 20 - Line 20 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line20(&mut self) -> LINE_W<INTSTS_SPEC, 20> {
        LINE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt status register (EXTINT_INTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
