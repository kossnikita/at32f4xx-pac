#[doc = "Register `MACCTRL` reader"]
pub type R = crate::R<MACCTRL_SPEC>;
#[doc = "Register `MACCTRL` writer"]
pub type W = crate::W<MACCTRL_SPEC>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DC` reader - Deferral check"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Deferral check"]
pub type DC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ACS` reader - Automatic pad/CRC stripping"]
pub type ACS_R = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic pad/CRC stripping"]
pub type ACS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DR` reader - Disable retry"]
pub type DR_R = crate::BitReader;
#[doc = "Field `DR` writer - Disable retry"]
pub type DR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IPC` reader - IPv4 checksum offload"]
pub type IPC_R = crate::BitReader;
#[doc = "Field `IPC` writer - IPv4 checksum offload"]
pub type IPC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DM` reader - Duplex mode"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex mode"]
pub type DM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LM` reader - Loopback mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback mode"]
pub type LM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRO` reader - Disable receive own"]
pub type DRO_R = crate::BitReader;
#[doc = "Field `DRO` writer - Disable receive own"]
pub type DRO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FES` reader - Fast EMAC speed"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Fast EMAC speed"]
pub type FES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCS` reader - Disable carrier sense"]
pub type DCS_R = crate::BitReader;
#[doc = "Field `DCS` writer - Disable carrier sense"]
pub type DCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `JD` reader - Jabber disable"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber disable"]
pub type JD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable retry"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable receive own"]
    #[inline(always)]
    pub fn dro(&self) -> DRO_R {
        DRO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast EMAC speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable carrier sense"]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<MACCTRL_SPEC, 2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<MACCTRL_SPEC, 3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<MACCTRL_SPEC, 4> {
        DC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<MACCTRL_SPEC, 5> {
        BL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> ACS_W<MACCTRL_SPEC, 7> {
        ACS_W::new(self)
    }
    #[doc = "Bit 9 - Disable retry"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<MACCTRL_SPEC, 9> {
        DR_W::new(self)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IPC_W<MACCTRL_SPEC, 10> {
        IPC_W::new(self)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<MACCTRL_SPEC, 11> {
        DM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<MACCTRL_SPEC, 12> {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - Disable receive own"]
    #[inline(always)]
    #[must_use]
    pub fn dro(&mut self) -> DRO_W<MACCTRL_SPEC, 13> {
        DRO_W::new(self)
    }
    #[doc = "Bit 14 - Fast EMAC speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<MACCTRL_SPEC, 14> {
        FES_W::new(self)
    }
    #[doc = "Bit 16 - Disable carrier sense"]
    #[inline(always)]
    #[must_use]
    pub fn dcs(&mut self) -> DCS_W<MACCTRL_SPEC, 16> {
        DCS_W::new(self)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<MACCTRL_SPEC, 17> {
        IFG_W::new(self)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<MACCTRL_SPEC, 22> {
        JD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<MACCTRL_SPEC, 23> {
        WD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACCTRL_SPEC;
impl crate::RegisterSpec for MACCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macctrl::R`](R) reader structure"]
impl crate::Readable for MACCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macctrl::W`](W) writer structure"]
impl crate::Writable for MACCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACCTRL to value 0x8000"]
impl crate::Resettable for MACCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
