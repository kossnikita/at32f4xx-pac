#[doc = "Register `MUXC4CTRL` reader"]
pub type R = crate::R<MUXC4CTRL_SPEC>;
#[doc = "Register `MUXC4CTRL` writer"]
pub type W = crate::W<MUXC4CTRL_SPEC>;
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
#[doc = "Field `SYNCSEL` reader - Synchronization Identification"]
pub type SYNCSEL_R = crate::FieldReader;
#[doc = "Field `SYNCSEL` writer - Synchronization Identification"]
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
    #[doc = "Bits 24:28 - Synchronization Identification"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXC4CTRL")
            .field("reqsel", &self.reqsel())
            .field("syncovien", &self.syncovien())
            .field("evtgen", &self.evtgen())
            .field("syncen", &self.syncen())
            .field("syncpol", &self.syncpol())
            .field("reqcnt", &self.reqcnt())
            .field("syncsel", &self.syncsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<MUXC4CTRL_SPEC> {
        REQSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncovien(&mut self) -> SYNCOVIEN_W<MUXC4CTRL_SPEC> {
        SYNCOVIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn evtgen(&mut self) -> EVTGEN_W<MUXC4CTRL_SPEC> {
        EVTGEN_W::new(self, 9)
    }
    #[doc = "Bit 16 - Synchroniztion enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<MUXC4CTRL_SPEC> {
        SYNCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SYNCPOL_W<MUXC4CTRL_SPEC> {
        SYNCPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    #[must_use]
    pub fn reqcnt(&mut self) -> REQCNT_W<MUXC4CTRL_SPEC> {
        REQCNT_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization Identification"]
    #[inline(always)]
    #[must_use]
    pub fn syncsel(&mut self) -> SYNCSEL_W<MUXC4CTRL_SPEC> {
        SYNCSEL_W::new(self, 24)
    }
}
#[doc = "Channel 4 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxc4ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxc4ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXC4CTRL_SPEC;
impl crate::RegisterSpec for MUXC4CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxc4ctrl::R`](R) reader structure"]
impl crate::Readable for MUXC4CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxc4ctrl::W`](W) writer structure"]
impl crate::Writable for MUXC4CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXC4CTRL to value 0"]
impl crate::Resettable for MUXC4CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
