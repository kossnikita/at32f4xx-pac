#[doc = "Register `DIEPCTL0` reader"]
pub type R = crate::R<DIEPCTL0_SPEC>;
#[doc = "Register `DIEPCTL0` writer"]
pub type W = crate::W<DIEPCTL0_SPEC>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MPS_R = crate::FieldReader;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBACEPT` reader - USB active endpoint"]
pub type USBACEPT_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
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
#[doc = "Field `EPTDIS` reader - Endpoint disable"]
pub type EPTDIS_R = crate::BitReader;
#[doc = "Field `EPTENA` reader - Endpoint enable"]
pub type EPTENA_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbacept(&self) -> USBACEPT_R {
        USBACEPT_R::new(((self.bits >> 15) & 1) != 0)
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
        f.debug_struct("DIEPCTL0")
            .field("mps", &format_args!("{}", self.mps().bits()))
            .field("usbacept", &format_args!("{}", self.usbacept().bit()))
            .field("naksts", &format_args!("{}", self.naksts().bit()))
            .field("eptype", &format_args!("{}", self.eptype().bits()))
            .field("stall", &format_args!("{}", self.stall().bit()))
            .field("txfnum", &format_args!("{}", self.txfnum().bits()))
            .field("eptdis", &format_args!("{}", self.eptdis().bit()))
            .field("eptena", &format_args!("{}", self.eptena().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DIEPCTL0_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DIEPCTL0_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<DIEPCTL0_SPEC> {
        TXFNUM_W::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DIEPCTL0_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DIEPCTL0_SPEC> {
        SNAK_W::new(self, 27)
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
#[doc = "OTGFS device control IN endpoint 0 control register (OTGFS_DIEPCTL0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL0_SPEC;
impl crate::RegisterSpec for DIEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl0::R`](R) reader structure"]
impl crate::Readable for DIEPCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl0::W`](W) writer structure"]
impl crate::Writable for DIEPCTL0_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL0 to value 0"]
impl crate::Resettable for DIEPCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
