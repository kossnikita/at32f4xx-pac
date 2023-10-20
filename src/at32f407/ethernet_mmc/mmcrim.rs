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
            .field("rcefcim", &format_args!("{}", self.rcefcim().bit()))
            .field("raefacim", &format_args!("{}", self.raefacim().bit()))
            .field("rugfcim", &format_args!("{}", self.rugfcim().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMCRIM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 5 - Received CRC error frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rcefcim(&mut self) -> RCEFCIM_W<MMCRIM_SPEC> {
        RCEFCIM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received alignment error frame alignment counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn raefacim(&mut self) -> RAEFACIM_W<MMCRIM_SPEC> {
        RAEFACIM_W::new(self, 6)
    }
    #[doc = "Bit 17 - Received unicast good frame counter interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rugfcim(&mut self) -> RUGFCIM_W<MMCRIM_SPEC> {
        RUGFCIM_W::new(self, 17)
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
#[doc = "Ethernet MMC receive interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRIM_SPEC;
impl crate::RegisterSpec for MMCRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrim::R`](R) reader structure"]
impl crate::Readable for MMCRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmcrim::W`](W) writer structure"]
impl crate::Writable for MMCRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCRIM to value 0"]
impl crate::Resettable for MMCRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
