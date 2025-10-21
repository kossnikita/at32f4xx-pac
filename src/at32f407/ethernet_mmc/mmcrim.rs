#[doc = "Register `MMCRIM` reader"]
pub type R = crate::R<MMCRIM_SPEC>;
#[doc = "Register `MMCRIM` writer"]
pub type W = crate::W<MMCRIM_SPEC>;
#[doc = "Field `RCEFCIM` reader - Received CRC error frame counter interrupt mask"]
pub type RCEFCIM_R = crate::BitReader;
#[doc = "Field `RCEFCIM` writer - Received CRC error frame counter interrupt mask"]
pub type RCEFCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAEFACIM` reader - Received alignment error frame alignment counter interrupt mask"]
pub type RAEFACIM_R = crate::BitReader;
#[doc = "Field `RAEFACIM` writer - Received alignment error frame alignment counter interrupt mask"]
pub type RAEFACIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUGFCIM` reader - Received unicast good frame counter interrupt mask"]
pub type RUGFCIM_R = crate::BitReader;
#[doc = "Field `RUGFCIM` writer - Received unicast good frame counter interrupt mask"]
pub type RUGFCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received CRC error frame counter interrupt mask"]
    #[inline(always)]
    pub fn rcefcim(&self) -> RCEFCIM_R {
        RCEFCIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received alignment error frame alignment counter interrupt mask"]
    #[inline(always)]
    pub fn raefacim(&self) -> RAEFACIM_R {
        RAEFACIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received unicast good frame counter interrupt mask"]
    #[inline(always)]
    pub fn rugfcim(&self) -> RUGFCIM_R {
        RUGFCIM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCRIM")
            .field("rcefcim", &self.rcefcim())
            .field("raefacim", &self.raefacim())
            .field("rugfcim", &self.rugfcim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Received CRC error frame counter interrupt mask"]
    #[inline(always)]
    pub fn rcefcim(&mut self) -> RCEFCIM_W<'_, MMCRIM_SPEC> {
        RCEFCIM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received alignment error frame alignment counter interrupt mask"]
    #[inline(always)]
    pub fn raefacim(&mut self) -> RAEFACIM_W<'_, MMCRIM_SPEC> {
        RAEFACIM_W::new(self, 6)
    }
    #[doc = "Bit 17 - Received unicast good frame counter interrupt mask"]
    #[inline(always)]
    pub fn rugfcim(&mut self) -> RUGFCIM_W<'_, MMCRIM_SPEC> {
        RUGFCIM_W::new(self, 17)
    }
}
#[doc = "Ethernet MMC receive interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmcrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmcrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRIM_SPEC;
impl crate::RegisterSpec for MMCRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrim::R`](R) reader structure"]
impl crate::Readable for MMCRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmcrim::W`](W) writer structure"]
impl crate::Writable for MMCRIM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCRIM to value 0"]
impl crate::Resettable for MMCRIM_SPEC {}
