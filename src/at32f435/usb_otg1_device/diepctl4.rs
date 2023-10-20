#[doc = "Register `DIEPCTL4` reader"]
pub type R = crate::R<DIEPCTL4_SPEC>;
#[doc = "Register `DIEPCTL4` writer"]
pub type W = crate::W<DIEPCTL4_SPEC>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACEPT` reader - USB active endpoint"]
pub type USBACEPT_R = crate::BitReader;
#[doc = "Field `USBACEPT` writer - USB active endpoint"]
pub type USBACEPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPID` reader - Endpoint Data PID"]
pub type DPID_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PID` writer - Set DATA0 PID"]
pub type SETD0PID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PID` writer - Set DATA1 PID"]
pub type SETD1PID_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - Endpoint Data PID"]
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
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
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
        f.debug_struct("DIEPCTL4")
            .field("mps", &format_args!("{}", self.mps().bits()))
            .field("usbacept", &format_args!("{}", self.usbacept().bit()))
            .field("dpid", &format_args!("{}", self.dpid().bit()))
            .field("naksts", &format_args!("{}", self.naksts().bit()))
            .field("eptype", &format_args!("{}", self.eptype().bits()))
            .field("stall", &format_args!("{}", self.stall().bit()))
            .field("txfnum", &format_args!("{}", self.txfnum().bits()))
            .field("eptdis", &format_args!("{}", self.eptdis().bit()))
            .field("eptena", &format_args!("{}", self.eptena().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DIEPCTL4_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbacept(&mut self) -> USBACEPT_W<DIEPCTL4_SPEC> {
        USBACEPT_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DIEPCTL4_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DIEPCTL4_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<DIEPCTL4_SPEC> {
        TXFNUM_W::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DIEPCTL4_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DIEPCTL4_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID"]
    #[inline(always)]
    #[must_use]
    pub fn setd0pid(&mut self) -> SETD0PID_W<DIEPCTL4_SPEC> {
        SETD0PID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set DATA1 PID"]
    #[inline(always)]
    #[must_use]
    pub fn setd1pid(&mut self) -> SETD1PID_W<DIEPCTL4_SPEC> {
        SETD1PID_W::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    #[must_use]
    pub fn eptdis(&mut self) -> EPTDIS_W<DIEPCTL4_SPEC> {
        EPTDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn eptena(&mut self) -> EPTENA_W<DIEPCTL4_SPEC> {
        EPTENA_W::new(self, 31)
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
#[doc = "OTGFS device IN endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL4_SPEC;
impl crate::RegisterSpec for DIEPCTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl4::R`](R) reader structure"]
impl crate::Readable for DIEPCTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl4::W`](W) writer structure"]
impl crate::Writable for DIEPCTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL4 to value 0"]
impl crate::Resettable for DIEPCTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
