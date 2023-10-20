#[doc = "Register `DOEPTSIZ0` reader"]
pub type R = crate::R<DOEPTSIZ0_SPEC>;
#[doc = "Register `DOEPTSIZ0` writer"]
pub type W = crate::W<DOEPTSIZ0_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XFERSIZE_R = crate::FieldReader;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::BitReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPCNT` reader - SETUP packet count"]
pub type SETUPCNT_R = crate::FieldReader;
#[doc = "Field `SETUPCNT` writer - SETUP packet count"]
pub type SETUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    pub fn setupcnt(&self) -> SETUPCNT_R {
        SETUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ0")
            .field("xfersize", &format_args!("{}", self.xfersize().bits()))
            .field("pktcnt", &format_args!("{}", self.pktcnt().bit()))
            .field("setupcnt", &format_args!("{}", self.setupcnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DOEPTSIZ0_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ0_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    #[must_use]
    pub fn setupcnt(&mut self) -> SETUPCNT_W<DOEPTSIZ0_SPEC> {
        SETUPCNT_W::new(self, 29)
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
#[doc = "OTGFS device OUT endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ0_SPEC;
impl crate::RegisterSpec for DOEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz0::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz0::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for DOEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
