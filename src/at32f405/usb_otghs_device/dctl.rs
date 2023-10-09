#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RWKUPSIG` reader - Remote wakeup signaling"]
pub type RWKUPSIG_R = crate::BitReader;
#[doc = "Field `RWKUPSIG` writer - Remote wakeup signaling"]
pub type RWKUPSIG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SFTDISCON` reader - Soft disconnect"]
pub type SFTDISCON_R = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - Soft disconnect"]
pub type SFTDISCON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GNPINNAKSTS` reader - Global IN NAK status"]
pub type GNPINNAKSTS_R = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK status"]
pub type GOUTNAKSTS_R = crate::BitReader;
#[doc = "Field `TSTCTL` reader - Test control"]
pub type TSTCTL_R = crate::FieldReader;
#[doc = "Field `TSTCTL` writer - Test control"]
pub type TSTCTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SGNPINNAK` reader - Set global IN NAK"]
pub type SGNPINNAK_R = crate::BitReader;
#[doc = "Field `SGNPINNAK` writer - Set global IN NAK"]
pub type SGNPINNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGNPINNAK` reader - Clear global IN NAK"]
pub type CGNPINNAK_R = crate::BitReader;
#[doc = "Field `CGNPINNAK` writer - Clear global IN NAK"]
pub type CGNPINNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SGOUTNAK` reader - Set global OUT NAK"]
pub type SGOUTNAK_R = crate::BitReader;
#[doc = "Field `SGOUTNAK` writer - Set global OUT NAK"]
pub type SGOUTNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CGOUTNAK` reader - Clear global OUT NAK"]
pub type CGOUTNAK_R = crate::BitReader;
#[doc = "Field `CGOUTNAK` writer - Clear global OUT NAK"]
pub type CGOUTNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWROPRGDNE` reader - Power-on programming done"]
pub type PWROPRGDNE_R = crate::BitReader;
#[doc = "Field `PWROPRGDNE` writer - Power-on programming done"]
pub type PWROPRGDNE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
            .field("rwkupsig", &format_args!("{}", self.rwkupsig().bit()))
            .field("sftdiscon", &format_args!("{}", self.sftdiscon().bit()))
            .field("gnpinnaksts", &format_args!("{}", self.gnpinnaksts().bit()))
            .field("goutnaksts", &format_args!("{}", self.goutnaksts().bit()))
            .field("tstctl", &format_args!("{}", self.tstctl().bits()))
            .field("sgnpinnak", &format_args!("{}", self.sgnpinnak().bit()))
            .field("cgnpinnak", &format_args!("{}", self.cgnpinnak().bit()))
            .field("sgoutnak", &format_args!("{}", self.sgoutnak().bit()))
            .field("cgoutnak", &format_args!("{}", self.cgoutnak().bit()))
            .field("pwroprgdne", &format_args!("{}", self.pwroprgdne().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    #[must_use]
    pub fn rwkupsig(&mut self) -> RWKUPSIG_W<DCTL_SPEC, 0> {
        RWKUPSIG_W::new(self)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W<DCTL_SPEC, 1> {
        SFTDISCON_W::new(self)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    #[must_use]
    pub fn tstctl(&mut self) -> TSTCTL_W<DCTL_SPEC, 4> {
        TSTCTL_W::new(self)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W<DCTL_SPEC, 7> {
        SGNPINNAK_W::new(self)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W<DCTL_SPEC, 8> {
        CGNPINNAK_W::new(self)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W<DCTL_SPEC, 9> {
        SGOUTNAK_W::new(self)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W<DCTL_SPEC, 10> {
        CGOUTNAK_W::new(self)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    #[must_use]
    pub fn pwroprgdne(&mut self) -> PWROPRGDNE_W<DCTL_SPEC, 11> {
        PWROPRGDNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGHS device control register (OTGHS_DCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
