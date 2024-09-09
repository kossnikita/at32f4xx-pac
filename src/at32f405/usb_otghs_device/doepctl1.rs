#[doc = "Register `DOEPCTL1` reader"]
pub type R = crate::R<DOEPCTL1_SPEC>;
#[doc = "Register `DOEPCTL1` writer"]
pub type W = crate::W<DOEPCTL1_SPEC>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACEPT` reader - USB active endpoint"]
pub type USBACEPT_R = crate::BitReader;
#[doc = "Field `USBACEPT` writer - USB active endpoint"]
pub type USBACEPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPID` reader - Endpoint data PID"]
pub type DPID_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNP` reader - Snoop mode"]
pub type SNP_R = crate::BitReader;
#[doc = "Field `SNP` writer - Snoop mode"]
pub type SNP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDIS` reader - Endpoint disable"]
pub type EPTDIS_R = crate::BitReader;
#[doc = "Field `EPTDIS` writer - Endpoint disable"]
pub type EPTDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTENA` reader - Endpoint enable"]
pub type EPTENA_R = crate::BitReader;
#[doc = "Field `EPTENA` writer - Endpoint enable"]
pub type EPTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbacept(&self) -> USBACEPT_R {
        USBACEPT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn eptdis(&self) -> EPTDIS_R {
        EPTDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn eptena(&self) -> EPTENA_R {
        EPTENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL1")
            .field("mps", &self.mps())
            .field("usbacept", &self.usbacept())
            .field("dpid", &self.dpid())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("eptdis", &self.eptdis())
            .field("eptena", &self.eptena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DOEPCTL1_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbacept(&mut self) -> USBACEPT_W<DOEPCTL1_SPEC> {
        USBACEPT_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DOEPCTL1_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<DOEPCTL1_SPEC> {
        SNP_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEPCTL1_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEPCTL1_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEPCTL1_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    #[must_use]
    pub fn eptdis(&mut self) -> EPTDIS_W<DOEPCTL1_SPEC> {
        EPTDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn eptena(&mut self) -> EPTENA_W<DOEPCTL1_SPEC> {
        EPTENA_W::new(self, 31)
    }
}
#[doc = "OTGHS device OUT endpoint-1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL1_SPEC;
impl crate::RegisterSpec for DOEPCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl1::R`](R) reader structure"]
impl crate::Readable for DOEPCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl1::W`](W) writer structure"]
impl crate::Writable for DOEPCTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL1 to value 0"]
impl crate::Resettable for DOEPCTL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
