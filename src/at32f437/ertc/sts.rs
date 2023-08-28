#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `ALAWF` reader - Alarm A register allows write flag"]
pub type ALAWF_R = crate::BitReader;
#[doc = "Field `ALBWF` reader - Alarm B register allows write flag"]
pub type ALBWF_R = crate::BitReader;
#[doc = "Field `WATWF` reader - Wakeup timer register allows write flag"]
pub type WATWF_R = crate::BitReader;
#[doc = "Field `TADJF` reader - Time adjustment flag"]
pub type TADJF_R = crate::BitReader;
#[doc = "Field `TADJF` writer - Time adjustment flag"]
pub type TADJF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INITF` reader - Calendar initialization flag"]
pub type INITF_R = crate::BitReader;
#[doc = "Field `UPDF` reader - Calendar update flag"]
pub type UPDF_R = crate::BitReader;
#[doc = "Field `UPDF` writer - Calendar update flag"]
pub type UPDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMF` reader - Enter initialization mode flag"]
pub type IMF_R = crate::BitReader;
#[doc = "Field `IMEN` reader - Initialization mode enable"]
pub type IMEN_R = crate::BitReader;
#[doc = "Field `IMEN` writer - Initialization mode enable"]
pub type IMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALAF` reader - Alarm A flag"]
pub type ALAF_R = crate::BitReader;
#[doc = "Field `ALAF` writer - Alarm A flag"]
pub type ALAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALBF` reader - Alarm B flag"]
pub type ALBF_R = crate::BitReader;
#[doc = "Field `ALBF` writer - Alarm B flag"]
pub type ALBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WATF` reader - Wakeup timer flag"]
pub type WATF_R = crate::BitReader;
#[doc = "Field `WATF` writer - Wakeup timer flag"]
pub type WATF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSF` reader - Timestamp flag"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Timestamp flag"]
pub type TSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSOF` reader - Timestamp overflow flag"]
pub type TSOF_R = crate::BitReader;
#[doc = "Field `TSOF` writer - Timestamp overflow flag"]
pub type TSOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TP1F` reader - Tamper detection 1 flag"]
pub type TP1F_R = crate::BitReader;
#[doc = "Field `TP1F` writer - Tamper detection 1 flag"]
pub type TP1F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TP2F` reader - Tamper detection 2 flag"]
pub type TP2F_R = crate::BitReader;
#[doc = "Field `TP2F` writer - Tamper detection 2 flag"]
pub type TP2F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALUPDF` reader - Calibration value update completed flag"]
pub type CALUPDF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A register allows write flag"]
    #[inline(always)]
    pub fn alawf(&self) -> ALAWF_R {
        ALAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B register allows write flag"]
    #[inline(always)]
    pub fn albwf(&self) -> ALBWF_R {
        ALBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer register allows write flag"]
    #[inline(always)]
    pub fn watwf(&self) -> WATWF_R {
        WATWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time adjustment flag"]
    #[inline(always)]
    pub fn tadjf(&self) -> TADJF_R {
        TADJF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calendar update flag"]
    #[inline(always)]
    pub fn updf(&self) -> UPDF_R {
        UPDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enter initialization mode flag"]
    #[inline(always)]
    pub fn imf(&self) -> IMF_R {
        IMF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode enable"]
    #[inline(always)]
    pub fn imen(&self) -> IMEN_R {
        IMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alaf(&self) -> ALAF_R {
        ALAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn albf(&self) -> ALBF_R {
        ALBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn watf(&self) -> WATF_R {
        WATF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timestamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timestamp overflow flag"]
    #[inline(always)]
    pub fn tsof(&self) -> TSOF_R {
        TSOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper detection 1 flag"]
    #[inline(always)]
    pub fn tp1f(&self) -> TP1F_R {
        TP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper detection 2 flag"]
    #[inline(always)]
    pub fn tp2f(&self) -> TP2F_R {
        TP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Calibration value update completed flag"]
    #[inline(always)]
    pub fn calupdf(&self) -> CALUPDF_R {
        CALUPDF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Time adjustment flag"]
    #[inline(always)]
    #[must_use]
    pub fn tadjf(&mut self) -> TADJF_W<STS_SPEC, 3> {
        TADJF_W::new(self)
    }
    #[doc = "Bit 5 - Calendar update flag"]
    #[inline(always)]
    #[must_use]
    pub fn updf(&mut self) -> UPDF_W<STS_SPEC, 5> {
        UPDF_W::new(self)
    }
    #[doc = "Bit 7 - Initialization mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn imen(&mut self) -> IMEN_W<STS_SPEC, 7> {
        IMEN_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alaf(&mut self) -> ALAF_W<STS_SPEC, 8> {
        ALAF_W::new(self)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    #[must_use]
    pub fn albf(&mut self) -> ALBF_W<STS_SPEC, 9> {
        ALBF_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn watf(&mut self) -> WATF_W<STS_SPEC, 10> {
        WATF_W::new(self)
    }
    #[doc = "Bit 11 - Timestamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<STS_SPEC, 11> {
        TSF_W::new(self)
    }
    #[doc = "Bit 12 - Timestamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsof(&mut self) -> TSOF_W<STS_SPEC, 12> {
        TSOF_W::new(self)
    }
    #[doc = "Bit 13 - Tamper detection 1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn tp1f(&mut self) -> TP1F_W<STS_SPEC, 13> {
        TP1F_W::new(self)
    }
    #[doc = "Bit 14 - Tamper detection 2 flag"]
    #[inline(always)]
    #[must_use]
    pub fn tp2f(&mut self) -> TP2F_W<STS_SPEC, 14> {
        TP2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0x07"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
