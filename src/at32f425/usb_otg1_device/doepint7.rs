#[doc = "Register `DOEPINT7` reader"]
pub type R = crate::R<DOEPINT7_SPEC>;
#[doc = "Register `DOEPINT7` writer"]
pub type W = crate::W<DOEPINT7_SPEC>;
#[doc = "Field `XFERC` reader - Transfer completed interrupt"]
pub type XFERC_R = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed interrupt"]
pub type XFERC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDISD` reader - Endpoint disabled interrupt"]
pub type EPTDISD_R = crate::BitReader;
#[doc = "Field `EPTDISD` writer - Endpoint disabled interrupt"]
pub type EPTDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP` reader - SETUP phase done"]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `SETUP` writer - SETUP phase done"]
pub type SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTEPD` reader - OUT token received when endpoint disabled"]
pub type OUTTEPD_R = crate::BitReader;
#[doc = "Field `OUTTEPD` writer - OUT token received when endpoint disabled"]
pub type OUTTEPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2BSTUP_R = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xferc(&self) -> XFERC_R {
        XFERC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn eptdisd(&self) -> EPTDISD_R {
        EPTDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn outtepd(&self) -> OUTTEPD_R {
        OUTTEPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT7")
            .field("xferc", &self.xferc())
            .field("eptdisd", &self.eptdisd())
            .field("setup", &self.setup())
            .field("outtepd", &self.outtepd())
            .field("b2bstup", &self.b2bstup())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XFERC_W<DOEPINT7_SPEC> {
        XFERC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EPTDISD_W<DOEPINT7_SPEC> {
        EPTDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<DOEPINT7_SPEC> {
        SETUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn outtepd(&mut self) -> OUTTEPD_W<DOEPINT7_SPEC> {
        OUTTEPD_W::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<DOEPINT7_SPEC> {
        B2BSTUP_W::new(self, 6)
    }
}
#[doc = "OTGFS device OUT endpoint-7 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT7_SPEC;
impl crate::RegisterSpec for DOEPINT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint7::R`](R) reader structure"]
impl crate::Readable for DOEPINT7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint7::W`](W) writer structure"]
impl crate::Writable for DOEPINT7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT7 to value 0x80"]
impl crate::Resettable for DOEPINT7_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
