#[doc = "Register `DOEPTSIZ6` reader"]
pub type R = crate::R<DOEPTSIZ6_SPEC>;
#[doc = "Register `DOEPTSIZ6` writer"]
pub type W = crate::W<DOEPTSIZ6_SPEC>;
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
        f.debug_struct("DOEPTSIZ6")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpid", &self.rxdpid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W<'_, DOEPTSIZ6_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DOEPTSIZ6_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Received data PID"]
    #[inline(always)]
    pub fn rxdpid(&mut self) -> RXDPID_W<'_, DOEPTSIZ6_SPEC> {
        RXDPID_W::new(self, 29)
    }
}
#[doc = "OTGHS device OUT endpoint-6 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ6_SPEC;
impl crate::RegisterSpec for DOEPTSIZ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz6::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz6::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPTSIZ6 to value 0"]
impl crate::Resettable for DOEPTSIZ6_SPEC {}
