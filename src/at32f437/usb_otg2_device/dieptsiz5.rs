#[doc = "Register `DIEPTSIZ5` reader"]
pub type R = crate::R<DIEPTSIZ5_SPEC>;
#[doc = "Register `DIEPTSIZ5` writer"]
pub type W = crate::W<DIEPTSIZ5_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MC` reader - Multi count"]
pub type MC_R = crate::FieldReader;
#[doc = "Field `MC` writer - Multi count"]
pub type MC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ5")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("mc", &self.mc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DIEPTSIZ5_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ5_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<DIEPTSIZ5_SPEC> {
        MC_W::new(self, 29)
    }
}
#[doc = "OTG device IN endpoint-5 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ5_SPEC;
impl crate::RegisterSpec for DIEPTSIZ5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz5::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz5::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ5 to value 0"]
impl crate::Resettable for DIEPTSIZ5_SPEC {
    const RESET_VALUE: u32 = 0;
}
