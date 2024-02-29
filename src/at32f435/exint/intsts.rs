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
#[doc = "Field `LINE(0-22)` reader - Line %s status bit"]
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
#[doc = "Field `LINE(0-22)` writer - Line %s status bit"]
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
impl R {
    #[doc = "Line (0-22) status bit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `LINE0` field"]
    #[inline(always)]
    pub fn line(&self, n: u8) -> LINE_R {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        LINE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Line (0-22) status bit"]
    #[inline(always)]
    pub fn line_iter(&self) -> impl Iterator<Item = LINE_R> + '_ {
        (0..23).map(move |n| LINE_R::new(((self.bits >> n) & 1) != 0))
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
    #[doc = "Bit 21 - Line 21 status bit"]
    #[inline(always)]
    pub fn line21(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Line 22 status bit"]
    #[inline(always)]
    pub fn line22(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS")
            .field("line0", &format_args!("{}", self.line0().bit()))
            .field("line1", &format_args!("{}", self.line1().bit()))
            .field("line2", &format_args!("{}", self.line2().bit()))
            .field("line3", &format_args!("{}", self.line3().bit()))
            .field("line4", &format_args!("{}", self.line4().bit()))
            .field("line5", &format_args!("{}", self.line5().bit()))
            .field("line6", &format_args!("{}", self.line6().bit()))
            .field("line7", &format_args!("{}", self.line7().bit()))
            .field("line8", &format_args!("{}", self.line8().bit()))
            .field("line9", &format_args!("{}", self.line9().bit()))
            .field("line10", &format_args!("{}", self.line10().bit()))
            .field("line11", &format_args!("{}", self.line11().bit()))
            .field("line12", &format_args!("{}", self.line12().bit()))
            .field("line13", &format_args!("{}", self.line13().bit()))
            .field("line14", &format_args!("{}", self.line14().bit()))
            .field("line15", &format_args!("{}", self.line15().bit()))
            .field("line16", &format_args!("{}", self.line16().bit()))
            .field("line17", &format_args!("{}", self.line17().bit()))
            .field("line18", &format_args!("{}", self.line18().bit()))
            .field("line19", &format_args!("{}", self.line19().bit()))
            .field("line20", &format_args!("{}", self.line20().bit()))
            .field("line21", &format_args!("{}", self.line21().bit()))
            .field("line22", &format_args!("{}", self.line22().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Line (0-22) status bit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `LINE0` field"]
    #[inline(always)]
    #[must_use]
    pub fn line(&mut self, n: u8) -> LINE_W<INTSTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        LINE_W::new(self, n)
    }
    #[doc = "Bit 0 - Line 0 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line0(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Line 1 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line1(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Line 2 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line2(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Line 3 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line3(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Line 4 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line4(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Line 5 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line5(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Line 6 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line6(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Line 7 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line7(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Line 8 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line8(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Line 9 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line9(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Line 10 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line10(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Line 11 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line11(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Line 12 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line12(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Line 13 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line13(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Line 14 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line14(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Line 15 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line15(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Line 16 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line16(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Line 17 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line17(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Line 18 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line18(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Line 19 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line19(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Line 20 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line20(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Line 21 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line21(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Line 22 status bit"]
    #[inline(always)]
    #[must_use]
    pub fn line22(&mut self) -> LINE_W<INTSTS_SPEC> {
        LINE_W::new(self, 22)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
