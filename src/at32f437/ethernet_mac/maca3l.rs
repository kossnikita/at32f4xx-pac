#[doc = "Register `MACA3L` reader"]
pub type R = crate::R<MACA3L_SPEC>;
#[doc = "Register `MACA3L` writer"]
pub type W = crate::W<MACA3L_SPEC>;
#[doc = "Field `MA3L` reader - MAC address3 low"]
pub type MA3L_R = crate::FieldReader<u32>;
#[doc = "Field `MA3L` writer - MAC address3 low"]
pub type MA3L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    pub fn ma3l(&self) -> MA3L_R {
        MA3L_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA3L")
            .field("ma3l", &format_args!("{}", self.ma3l().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACA3L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    #[must_use]
    pub fn ma3l(&mut self) -> MA3L_W<MACA3L_SPEC, 0> {
        MA3L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA3L_SPEC;
impl crate::RegisterSpec for MACA3L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3l::R`](R) reader structure"]
impl crate::Readable for MACA3L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca3l::W`](W) writer structure"]
impl crate::Writable for MACA3L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA3L to value 0xffff_ffff"]
impl crate::Resettable for MACA3L_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
