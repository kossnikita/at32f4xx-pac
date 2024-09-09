#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DOEPMSK_SPEC>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DOEPMSK_SPEC>;
#[doc = "Field `XFERCMSK` reader - Transfer completed interrupt mask"]
pub type XFERCMSK_R = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed interrupt mask"]
pub type XFERCMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISMSK` reader - Endpoint disabled interrupt mask"]
pub type EPTDISMSK_R = crate::BitReader;
#[doc = "Field `EPTDISMSK` writer - Endpoint disabled interrupt mask"]
pub type EPTDISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPMSK` reader - SETUP phase done mask"]
pub type SETUPMSK_R = crate::BitReader;
#[doc = "Field `SETUPMSK` writer - SETUP phase done mask"]
pub type SETUPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTEPDMSK` reader - OUT token received when endpoint disabled mask"]
pub type OUTTEPDMSK_R = crate::BitReader;
#[doc = "Field `OUTTEPDMSK` writer - OUT token received when endpoint disabled mask"]
pub type OUTTEPDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSETUPMSK` reader - Back-to-back SETUP packets received mask"]
pub type B2BSETUPMSK_R = crate::BitReader;
#[doc = "Field `B2BSETUPMSK` writer - Back-to-back SETUP packets received mask"]
pub type B2BSETUPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPERRMSK` reader - OUT packet error mask"]
pub type OUTPERRMSK_R = crate::BitReader;
#[doc = "Field `OUTPERRMSK` writer - OUT packet error mask"]
pub type OUTPERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAOUTMSK` reader - BNA interrupt mask"]
pub type BNAOUTMSK_R = crate::BitReader;
#[doc = "Field `BNAOUTMSK` writer - BNA interrupt mask"]
pub type BNAOUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn setupmsk(&self) -> SETUPMSK_R {
        SETUPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn outtepdmsk(&self) -> OUTTEPDMSK_R {
        OUTTEPDMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bsetupmsk(&self) -> B2BSETUPMSK_R {
        B2BSETUPMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn outperrmsk(&self) -> OUTPERRMSK_R {
        OUTPERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bnaoutmsk(&self) -> BNAOUTMSK_R {
        BNAOUTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPMSK")
            .field("xfercmsk", &self.xfercmsk())
            .field("eptdismsk", &self.eptdismsk())
            .field("setupmsk", &self.setupmsk())
            .field("outtepdmsk", &self.outtepdmsk())
            .field("b2bsetupmsk", &self.b2bsetupmsk())
            .field("outperrmsk", &self.outperrmsk())
            .field("bnaoutmsk", &self.bnaoutmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XFERCMSK_W<DOEPMSK_SPEC> {
        XFERCMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eptdismsk(&mut self) -> EPTDISMSK_W<DOEPMSK_SPEC> {
        EPTDISMSK_W::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    #[must_use]
    pub fn setupmsk(&mut self) -> SETUPMSK_W<DOEPMSK_SPEC> {
        SETUPMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    #[must_use]
    pub fn outtepdmsk(&mut self) -> OUTTEPDMSK_W<DOEPMSK_SPEC> {
        OUTTEPDMSK_W::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    #[must_use]
    pub fn b2bsetupmsk(&mut self) -> B2BSETUPMSK_W<DOEPMSK_SPEC> {
        B2BSETUPMSK_W::new(self, 6)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    #[must_use]
    pub fn outperrmsk(&mut self) -> OUTPERRMSK_W<DOEPMSK_SPEC> {
        OUTPERRMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bnaoutmsk(&mut self) -> BNAOUTMSK_W<DOEPMSK_SPEC> {
        BNAOUTMSK_W::new(self, 9)
    }
}
#[doc = "OTGFS device OUT endpoint common interrupt mask register (OTGFS_DOEPMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
