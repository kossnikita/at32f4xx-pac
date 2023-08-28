#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `WATCLK` reader - Wakeup timer clock selection"]
pub type WATCLK_R = crate::FieldReader;
#[doc = "Field `WATCLK` writer - Wakeup timer clock selection"]
pub type WATCLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TSEDG` reader - Timestamp trigger edge"]
pub type TSEDG_R = crate::BitReader;
#[doc = "Field `TSEDG` writer - Timestamp trigger edge"]
pub type TSEDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RCDEN` reader - Reference clock detection enable"]
pub type RCDEN_R = crate::BitReader;
#[doc = "Field `RCDEN` writer - Reference clock detection enable"]
pub type RCDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DREN` reader - Date/time register direct read enable"]
pub type DREN_R = crate::BitReader;
#[doc = "Field `DREN` writer - Date/time register direct read enable"]
pub type DREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HM` reader - Hour mode"]
pub type HM_R = crate::BitReader;
#[doc = "Field `HM` writer - Hour mode"]
pub type HM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALAEN` reader - Alarm A enable"]
pub type ALAEN_R = crate::BitReader;
#[doc = "Field `ALAEN` writer - Alarm A enable"]
pub type ALAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WATEN` reader - Wakeup timer enable"]
pub type WATEN_R = crate::BitReader;
#[doc = "Field `WATEN` writer - Wakeup timer enable"]
pub type WATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSEN` reader - Timestamp enable"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - Timestamp enable"]
pub type TSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALAIEN` reader - Alarm A interrupt enable"]
pub type ALAIEN_R = crate::BitReader;
#[doc = "Field `ALAIEN` writer - Alarm A interrupt enable"]
pub type ALAIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WATIEN` reader - Wakeup timer interrupt enable"]
pub type WATIEN_R = crate::BitReader;
#[doc = "Field `WATIEN` writer - Wakeup timer interrupt enable"]
pub type WATIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIEN` reader - Timestamp interrupt enable"]
pub type TSIEN_R = crate::BitReader;
#[doc = "Field `TSIEN` writer - Timestamp interrupt enable"]
pub type TSIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADD1H` reader - Add 1 hour"]
pub type ADD1H_R = crate::BitReader;
#[doc = "Field `ADD1H` writer - Add 1 hour"]
pub type ADD1H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEC1H` reader - Decrease 1 hour"]
pub type DEC1H_R = crate::BitReader;
#[doc = "Field `DEC1H` writer - Decrease 1 hour"]
pub type DEC1H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BPR` reader - Battery power domain data register"]
pub type BPR_R = crate::BitReader;
#[doc = "Field `BPR` writer - Battery power domain data register"]
pub type BPR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALOSEL` reader - Calibration output selection"]
pub type CALOSEL_R = crate::BitReader;
#[doc = "Field `CALOSEL` writer - Calibration output selection"]
pub type CALOSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTP` reader - Output polarity"]
pub type OUTP_R = crate::BitReader;
#[doc = "Field `OUTP` writer - Output polarity"]
pub type OUTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTSEL` reader - Output source selection"]
pub type OUTSEL_R = crate::FieldReader;
#[doc = "Field `OUTSEL` writer - Output source selection"]
pub type OUTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CALOEN` reader - Calibration output enable"]
pub type CALOEN_R = crate::BitReader;
#[doc = "Field `CALOEN` writer - Calibration output enable"]
pub type CALOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Wakeup timer clock selection"]
    #[inline(always)]
    pub fn watclk(&self) -> WATCLK_R {
        WATCLK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp trigger edge"]
    #[inline(always)]
    pub fn tsedg(&self) -> TSEDG_R {
        TSEDG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection enable"]
    #[inline(always)]
    pub fn rcden(&self) -> RCDEN_R {
        RCDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Date/time register direct read enable"]
    #[inline(always)]
    pub fn dren(&self) -> DREN_R {
        DREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour mode"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alaen(&self) -> ALAEN_R {
        ALAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn waten(&self) -> WATEN_R {
        WATEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timestamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alaien(&self) -> ALAIEN_R {
        ALAIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn watien(&self) -> WATIEN_R {
        WATIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add 1 hour"]
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Decrease 1 hour"]
    #[inline(always)]
    pub fn dec1h(&self) -> DEC1H_R {
        DEC1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Battery power domain data register"]
    #[inline(always)]
    pub fn bpr(&self) -> BPR_R {
        BPR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn calosel(&self) -> CALOSEL_R {
        CALOSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn outp(&self) -> OUTP_R {
        OUTP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output source selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn caloen(&self) -> CALOEN_R {
        CALOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup timer clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn watclk(&mut self) -> WATCLK_W<CTRL_SPEC, 0> {
        WATCLK_W::new(self)
    }
    #[doc = "Bit 3 - Timestamp trigger edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedg(&mut self) -> TSEDG_W<CTRL_SPEC, 3> {
        TSEDG_W::new(self)
    }
    #[doc = "Bit 4 - Reference clock detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn rcden(&mut self) -> RCDEN_W<CTRL_SPEC, 4> {
        RCDEN_W::new(self)
    }
    #[doc = "Bit 5 - Date/time register direct read enable"]
    #[inline(always)]
    #[must_use]
    pub fn dren(&mut self) -> DREN_W<CTRL_SPEC, 5> {
        DREN_W::new(self)
    }
    #[doc = "Bit 6 - Hour mode"]
    #[inline(always)]
    #[must_use]
    pub fn hm(&mut self) -> HM_W<CTRL_SPEC, 6> {
        HM_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaen(&mut self) -> ALAEN_W<CTRL_SPEC, 8> {
        ALAEN_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn waten(&mut self) -> WATEN_W<CTRL_SPEC, 10> {
        WATEN_W::new(self)
    }
    #[doc = "Bit 11 - Timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<CTRL_SPEC, 11> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alaien(&mut self) -> ALAIEN_W<CTRL_SPEC, 12> {
        ALAIEN_W::new(self)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn watien(&mut self) -> WATIEN_W<CTRL_SPEC, 14> {
        WATIEN_W::new(self)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsien(&mut self) -> TSIEN_W<CTRL_SPEC, 15> {
        TSIEN_W::new(self)
    }
    #[doc = "Bit 16 - Add 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CTRL_SPEC, 16> {
        ADD1H_W::new(self)
    }
    #[doc = "Bit 17 - Decrease 1 hour"]
    #[inline(always)]
    #[must_use]
    pub fn dec1h(&mut self) -> DEC1H_W<CTRL_SPEC, 17> {
        DEC1H_W::new(self)
    }
    #[doc = "Bit 18 - Battery power domain data register"]
    #[inline(always)]
    #[must_use]
    pub fn bpr(&mut self) -> BPR_W<CTRL_SPEC, 18> {
        BPR_W::new(self)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn calosel(&mut self) -> CALOSEL_W<CTRL_SPEC, 19> {
        CALOSEL_W::new(self)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn outp(&mut self) -> OUTP_W<CTRL_SPEC, 20> {
        OUTP_W::new(self)
    }
    #[doc = "Bits 21:22 - Output source selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<CTRL_SPEC, 21> {
        OUTSEL_W::new(self)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn caloen(&mut self) -> CALOEN_W<CTRL_SPEC, 23> {
        CALOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
