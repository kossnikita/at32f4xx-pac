#[doc = "Register `DOEPTSIZ3` reader"]
pub type R = crate::R<DOEPTSIZ3_SPEC>;
#[doc = "Register `DOEPTSIZ3` writer"]
pub type W = crate::W<DOEPTSIZ3_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPID` reader - Received data PID"]
pub type RXDPID_R = crate::FieldReader;
#[doc = "Field `RXDPID` writer - Received data PID"]
pub type RXDPID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Received data PID"]
    #[inline(always)]
    pub fn rxdpid(&self) -> RXDPID_R {
        RXDPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ3")
            .field("xfersize", &format_args!("{}", self.xfersize().bits()))
            .field("pktcnt", &format_args!("{}", self.pktcnt().bits()))
            .field("rxdpid", &format_args!("{}", self.rxdpid().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DOEPTSIZ3_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ3_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Received data PID"]
    #[inline(always)]
    #[must_use]
    pub fn rxdpid(&mut self) -> RXDPID_W<DOEPTSIZ3_SPEC> {
        RXDPID_W::new(self, 29)
    }
}
#[doc = "OTGHS device OUT endpoint-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ3_SPEC;
impl crate::RegisterSpec for DOEPTSIZ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz3::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz3::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ3 to value 0"]
impl crate::Resettable for DOEPTSIZ3_SPEC {
    const RESET_VALUE: u32 = 0;
}
