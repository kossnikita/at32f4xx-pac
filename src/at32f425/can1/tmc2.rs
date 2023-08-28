#[doc = "Register `TMC2` reader"]
pub type R = crate::R<TMC2_SPEC>;
#[doc = "Register `TMC2` writer"]
pub type W = crate::W<TMC2_SPEC>;
#[doc = "Field `TMDTBL` reader - Transmit mailbox data byte length"]
pub type TMDTBL_R = crate::FieldReader;
#[doc = "Field `TMDTBL` writer - Transmit mailbox data byte length"]
pub type TMDTBL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TMTSTEN` reader - Transmit mailbox time stamp transmit enable"]
pub type TMTSTEN_R = crate::BitReader;
#[doc = "Field `TMTSTEN` writer - Transmit mailbox time stamp transmit enable"]
pub type TMTSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMTS` reader - Transmit mailbox time stamp"]
pub type TMTS_R = crate::FieldReader<u16>;
#[doc = "Field `TMTS` writer - Transmit mailbox time stamp"]
pub type TMTS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    pub fn tmdtbl(&self) -> TMDTBL_R {
        TMDTBL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    pub fn tmtsten(&self) -> TMTSTEN_R {
        TMTSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    pub fn tmts(&self) -> TMTS_R {
        TMTS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    #[must_use]
    pub fn tmdtbl(&mut self) -> TMDTBL_W<TMC2_SPEC, 0> {
        TMDTBL_W::new(self)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmtsten(&mut self) -> TMTSTEN_W<TMC2_SPEC, 8> {
        TMTSTEN_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn tmts(&mut self) -> TMTS_W<TMC2_SPEC, 16> {
        TMTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit mailbox 2 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMC2_SPEC;
impl crate::RegisterSpec for TMC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmc2::R`](R) reader structure"]
impl crate::Readable for TMC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmc2::W`](W) writer structure"]
impl crate::Writable for TMC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMC2 to value 0"]
impl crate::Resettable for TMC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
