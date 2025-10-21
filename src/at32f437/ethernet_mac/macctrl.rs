#[doc = "Register `MACCTRL` reader"]
pub type R = crate::R<MACCTRL_SPEC>;
#[doc = "Register `MACCTRL` writer"]
pub type W = crate::W<MACCTRL_SPEC>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral check"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Deferral check"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACS` reader - Automatic pad/CRC stripping"]
pub type ACS_R = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic pad/CRC stripping"]
pub type ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR` reader - Disable retry"]
pub type DR_R = crate::BitReader;
#[doc = "Field `DR` writer - Disable retry"]
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - IPv4 checksum offload"]
pub type IPC_R = crate::BitReader;
#[doc = "Field `IPC` writer - IPv4 checksum offload"]
pub type IPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex mode"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex mode"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRO` reader - Disable receive own"]
pub type DRO_R = crate::BitReader;
#[doc = "Field `DRO` writer - Disable receive own"]
pub type DRO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Fast EMAC speed"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Fast EMAC speed"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS` reader - Disable carrier sense"]
pub type DCS_R = crate::BitReader;
#[doc = "Field `DCS` writer - Disable carrier sense"]
pub type DCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JD` reader - Jabber disable"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber disable"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACCTRL")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("acs", &self.acs())
            .field("dr", &self.dr())
            .field("ipc", &self.ipc())
            .field("dm", &self.dm())
            .field("lm", &self.lm())
            .field("dro", &self.dro())
            .field("fes", &self.fes())
            .field("dcs", &self.dcs())
            .field("ifg", &self.ifg())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, MACCTRL_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, MACCTRL_SPEC> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, MACCTRL_SPEC> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, MACCTRL_SPEC> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W<'_, MACCTRL_SPEC> {
        ACS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Disable retry"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, MACCTRL_SPEC> {
        DR_W::new(self, 9)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<'_, MACCTRL_SPEC> {
        IPC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<'_, MACCTRL_SPEC> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<'_, MACCTRL_SPEC> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Disable receive own"]
    #[inline(always)]
    pub fn dro(&mut self) -> DRO_W<'_, MACCTRL_SPEC> {
        DRO_W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast EMAC speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<'_, MACCTRL_SPEC> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - Disable carrier sense"]
    #[inline(always)]
    pub fn dcs(&mut self) -> DCS_W<'_, MACCTRL_SPEC> {
        DCS_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W<'_, MACCTRL_SPEC> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<'_, MACCTRL_SPEC> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<'_, MACCTRL_SPEC> {
        WD_W::new(self, 23)
    }
}
#[doc = "Ethernet MAC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`macctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACCTRL_SPEC;
impl crate::RegisterSpec for MACCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macctrl::R`](R) reader structure"]
impl crate::Readable for MACCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macctrl::W`](W) writer structure"]
impl crate::Writable for MACCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACCTRL to value 0x8000"]
impl crate::Resettable for MACCTRL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
