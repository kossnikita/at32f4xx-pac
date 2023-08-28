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
#[doc = "Field `SWPEN` reader - Standby wake-up pin enable"]
pub type SWPEN_R = crate::BitReader;
#[doc = "Field `SWPEN` writer - Standby wake-up pin enable"]
pub type SWPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 8 - Standby wake-up pin enable"]
    #[inline(always)]
    pub fn swpen(&self) -> SWPEN_R {
        SWPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Standby wake-up pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpen(&mut self) -> SWPEN_W<CTRLSTS_SPEC, 8> {
        SWPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Power control register (PWC_CTRLST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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