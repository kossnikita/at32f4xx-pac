#[doc = "Register `TMI0` reader"]
pub type R = crate::R<TMI0_SPEC>;
#[doc = "Register `TMI0` writer"]
pub type W = crate::W<TMI0_SPEC>;
#[doc = "Field `TMSR` reader - Transmit mailbox send request"]
pub type TMSR_R = crate::BitReader;
#[doc = "Field `TMSR` writer - Transmit mailbox send request"]
pub type TMSR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMFRSEL` reader - Transmit mailbox frame type select"]
pub type TMFRSEL_R = crate::BitReader;
#[doc = "Field `TMFRSEL` writer - Transmit mailbox frame type select"]
pub type TMFRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMIDSEL` reader - Transmit mailbox identifier type select"]
pub type TMIDSEL_R = crate::BitReader;
#[doc = "Field `TMIDSEL` writer - Transmit mailbox identifier type select"]
pub type TMIDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMEID` reader - Ttransmit mailbox extended identifier"]
pub type TMEID_R = crate::FieldReader<u32>;
#[doc = "Field `TMEID` writer - Ttransmit mailbox extended identifier"]
pub type TMEID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
#[doc = "Field `TMSID` reader - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type TMSID_R = crate::FieldReader<u16>;
#[doc = "Field `TMSID` writer - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type TMSID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    pub fn tmsr(&self) -> TMSR_R {
        TMSR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    pub fn tmfrsel(&self) -> TMFRSEL_R {
        TMFRSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    pub fn tmidsel(&self) -> TMIDSEL_R {
        TMIDSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    pub fn tmeid(&self) -> TMEID_R {
        TMEID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    pub fn tmsid(&self) -> TMSID_R {
        TMSID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    #[must_use]
    pub fn tmsr(&mut self) -> TMSR_W<TMI0_SPEC, 0> {
        TMSR_W::new(self)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    #[must_use]
    pub fn tmfrsel(&mut self) -> TMFRSEL_W<TMI0_SPEC, 1> {
        TMFRSEL_W::new(self)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    #[must_use]
    pub fn tmidsel(&mut self) -> TMIDSEL_W<TMI0_SPEC, 2> {
        TMIDSEL_W::new(self)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    #[must_use]
    pub fn tmeid(&mut self) -> TMEID_W<TMI0_SPEC, 3> {
        TMEID_W::new(self)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    #[must_use]
    pub fn tmsid(&mut self) -> TMSID_W<TMI0_SPEC, 21> {
        TMSID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit mailbox 0 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMI0_SPEC;
impl crate::RegisterSpec for TMI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi0::R`](R) reader structure"]
impl crate::Readable for TMI0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmi0::W`](W) writer structure"]
impl crate::Writable for TMI0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMI0 to value 0"]
impl crate::Resettable for TMI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
