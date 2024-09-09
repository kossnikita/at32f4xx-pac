#[doc = "Register `MMCTIM` reader"]
pub type R = crate::R<MMCTIM_SPEC>;
#[doc = "Register `MMCTIM` writer"]
pub type W = crate::W<MMCTIM_SPEC>;
#[doc = "Field `TSCGFCIM` reader - Transmit single collision good frame counter interrupt mask"]
pub type TSCGFCIM_R = crate::BitReader;
#[doc = "Field `TSCGFCIM` writer - Transmit single collision good frame counter interrupt mask"]
pub type TSCGFCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMCGFCIM` reader - Transmit multiple collision good frame counter interrupt mask"]
pub type TMCGFCIM_R = crate::BitReader;
#[doc = "Field `TMCGFCIM` writer - Transmit multiple collision good frame counter interrupt mask"]
pub type TMCGFCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFCIM` reader - Transmitted good frame counter interrupt mask"]
pub type TGFCIM_R = crate::BitReader;
#[doc = "Field `TGFCIM` writer - Transmitted good frame counter interrupt mask"]
pub type TGFCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt mask"]
    #[inline(always)]
    pub fn tscgfcim(&self) -> TSCGFCIM_R {
        TSCGFCIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit multiple collision good frame counter interrupt mask"]
    #[inline(always)]
    pub fn tmcgfcim(&self) -> TMCGFCIM_R {
        TMCGFCIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frame counter interrupt mask"]
    #[inline(always)]
    pub fn tgfcim(&self) -> TGFCIM_R {
        TGFCIM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTIM")
            .field("tscgfcim", &self.tscgfcim())
            .field("tmcgfcim", &self.tmcgfcim())
            .field("tgfcim", &self.tgfcim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - Transmit single collision good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tscgfcim(&mut self) -> TSCGFCIM_W<MMCTIM_SPEC> {
        TSCGFCIM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit multiple collision good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tmcgfcim(&mut self) -> TMCGFCIM_W<MMCTIM_SPEC> {
        TMCGFCIM_W::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfcim(&mut self) -> TGFCIM_W<MMCTIM_SPEC> {
        TGFCIM_W::new(self, 21)
    }
}
#[doc = "Ethernet MMC transmit interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTIM_SPEC;
impl crate::RegisterSpec for MMCTIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctim::R`](R) reader structure"]
impl crate::Readable for MMCTIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmctim::W`](W) writer structure"]
impl crate::Writable for MMCTIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMCTIM to value 0"]
impl crate::Resettable for MMCTIM_SPEC {
    const RESET_VALUE: u32 = 0;
}
