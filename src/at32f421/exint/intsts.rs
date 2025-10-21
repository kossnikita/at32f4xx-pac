#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<INTSTS_SPEC>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<INTSTS_SPEC>;
#[doc = "Line %s status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line0r {
    #[doc = "0: No interrupt occurred"]
    NoInterrupt = 0,
    #[doc = "1: Interrupt occurred"]
    Interrupt = 1,
}
impl From<Line0r> for bool {
    #[inline(always)]
    fn from(variant: Line0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE(0-17)` reader - Line %s status bit"]
pub type LINE_R = crate::BitReader<Line0r>;
impl LINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Line0r {
        match self.bits {
            false => Line0r::NoInterrupt,
            true => Line0r::Interrupt,
        }
    }
    #[doc = "No interrupt occurred"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == Line0r::NoInterrupt
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Line0r::Interrupt
    }
}
#[doc = "Line %s status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line0wWO {
    #[doc = "1: Clear status flag"]
    Clear = 1,
}
impl From<Line0wWO> for bool {
    #[inline(always)]
    fn from(variant: Line0wWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE(0-17)` writer - Line %s status bit"]
pub type LINE_W<'a, REG> = crate::BitWriter1C<'a, REG, Line0wWO>;
impl<'a, REG> LINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear status flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Line0wWO::Clear)
    }
}
#[doc = "Field `LINE19` reader - Line 19 state bit"]
pub use LINE_R as LINE19_R;
#[doc = "Field `LINE21` reader - Line 21 state bit"]
pub use LINE_R as LINE21_R;
#[doc = "Field `LINE19` writer - Line 19 state bit"]
pub use LINE_W as LINE19_W;
#[doc = "Field `LINE21` writer - Line 21 state bit"]
pub use LINE_W as LINE21_W;
impl R {
    #[doc = "Line (0-17) status bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LINE0` field.</div>"]
    #[inline(always)]
    pub fn line(&self, n: u8) -> LINE_R {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        LINE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Line (0-17) status bit"]
    #[inline(always)]
    pub fn line_iter(&self) -> impl Iterator<Item = LINE_R> + '_ {
        (0..18).map(move |n| LINE_R::new(((self.bits >> n) & 1) != 0))
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
    #[doc = "Bit 19 - Line 19 state bit"]
    #[inline(always)]
    pub fn line19(&self) -> LINE19_R {
        LINE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Line 21 state bit"]
    #[inline(always)]
    pub fn line21(&self) -> LINE21_R {
        LINE21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS")
            .field("line0", &self.line0())
            .field("line1", &self.line1())
            .field("line2", &self.line2())
            .field("line3", &self.line3())
            .field("line4", &self.line4())
            .field("line5", &self.line5())
            .field("line6", &self.line6())
            .field("line7", &self.line7())
            .field("line8", &self.line8())
            .field("line9", &self.line9())
            .field("line10", &self.line10())
            .field("line11", &self.line11())
            .field("line12", &self.line12())
            .field("line13", &self.line13())
            .field("line14", &self.line14())
            .field("line15", &self.line15())
            .field("line16", &self.line16())
            .field("line17", &self.line17())
            .field("line19", &self.line19())
            .field("line21", &self.line21())
            .finish()
    }
}
impl W {
    #[doc = "Line (0-17) status bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LINE0` field.</div>"]
    #[inline(always)]
    pub fn line(&mut self, n: u8) -> LINE_W<'_, INTSTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        LINE_W::new(self, n)
    }
    #[doc = "Bit 0 - Line 0 status bit"]
    #[inline(always)]
    pub fn line0(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Line 1 status bit"]
    #[inline(always)]
    pub fn line1(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Line 2 status bit"]
    #[inline(always)]
    pub fn line2(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Line 3 status bit"]
    #[inline(always)]
    pub fn line3(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Line 4 status bit"]
    #[inline(always)]
    pub fn line4(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Line 5 status bit"]
    #[inline(always)]
    pub fn line5(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Line 6 status bit"]
    #[inline(always)]
    pub fn line6(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Line 7 status bit"]
    #[inline(always)]
    pub fn line7(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Line 8 status bit"]
    #[inline(always)]
    pub fn line8(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Line 9 status bit"]
    #[inline(always)]
    pub fn line9(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Line 10 status bit"]
    #[inline(always)]
    pub fn line10(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Line 11 status bit"]
    #[inline(always)]
    pub fn line11(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Line 12 status bit"]
    #[inline(always)]
    pub fn line12(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Line 13 status bit"]
    #[inline(always)]
    pub fn line13(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Line 14 status bit"]
    #[inline(always)]
    pub fn line14(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Line 15 status bit"]
    #[inline(always)]
    pub fn line15(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Line 16 status bit"]
    #[inline(always)]
    pub fn line16(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Line 17 status bit"]
    #[inline(always)]
    pub fn line17(&mut self) -> LINE_W<'_, INTSTS_SPEC> {
        LINE_W::new(self, 17)
    }
    #[doc = "Bit 19 - Line 19 state bit"]
    #[inline(always)]
    pub fn line19(&mut self) -> LINE19_W<'_, INTSTS_SPEC> {
        LINE19_W::new(self, 19)
    }
    #[doc = "Bit 21 - Line 21 state bit"]
    #[inline(always)]
    pub fn line21(&mut self) -> LINE21_W<'_, INTSTS_SPEC> {
        LINE21_W::new(self, 21)
    }
}
#[doc = "Interrupt status register (EXTINT_INTSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x002b_ffff;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {}
