#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AHBRST_SPEC>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AHBRST_SPEC>;
#[doc = "Field `GPIOA` reader - IO port A reset"]
pub type GPIOA_R = crate::BitReader;
#[doc = "Field `GPIOA` writer - IO port A reset"]
pub type GPIOA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOB` reader - IO port B reset"]
pub type GPIOB_R = crate::BitReader;
#[doc = "Field `GPIOB` writer - IO port B reset"]
pub type GPIOB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOC` reader - IO port C reset"]
pub type GPIOC_R = crate::BitReader;
#[doc = "Field `GPIOC` writer - IO port C reset"]
pub type GPIOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOF` reader - IO port F reset"]
pub type GPIOF_R = crate::BitReader;
#[doc = "Field `GPIOF` writer - IO port F reset"]
pub type GPIOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 17 - IO port A reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IO port B reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IO port C reset"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - IO port F reset"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST")
            .field("gpioa", &format_args!("{}", self.gpioa().bit()))
            .field("gpiob", &format_args!("{}", self.gpiob().bit()))
            .field("gpioc", &format_args!("{}", self.gpioc().bit()))
            .field("gpiof", &format_args!("{}", self.gpiof().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBRST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 17 - IO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioa(&mut self) -> GPIOA_W<AHBRST_SPEC, 17> {
        GPIOA_W::new(self)
    }
    #[doc = "Bit 18 - IO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiob(&mut self) -> GPIOB_W<AHBRST_SPEC, 18> {
        GPIOB_W::new(self)
    }
    #[doc = "Bit 19 - IO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioc(&mut self) -> GPIOC_W<AHBRST_SPEC, 19> {
        GPIOC_W::new(self)
    }
    #[doc = "Bit 22 - IO port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiof(&mut self) -> GPIOF_W<AHBRST_SPEC, 22> {
        GPIOF_W::new(self)
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
#[doc = "AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST_SPEC;
impl crate::RegisterSpec for AHBRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AHBRST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AHBRST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AHBRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
