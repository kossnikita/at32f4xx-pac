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
pub type SWPEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPEN2` reader - Standby wake-up pin2 enable"]
pub type SWPEN2_R = crate::BitReader;
#[doc = "Field `SWPEN2` writer - Standby wake-up pin2 enable"]
pub type SWPEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPEN4` reader - Standby wake-up pin4 enable"]
pub type SWPEN4_R = crate::BitReader;
#[doc = "Field `SWPEN4` writer - Standby wake-up pin4 enable"]
pub type SWPEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPEN5` reader - Standby wake-up pin5 enable"]
pub type SWPEN5_R = crate::BitReader;
#[doc = "Field `SWPEN5` writer - Standby wake-up pin5 enable"]
pub type SWPEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPEN6` reader - Standby wake-up pin6 enable"]
pub type SWPEN6_R = crate::BitReader;
#[doc = "Field `SWPEN6` writer - Standby wake-up pin6 enable"]
pub type SWPEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPEN7` reader - Standby wake-up pin7 enable"]
pub type SWPEN7_R = crate::BitReader;
#[doc = "Field `SWPEN7` writer - Standby wake-up pin7 enable"]
pub type SWPEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bit 8 - Standby wake-up pin1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen1(&mut self) -> SWPEN1_W<CTRLSTS_SPEC, 8> {
        SWPEN1_W::new(self)
    }
    #[doc = "Bit 9 - Standby wake-up pin2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen2(&mut self) -> SWPEN2_W<CTRLSTS_SPEC, 9> {
        SWPEN2_W::new(self)
    }
    #[doc = "Bit 11 - Standby wake-up pin4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen4(&mut self) -> SWPEN4_W<CTRLSTS_SPEC, 11> {
        SWPEN4_W::new(self)
    }
    #[doc = "Bit 12 - Standby wake-up pin5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen5(&mut self) -> SWPEN5_W<CTRLSTS_SPEC, 12> {
        SWPEN5_W::new(self)
    }
    #[doc = "Bit 13 - Standby wake-up pin6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen6(&mut self) -> SWPEN6_W<CTRLSTS_SPEC, 13> {
        SWPEN6_W::new(self)
    }
    #[doc = "Bit 14 - Standby wake-up pin7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen7(&mut self) -> SWPEN7_W<CTRLSTS_SPEC, 14> {
        SWPEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Power control register (PWC_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CTRLSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}