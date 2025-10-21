#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Field `SWEF` reader - Standby wake-up event flag"]
pub type SWEF_R = crate::BitReader;
#[doc = "Field `SEF` reader - Standby mode entry flag"]
pub type SEF_R = crate::BitReader;
#[doc = "Field `PVMOF` reader - Power voltage monitoring output flag"]
pub type PVMOF_R = crate::BitReader;
#[doc = "Field `SWPEN1` reader - Standby wake-up pin1 enable"]
pub type SWPEN1_R = crate::BitReader;
#[doc = "Field `SWPEN1` writer - Standby wake-up pin1 enable"]
pub type SWPEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN2` reader - Standby wake-up pin2 enable"]
pub type SWPEN2_R = crate::BitReader;
#[doc = "Field `SWPEN2` writer - Standby wake-up pin2 enable"]
pub type SWPEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN4` reader - Standby wake-up pin4 enable"]
pub type SWPEN4_R = crate::BitReader;
#[doc = "Field `SWPEN4` writer - Standby wake-up pin4 enable"]
pub type SWPEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN5` reader - Standby wake-up pin5 enable"]
pub type SWPEN5_R = crate::BitReader;
#[doc = "Field `SWPEN5` writer - Standby wake-up pin5 enable"]
pub type SWPEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN6` reader - Standby wake-up pin6 enable"]
pub type SWPEN6_R = crate::BitReader;
#[doc = "Field `SWPEN6` writer - Standby wake-up pin6 enable"]
pub type SWPEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN7` reader - Standby wake-up pin7 enable"]
pub type SWPEN7_R = crate::BitReader;
#[doc = "Field `SWPEN7` writer - Standby wake-up pin7 enable"]
pub type SWPEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Standby wake-up event flag"]
    #[inline(always)]
    pub fn swef(&self) -> SWEF_R {
        SWEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby mode entry flag"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power voltage monitoring output flag"]
    #[inline(always)]
    pub fn pvmof(&self) -> PVMOF_R {
        PVMOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby wake-up pin1 enable"]
    #[inline(always)]
    pub fn swpen1(&self) -> SWPEN1_R {
        SWPEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen2(&self) -> SWPEN2_R {
        SWPEN2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Standby wake-up pin4 enable"]
    #[inline(always)]
    pub fn swpen4(&self) -> SWPEN4_R {
        SWPEN4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Standby wake-up pin5 enable"]
    #[inline(always)]
    pub fn swpen5(&self) -> SWPEN5_R {
        SWPEN5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Standby wake-up pin6 enable"]
    #[inline(always)]
    pub fn swpen6(&self) -> SWPEN6_R {
        SWPEN6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Standby wake-up pin7 enable"]
    #[inline(always)]
    pub fn swpen7(&self) -> SWPEN7_R {
        SWPEN7_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("swef", &self.swef())
            .field("sef", &self.sef())
            .field("pvmof", &self.pvmof())
            .field("swpen1", &self.swpen1())
            .field("swpen2", &self.swpen2())
            .field("swpen4", &self.swpen4())
            .field("swpen5", &self.swpen5())
            .field("swpen6", &self.swpen6())
            .field("swpen7", &self.swpen7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Standby wake-up pin1 enable"]
    #[inline(always)]
    pub fn swpen1(&mut self) -> SWPEN1_W<'_, CTRLSTS_SPEC> {
        SWPEN1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen2(&mut self) -> SWPEN2_W<'_, CTRLSTS_SPEC> {
        SWPEN2_W::new(self, 9)
    }
    #[doc = "Bit 11 - Standby wake-up pin4 enable"]
    #[inline(always)]
    pub fn swpen4(&mut self) -> SWPEN4_W<'_, CTRLSTS_SPEC> {
        SWPEN4_W::new(self, 11)
    }
    #[doc = "Bit 12 - Standby wake-up pin5 enable"]
    #[inline(always)]
    pub fn swpen5(&mut self) -> SWPEN5_W<'_, CTRLSTS_SPEC> {
        SWPEN5_W::new(self, 12)
    }
    #[doc = "Bit 13 - Standby wake-up pin6 enable"]
    #[inline(always)]
    pub fn swpen6(&mut self) -> SWPEN6_W<'_, CTRLSTS_SPEC> {
        SWPEN6_W::new(self, 13)
    }
    #[doc = "Bit 14 - Standby wake-up pin7 enable"]
    #[inline(always)]
    pub fn swpen7(&mut self) -> SWPEN7_W<'_, CTRLSTS_SPEC> {
        SWPEN7_W::new(self, 14)
    }
}
#[doc = "Power control register (PWC_CTRLSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CTRLSTS_SPEC {}
