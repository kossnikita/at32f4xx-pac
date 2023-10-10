#[doc = "Register `MISC1` reader"]
pub type R = crate::R<MISC1_SPEC>;
#[doc = "Register `MISC1` writer"]
pub type W = crate::W<MISC1_SPEC>;
#[doc = "Field `HICKCAL_KEY` reader - HICKCAL write key value"]
pub type HICKCAL_KEY_R = crate::FieldReader;
#[doc = "Field `HICKCAL_KEY` writer - HICKCAL write key value"]
pub type HICKCAL_KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLKOUT_SEL3` reader - Clock output bit3"]
pub type CLKOUT_SEL3_R = crate::BitReader;
#[doc = "Field `CLKOUT_SEL3` writer - Clock output bit3"]
pub type CLKOUT_SEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBBUFS` reader - USB buffer size selection"]
pub type USBBUFS_R = crate::BitReader;
#[doc = "Field `USBBUFS` writer - USB buffer size selection"]
pub type USBBUFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HICKDIV` reader - HICK 6 divider selection"]
pub type HICKDIV_R = crate::BitReader;
#[doc = "Field `HICKDIV` writer - HICK 6 divider selection"]
pub type HICKDIV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKOUTDIV` reader - Clock output division"]
pub type CLKOUTDIV_R = crate::FieldReader;
#[doc = "Field `CLKOUTDIV` writer - Clock output division"]
pub type CLKOUTDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    pub fn hickcal_key(&self) -> HICKCAL_KEY_R {
        HICKCAL_KEY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Clock output bit3"]
    #[inline(always)]
    pub fn clkout_sel3(&self) -> CLKOUT_SEL3_R {
        CLKOUT_SEL3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - USB buffer size selection"]
    #[inline(always)]
    pub fn usbbufs(&self) -> USBBUFS_R {
        USBBUFS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HICK 6 divider selection"]
    #[inline(always)]
    pub fn hickdiv(&self) -> HICKDIV_R {
        HICKDIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Clock output division"]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> CLKOUTDIV_R {
        CLKOUTDIV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC1")
            .field(
                "hickcal_key",
                &format_args!("{}", self.hickcal_key().bits()),
            )
            .field("clkout_sel3", &format_args!("{}", self.clkout_sel3().bit()))
            .field("usbbufs", &format_args!("{}", self.usbbufs().bit()))
            .field("hickdiv", &format_args!("{}", self.hickdiv().bit()))
            .field("clkoutdiv", &format_args!("{}", self.clkoutdiv().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MISC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    #[must_use]
    pub fn hickcal_key(&mut self) -> HICKCAL_KEY_W<MISC1_SPEC, 0> {
        HICKCAL_KEY_W::new(self)
    }
    #[doc = "Bit 16 - Clock output bit3"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_sel3(&mut self) -> CLKOUT_SEL3_W<MISC1_SPEC, 16> {
        CLKOUT_SEL3_W::new(self)
    }
    #[doc = "Bit 24 - USB buffer size selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbbufs(&mut self) -> USBBUFS_W<MISC1_SPEC, 24> {
        USBBUFS_W::new(self)
    }
    #[doc = "Bit 25 - HICK 6 divider selection"]
    #[inline(always)]
    #[must_use]
    pub fn hickdiv(&mut self) -> HICKDIV_W<MISC1_SPEC, 25> {
        HICKDIV_W::new(self)
    }
    #[doc = "Bits 28:31 - Clock output division"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutdiv(&mut self) -> CLKOUTDIV_W<MISC1_SPEC, 28> {
        CLKOUTDIV_W::new(self)
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
#[doc = "Miscellaneous register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC1_SPEC;
impl crate::RegisterSpec for MISC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc1::R`](R) reader structure"]
impl crate::Readable for MISC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc1::W`](W) writer structure"]
impl crate::Writable for MISC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC1 to value 0"]
impl crate::Resettable for MISC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
