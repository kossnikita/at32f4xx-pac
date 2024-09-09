#[doc = "Register `BRK` reader"]
pub type R = crate::R<BRK_SPEC>;
#[doc = "Register `BRK` writer"]
pub type W = crate::W<BRK_SPEC>;
#[doc = "Field `DTC` reader - Dead-time configuration"]
pub type DTC_R = crate::FieldReader;
#[doc = "Field `DTC` writer - Dead-time configuration"]
pub type DTC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WPC` reader - Write protected configuration"]
pub type WPC_R = crate::FieldReader;
#[doc = "Field `WPC` writer - Write protected configuration"]
pub type WPC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FCSODIS` reader - Frozen channel status when holistic output disable"]
pub type FCSODIS_R = crate::BitReader;
#[doc = "Field `FCSODIS` writer - Frozen channel status when holistic output disable"]
pub type FCSODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSOEN` reader - Frozen channel status when holistic output enable"]
pub type FCSOEN_R = crate::BitReader;
#[doc = "Field `FCSOEN` writer - Frozen channel status when holistic output enable"]
pub type FCSOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKEN` reader - Brake enable"]
pub type BRKEN_R = crate::BitReader;
#[doc = "Field `BRKEN` writer - Brake enable"]
pub type BRKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKV` reader - Brake input validity"]
pub type BRKV_R = crate::BitReader;
#[doc = "Field `BRKV` writer - Brake input validity"]
pub type BRKV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOEN` reader - Automatic output enable"]
pub type AOEN_R = crate::BitReader;
#[doc = "Field `AOEN` writer - Automatic output enable"]
pub type AOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN` reader - Output enable"]
pub type OEN_R = crate::BitReader;
#[doc = "Field `OEN` writer - Output enable"]
pub type OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKF` reader - brake input filter"]
pub type BKF_R = crate::FieldReader;
#[doc = "Field `BKF` writer - brake input filter"]
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    pub fn dtc(&self) -> DTC_R {
        DTC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    pub fn wpc(&self) -> WPC_R {
        WPC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    pub fn fcsodis(&self) -> FCSODIS_R {
        FCSODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    pub fn fcsoen(&self) -> FCSOEN_R {
        FCSOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    pub fn brken(&self) -> BRKEN_R {
        BRKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    pub fn brkv(&self) -> BRKV_R {
        BRKV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoen(&self) -> AOEN_R {
        AOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - brake input filter"]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRK")
            .field("bkf", &self.bkf())
            .field("oen", &self.oen())
            .field("aoen", &self.aoen())
            .field("brkv", &self.brkv())
            .field("brken", &self.brken())
            .field("fcsoen", &self.fcsoen())
            .field("fcsodis", &self.fcsodis())
            .field("wpc", &self.wpc())
            .field("dtc", &self.dtc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dtc(&mut self) -> DTC_W<BRK_SPEC> {
        DTC_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wpc(&mut self) -> WPC_W<BRK_SPEC> {
        WPC_W::new(self, 8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    #[must_use]
    pub fn fcsodis(&mut self) -> FCSODIS_W<BRK_SPEC> {
        FCSODIS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcsoen(&mut self) -> FCSOEN_W<BRK_SPEC> {
        FCSOEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    #[must_use]
    pub fn brken(&mut self) -> BRKEN_W<BRK_SPEC> {
        BRKEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    #[must_use]
    pub fn brkv(&mut self) -> BRKV_W<BRK_SPEC> {
        BRKV_W::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn aoen(&mut self) -> AOEN_W<BRK_SPEC> {
        AOEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OEN_W<BRK_SPEC> {
        OEN_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - brake input filter"]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<BRK_SPEC> {
        BKF_W::new(self, 16)
    }
}
#[doc = "Brake register\n\nYou can [`read`](crate::Reg::read) this register and get [`brk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRK_SPEC;
impl crate::RegisterSpec for BRK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brk::R`](R) reader structure"]
impl crate::Readable for BRK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brk::W`](W) writer structure"]
impl crate::Writable for BRK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRK to value 0"]
impl crate::Resettable for BRK_SPEC {
    const RESET_VALUE: u32 = 0;
}
