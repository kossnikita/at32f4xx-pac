#[doc = "Register `MACIMR` reader"]
pub type R = crate::R<MACIMR_SPEC>;
#[doc = "Register `MACIMR` writer"]
pub type W = crate::W<MACIMR_SPEC>;
#[doc = "Field `PIM` reader - PMT interrupt mask"]
pub type PIM_R = crate::BitReader;
#[doc = "Field `PIM` writer - PMT interrupt mask"]
pub type PIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM` reader - Timestamp interrupt mask"]
pub type TIM_R = crate::BitReader;
#[doc = "Field `TIM` writer - Timestamp interrupt mask"]
pub type TIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
            .field("pim", &format_args!("{}", self.pim().bit()))
            .field("tim", &format_args!("{}", self.tim().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACIMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pim(&mut self) -> PIM_W<MACIMR_SPEC, 3> {
        PIM_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn tim(&mut self) -> TIM_W<MACIMR_SPEC, 9> {
        TIM_W::new(self)
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
#[doc = "Ethernet MAC interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIMR_SPEC;
impl crate::RegisterSpec for MACIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macimr::R`](R) reader structure"]
impl crate::Readable for MACIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macimr::W`](W) writer structure"]
impl crate::Writable for MACIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MACIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
