#[doc = "Register `DOEPINT6` reader"]
pub type R = crate::R<DOEPINT6_SPEC>;
#[doc = "Register `DOEPINT6` writer"]
pub type W = crate::W<DOEPINT6_SPEC>;
#[doc = "Field `XFERC` reader - Transfer completed interrupt"]
pub type XFERC_R = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed interrupt"]
pub type XFERC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTDISD` reader - Endpoint disabled interrupt"]
pub type EPTDISD_R = crate::BitReader;
#[doc = "Field `EPTDISD` writer - Endpoint disabled interrupt"]
pub type EPTDISD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETUP` reader - SETUP phase done"]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `SETUP` writer - SETUP phase done"]
pub type SETUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTTEPD` reader - OUT token received when endpoint disabled"]
pub type OUTTEPD_R = crate::BitReader;
#[doc = "Field `OUTTEPD` writer - OUT token received when endpoint disabled"]
pub type OUTTEPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2BSTUP_R = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2BSTUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTPKTERR` reader - OUT Packet Error"]
pub type OUTPKTERR_R = crate::BitReader;
#[doc = "Field `OUTPKTERR` writer - OUT Packet Error"]
pub type OUTPKTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BNAINTR` reader - Buffer Not Available Error"]
pub type BNAINTR_R = crate::BitReader;
#[doc = "Field `BNAINTR` writer - Buffer Not Available Error"]
pub type BNAINTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PKTDRPSTS` reader - Packet Drop Status"]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet Drop Status"]
pub type PKTDRPSTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBLEERR` reader - BBLE error"]
pub type BBLEERR_R = crate::BitReader;
#[doc = "Field `BBLEERR` writer - BBLE error"]
pub type BBLEERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKINTPT` reader - NAK interrupt"]
pub type NAKINTPT_R = crate::BitReader;
#[doc = "Field `NAKINTPT` writer - NAK interrupt"]
pub type NAKINTPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYETINTPT` reader - NYET interrupt"]
pub type NYETINTPT_R = crate::BitReader;
#[doc = "Field `NYETINTPT` writer - NYET interrupt"]
pub type NYETINTPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STUPPKTRCVD` reader - Setup Packet Received"]
pub type STUPPKTRCVD_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD` writer - Setup Packet Received"]
pub type STUPPKTRCVD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 8 - OUT Packet Error"]
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer Not Available Error"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BBLE error"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BBLEERR_R {
        BBLEERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nakintpt(&self) -> NAKINTPT_R {
        NAKINTPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyetintpt(&self) -> NYETINTPT_R {
        NYETINTPT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Setup Packet Received"]
    #[inline(always)]
    pub fn stuppktrcvd(&self) -> STUPPKTRCVD_R {
        STUPPKTRCVD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT6")
            .field("xferc", &format_args!("{}", self.xferc().bit()))
            .field("eptdisd", &format_args!("{}", self.eptdisd().bit()))
            .field("setup", &format_args!("{}", self.setup().bit()))
            .field("outtepd", &format_args!("{}", self.outtepd().bit()))
            .field("b2bstup", &format_args!("{}", self.b2bstup().bit()))
            .field("outpkterr", &format_args!("{}", self.outpkterr().bit()))
            .field("bnaintr", &format_args!("{}", self.bnaintr().bit()))
            .field("pktdrpsts", &format_args!("{}", self.pktdrpsts().bit()))
            .field("bbleerr", &format_args!("{}", self.bbleerr().bit()))
            .field("nakintpt", &format_args!("{}", self.nakintpt().bit()))
            .field("nyetintpt", &format_args!("{}", self.nyetintpt().bit()))
            .field("stuppktrcvd", &format_args!("{}", self.stuppktrcvd().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DOEPINT6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XFERC_W<DOEPINT6_SPEC, 0> {
        XFERC_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eptdisd(&mut self) -> EPTDISD_W<DOEPINT6_SPEC, 1> {
        EPTDISD_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<DOEPINT6_SPEC, 3> {
        SETUP_W::new(self)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn outtepd(&mut self) -> OUTTEPD_W<DOEPINT6_SPEC, 4> {
        OUTTEPD_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<DOEPINT6_SPEC, 6> {
        B2BSTUP_W::new(self)
    }
    #[doc = "Bit 8 - OUT Packet Error"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W<DOEPINT6_SPEC, 8> {
        OUTPKTERR_W::new(self)
    }
    #[doc = "Bit 9 - Buffer Not Available Error"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<DOEPINT6_SPEC, 9> {
        BNAINTR_W::new(self)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DOEPINT6_SPEC, 11> {
        PKTDRPSTS_W::new(self)
    }
    #[doc = "Bit 12 - BBLE error"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr(&mut self) -> BBLEERR_W<DOEPINT6_SPEC, 12> {
        BBLEERR_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nakintpt(&mut self) -> NAKINTPT_W<DOEPINT6_SPEC, 13> {
        NAKINTPT_W::new(self)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyetintpt(&mut self) -> NYETINTPT_W<DOEPINT6_SPEC, 14> {
        NYETINTPT_W::new(self)
    }
    #[doc = "Bit 15 - Setup Packet Received"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd(&mut self) -> STUPPKTRCVD_W<DOEPINT6_SPEC, 15> {
        STUPPKTRCVD_W::new(self)
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
#[doc = "OTGHS device OUT endpoint-6 interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT6_SPEC;
impl crate::RegisterSpec for DOEPINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint6::R`](R) reader structure"]
impl crate::Readable for DOEPINT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint6::W`](W) writer structure"]
impl crate::Writable for DOEPINT6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPINT6 to value 0x80"]
impl crate::Resettable for DOEPINT6_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
