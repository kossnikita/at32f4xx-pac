#[doc = "Register `DIEPMSK` reader"]
pub type R = crate::R<DIEPMSK_SPEC>;
#[doc = "Register `DIEPMSK` writer"]
pub type W = crate::W<DIEPMSK_SPEC>;
#[doc = "Field `XFERCMSK` reader - Transfer completed interrupt mask"]
pub type XFERCMSK_R = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed interrupt mask"]
pub type XFERCMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISMSK` reader - Endpoint disabled interrupt mask"]
pub type EPTDISMSK_R = crate::BitReader;
#[doc = "Field `EPTDISMSK` writer - Endpoint disabled interrupt mask"]
pub type EPTDISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTMSK` reader - Timeout condition mask (Non-isochronous endpoints)"]
pub type TIMEOUTMSK_R = crate::BitReader;
#[doc = "Field `TIMEOUTMSK` writer - Timeout condition mask (Non-isochronous endpoints)"]
pub type TIMEOUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMPMSK` reader - IN token received when TxFIFO empty mask"]
pub type INTKNTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMPMSK` writer - IN token received when TxFIFO empty mask"]
pub type INTKNTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPTMISMSK` reader - IN token received with EP mismatch mask"]
pub type INTKNEPTMISMSK_R = crate::BitReader;
#[doc = "Field `INTKNEPTMISMSK` writer - IN token received with EP mismatch mask"]
pub type INTKNEPTMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPTNAKMSK` reader - IN endpoint NAK effective mask"]
pub type INEPTNAKMSK_R = crate::BitReader;
#[doc = "Field `INEPTNAKMSK` writer - IN endpoint NAK effective mask"]
pub type INEPTNAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOUDRMSK` reader - FIFO underrun mask"]
pub type TXFIFOUDRMSK_R = crate::BitReader;
#[doc = "Field `TXFIFOUDRMSK` writer - FIFO underrun mask"]
pub type TXFIFOUDRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINMSK` reader - BNA interrupt mask"]
pub type BNAINMSK_R = crate::BitReader;
#[doc = "Field `BNAINMSK` writer - BNA interrupt mask"]
pub type BNAINMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfercmsk(&self) -> XFERCMSK_R {
        XFERCMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn eptdismsk(&self) -> EPTDISMSK_R {
        EPTDISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn intkneptmismsk(&self) -> INTKNEPTMISMSK_R {
        INTKNEPTMISMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn ineptnakmsk(&self) -> INEPTNAKMSK_R {
        INEPTNAKMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfifoudrmsk(&self) -> TXFIFOUDRMSK_R {
        TXFIFOUDRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnainmsk(&self) -> BNAINMSK_R {
        BNAINMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field("xfercmsk", &self.xfercmsk())
            .field("eptdismsk", &self.eptdismsk())
            .field("timeoutmsk", &self.timeoutmsk())
            .field("intkntxfempmsk", &self.intkntxfempmsk())
            .field("intkneptmismsk", &self.intkneptmismsk())
            .field("ineptnakmsk", &self.ineptnakmsk())
            .field("txfifoudrmsk", &self.txfifoudrmsk())
            .field("bnainmsk", &self.bnainmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfercmsk(&mut self) -> XFERCMSK_W<'_, DIEPMSK_SPEC> {
        XFERCMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn eptdismsk(&mut self) -> EPTDISMSK_W<'_, DIEPMSK_SPEC> {
        EPTDISMSK_W::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn timeoutmsk(&mut self) -> TIMEOUTMSK_W<'_, DIEPMSK_SPEC> {
        TIMEOUTMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn intkntxfempmsk(&mut self) -> INTKNTXFEMPMSK_W<'_, DIEPMSK_SPEC> {
        INTKNTXFEMPMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn intkneptmismsk(&mut self) -> INTKNEPTMISMSK_W<'_, DIEPMSK_SPEC> {
        INTKNEPTMISMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn ineptnakmsk(&mut self) -> INEPTNAKMSK_W<'_, DIEPMSK_SPEC> {
        INEPTNAKMSK_W::new(self, 6)
    }
    #[doc = "Bit 8 - FIFO underrun mask"]
    #[inline(always)]
    pub fn txfifoudrmsk(&mut self) -> TXFIFOUDRMSK_W<'_, DIEPMSK_SPEC> {
        TXFIFOUDRMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnainmsk(&mut self) -> BNAINMSK_W<'_, DIEPMSK_SPEC> {
        BNAINMSK_W::new(self, 9)
    }
}
#[doc = "OTGFS device IN endpoint common interrupt mask register (OTGFS_DIEPMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepmsk::R`](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {}
