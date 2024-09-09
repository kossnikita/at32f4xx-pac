#[doc = "Register `MACIMR` reader"]
pub type R = crate::R<MACIMR_SPEC>;
#[doc = "Register `MACIMR` writer"]
pub type W = crate::W<MACIMR_SPEC>;
#[doc = "Field `PIM` reader - PMT interrupt mask"]
pub type PIM_R = crate::BitReader;
#[doc = "Field `PIM` writer - PMT interrupt mask"]
pub type PIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM` reader - Timestamp interrupt mask"]
pub type TIM_R = crate::BitReader;
#[doc = "Field `TIM` writer - Timestamp interrupt mask"]
pub type TIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pim(&self) -> PIM_R {
        PIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp interrupt mask"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACIMR")
            .field("pim", &self.pim())
            .field("tim", &self.tim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pim(&mut self) -> PIM_W<MACIMR_SPEC> {
        PIM_W::new(self, 3)
    }
    #[doc = "Bit 9 - Timestamp interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tim(&mut self) -> TIM_W<MACIMR_SPEC> {
        TIM_W::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`macimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIMR_SPEC;
impl crate::RegisterSpec for MACIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macimr::R`](R) reader structure"]
impl crate::Readable for MACIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macimr::W`](W) writer structure"]
impl crate::Writable for MACIMR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MACIMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
