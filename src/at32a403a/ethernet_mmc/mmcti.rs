#[doc = "Register `MMCTI` reader"]
pub type R = crate::R<MMCTI_SPEC>;
#[doc = "Register `MMCTI` writer"]
pub type W = crate::W<MMCTI_SPEC>;
#[doc = "Field `TSCGFCI` reader - Transmit single collision good frame counter interrupt"]
pub type TSCGFCI_R = crate::BitReader;
#[doc = "Field `TSCGFCI` writer - Transmit single collision good frame counter interrupt"]
pub type TSCGFCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFMSC` reader - Transmit good frames more single collision"]
pub type TGFMSC_R = crate::BitReader;
#[doc = "Field `TGFMSC` writer - Transmit good frames more single collision"]
pub type TGFMSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGF` reader - Transmitted good frames"]
pub type TGF_R = crate::BitReader;
#[doc = "Field `TGF` writer - Transmitted good frames"]
pub type TGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt"]
    #[inline(always)]
    pub fn tscgfci(&self) -> TSCGFCI_R {
        TSCGFCI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit good frames more single collision"]
    #[inline(always)]
    pub fn tgfmsc(&self) -> TGFMSC_R {
        TGFMSC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames"]
    #[inline(always)]
    pub fn tgf(&self) -> TGF_R {
        TGF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTI")
            .field("tscgfci", &self.tscgfci())
            .field("tgfmsc", &self.tgfmsc())
            .field("tgf", &self.tgf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt"]
    #[inline(always)]
    pub fn tscgfci(&mut self) -> TSCGFCI_W<'_, MMCTI_SPEC> {
        TSCGFCI_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit good frames more single collision"]
    #[inline(always)]
    pub fn tgfmsc(&mut self) -> TGFMSC_W<'_, MMCTI_SPEC> {
        TGFMSC_W::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames"]
    #[inline(always)]
    pub fn tgf(&mut self) -> TGF_W<'_, MMCTI_SPEC> {
        TGF_W::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTI_SPEC;
impl crate::RegisterSpec for MMCTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcti::R`](R) reader structure"]
impl crate::Readable for MMCTI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmcti::W`](W) writer structure"]
impl crate::Writable for MMCTI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCTI to value 0"]
impl crate::Resettable for MMCTI_SPEC {}
