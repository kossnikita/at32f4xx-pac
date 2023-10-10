#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<INTSTS_SPEC>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<INTSTS_SPEC>;
#[doc = "Field `LINE0` reader - Line 0 state bit"]
pub type LINE0_R = crate::BitReader;
#[doc = "Field `LINE0` writer - Line 0 state bit"]
pub type LINE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE1` reader - Line 1 state bit"]
pub type LINE1_R = crate::BitReader;
#[doc = "Field `LINE1` writer - Line 1 state bit"]
pub type LINE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE2` reader - Line 2 state bit"]
pub type LINE2_R = crate::BitReader;
#[doc = "Field `LINE2` writer - Line 2 state bit"]
pub type LINE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE3` reader - Line 3 state bit"]
pub type LINE3_R = crate::BitReader;
#[doc = "Field `LINE3` writer - Line 3 state bit"]
pub type LINE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE4` reader - Line 4 state bit"]
pub type LINE4_R = crate::BitReader;
#[doc = "Field `LINE4` writer - Line 4 state bit"]
pub type LINE4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE5` reader - Line 5 state bit"]
pub type LINE5_R = crate::BitReader;
#[doc = "Field `LINE5` writer - Line 5 state bit"]
pub type LINE5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE6` reader - Line 6 state bit"]
pub type LINE6_R = crate::BitReader;
#[doc = "Field `LINE6` writer - Line 6 state bit"]
pub type LINE6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE7` reader - Line 7 state bit"]
pub type LINE7_R = crate::BitReader;
#[doc = "Field `LINE7` writer - Line 7 state bit"]
pub type LINE7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE8` reader - Line 8 state bit"]
pub type LINE8_R = crate::BitReader;
#[doc = "Field `LINE8` writer - Line 8 state bit"]
pub type LINE8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE9` reader - Line 9 state bit"]
pub type LINE9_R = crate::BitReader;
#[doc = "Field `LINE9` writer - Line 9 state bit"]
pub type LINE9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE10` reader - Line 10 state bit"]
pub type LINE10_R = crate::BitReader;
#[doc = "Field `LINE10` writer - Line 10 state bit"]
pub type LINE10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE11` reader - Line 11 state bit"]
pub type LINE11_R = crate::BitReader;
#[doc = "Field `LINE11` writer - Line 11 state bit"]
pub type LINE11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE12` reader - Line 12 state bit"]
pub type LINE12_R = crate::BitReader;
#[doc = "Field `LINE12` writer - Line 12 state bit"]
pub type LINE12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE13` reader - Line 13 state bit"]
pub type LINE13_R = crate::BitReader;
#[doc = "Field `LINE13` writer - Line 13 state bit"]
pub type LINE13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE14` reader - Line 14 state bit"]
pub type LINE14_R = crate::BitReader;
#[doc = "Field `LINE14` writer - Line 14 state bit"]
pub type LINE14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE15` reader - Line 15 state bit"]
pub type LINE15_R = crate::BitReader;
#[doc = "Field `LINE15` writer - Line 15 state bit"]
pub type LINE15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE16` reader - Line 16 state bit"]
pub type LINE16_R = crate::BitReader;
#[doc = "Field `LINE16` writer - Line 16 state bit"]
pub type LINE16_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE17` reader - Line 17 state bit"]
pub type LINE17_R = crate::BitReader;
#[doc = "Field `LINE17` writer - Line 17 state bit"]
pub type LINE17_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LINE18` reader - Line 18 state bit"]
pub type LINE18_R = crate::BitReader;
#[doc = "Field `LINE18` writer - Line 18 state bit"]
pub type LINE18_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Line 0 state bit"]
    #[inline(always)]
    pub fn line0(&self) -> LINE0_R {
        LINE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Line 1 state bit"]
    #[inline(always)]
    pub fn line1(&self) -> LINE1_R {
        LINE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Line 2 state bit"]
    #[inline(always)]
    pub fn line2(&self) -> LINE2_R {
        LINE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line 3 state bit"]
    #[inline(always)]
    pub fn line3(&self) -> LINE3_R {
        LINE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line 4 state bit"]
    #[inline(always)]
    pub fn line4(&self) -> LINE4_R {
        LINE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Line 5 state bit"]
    #[inline(always)]
    pub fn line5(&self) -> LINE5_R {
        LINE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Line 6 state bit"]
    #[inline(always)]
    pub fn line6(&self) -> LINE6_R {
        LINE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Line 7 state bit"]
    #[inline(always)]
    pub fn line7(&self) -> LINE7_R {
        LINE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Line 8 state bit"]
    #[inline(always)]
    pub fn line8(&self) -> LINE8_R {
        LINE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Line 9 state bit"]
    #[inline(always)]
    pub fn line9(&self) -> LINE9_R {
        LINE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Line 10 state bit"]
    #[inline(always)]
    pub fn line10(&self) -> LINE10_R {
        LINE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Line 11 state bit"]
    #[inline(always)]
    pub fn line11(&self) -> LINE11_R {
        LINE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Line 12 state bit"]
    #[inline(always)]
    pub fn line12(&self) -> LINE12_R {
        LINE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Line 13 state bit"]
    #[inline(always)]
    pub fn line13(&self) -> LINE13_R {
        LINE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Line 14 state bit"]
    #[inline(always)]
    pub fn line14(&self) -> LINE14_R {
        LINE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Line 15 state bit"]
    #[inline(always)]
    pub fn line15(&self) -> LINE15_R {
        LINE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Line 16 state bit"]
    #[inline(always)]
    pub fn line16(&self) -> LINE16_R {
        LINE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Line 17 state bit"]
    #[inline(always)]
    pub fn line17(&self) -> LINE17_R {
        LINE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Line 18 state bit"]
    #[inline(always)]
    pub fn line18(&self) -> LINE18_R {
        LINE18_R::new(((self.bits >> 18) & 1) != 0)
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
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Line 0 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line0(&mut self) -> LINE0_W<INTSTS_SPEC, 0> {
        LINE0_W::new(self)
    }
    #[doc = "Bit 1 - Line 1 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line1(&mut self) -> LINE1_W<INTSTS_SPEC, 1> {
        LINE1_W::new(self)
    }
    #[doc = "Bit 2 - Line 2 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line2(&mut self) -> LINE2_W<INTSTS_SPEC, 2> {
        LINE2_W::new(self)
    }
    #[doc = "Bit 3 - Line 3 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line3(&mut self) -> LINE3_W<INTSTS_SPEC, 3> {
        LINE3_W::new(self)
    }
    #[doc = "Bit 4 - Line 4 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line4(&mut self) -> LINE4_W<INTSTS_SPEC, 4> {
        LINE4_W::new(self)
    }
    #[doc = "Bit 5 - Line 5 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line5(&mut self) -> LINE5_W<INTSTS_SPEC, 5> {
        LINE5_W::new(self)
    }
    #[doc = "Bit 6 - Line 6 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line6(&mut self) -> LINE6_W<INTSTS_SPEC, 6> {
        LINE6_W::new(self)
    }
    #[doc = "Bit 7 - Line 7 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line7(&mut self) -> LINE7_W<INTSTS_SPEC, 7> {
        LINE7_W::new(self)
    }
    #[doc = "Bit 8 - Line 8 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line8(&mut self) -> LINE8_W<INTSTS_SPEC, 8> {
        LINE8_W::new(self)
    }
    #[doc = "Bit 9 - Line 9 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line9(&mut self) -> LINE9_W<INTSTS_SPEC, 9> {
        LINE9_W::new(self)
    }
    #[doc = "Bit 10 - Line 10 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line10(&mut self) -> LINE10_W<INTSTS_SPEC, 10> {
        LINE10_W::new(self)
    }
    #[doc = "Bit 11 - Line 11 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line11(&mut self) -> LINE11_W<INTSTS_SPEC, 11> {
        LINE11_W::new(self)
    }
    #[doc = "Bit 12 - Line 12 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line12(&mut self) -> LINE12_W<INTSTS_SPEC, 12> {
        LINE12_W::new(self)
    }
    #[doc = "Bit 13 - Line 13 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line13(&mut self) -> LINE13_W<INTSTS_SPEC, 13> {
        LINE13_W::new(self)
    }
    #[doc = "Bit 14 - Line 14 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line14(&mut self) -> LINE14_W<INTSTS_SPEC, 14> {
        LINE14_W::new(self)
    }
    #[doc = "Bit 15 - Line 15 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line15(&mut self) -> LINE15_W<INTSTS_SPEC, 15> {
        LINE15_W::new(self)
    }
    #[doc = "Bit 16 - Line 16 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line16(&mut self) -> LINE16_W<INTSTS_SPEC, 16> {
        LINE16_W::new(self)
    }
    #[doc = "Bit 17 - Line 17 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line17(&mut self) -> LINE17_W<INTSTS_SPEC, 17> {
        LINE17_W::new(self)
    }
    #[doc = "Bit 18 - Line 18 state bit"]
    #[inline(always)]
    #[must_use]
    pub fn line18(&mut self) -> LINE18_W<INTSTS_SPEC, 18> {
        LINE18_W::new(self)
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
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
