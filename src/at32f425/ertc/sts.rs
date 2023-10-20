#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `ALAWF` reader - Alarm A register allows write flag"]
pub type ALAWF_R = crate::BitReader;
#[doc = "Field `WATWF` reader - Wakeup timer register allows write flag"]
pub type WATWF_R = crate::BitReader;
#[doc = "Field `TADJF` reader - Time adjustment flag"]
pub type TADJF_R = crate::BitReader;
#[doc = "Field `TADJF` writer - Time adjustment flag"]
pub type TADJF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Calendar initialization flag"]
pub type INITF_R = crate::BitReader;
#[doc = "Field `UPDF` reader - Calendar update flag"]
pub type UPDF_R = crate::BitReader;
#[doc = "Field `UPDF` writer - Calendar update flag"]
pub type UPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMF` reader - Enter initialization mode flag"]
pub type IMF_R = crate::BitReader;
#[doc = "Field `IMEN` reader - Initialization mode enable"]
pub type IMEN_R = crate::BitReader;
#[doc = "Field `IMEN` writer - Initialization mode enable"]
pub type IMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALAF` reader - Alarm A flag"]
pub type ALAF_R = crate::BitReader;
#[doc = "Field `ALAF` writer - Alarm A flag"]
pub type ALAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATF` reader - Wakeup timer flag"]
pub type WATF_R = crate::BitReader;
#[doc = "Field `WATF` writer - Wakeup timer flag"]
pub type WATF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Timestamp flag"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Timestamp flag"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSOF` reader - Timestamp overflow flag"]
pub type TSOF_R = crate::BitReader;
#[doc = "Field `TSOF` writer - Timestamp overflow flag"]
pub type TSOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1F` reader - Tamper detection 1 flag"]
pub type TP1F_R = crate::BitReader;
#[doc = "Field `TP1F` writer - Tamper detection 1 flag"]
pub type TP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALUPDF` reader - Calibration value update completed flag"]
pub type CALUPDF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A register allows write flag"]
    #[inline(always)]
    pub fn alawf(&self) -> ALAWF_R {
        ALAWF_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 16 - Calibration value update completed flag"]
    #[inline(always)]
    pub fn calupdf(&self) -> CALUPDF_R {
        CALUPDF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("alawf", &format_args!("{}", self.alawf().bit()))
            .field("watwf", &format_args!("{}", self.watwf().bit()))
            .field("tadjf", &format_args!("{}", self.tadjf().bit()))
            .field("initf", &format_args!("{}", self.initf().bit()))
            .field("updf", &format_args!("{}", self.updf().bit()))
            .field("imf", &format_args!("{}", self.imf().bit()))
            .field("imen", &format_args!("{}", self.imen().bit()))
            .field("alaf", &format_args!("{}", self.alaf().bit()))
            .field("watf", &format_args!("{}", self.watf().bit()))
            .field("tsf", &format_args!("{}", self.tsf().bit()))
            .field("tsof", &format_args!("{}", self.tsof().bit()))
            .field("tp1f", &format_args!("{}", self.tp1f().bit()))
            .field("calupdf", &format_args!("{}", self.calupdf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Time adjustment flag"]
    #[inline(always)]
    #[must_use]
    pub fn tadjf(&mut self) -> TADJF_W<STS_SPEC> {
        TADJF_W::new(self, 3)
    }
    #[doc = "Bit 5 - Calendar update flag"]
    #[inline(always)]
    #[must_use]
    pub fn updf(&mut self) -> UPDF_W<STS_SPEC> {
        UPDF_W::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn imen(&mut self) -> IMEN_W<STS_SPEC> {
        IMEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alaf(&mut self) -> ALAF_W<STS_SPEC> {
        ALAF_W::new(self, 8)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn watf(&mut self) -> WATF_W<STS_SPEC> {
        WATF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Timestamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<STS_SPEC> {
        TSF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Timestamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsof(&mut self) -> TSOF_W<STS_SPEC> {
        TSOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Tamper detection 1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn tp1f(&mut self) -> TP1F_W<STS_SPEC> {
        TP1F_W::new(self, 13)
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
#[doc = "initialization and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0x07"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
