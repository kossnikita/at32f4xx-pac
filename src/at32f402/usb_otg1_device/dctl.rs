#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RWKUPSIG` reader - Remote wakeup signaling"]
pub type RWKUPSIG_R = crate::BitReader;
#[doc = "Field `RWKUPSIG` writer - Remote wakeup signaling"]
pub type RWKUPSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTDISCON` reader - Soft disconnect"]
pub type SFTDISCON_R = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - Soft disconnect"]
pub type SFTDISCON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNPINNAKSTS` reader - Global IN NAK status"]
pub type GNPINNAKSTS_R = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK status"]
pub type GOUTNAKSTS_R = crate::BitReader;
#[doc = "Field `TSTCTL` reader - Test control"]
pub type TSTCTL_R = crate::FieldReader;
#[doc = "Field `TSTCTL` writer - Test control"]
pub type TSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGNPINNAK` reader - Set global IN NAK"]
pub type SGNPINNAK_R = crate::BitReader;
#[doc = "Field `SGNPINNAK` writer - Set global IN NAK"]
pub type SGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPINNAK` reader - Clear global IN NAK"]
pub type CGNPINNAK_R = crate::BitReader;
#[doc = "Field `CGNPINNAK` writer - Clear global IN NAK"]
pub type CGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNAK` reader - Set global OUT NAK"]
pub type SGOUTNAK_R = crate::BitReader;
#[doc = "Field `SGOUTNAK` writer - Set global OUT NAK"]
pub type SGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNAK` reader - Clear global OUT NAK"]
pub type CGOUTNAK_R = crate::BitReader;
#[doc = "Field `CGOUTNAK` writer - Clear global OUT NAK"]
pub type CGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROPRGDNE` reader - Power-on programming done"]
pub type PWROPRGDNE_R = crate::BitReader;
#[doc = "Field `PWROPRGDNE` writer - Power-on programming done"]
pub type PWROPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwkupsig(&self) -> RWKUPSIG_R {
        RWKUPSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sgnpinnak(&self) -> SGNPINNAK_R {
        SGNPINNAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cgnpinnak(&self) -> CGNPINNAK_R {
        CGNPINNAK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&self) -> SGOUTNAK_R {
        SGOUTNAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&self) -> CGOUTNAK_R {
        CGOUTNAK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn pwroprgdne(&self) -> PWROPRGDNE_R {
        PWROPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rwkupsig", &self.rwkupsig())
            .field("sftdiscon", &self.sftdiscon())
            .field("gnpinnaksts", &self.gnpinnaksts())
            .field("goutnaksts", &self.goutnaksts())
            .field("tstctl", &self.tstctl())
            .field("sgnpinnak", &self.sgnpinnak())
            .field("cgnpinnak", &self.cgnpinnak())
            .field("sgoutnak", &self.sgoutnak())
            .field("cgoutnak", &self.cgoutnak())
            .field("pwroprgdne", &self.pwroprgdne())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwkupsig(&mut self) -> RWKUPSIG_W<'_, DCTL_SPEC> {
        RWKUPSIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W<'_, DCTL_SPEC> {
        SFTDISCON_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tstctl(&mut self) -> TSTCTL_W<'_, DCTL_SPEC> {
        TSTCTL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W<'_, DCTL_SPEC> {
        SGNPINNAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W<'_, DCTL_SPEC> {
        CGNPINNAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W<'_, DCTL_SPEC> {
        SGOUTNAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W<'_, DCTL_SPEC> {
        CGOUTNAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn pwroprgdne(&mut self) -> PWROPRGDNE_W<'_, DCTL_SPEC> {
        PWROPRGDNE_W::new(self, 11)
    }
}
#[doc = "OTGFS device control register (OTGFS_DCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DCTL_SPEC {}
