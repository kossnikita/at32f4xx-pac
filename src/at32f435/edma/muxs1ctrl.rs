#[doc = "Register `MUXS1CTRL` reader"]
pub type R = crate::R<MUXS1CTRL_SPEC>;
#[doc = "Register `MUXS1CTRL` writer"]
pub type W = crate::W<MUXS1CTRL_SPEC>;
#[doc = "Field `REQSEL` reader - DMA request select"]
pub type REQSEL_R = crate::FieldReader;
#[doc = "Field `REQSEL` writer - DMA request select"]
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SYNCOVIEN` reader - Synchronization overrun interrupt enable"]
pub type SYNCOVIEN_R = crate::BitReader;
#[doc = "Field `SYNCOVIEN` writer - Synchronization overrun interrupt enable"]
pub type SYNCOVIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTGEN` reader - Event generation enable"]
pub type EVTGEN_R = crate::BitReader;
#[doc = "Field `EVTGEN` writer - Event generation enable"]
pub type EVTGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - Synchroniztion enable"]
pub type SYNCEN_R = crate::BitReader;
#[doc = "Field `SYNCEN` writer - Synchroniztion enable"]
pub type SYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCPOL` reader - Synchronization polarity"]
pub type SYNCPOL_R = crate::FieldReader;
#[doc = "Field `SYNCPOL` writer - Synchronization polarity"]
pub type SYNCPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REQCNT` reader - Number of DMA requests"]
pub type REQCNT_R = crate::FieldReader;
#[doc = "Field `REQCNT` writer - Number of DMA requests"]
pub type REQCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNCSEL` reader - Synchronization select"]
pub type SYNCSEL_R = crate::FieldReader;
#[doc = "Field `SYNCSEL` writer - Synchronization select"]
pub type SYNCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn syncovien(&self) -> SYNCOVIEN_R {
        SYNCOVIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn evtgen(&self) -> EVTGEN_R {
        EVTGEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchroniztion enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    pub fn reqcnt(&self) -> REQCNT_R {
        REQCNT_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization select"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXS1CTRL")
            .field("reqsel", &format_args!("{}", self.reqsel().bits()))
            .field("syncovien", &format_args!("{}", self.syncovien().bit()))
            .field("evtgen", &format_args!("{}", self.evtgen().bit()))
            .field("syncen", &format_args!("{}", self.syncen().bit()))
            .field("syncpol", &format_args!("{}", self.syncpol().bits()))
            .field("reqcnt", &format_args!("{}", self.reqcnt().bits()))
            .field("syncsel", &format_args!("{}", self.syncsel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MUXS1CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<MUXS1CTRL_SPEC> {
        REQSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncovien(&mut self) -> SYNCOVIEN_W<MUXS1CTRL_SPEC> {
        SYNCOVIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn evtgen(&mut self) -> EVTGEN_W<MUXS1CTRL_SPEC> {
        EVTGEN_W::new(self, 9)
    }
    #[doc = "Bit 16 - Synchroniztion enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<MUXS1CTRL_SPEC> {
        SYNCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SYNCPOL_W<MUXS1CTRL_SPEC> {
        SYNCPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    #[must_use]
    pub fn reqcnt(&mut self) -> REQCNT_W<MUXS1CTRL_SPEC> {
        REQCNT_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization select"]
    #[inline(always)]
    #[must_use]
    pub fn syncsel(&mut self) -> SYNCSEL_W<MUXS1CTRL_SPEC> {
        SYNCSEL_W::new(self, 24)
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
#[doc = "Stream 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs1ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs1ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXS1CTRL_SPEC;
impl crate::RegisterSpec for MUXS1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxs1ctrl::R`](R) reader structure"]
impl crate::Readable for MUXS1CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxs1ctrl::W`](W) writer structure"]
impl crate::Writable for MUXS1CTRL_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXS1CTRL to value 0"]
impl crate::Resettable for MUXS1CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
