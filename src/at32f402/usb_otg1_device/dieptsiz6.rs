#[doc = "Register `DIEPTSIZ6` reader"]
pub type R = crate::R<DIEPTSIZ6_SPEC>;
#[doc = "Register `DIEPTSIZ6` writer"]
pub type W = crate::W<DIEPTSIZ6_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XFERSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 19, O, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `MC` reader - Multi count"]
pub type MC_R = crate::FieldReader;
#[doc = "Field `MC` writer - Multi count"]
pub type MC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
        f.debug_struct("DIEPTSIZ6")
            .field("xfersize", &format_args!("{}", self.xfersize().bits()))
            .field("pktcnt", &format_args!("{}", self.pktcnt().bits()))
            .field("mc", &format_args!("{}", self.mc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPTSIZ6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DIEPTSIZ6_SPEC, 0> {
        XFERSIZE_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ6_SPEC, 19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<DIEPTSIZ6_SPEC, 29> {
        MC_W::new(self)
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
#[doc = "OTG device IN endpoint-6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ6_SPEC;
impl crate::RegisterSpec for DIEPTSIZ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz6::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz6::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ6 to value 0"]
impl crate::Resettable for DIEPTSIZ6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
